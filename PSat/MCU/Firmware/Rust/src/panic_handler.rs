use msp430fr2x5x_hal::{clock::{DcoclkFreqSel, MclkDiv, SmclkDiv}, gpio::Batch, pmm::Pmm};

// Our panic handler. Currently we print strings here for maximum debuggability. String printing is quite expensive in terms of executable size,
// so if you're running out of space consider commenting out some of these print statements (or uncommenting `strip = true` in cargo.toml!).
use crate::{print, println, stdlib_println};
use core::panic::PanicInfo;
#[panic_handler]
fn panic_handler(panic_info: &PanicInfo) -> ! {
    msp430::interrupt::disable();

    ensure_serial_configured();
    print!("Panic: ");
    if let Some(location) = panic_info.location() {
        // Printing code locations adds a lot of executable size
        println!("File: {}, line: {}, col: {},", location.file(), location.line(), location.column())
    }
    if let Some(message) = panic_info.message().as_str() {
        print!("{}", message);
    }
    else {
        // Unfortunately we can't print PanicMessage using ufmt because it doesn't implement uDisplay or uDebug.
        // The below code pulls in Rust's standard printing library.
        stdlib_println!("{}", panic_info.message());
        //println!("Can't print message");
    }

    loop { msp430::asm::barrier(); }
}

// *Ensure* debug serial is configured, stealing registers and pins if necessary.
fn ensure_serial_configured() {
    let not_configured = msp430::critical_section::with(|cs| { crate::serial::SERIAL.borrow_ref(cs).is_none() });
    if not_configured {
        let regs = unsafe{ msp430fr2x5x_hal::pac::Peripherals::steal() };
        let pin1_7: crate::pin_mappings::DebugTxPin = Batch::new(regs.P1).split(&Pmm::new(regs.PMM)).pin7.to_alternate1();
        let mut fram = msp430fr2x5x_hal::fram::Fram::new(regs.FRCTL);
        let (smclk, _aclk, _delay) = msp430fr2x5x_hal::clock::ClockConfig::new(regs.CS)
            .mclk_dcoclk(DcoclkFreqSel::_8MHz, MclkDiv::_1)
            .smclk_on(SmclkDiv::_1)
            .freeze(&mut fram);
        crate::serial::configure_debug_serial(pin1_7, &smclk, regs.E_USCI_A0);
    }
}