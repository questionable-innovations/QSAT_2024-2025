//! LoRa-related operations

use crate::lora::config::Config;
use crate::lora::types::{Bandwidth, SpreadingFactor};
use core::cmp;
use core::time::Duration;

/// Utility function to compute a ceiling integer division
///
/// # Panics
/// This function panics when attempting to divide by zero.
#[inline]
#[must_use]
pub const fn ceildiv(num: i32, divided_by: i32) -> i32 {
    // Assert non-zero divisor
    assert!(divided_by != 0, "Cannot divide by zero");

    // Perform operation as u64 to avoid overflows
    #[allow(clippy::arithmetic_side_effects, reason = "This will never overflow")]
    let result = (num as i64 + divided_by as i64 - 1) / (divided_by as i64);
    result as i32
}

/// Computes the duration of a single chip (not chirp!) for the given bandwidth
#[inline]
#[must_use]
const fn chip_duration(bandwidth: Bandwidth) -> Duration {
    // Duration is `1/frequency`
    match bandwidth {
        Bandwidth::B500 => Duration::from_micros(2),
        Bandwidth::B250 => Duration::from_micros(4),
        Bandwidth::B125 => Duration::from_micros(8),
        Bandwidth::B62_5 => Duration::from_micros(16),
        Bandwidth::B41_7 => Duration::from_micros(24),
        Bandwidth::B31_25 => Duration::from_micros(32),
        Bandwidth::B20_8 => Duration::from_micros(48),
        Bandwidth::B15_6 => Duration::from_micros(64),
        Bandwidth::B10_4 => Duration::from_micros(96),
        Bandwidth::B7_8 => Duration::from_micros(128),
    }
}

/// The amount of chips per symbol for the given spreading factor
#[inline]
#[must_use]
const fn chip_count(spreading_factor: SpreadingFactor) -> u32 {
    // Chip count is `2^spreading_factor`
    let spreading_factor = spreading_factor as u8 as u32;
    2u32.pow(spreading_factor)
}

/// Computes the airtime of a single symbol for the given bandwidth and spreading factor
#[must_use]
pub const fn symbol_airtime(spreading_factor: SpreadingFactor, bandwidth: Bandwidth) -> Duration {
    // Get chip duration and count
    let chip_duration = chip_duration(bandwidth).as_micros() as u64;
    let chip_count = chip_count(spreading_factor) as u64;

    // The airtime of a single symbol is the duration of one chip times the number of chips per symbol
    #[allow(clippy::arithmetic_side_effects, reason = "This will never overflow")]
    Duration::from_micros(chip_duration * chip_count)
}

/// Computes if a configuration needs low-datarate-optimization
///
/// # Note
/// Low-datarate-optimization is a special mode that needs to be enabled on the modem if a single symbol needs more
/// than 16ms airtime.
#[inline]
pub fn needs_ldo(spreading_factor: SpreadingFactor, bandwidth: Bandwidth) -> bool {
    /// The threshold for low-datarate optimization is 16ms per symbol
    pub const THRESHOLD: Duration = Duration::from_millis(16);
    symbol_airtime(spreading_factor, bandwidth) > THRESHOLD
}

/// Gets the airtime of the preamble
///
/// # Implementation note
/// This function assumes `5` symbols as preamble-overhead; not the theoretically correct `4.25` symbols.
/// Subsequently, the computed airtime is always a little bit too long, which should not matter in practice and
/// gives us a bit of "safety margin".
#[must_use]
fn preamble_airtime(config: Config) -> Duration {
    // Get preamble length and symbol airtime
    #[allow(clippy::arithmetic_side_effects, reason = "This will never overflow")]
    let preamble_len = u16::from(config.preamble_len()) as u64 + 5;
    let symbol_airtime = symbol_airtime(config.spreading_factor(), config.bandwidth()).as_micros() as u64;

    // The airtime of the preamble is the amount of preamble symbols times the airtime of one symbol
    #[allow(clippy::arithmetic_side_effects, reason = "This will never overflow")]
    Duration::from_micros(preamble_len * symbol_airtime)
}

/// Computes the airtime of a payload
///
/// # Formula
/// Formula from `SX1276, SX1277, SX1278, SX1279` datasheet, where
/// - `PL` is the payload length in bytes
/// - `SF` is the spreading factor
/// - `CRC` specifies if a CRC is present
/// - `IH` specifies if a header is absent
/// - `DE` specifies if the low-datarate-optimization is enabled
/// - `CR` is the coding rate
///
/// `8 + max(ceil((8PL - 4SF + 28 + 16CRC - 20IH) / 4(SF - 2DE)) * (CR + 4), 0)`
#[must_use]
fn payload_airtime(payload_len: usize, config: Config) -> Duration {
    // Prepare vars
    let pl = payload_len as i32;
    let sf = config.spreading_factor() as u8 as i32;
    let crc = config.crc_mode() as u8 as i32;
    let ih = config.header_mode() as u8 as i32;
    let de = needs_ldo(config.spreading_factor(), config.bandwidth()) as u8 as i32;
    let cr = config.coding_rate() as u8 as i32;

    // Compute the payload
    #[allow(clippy::arithmetic_side_effects, reason = "This should never overflow")]
    {
        // Compute the payload symbol count
        let payload_symbol_count =
            ceildiv((8 * pl) - (4 * sf) + 28 + (16 * crc) - (20 * ih), 4 * (sf - (2 * de))) * (cr + 4);
        let symbol_count = cmp::max(payload_symbol_count, 0) as u64 + 8;
        let symbol_airtime = symbol_airtime(config.spreading_factor(), config.bandwidth()).as_micros() as u64;

        // The airtime of the preamble is the amount of preamble symbols times the airtime of one symbol
        Duration::from_micros(symbol_count * symbol_airtime)
    }
}

/// Computes the total airtime of a message
#[must_use]
pub fn airtime(payload_len: usize, config: Config) -> Duration {
    // Get airtimes of the preamble and payload
    let preamble_airtime = preamble_airtime(config).as_micros() as u64;
    let payload_airtime = payload_airtime(payload_len, config).as_micros() as u64;

    // The airtime of the message is the preamble plus the payload
    #[allow(clippy::arithmetic_side_effects, reason = "This will never overflow")]
    Duration::from_micros(preamble_airtime + payload_airtime)
}
