[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/embedded-lora-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/embedded-lora-rust)
[![docs.rs](https://docs.rs/embedded-lora-rfm95/badge.svg)](https://docs.rs/embedded-lora-rfm95)
[![crates.io](https://img.shields.io/crates/v/embedded-lora-rfm95.svg)](https://crates.io/crates/embedded-lora-rfm95)
[![Download numbers](https://img.shields.io/crates/d/embedded-lora.svg)](https://crates.io/crates/embedded-lora-rfm95)
[![dependency status](https://deps.rs/crate/embedded-lora-rfm95/latest/status.svg)](https://deps.rs/crate/embedded-lora-rfm95)

# `embedded-lora-rfm95`
A `no-std`-compatible, opinionated driver for the RFM95 LoRa modem. It only supports the LoRa mode, and has only been
tested with the EU 868 MHz ISM bands for now.

## Features
The crate supports the following optional `cargo` features:

### `fugit` (disabled by default)
The `fugit`-feature implements simple `From`/`Into`-conversions between the built-in frequency type and
[`fugit`'s](https://crates.io/crates/fugit) [`HertzU32` type](https://docs.rs/fugit/latest/fugit/type.HertzU32.html).
This is a comfort-feature only, and does not enable additional functionality.

### `debug` (disabled by default)
The `debug` feature enables some debug functionality, namely an SPI debug callback which can be used to log all SPI
transactions with the RFM95 modem, and provides some helper functions to dump the register state and FIFO contents. The
`debug` feature also disables the modem silicon revision check.

To use this feature, you __MUST__ implement this extern callback function in your crate (otherwise you'll get a cryptic
linker error):
```rust
extern "Rust" {
    /// A debug callback that is called for every SPI transaction
    /// 
    /// # About
    /// This function is called for every SPI transaction, where `operation` is the operation type (`0x00` for read,
    /// `0x80` for replace), `address` is the register address, and `input` and `output` are the values written and read
    /// respectively.
    fn embeddedrfm95_spidebug_AwiUzTRu(operation: u8, address: u8, input: u8, output: u8);
}
```

Example implementation:
```rust
/// The debug callback implementation for `embedded-rfm95`
#[no_mangle]
pub extern "Rust" fn embeddedrfm95_spidebug_AwiUzTRu(operation: u8, address: u8, input: u8, output: u8) {
    // Print the transaction to stdout
    println!("[SPI 0x{operation:02X} @[0x{address:02X}] tx:0x{input:02X} rx:0x{output:02X}");
}
```
