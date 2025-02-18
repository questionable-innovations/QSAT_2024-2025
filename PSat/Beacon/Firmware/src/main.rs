#![no_main]
#![no_std]

use embedded_hal::{digital::v2::*, spi::MODE_0};
use embedded_hal_compat::{eh1_0::delay::DelayNs, ForwardCompat};
use embedded_lora_rfm95::{lora::types::{Bandwidth, CodingRate, CrcMode, HeaderMode, Polarity, PreambleLength, SpreadingFactor, SyncWord}, rfm95::{Rfm95Driver, RFM95_FIFO_SIZE}};
use msp430::critical_section;
use msp430_rt::entry;
use msp430fr2355::{E_USCI_A1, P2};
use msp430fr2x5x_hal::{
    clock::{ClockConfig, DcoclkFreqSel, MclkDiv, SmclkDiv}, delay::Delay, fram::Fram, gpio::{Batch, Output, Pin, Pin0, Pin1, Pin2}, hal::blocking::delay::DelayMs, pmm::Pmm, serial::SerialConfig, spi::SpiBusConfig, watchdog::Wdt
};

use core::{cell::RefCell, panic::PanicInfo};
#[panic_handler]
fn panic_handler(panic_info: &PanicInfo) -> ! {
    msp430::interrupt::disable();
    println!("Panic: {:?}", panic_info);
    loop { msp430::asm::barrier(); }
}

/// Standard printing. Fails to print if SERIAL hasn't been configured yet.
#[macro_export]
macro_rules! println {
    ($first:tt $(, $( $rest:tt )* )?) => {
        {
            critical_section::with(|cs| {
                let Some(ref mut serial) = *$crate::SERIAL.borrow_ref_mut(cs) else {loop{}};
                use core::fmt::Write;
                writeln!(serial, $first,  $( $($rest)* )*).ok();
            });
        }
    };
}

use msp430::interrupt::Mutex;
static SERIAL: Mutex<RefCell<Option< MySerial >>> = Mutex::new(RefCell::new(None));

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
    let port5 = Batch::new(regs.P5).split(&pmm);

    let mut lora_reset = port5.pin3.to_output(); // Not actually connected...
    lora_reset.set_high().ok();
    let miso = port4.pin7.to_alternate1();
    let mosi = port4.pin6.to_alternate1();
    let sclk = port4.pin5.to_alternate1();
    let mut cs = port4.pin4.to_output();
    cs.set_high().ok();

    let tx_pin = port4.pin3.to_alternate1();

    // Configure clocks to get accurate delay timing
    let mut fram = Fram::new(regs.FRCTL);
    let (smclk, _aclk, mut delay) = ClockConfig::new(regs.CS)
        .mclk_dcoclk(DcoclkFreqSel::_8MHz, MclkDiv::_1)
        .smclk_on(SmclkDiv::_1)
        .freeze(&mut fram);

    let spi = SpiBusConfig::new(regs.E_USCI_B1, MODE_0, true)
        .use_smclk(&smclk, 32)
        .configure_with_software_cs(miso, mosi, sclk);

    let serial = SerialConfig::new(regs.E_USCI_A1, 
        msp430fr2x5x_hal::serial::BitOrder::LsbFirst, 
        msp430fr2x5x_hal::serial::BitCount::EightBits, 
        msp430fr2x5x_hal::serial::StopBits::OneStopBit, 
        msp430fr2x5x_hal::serial::Parity::NoParity, 
        msp430fr2x5x_hal::serial::Loopback::NoLoop, 
        9600)
        .use_smclk(&smclk)
        .tx_only(tx_pin);
    use core::fmt::Write;
    let mut serial = MySerial(serial);

    lora_reset.set_low().ok();
    delay.delay_ms(1);
    lora_reset.set_high().ok();
    delay.delay_ms(6);

    let mut rfm95 = Rfm95Driver::new(spi.forward(), cs.forward(), lora_reset.forward(), MyDelay(delay)).unwrap();
    writeln!(serial, "RFM95 config success!").ok();

    delay.delay_ms(1000);

    let lora_config = embedded_lora_rfm95::lora::config::Builder::builder()
        .set_bandwidth(Bandwidth::B7_8) // lowest bandwidth == longest range
        .set_coding_rate(CodingRate::C4_8) // Maximum error correction
        .set_crc_mode(CrcMode::Disabled)
        .set_frequency(915_000_000.into()) // Hz
        .set_header_mode(HeaderMode::Explicit)
        .set_polarity(Polarity::Normal)
        .set_preamble_length(PreambleLength::L8)
        .set_spreading_factor(SpreadingFactor::S12) // High SF == Best range
        .set_sync_word(SyncWord::PRIVATE);
    rfm95.set_config(&lora_config).unwrap();

    writeln!(serial, "Beginning transmission...").ok();
    transmit(&mut rfm95, b"Hello world!");
    writeln!(serial, "Transmission complete.").ok();

    writeln!(serial, "Beginning recieve...").ok();
    let mut recieve_buf = [0u8; RFM95_FIFO_SIZE];
    let packet = recieve(&mut rfm95, &mut recieve_buf);
    writeln!(serial, "Recieve ended.").ok();
    println!("{:?}", packet);

    idle_loop(red_led, green_led, blue_led, delay);
}

type HorribleEmbeddedHalCompatRFM95Type = Rfm95Driver<embedded_hal_compat::Forward<msp430fr2x5x_hal::spi::SpiBus<msp430fr2355::E_USCI_B1>>, embedded_hal_compat::Forward<Pin<msp430fr2355::P4, msp430fr2x5x_hal::gpio::Pin4, Output>, embedded_hal_compat::markers::ForwardOutputPin>>;

fn recieve<'b>(rfm95: &mut HorribleEmbeddedHalCompatRFM95Type, buf: &'b mut [u8; RFM95_FIFO_SIZE]) -> &'b [u8] {
    let max_timeout = rfm95.rx_timeout_max().unwrap();
    rfm95.start_rx(max_timeout).unwrap();

    let size;
    loop {
        match rfm95.complete_rx(buf) {
            Ok(Some(n)) => {size = n; break;},
            Ok(None) => continue,
            Err(e) => panic!("{e}"),
        };
    };

    &buf[0..size]
}

fn transmit(rfm95: &mut HorribleEmbeddedHalCompatRFM95Type, data: &[u8]) {
    rfm95.start_tx(data).unwrap();
    let mut result = rfm95.complete_tx();
    while let Ok(None) = result {
        result = rfm95.complete_tx();
    }
}

fn idle_loop(mut red_led: Pin<P2, Pin0, Output>, mut green_led: Pin<P2, Pin2, Output>, mut blue_led: Pin<P2, Pin1, Output>, mut delay: Delay) -> ! {
    loop {
        const LED_DELAY_MS: u16 = 50; // ms
        // Returns a `Result` because of embedded_hal, but the result is always `Ok` with MSP430 GPIO.
        // Rust complains about unused Results, so we 'use' the Result by calling .ok()
        red_led.set_low().ok();
        delay.delay_ms(LED_DELAY_MS);

        green_led.set_low().ok();
        delay.delay_ms(LED_DELAY_MS);

        blue_led.set_low().ok();
        delay.delay_ms(LED_DELAY_MS);

        red_led.set_high().ok();
        delay.delay_ms(LED_DELAY_MS);

        green_led.set_high().ok();
        delay.delay_ms(LED_DELAY_MS);

        blue_led.set_high().ok();
        delay.delay_ms(LED_DELAY_MS);
    }
}

struct MySerial(msp430fr2x5x_hal::serial::Tx<E_USCI_A1>);
impl core::fmt::Write for MySerial {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for char in s.chars() {
            use embedded_hal::serial::Write;
            nb::block!( self.0.write(char as u8) ).ok(); // Assume ASCII-only characters
        }
        Ok(())
    }
}

struct MyDelay(msp430fr2x5x_hal::delay::Delay);
impl DelayNs for MyDelay {
    fn delay_ms(&mut self, ms: u32) {
        if ms < (u16::MAX as u32) {
            self.0.delay_ms(ms as u16);
        }
        else {
            let times = ms/(u16::MAX as u32);

            for _ in 0..times {
                self.0.delay_ms(u16::MAX);
            }
            let remainder = ms - times*(u16::MAX as u32);
            self.0.delay_ms(remainder as u16);
        }
    }
    
    fn delay_ns(&mut self, ns: u32) {
        let ms = ns / 1_000_000;
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