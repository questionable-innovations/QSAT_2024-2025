//! Small wrappers for type safety

/// A LoRa spreading factor
///
/// # Implementation Note
/// While the RFM95 modem supports SF6 partially, it is not implemented as it has several constraints and requires
/// special handling.
///
/// # Representation
/// The spreading factor can be represented as `u8`, where the value is the index of the spreading factor (i.e.
/// `S7 => 7`). The representation is compatible to the modem representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum SpreadingFactor {
    /// Spreading factor 7 aka 128 chirps per symbol
    S7 = 7,
    /// Spreading factor 8 aka 256 chirps per symbol
    S8 = 8,
    /// Spreading factor 9 aka 512 chirps per symbol
    S9 = 9,
    /// Spreading factor 10 aka 1024 chirps per symbol
    S10 = 10,
    /// Spreading factor 11 aka 2048 chirps per symbol
    S11 = 11,
    /// Spreading factor 12 aka 4096 chirps per symbol
    S12 = 12,
}
impl TryFrom<u8> for SpreadingFactor {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            sf if sf == Self::S7 as u8 => Ok(Self::S7),
            sf if sf == Self::S8 as u8 => Ok(Self::S8),
            sf if sf == Self::S9 as u8 => Ok(Self::S9),
            sf if sf == Self::S10 as u8 => Ok(Self::S10),
            sf if sf == Self::S11 as u8 => Ok(Self::S11),
            sf if sf == Self::S12 as u8 => Ok(Self::S12),
            _ => Err("Invalid or unsupported spreading factor"),
        }
    }
}

/// The bandwidth to use
///
/// # Representation
/// The bandwidth can be represented as `u8`, but should be treated as opaque. The representation is compatible to the
/// modem representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Bandwidth {
    /// 500 kHz bandwidth
    B500 = 0b1001,
    /// 250 kHz bandwidth
    B250 = 0b1000,
    /// 125 kHz bandwidth
    B125 = 0b0111,
    /// 62.5 kHz bandwidth
    B62_5 = 0b0110,
    /// 41.7 kHz bandwidth
    B41_7 = 0b0101,
    /// 31.25 kHz bandwidth
    B31_25 = 0b0100,
    /// 20.8 kHz bandwidth
    B20_8 = 0b0011,
    /// 15.6 kHz bandwidth
    B15_6 = 0b0010,
    /// 10.4 kHz bandwidth
    B10_4 = 0b0001,
    /// 7.8 kHz bandwidth
    B7_8 = 0b0000,
}
impl TryFrom<u8> for Bandwidth {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            bw if bw == Self::B500 as u8 => Ok(Self::B500),
            bw if bw == Self::B250 as u8 => Ok(Self::B250),
            bw if bw == Self::B125 as u8 => Ok(Self::B125),
            bw if bw == Self::B62_5 as u8 => Ok(Self::B62_5),
            bw if bw == Self::B41_7 as u8 => Ok(Self::B41_7),
            bw if bw == Self::B31_25 as u8 => Ok(Self::B31_25),
            bw if bw == Self::B20_8 as u8 => Ok(Self::B20_8),
            bw if bw == Self::B15_6 as u8 => Ok(Self::B15_6),
            bw if bw == Self::B10_4 as u8 => Ok(Self::B10_4),
            bw if bw == Self::B7_8 as u8 => Ok(Self::B7_8),
            _ => Err("Invalid or unsupported bandwidth"),
        }
    }
}

/// The coding rate for forward error correction
///
/// # Representation
/// The coding rate can be represented as `u8`, where the value is the difference to the overhead divisor (i.e.
/// `4/5 => 1`, `4/7 => 3`). The representation is compatible to the modem representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum CodingRate {
    /// Coding rate 4/5 aka 1.25x overhead
    C4_5 = 0b001,
    /// Coding rate 4/6 aka 1.5x overhead
    C4_6 = 0b010,
    /// Coding rate 4/7 aka 1.75x overhead
    C4_7 = 0b011,
    /// Coding rate 4/8 aka 2x overhead
    C4_8 = 0b100,
}
impl TryFrom<u8> for CodingRate {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            cr if cr == Self::C4_5 as u8 => Ok(Self::C4_5),
            cr if cr == Self::C4_6 as u8 => Ok(Self::C4_6),
            cr if cr == Self::C4_7 as u8 => Ok(Self::C4_7),
            cr if cr == Self::C4_8 as u8 => Ok(Self::C4_8),
            _ => Err("Invalid coding rate"),
        }
    }
}

/// The IQ polarity
///
/// # Representation
/// The polarity can be represented as `u8`, where `Normal => 0`, `Inverted => 1`. The representation is
/// compatible to the modem representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity {
    /// Normal polarity, usually used for uplinks
    Normal = 0,
    /// Inverted polarity, usually used for downlinks
    Inverted = 1,
}
impl TryFrom<u8> for Polarity {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            polarity if polarity == Self::Normal as u8 => Ok(Self::Normal),
            polarity if polarity == Self::Inverted as u8 => Ok(Self::Inverted),
            _ => Err("Invalid IQ polarity value"),
        }
    }
}

/// The LoRa header mode
///
/// # Representation
/// The header mode can be represented as `u8`, where `Explicit => 0`, `Implicit => 1`. The representation is
/// compatible to the modem representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HeaderMode {
    /// Explicit header mode to include the header to allow dynamic decoding
    Explicit = 0,
    /// Implicit header mode to omit the header if decoding parameters are known
    Implicit = 1,
}
impl TryFrom<u8> for HeaderMode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            mode if mode == Self::Explicit as u8 => Ok(Self::Explicit),
            mode if mode == Self::Implicit as u8 => Ok(Self::Implicit),
            _ => Err("Invalid header mode"),
        }
    }
}

/// The CRC configuration
///
/// # Representation
/// The CRC mode can be represented as `u8`, where `Disabled => 0`, `Enabled => 1`. The representation is
/// compatible to the modem representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CrcMode {
    /// CRC disabled
    Disabled = 0,
    /// CRC enabled
    Enabled = 1,
}
impl TryFrom<u8> for CrcMode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            mode if mode == Self::Disabled as u8 => Ok(Self::Disabled),
            mode if mode == Self::Enabled as u8 => Ok(Self::Enabled),
            _ => Err("Invalid CRC mode"),
        }
    }
}

/// The LoRa sync word to use
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SyncWord(u8);
impl SyncWord {
    /// Public sync word
    pub const PUBLIC: Self = Self(0x34);
    /// Private sync word
    pub const PRIVATE: Self = Self(0x12);

    /// Create a new sync word from the given raw sync word
    pub const fn new(word: u8) -> Self {
        Self(word)
    }

    /// The sync word as `u8`
    pub const fn as_u8(self) -> u8 {
        self.0
    }
}
impl From<u8> for SyncWord {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl From<SyncWord> for u8 {
    fn from(value: SyncWord) -> Self {
        value.0
    }
}

/// The preamble length in symbols
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PreambleLength(u16);
impl PreambleLength {
    /// A preamble length of 8 symbols, used for LoRaWAN
    pub const L8: Self = Self(8);

    /// Create a new preamble length from the given raw length
    pub const fn new(len: u16) -> Self {
        Self(len)
    }

    /// The preamble length as `u16`
    pub const fn as_u16(self) -> u16 {
        self.0
    }
}
impl From<u16> for PreambleLength {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl From<PreambleLength> for u16 {
    fn from(value: PreambleLength) -> Self {
        value.0
    }
}

/// The frequency in Hz
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Frequency(u32);
impl Frequency {
    /// 868.1 MHz (common LoRaWAN default frequency)
    pub const F868_1: Self = Self(868_100_000);
    /// 868.3 MHz (common LoRaWAN default frequency)
    pub const F868_3: Self = Self(868_300_000);
    /// 868.5 MHz (common LoRaWAN default frequency)
    pub const F868_5: Self = Self(868_500_000);
    /// 869.5 MHz (useful due to its 10% duty cycle in some areas)
    pub const F869_5: Self = Self(869_500_000);

    /// Create a new frequency from the given raw frequency in Hz
    pub const fn hz(hz: u32) -> Self {
        Self(hz)
    }

    /// The frequency in Hertz as `u32`
    pub const fn as_u32(self) -> u32 {
        self.0
    }
}
impl From<u32> for Frequency {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl From<Frequency> for u32 {
    fn from(value: Frequency) -> Self {
        value.0
    }
}
#[cfg(feature = "fugit")]
impl From<Frequency> for fugit::HertzU32 {
    fn from(value: Frequency) -> Self {
        use fugit::HertzU32;

        HertzU32::Hz(value.0)
    }
}
#[cfg(feature = "fugit")]
impl From<fugit::HertzU32> for Frequency {
    fn from(value: fugit::HertzU32) -> Self {
        Self(value.to_Hz())
    }
}
