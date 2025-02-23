#![no_main]
#![no_std]

// External imports
use msp430_rt::entry;
use msp430fr2x5x_hal::hal::blocking::delay::DelayMs;

// Internal modules
mod pin_mappings { include!("pin_mappings_v2_0.rs"); } // Import 'pin_mappings_v2_0' as 'pin_mappings'
mod board;
mod serial;
mod panic_handler;
mod lora;
mod gps;

// Internal imports
use board::Board;

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

        board.gpio.red_led.turn_on();
        board.delay.delay_ms(LED_DELAY_MS);

        board.gpio.green_led.turn_on();
        board.delay.delay_ms(LED_DELAY_MS);

        board.gpio.blue_led.turn_on();
        board.delay.delay_ms(LED_DELAY_MS);

        board.gpio.red_led.turn_off();
        board.delay.delay_ms(LED_DELAY_MS);

        board.gpio.green_led.turn_off();
        board.delay.delay_ms(LED_DELAY_MS);

        board.gpio.blue_led.turn_off();
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
