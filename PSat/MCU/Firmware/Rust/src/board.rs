// An example board support package for a stack using the MCU and Beacon boards.

#![allow(dead_code)]
use msp430fr2x5x_hal::{
    adc::{Adc, AdcConfig, ClockDivider, Predivider, Resolution, SampleTime, SamplingRate}, 
    clock::{Clock, ClockConfig, DcoclkFreqSel, MclkDiv, SmclkDiv}, delay::Delay, fram::Fram, 
    gpio::{Batch, Floating, Input, Pin, Pin0, Pin1, Pin2, Pin3, Pin4, Pin5, Pin6, Pin7, P1, P2, P3, P4, P5, P6}, 
    i2c::{GlitchFilter, I2CBusConfig, I2cBus}, 
    pac::{E_USCI_B0, PMM, TB0}, pmm::Pmm, pwm::TimerConfig, spi::SpiBusConfig, timer::{Timer, TimerParts3}, watchdog::Wdt
};
use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin};
use crate::{gps::Gps, lora::Radio, pin_mappings::*, println};

/// Top-level object representing the board.
/// 
/// Not all peripherals are configured. If you need more, add their configuration code to ::configure().
pub struct Board {
    pub delay: Delay,
    pub gps: Gps,
    // pub i2c: I2cBus<E_USCI_B0>,
    pub adc: Adc,
    pub radio: Radio,
    pub gpio: Gpio,
    pub timer_b0: Timer<TB0>,
}
// This is where you should implement top-level functionality. 
impl Board {
    pub fn battery_voltage_mv(&mut self) -> u16 {
        self.adc.read_voltage_mv(&mut self.gpio.half_vbat, 3300).unwrap() * 2
    }
}

/// Call this function ONCE at the beginning of your program.
/// Printing won't work until this function is called.
pub fn configure() -> Board {
    // Take hardware registers and disable watchdog
    let regs = msp430fr2355::Peripherals::take().unwrap();
    let _wdt = Wdt::constrain(regs.WDT_A);

    // Configure GPIO. `used` are pins consumed by other peripherals.
    let (gpio, used) = Gpio::configure(regs.P1, regs.P2, regs.P3, regs.P4, regs.P5, regs.P6, regs.PMM);
    
    // Configure clocks to get accurate delay timing, and used by other peripherals
    let mut fram = Fram::new(regs.FRCTL);
    let (smclk, aclk, delay) = ClockConfig::new(regs.CS)
        .mclk_dcoclk(DcoclkFreqSel::_8MHz, MclkDiv::_1)
        .smclk_on(SmclkDiv::_1)
        .aclk_refoclk() // 32768 Hz
        .freeze(&mut fram);

    // Spare UART, useful for debug printing to a computer
    crate::serial::configure_debug_serial(used.debug_tx_pin, &smclk, regs.E_USCI_A0);
    println!("Serial init"); // Like this!
    
    // SPI, used by the LoRa radio
    let spi = SpiBusConfig::new(regs.E_USCI_B0, embedded_hal::spi::MODE_0, true)
        .use_smclk(&smclk, 32)
        .configure_with_software_cs(used.miso, used.mosi, used.sclk);
    
    // LoRa radio
    let radio = crate::lora::new(spi, used.lora_cs, used.lora_reset, delay);

    // GPS
    let gps = crate::gps::Gps::new(regs.E_USCI_A1, &smclk, used.gps_tx_pin, used.gps_rx_pin);

    // Timer
    let timer_parts = TimerParts3::new(regs.TB0, TimerConfig::aclk(&aclk));
    let timer_b0 = timer_parts.timer;

    // I2C
    // const I2C_FREQ: u32 = 100_000; //Hz
    // let clk_div = (smclk.freq() / I2C_FREQ) as u16;
    // let i2c = I2CBusConfig::new(
    //     regs.E_USCI_B0, 
    //     GlitchFilter::Max50ns)
    //     .use_smclk(&smclk, clk_div)
    //     .configure(used.i2c_scl_pin, used.i2c_sda_pin);

    // ADC
    let adc = AdcConfig::new(
        ClockDivider::_1, 
        Predivider::_1, 
        Resolution::_10BIT, 
        SamplingRate::_50KSPS, 
        SampleTime::_8)
        .use_modclk()
        .configure(regs.ADC);

    Board {delay, gps, radio, adc, gpio, timer_b0}
}

/// The RGB LEDs are active low, which can be a little confusing. A helper struct to reduce cognitive load.
pub struct Led<PIN: OutputPin+ToggleableOutputPin>(PIN);
impl<PIN: OutputPin+ToggleableOutputPin> Led<PIN> {
    pub fn new(pin: PIN) -> Self {
        Self(pin)
    }
    pub fn turn_on(&mut self) {
        self.0.set_low().ok();
    }
    pub fn turn_off(&mut self) {
        self.0.set_high().ok();
    }
    pub fn toggle(&mut self) {
        self.0.toggle().ok();
    }
}
pub type RedLed     = Led<RedLedPin>;
pub type BlueLed    = Led<BlueLedPin>;
pub type GreenLed   = Led<GreenLedPin>;

pub struct Gpio {
    // LEDs
    pub red_led:   RedLed,
    pub green_led: GreenLed,
    pub blue_led:  BlueLed,
    
    pub lora_irq:       LoraIrqPin,
    pub gps_en:         GpsEnPin,
    pub half_vbat:      HalfVbatPin,

    // PSU monitoring and control pins
    // pub power_good_1v8: PowerGood1v8Pin,
    // pub power_good_3v3: PowerGood3v3Pin,
    // pub enable_1v8:     Enable1v8Pin,
    pub enable_5v:      Enable5vPin,

    // Unused UCA0 pins
    // pub pin1_4: Pin<P1, Pin4, Input<Floating>>,
    // pub pin1_5: Pin<P1, Pin5, Input<Floating>>,
    // pub pin1_6: Pin<P1, Pin6, Input<Floating>>,

    // Unused UCA1 pins
    // pub pin4_0: Pin<P4, Pin0, Input<Floating>>,

    // Unused UCB0 pins
    // pub pin1_0: Pin<P1, Pin0, Input<Floating>>,
    // pub pin1_1: Pin<P1, Pin1, Input<Floating>>,
    
    // Unused ADC pins
    // pub pin5_1: Pin<P5, Pin1, Input<Floating>>,

    // Unused GPIO pins
    // pub pin2_3: Pin<P2, Pin3, Input<Floating>>,
    // pub pin2_4: Pin<P2, Pin4, Input<Floating>>,
    // pub pin2_5: Pin<P2, Pin5, Input<Floating>>,
    // pub pin2_6: Pin<P2, Pin6, Input<Floating>>,
    // pub pin2_7: Pin<P2, Pin7, Input<Floating>>,

    // pub pin3_4: Pin<P3, Pin4, Input<Floating>>,
    // pub pin3_5: Pin<P3, Pin5, Input<Floating>>,
    // pub pin3_6: Pin<P3, Pin6, Input<Floating>>,
    // pub pin3_7: Pin<P3, Pin7, Input<Floating>>,

    // pub pin6_0: Pin<P6, Pin0, Input<Floating>>,
    // pub pin6_1: Pin<P6, Pin1, Input<Floating>>,
    // pub pin6_2: Pin<P6, Pin2, Input<Floating>>,
    // pub pin6_3: Pin<P6, Pin3, Input<Floating>>,
    // pub pin6_4: Pin<P6, Pin4, Input<Floating>>,
    // pub pin6_5: Pin<P6, Pin5, Input<Floating>>,
    // pub pin6_6: Pin<P6, Pin6, Input<Floating>>,
    // pub pin6_7: Pin<P6, Pin7, Input<Floating>>,
}
impl Gpio {
    fn configure(p1: P1, p2: P2, p3: P3, p4 :P4, p5: P5, p6: P6, pmm: PMM) -> (Self, ConsumedPins) {
        // Configure GPIO
        let pmm = Pmm::new(pmm);
        let port1 = Batch::new(p1).split(&pmm);
        let port2 = Batch::new(p2).split(&pmm);
        let port3 = Batch::new(p3).split(&pmm);
        let port4 = Batch::new(p4).split(&pmm);
        let port5 = Batch::new(p5).split(&pmm);
        let port6 = Batch::new(p6).split(&pmm);

        let half_vbat = port5.pin0.to_alternate3(); // ADC pin. Connected to Vbat/2.

        // LEDs
        let mut red_led = RedLed::new(port3.pin3.to_output());
        let mut blue_led = BlueLed::new(port3.pin2.to_output());
        let mut green_led = GreenLed::new(port3.pin1.to_output());
        red_led.turn_off();
        green_led.turn_off();
        blue_led.turn_off();

        let miso = port1.pin3.to_alternate1();
        let mosi = port1.pin2.to_alternate1();
        let sclk = port1.pin1.to_alternate1();
        
        let mut lora_reset = port2.pin7.to_output(); // Not actually connected...
        let mut lora_cs = port6.pin5.to_output();
        lora_reset.set_high().ok();
        lora_cs.set_high().ok();
        let lora_irq = port2.pin6;

        let gps_tx_pin = port4.pin3.to_alternate1();
        let gps_rx_pin = port4.pin2.to_alternate1();
        let mut gps_en = port2.pin3.to_output(); // active low
        gps_en.set_low().ok();

        let debug_tx_pin = port1.pin7.to_alternate1();

        // let i2c_sda_pin = port1.pin2.to_alternate1();
        // let i2c_scl_pin = port1.pin3.to_alternate1();

        // Pins consumed by other perihperals
        let used = ConsumedPins {mosi, miso, sclk, lora_cs, lora_reset, gps_rx_pin, gps_tx_pin, debug_tx_pin};

        // let pin1_0 = port1.pin0;
        // let pin1_1 = port1.pin1;
        // let pin1_4 = port1.pin4;
        // let pin1_5 = port1.pin5;
        // let pin1_6 = port1.pin6;

        // let pin2_3 = port2.pin3;
        // let pin2_4 = port2.pin4;
        // let pin2_5 = port2.pin5;
        // let pin2_6 = port2.pin6;
        // let pin2_7 = port2.pin7;

        // let power_good_1v8 = port3.pin0.pullup();
        // let power_good_3v3 = port3.pin1.pullup();
        // let mut enable_1v8 = port3.pin2.to_output();
        // enable_1v8.set_low().ok();
        let mut enable_5v = port3.pin0.to_output();
        enable_5v.set_low().ok();
        let pin3_4 = port3.pin4;
        let pin3_5 = port3.pin5;
        let pin3_6 = port3.pin6;
        let pin3_7 = port3.pin7;

        let pin4_0 = port4.pin0;

        let pin5_1 = port5.pin1;

        let pin6_0 = port6.pin0;
        let pin6_1 = port6.pin1;
        let pin6_2 = port6.pin2;
        let pin6_3 = port6.pin3;
        let pin6_4 = port6.pin4;
        // let pin6_5 = port6.pin5;
        let pin6_6 = port6.pin6;
        let pin6_7 = port6.pin7;

        let gpio = Self {
            red_led, green_led, blue_led, 
            lora_irq, 
            gps_en, 
            half_vbat, 
            // power_good_1v8, power_good_3v3, 
            // enable_1v8,
            enable_5v,
            // pin1_0, pin1_1, pin1_4, pin1_5, pin1_6,
            // pin2_3, pin2_4, pin2_5, pin2_6, pin2_7,
            // pin3_4, pin3_5, pin3_6, pin3_7,
            // pin4_0,
            // pin5_1,
            // pin6_0, pin6_1, pin6_2, pin6_3, pin6_4, pin6_5, pin6_6, pin6_7,
        };

        (gpio, used)
    }
}

// Pins used by other peripherals.
struct ConsumedPins {
    miso:           LoraMisoPin,
    mosi:           LoraMosiPin,
    sclk:           LoraSclkPin,
    lora_reset:     LoraResetPin,
    lora_cs:        LoraCSPin,
    gps_tx_pin:     GpsTxPin,
    gps_rx_pin:     GpsRxPin,
    debug_tx_pin:   DebugTxPin,
    // i2c_sda_pin:    I2cSdaPin,
    // i2c_scl_pin:    I2cSclPin,
}