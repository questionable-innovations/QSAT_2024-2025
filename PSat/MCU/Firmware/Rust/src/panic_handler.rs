// Our panic handler. Currently we print strings here for maximum debuggability. String printing is quite expensive in terms of executable size,
// so if you're running out of space consider commenting out some of these print statements (or uncommenting `strip = true` in cargo.toml!).
use crate::{print, println, stdlib_println};
use core::panic::PanicInfo;
#[panic_handler]
fn panic_handler(panic_info: &PanicInfo) -> ! {
    msp430::interrupt::disable();
    print!("Panic: ");
    if let Some(location) = panic_info.location() {
        // Printing integers adds ~3.5kB
        println!("File: {}, line: {}, col: {},", location.file(), location.line(), location.column())
    }
    if let Some(message) = panic_info.message().as_str() {
        print!("{}", message);
    }
    else {
        // Unfortunately we can't print PanicMessage using ufmt because it doesn't implement uDisplay or uDebug.
        // The below code pulls in Rust's standard printing library. Adds ~1.5kB.
        stdlib_println!("{}", panic_info.message());
        //println!("Can't print message");
    }

    loop { msp430::asm::barrier(); }
}