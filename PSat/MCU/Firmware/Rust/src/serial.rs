use msp430fr2355::E_USCI_A0;
use msp430fr2x5x_hal::serial::Tx;

// A little bit of magic to get println working.
// Tx can only print bytes by default, but by implementing this we can print arbitrary (ASCII) strings.
// Format strings are automatically implemented for implementers of core::fmt::Write, so
// with the custom println!() macro below this means we get full println! behaviour.
pub struct PrintableSerial( pub Tx<E_USCI_A0> );
impl core::fmt::Write for PrintableSerial {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for char in s.chars() {
            use embedded_hal::serial::Write;
            nb::block!( self.0.write(char as u8) ).ok(); // The cast to u8 assumes ASCII-only characters
        }
        Ok(())
    }
}

// Store our serial handle globally after it's been configured, so we don't have to carry it around with us everywhere.
use msp430::interrupt::Mutex;
use core::cell::RefCell;
pub static SERIAL: Mutex<RefCell<Option< PrintableSerial >>> = Mutex::new(RefCell::new(None));

// Make a macro equivalent to the regular println!() macro.
/// Prints over `eUSCI_A0` serial. Fails to print if `board::configure()` hasn't been called yet.
#[macro_export]
macro_rules! print {
    ($first:tt $(, $( $rest:tt )* )?) => {
        {
            msp430::critical_section::with(|cs| {
                let Some(ref mut serial) = *$crate::serial::SERIAL.borrow_ref_mut(cs) else {loop{}};
                use ufmt::uwrite;
                uwrite!(ufmt_utils::WriteAdapter(serial), $first,  $( $($rest)* )*).ok();
            });
        }
    };
}

/// Prints over `eUSCI_A0` serial. Fails to print if `board::configure()` hasn't been called yet.
#[macro_export]
macro_rules! println {
    ($first:tt $(, $( $rest:tt )* )?) => {
        {
            $crate::print!($first,  $( $($rest)* )*);
            $crate::print!("\n");
        }
    };
}

// Make a macro equivalent to the regular println!() macro.
/// Prints over `eUSCI_A0` serial. Fails to print if `board::configure()` hasn't been called yet.
/// 
/// This macro uses the Rust core library, which can print more things but bloats the executable. Avoid using this if you can.
#[macro_export]
macro_rules! stdlib_print {
    ($first:tt $(, $( $rest:tt )* )?) => {
        {
            msp430::critical_section::with(|cs| {
                let Some(ref mut serial) = *$crate::serial::SERIAL.borrow_ref_mut(cs) else {loop{}};
                use core::fmt::Write;
                write!(serial, $first,  $( $($rest)* )*).ok();
            });
        }
    };
}

/// Prints over `eUSCI_A0` serial. Fails to print if `board::configure()` hasn't been called yet.
/// 
/// This macro uses the Rust core library, which can print more things but bloats the executable. Avoid using this if you can.
#[macro_export]
macro_rules! stdlib_println {
    ($first:tt $(, $( $rest:tt )* )?) => {
        {
            $crate::stdlib_print!($first,  $( $($rest)* )*);
            $crate::stdlib_print!("\n");
        }
    };
}