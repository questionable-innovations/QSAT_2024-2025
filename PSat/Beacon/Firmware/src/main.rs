#![no_main]
#![no_std]

use embedded_hal::{digital::v2::*, spi::MODE_0};
use msp430_rt::entry;
use msp430fr2355::E_USCI_A1;
use msp430fr2x5x_hal::{
    clock::{ClockConfig, DcoclkFreqSel, MclkDiv, SmclkDiv}, fram::Fram, gpio::Batch, hal::blocking::delay::DelayMs, pmm::Pmm, serial::SerialConfig, spi::SpiBusConfig, watchdog::Wdt
};
use panic_msp430 as _;

// Red onboard LED should blink at a steady period.
#[entry]
fn main() -> ! {
    // Take peripherals and disable watchdog
    let regs = msp430fr2355::Peripherals::take().unwrap();
    let _wdt = Wdt::constrain(regs.WDT_A);

    // Configure GPIO
    let pmm = Pmm::new(regs.PMM);
    let port2 = Batch::new(regs.P2).split(&pmm);

    let mut red_led = port2.pin0.to_output();
    let mut blue_led = port2.pin1.to_output();
    let mut green_led = port2.pin2.to_output();

    red_led.set_high().ok();
    blue_led.set_high().ok();
    green_led.set_high().ok();

    let port4 = Batch::new(regs.P4).split(&pmm);
    let port6 = Batch::new(regs.P6).split(&pmm);

    let lora_reset = port6.pin0.to_output(); // Not actually connected...
    let miso = port4.pin7.to_alternate1();
    let mosi = port4.pin6.to_alternate1();
    let sclk = port4.pin5.to_alternate1();
    let cs = port4.pin4.to_output();

    let tx_pin = port4.pin3.to_alternate1();

    // Configure clocks to get accurate delay timing
    let mut fram = Fram::new(regs.FRCTL);
    let (smclk, _aclk, mut delay) = ClockConfig::new(regs.CS)
        .mclk_dcoclk(DcoclkFreqSel::_8MHz, MclkDiv::_1)
        .smclk_on(SmclkDiv::_1)
        .freeze(&mut fram);

    let spi = SpiBusConfig::new(regs.E_USCI_B1, MODE_0, true).use_smclk(&smclk, 8).configure_with_software_cs(miso, mosi, sclk);

    let serial = SerialConfig::new(regs.E_USCI_A1, 
        msp430fr2x5x_hal::serial::BitOrder::LsbFirst, 
        msp430fr2x5x_hal::serial::BitCount::EightBits, 
        msp430fr2x5x_hal::serial::StopBits::OneStopBit, 
        msp430fr2x5x_hal::serial::Parity::NoParity, 
        msp430fr2x5x_hal::serial::Loopback::NoLoop, 
        9600)
        .use_smclk(&smclk)
        .tx_only(tx_pin);

    const LORA_FREQ: i64 = 915; // MHz
    let res = sx127x_lora::LoRa::new(spi, cs, lora_reset, LORA_FREQ, MyDelay(delay));

    if let Err(e) = res {
        use core::fmt::Write;
        let mut s = MySerial(serial);
        write!(s, "{e:?}").ok();
    };

    loop {
        // Returns a `Result` because of embedded_hal, but the result is always `Ok` with MSP430 GPIO.
        // Rust complains about unused Results, so we 'use' the Result by calling .ok()
        red_led.set_low().ok();
        delay.delay_ms(100);

        green_led.set_low().ok();
        delay.delay_ms(100);

        blue_led.set_low().ok();
        delay.delay_ms(100);

        red_led.set_high().ok();
        delay.delay_ms(100);

        green_led.set_high().ok();
        delay.delay_ms(100);

        blue_led.set_high().ok();
        delay.delay_ms(100);
        
    }
}

struct MySerial(msp430fr2x5x_hal::serial::Tx<E_USCI_A1>);
impl core::fmt::Write for MySerial {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for char in s.chars() {
            use embedded_hal::serial::Write;
            nb::block!( self.0.write(char as u8) ).ok();
        }
        Ok(())
    }
}

struct MyDelay(msp430fr2x5x_hal::delay::Delay);
impl DelayMs<u8> for MyDelay {
    fn delay_ms(&mut self, ms: u8) {
        self.0.delay_ms(ms as u16);
    }
}

// The compiler will emit calls to the abort() compiler intrinsic if debug assertions are
// enabled (default for dev profile). MSP430 does not actually have meaningful abort() support
// so for now, we create our own in each application where debug assertions are present.
#[no_mangle]
extern "C" fn abort() -> ! {
    panic!();
}