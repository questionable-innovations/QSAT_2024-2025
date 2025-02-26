//! A LoRa radio config object with builder pattern for initial initialization

use crate::lora::types::{
    Bandwidth, CodingRate, CrcMode, Frequency, HeaderMode, Polarity, PreambleLength, SpreadingFactor, SyncWord,
};

/// An LoRa `Config` builder
///
/// # Note
/// The builder uses some type magic to ensure values can only be set once, and once once all values are set, it is
/// automatically coerced to the final `Config` without the need for an additional "build" method.
///
/// Or, to be more precised: The final `Config` exactly the same as the `Builder` type with all fields set.
// Note: We use 1-letter abbreviations for the config fields to keep the code readable and to not bloat the file with
// dozens of repetitions
#[derive(Debug, Clone, Copy)]
pub struct Builder<S = (), B = (), R = (), P = (), H = (), C = (), W = (), L = (), F = ()> {
    /// Spreading factor
    s: S,
    /// Bandwidth
    b: B,
    /// Coding rate
    r: R,
    /// P polarity
    p: P,
    /// Header mode
    h: H,
    /// CRC mode (checksum mode)
    c: C,
    /// Sync word
    w: W,
    /// Preamble length
    l: L,
    /// Frequency
    f: F,
}
impl<B, R, P, H, C, W, L, F> Builder<(), B, R, P, H, C, W, L, F> {
    /// Sets the spreading factor
    pub fn set_spreading_factor(self, s: SpreadingFactor) -> Builder<SpreadingFactor, B, R, P, H, C, W, L, F> {
        Builder { s, b: self.b, r: self.r, p: self.p, h: self.h, c: self.c, w: self.w, l: self.l, f: self.f }
    }
}
impl<S, R, P, H, C, W, L, F> Builder<S, (), R, P, H, C, W, L, F> {
    /// Sets the bandwidth
    pub fn set_bandwidth(self, b: Bandwidth) -> Builder<S, Bandwidth, R, P, H, C, W, L, F> {
        Builder { s: self.s, b, r: self.r, p: self.p, h: self.h, c: self.c, w: self.w, l: self.l, f: self.f }
    }
}
impl<S, B, P, H, C, W, L, F> Builder<S, B, (), P, H, C, W, L, F> {
    /// Sets the coding rate
    pub fn set_coding_rate(self, r: CodingRate) -> Builder<S, B, CodingRate, P, H, C, W, L, F> {
        Builder { s: self.s, b: self.b, r, p: self.p, h: self.h, c: self.c, w: self.w, l: self.l, f: self.f }
    }
}
impl<S, B, R, H, C, W, L, F> Builder<S, B, R, (), H, C, W, L, F> {
    /// Sets the P polarity
    pub fn set_polarity(self, p: Polarity) -> Builder<S, B, R, Polarity, H, C, W, L, F> {
        Builder { s: self.s, b: self.b, r: self.r, p, h: self.h, c: self.c, w: self.w, l: self.l, f: self.f }
    }
}
impl<S, B, R, P, C, W, L, F> Builder<S, B, R, P, (), C, W, L, F> {
    /// Sets the header mode
    pub fn set_header_mode(self, h: HeaderMode) -> Builder<S, B, R, P, HeaderMode, C, W, L, F> {
        Builder { s: self.s, b: self.b, r: self.r, p: self.p, h, c: self.c, w: self.w, l: self.l, f: self.f }
    }
}
impl<S, B, R, P, H, W, L, F> Builder<S, B, R, P, H, (), W, L, F> {
    /// Sets the CC mode
    pub fn set_crc_mode(self, c: CrcMode) -> Builder<S, B, R, P, H, CrcMode, W, L, F> {
        Builder { s: self.s, b: self.b, r: self.r, p: self.p, h: self.h, c, w: self.w, l: self.l, f: self.f }
    }
}
impl<S, B, R, P, H, C, L, F> Builder<S, B, R, P, H, C, (), L, F> {
    /// Sets the sync word
    pub fn set_sync_word(self, w: SyncWord) -> Builder<S, B, R, P, H, C, SyncWord, L, F> {
        Builder { s: self.s, b: self.b, r: self.r, p: self.p, h: self.h, c: self.c, w, l: self.l, f: self.f }
    }
}
impl<S, B, R, P, H, C, W, F> Builder<S, B, R, P, H, C, W, (), F> {
    /// Sets the preamble length
    pub fn set_preamble_length(self, l: PreambleLength) -> Builder<S, B, R, P, H, C, W, PreambleLength, F> {
        Builder { s: self.s, b: self.b, r: self.r, p: self.p, h: self.h, c: self.c, w: self.w, l, f: self.f }
    }
}
impl<S, B, R, P, H, C, W, L> Builder<S, B, R, P, H, C, W, L, ()> {
    /// Sets the frequency
    pub fn set_frequency(self, f: Frequency) -> Builder<S, B, R, P, H, C, W, L, Frequency> {
        Builder { s: self.s, b: self.b, r: self.r, p: self.p, h: self.h, c: self.c, w: self.w, l: self.l, f }
    }
}

/// A LoRa radio config
pub type Config =
    Builder<SpreadingFactor, Bandwidth, CodingRate, Polarity, HeaderMode, CrcMode, SyncWord, PreambleLength, Frequency>;
impl Config {
    /// Creates a new config builder
    #[allow(clippy::self_named_constructors, reason = "Mislint due to type alias")]
    pub fn builder() -> Builder {
        Builder { s: (), b: (), r: (), p: (), h: (), c: (), w: (), l: (), f: () }
    }

    /// The spreading factor
    pub const fn spreading_factor(&self) -> SpreadingFactor {
        self.s
    }
    /// The bandwidth
    pub const fn bandwidth(&self) -> Bandwidth {
        self.b
    }
    /// The coding rate
    pub const fn coding_rate(&self) -> CodingRate {
        self.r
    }
    /// The polarity
    pub const fn polarity(&self) -> Polarity {
        self.p
    }
    /// The header mode
    pub const fn header_mode(&self) -> HeaderMode {
        self.h
    }
    /// The CRC mode
    pub const fn crc_mode(&self) -> CrcMode {
        self.c
    }
    /// The sync word
    pub const fn sync_word(&self) -> SyncWord {
        self.w
    }
    /// The preamble length
    pub const fn preamble_len(&self) -> PreambleLength {
        self.l
    }
    /// The frequency
    pub const fn frequency(&self) -> Frequency {
        self.f
    }
}
