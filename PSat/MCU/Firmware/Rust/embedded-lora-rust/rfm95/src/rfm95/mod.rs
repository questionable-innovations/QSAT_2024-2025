//! RFM95 LoRa implementation

mod connection;
mod driver;
mod registers;

use crate::lora::types::Frequency;
use embedded_hal::spi::{Mode, MODE_0};

/// Recommended SPI frequency
pub const RFM95_SPI_FREQUENCY: Frequency = Frequency::hz(10_000_000);
/// Recommended SPI baudrate
pub const RFM95_SPI_BAUDRATE: Frequency = Frequency::hz(1_000_000);
/// SPI frame mode
pub const RFM95_SPI_MODE: Mode = MODE_0;
/// The RFM95 FIFO size
pub const RFM95_FIFO_SIZE: usize = 0xFF;

// Expose the driver implementation
pub use crate::rfm95::driver::Rfm95Driver;
