#![no_main]
#![no_std]

// External imports
use msp430_rt::entry;
use msp430fr2x5x_hal::hal::blocking::delay::DelayMs;

// Internal modules
mod board;
mod serial;
mod panic_handler;

// Internal imports
use board::Board;

// Red onboard LED should blink at a steady period.
#[entry]
fn main() -> ! {
    let board = board::configure(); // Collect board elements, configure printing, etc.

    // Printing can be expensive in terms of executable size. We only have 32kB on the MSP430, use it sparingly.
    // Prints over eUSCI A0. See board::configure() for details.
    println!("Hello world!");

    idle_loop(board);
}

fn idle_loop(mut board: Board) -> ! {
    loop {
        // Snake the LEDs through the rainbow
        const LED_DELAY_MS: u16 = 50; // ms
        // Returns a `Result` because of embedded_hal, but the result is always `Ok` with MSP430 GPIO.
        // Rust complains about unused Results, so we 'use' the Result by calling .ok()
        board.red_led.turn_on();
        board.delay.delay_ms(LED_DELAY_MS);

        board.green_led.turn_on();
        board.delay.delay_ms(LED_DELAY_MS);

        board.blue_led.turn_on();
        board.delay.delay_ms(LED_DELAY_MS);

        board.red_led.turn_off();
        board.delay.delay_ms(LED_DELAY_MS);

        board.green_led.turn_off();
        board.delay.delay_ms(LED_DELAY_MS);

        board.blue_led.turn_off();
        board.delay.delay_ms(LED_DELAY_MS);
    }
}

// The compiler will emit calls to the abort() compiler intrinsic if debug assertions are
// enabled (default for dev profile). MSP430 does not actually have meaningful abort() support
// so for now, we create our own in each application where debug assertions are present.
#[no_mangle]
extern "C" fn abort() -> ! {
    panic!();
}
