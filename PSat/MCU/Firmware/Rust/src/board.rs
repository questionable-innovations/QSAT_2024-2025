// An example board support package for a stack using the MCU and Beacon boards.

#![allow(dead_code)]
use msp430::critical_section;
use msp430fr2355::{E_USCI_A1, E_USCI_B0, E_USCI_B1, P2, P4, P5};
use msp430fr2x5x_hal::{
    adc::{Adc, AdcConfig, ClockDivider, Predivider, Resolution, SampleTime, SamplingRate}, 
    clock::{Clock, ClockConfig, DcoclkFreqSel, MclkDiv, SmclkDiv}, 
    delay::Delay, fram::Fram, 
    gpio::{Alternate3, Batch, Floating, Input, Output, Pin, Pin0, Pin1, Pin2, Pin3, Pin4, PinNum, PortNum}, 
    i2c::{GlitchFilter, I2CBusConfig, I2cBus}, pmm::Pmm, 
    serial::{BitCount, BitOrder, Loopback, Parity, Rx, SerialConfig, StopBits, Tx}, 
    spi::{SpiBus, SpiBusConfig}, 
    watchdog::Wdt
};
use embedded_hal::digital::v2::OutputPin;
use crate::println;

/// Top-level object representing the board.
pub struct Board {
    pub delay: Delay,
    pub red_led:   RedLED,
    pub green_led: GreenLED,
    pub blue_led:  BlueLED,
    pub gps_uart:   (Tx<E_USCI_A1>, Rx<E_USCI_A1>),
    pub i2c: I2cBus<E_USCI_B0>,
    pub spi: SpiBus<E_USCI_B1>,
    pub adc: Adc,
    pub lora_cs: LoraChipSel, 
    pub lora_irq: LoraIrq,
    pub gps_en: GpsEn,
    pub half_vbat: HalfVbat,
}
// This is where you should implement functionality, like sending SPI packets to specific devices, etc. 
impl Board {
    pub fn battery_voltage_mv(&mut self) -> u16 {
        self.adc.read_voltage_mv(&mut self.half_vbat, 3300).unwrap() * 2
    }
}

/// Call this function ONCE at the beginning of your program.
/// Printing won't work until this function is called.
pub fn configure() -> Board {
    // Take hardware registers and disable watchdog
    let regs = msp430fr2355::Peripherals::take().unwrap();
    let _wdt = Wdt::constrain(regs.WDT_A);

    // Configure GPIO
    let pmm = Pmm::new(regs.PMM);
    let port1 = Batch::new(regs.P1).split(&pmm);
    let port2 = Batch::new(regs.P2).split(&pmm);
    let port4 = Batch::new(regs.P4).split(&pmm);
    let port5 = Batch::new(regs.P5).split(&pmm);

    let half_vbat = port5.pin0.to_alternate3(); // ADC pin. Connected to Vbat/2.

    // LEDs
    let mut red_led = RedLED::new(port2.pin0.to_output());
    let mut blue_led = BlueLED::new(port2.pin1.to_output());
    let mut green_led = GreenLED::new(port2.pin2.to_output());
    red_led.turn_off();
    green_led.turn_off();
    blue_led.turn_off();

    // Configure clocks to get accurate delay timing, and used by other peripherals
    let mut fram = Fram::new(regs.FRCTL);
    let (smclk, _aclk, delay) = ClockConfig::new(regs.CS)
        .mclk_dcoclk(DcoclkFreqSel::_8MHz, MclkDiv::_1)
        .smclk_on(SmclkDiv::_1)
        .freeze(&mut fram);

    // LoRa radio
    let mut lora_reset = port5.pin4.to_output(); // Not actually connected...
    let mut lora_cs = port4.pin4.to_output();
    lora_reset.set_high().ok();
    lora_cs.set_high().ok();
    let lora_irq = port5.pin3;

    // SPI, used by the LoRa radio
    let miso_pin = port4.pin7.to_alternate1();
    let mosi_pin = port4.pin6.to_alternate1();
    let sclk_pin = port4.pin5.to_alternate1();
    let spi = SpiBusConfig::new(regs.E_USCI_B1, embedded_hal::spi::MODE_0, true)
        .use_smclk(&smclk, 32)
        .configure_with_software_cs(miso_pin, mosi_pin, sclk_pin);

    // GPS UART
    let gps_tx_pin = port4.pin3.to_alternate1();
    let gps_rx_pin = port4.pin2.to_alternate1();
    let mut gps_en = port4.pin1.to_output(); // active low
    gps_en.set_low().ok();
    let (gps_tx, gps_rx) = SerialConfig::new(regs.E_USCI_A1, 
        BitOrder::LsbFirst, 
        BitCount::EightBits, 
        StopBits::OneStopBit, 
        Parity::NoParity, 
        Loopback::NoLoop, 
        9600)
        .use_smclk(&smclk)
        .split(gps_tx_pin, gps_rx_pin);

    // Spare UART, useful for debug printing to a computer
    let debug_tx_pin = port1.pin7.to_alternate1();
    let debug_uart = SerialConfig::new(regs.E_USCI_A0, 
        BitOrder::LsbFirst, 
        BitCount::EightBits, 
        StopBits::OneStopBit, 
        Parity::NoParity, 
        Loopback::NoLoop, 
        115200)
        .use_smclk(&smclk)
        .tx_only(debug_tx_pin);

    // Wrap the UART in a newtype that can print arbitrary strings, utilising core::fmt::Write
    let debug_uart = crate::serial::PrintableSerial(debug_uart);

    // Move the UART into a global so it can be called anywhere, including in panics.
    critical_section::with(|cs| {
        crate::serial::SERIAL.replace(cs, Some(debug_uart))
    });
    println!("Serial init"); // Like this!

    // I2C
    let i2c_sda_pin = port1.pin2.to_alternate1();
    let i2c_scl_pin = port1.pin3.to_alternate1();
    const I2C_FREQ: u32 = 100_000; //Hz
    let clk_div = (smclk.freq() / I2C_FREQ) as u16;
    let i2c = I2CBusConfig::new(
        regs.E_USCI_B0, 
        GlitchFilter::Max50ns)
        .use_smclk(&smclk, clk_div)
        .configure(i2c_scl_pin, i2c_sda_pin);

    // ADC
    let adc = AdcConfig::new(
        ClockDivider::_1, 
        Predivider::_1, 
        Resolution::_10BIT, 
        SamplingRate::_50KSPS, 
        SampleTime::_8)
        .use_modclk()
        .configure(regs.ADC);

    Board {delay, red_led, green_led, blue_led, gps_uart: (gps_tx, gps_rx), spi, lora_cs, i2c, adc, gps_en, lora_irq, half_vbat}
}
/// Pin 2.0
pub type RedLED    = Led<P2, Pin0>;

/// Pin 2.2
pub type GreenLED  = Led<P2, Pin2>;

// Depending on whether you soldered the RGB LED or the individual LEDs:
/// Pin 2.1
pub type BlueLED   = Led<P2, Pin1>;
///// Pin 2.1
//type YellowLED = Pin<P2, Pin1, Output>;

/// Pin 4.4
pub type LoraChipSel  = Pin<P4, Pin4, Output>;

/// Pin 4.1
pub type GpsEn  = Pin<P4, Pin1, Output>;

/// Pin 5.3
pub type LoraIrq = Pin<P5, Pin3, Input<Floating>>;

/// Pin 5.0. Connected to Vbat/2. Sense with the ADC.
pub type HalfVbat = Pin<P5, Pin0, Alternate3<Input<Floating>>>;

/// The RGB LEDs are active low, which can be a little confusing. A helper struct to reduce cognitive load.
pub struct Led<PORT: PortNum, PIN: PinNum> {
    pub pin: Pin<PORT, PIN, Output>,
}
impl<PORT: PortNum, PIN: PinNum> Led<PORT, PIN> {
    pub fn new(pin: Pin<PORT, PIN, Output>) -> Self {
        Self {pin}
    }
    pub fn turn_on(&mut self) {
        self.pin.set_low().ok();
    }
    pub fn turn_off(&mut self) {
        self.pin.set_high().ok();
    }
    pub fn toggle(&mut self) {
        use embedded_hal::digital::v2::ToggleableOutputPin;
        self.pin.toggle().ok();
    }
}
