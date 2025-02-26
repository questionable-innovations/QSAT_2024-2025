[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/embedded-lora-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/embedded-lora-rust)

# `embedded-lora`
`no-std`-compatible, opinionated drivers for some LoRa modems. Despite almost all modems supporting multiple radio
modulations, these drivers only implement LoRa. Furthermore, we currently only test the EU 868 MHz variants; other
variants may or may not work.

## `embedded-lora-rfm95`
[![docs.rs](https://docs.rs/embedded-lora-rfm95/badge.svg)](https://docs.rs/embedded-lora-rfm95)
[![crates.io](https://img.shields.io/crates/v/embedded-lora-rfm95.svg)](https://crates.io/crates/embedded-lora-rfm95)
[![Download numbers](https://img.shields.io/crates/d/embedded-lora.svg)](https://crates.io/crates/embedded-lora-rfm95)
[![dependency status](https://deps.rs/crate/embedded-lora-rfm95/latest/status.svg)](https://deps.rs/crate/embedded-lora-rfm95)

The [embedded-lora-rfm95](./rfm95/README.md) crate implements a driver for the popular RFM95 chips.

Currently supported features are:
- [x] Single RX
- [x] Single TX
- [x] Advanced LoRa modem configuration
- [x] SPI and modem register debugging
- [x] LoRa utils for AirTime computation
