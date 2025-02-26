//! The modem register map

/// A RFM95 register
pub trait Register {
    /// The register address
    fn address(&self) -> u8;
    /// The bitfield offset
    fn offset(&self) -> u8 {
        0
    }
    /// The bitfield mask
    fn mask(&self) -> u8 {
        u8::MAX
    }
}
/// Declares a register type
macro_rules! register {
    ($doc:expr, $type:ident < $address:literal, $offset:literal, $length:literal >) => {
        /// A specific register description
        #[derive(Debug, Clone, Copy)]
        #[doc = $doc]
        pub struct $type;
        impl $type {
            /// The bitfield mask
            const MASK: u8 = match $offset + $length {
                ..=8 => (u8::MAX >> (8 - $length)) << $offset,
                _ => panic!("Invalid bitmap offset/length"),
            };
        }
        impl Register for $type {
            fn address(&self) -> u8 {
                $address
            }
            fn offset(&self) -> u8 {
                $offset
            }
            fn mask(&self) -> u8 {
                Self::MASK
            }
        }
    };
}

// Register definitions
register! {
    "LoRa base-band FIFO data input/output; FIFO is cleared an not accessible when device is in SLEEP mode",
    RegFifo<0x00, 0, 8>
}
register! {
    "0 -> FSK/OOK Mode, 1 -> LoRa Mode; this bit can be modified only in Sleep mode, a write operation on other device modes is ignored",
    RegOpModeLongRangeMode<0x01, 7, 1>
}
register! {
    "This bit operates when device is in Lora mode (see datasheet for more info)",
    RegOpModeAccessSharedReg<0x01, 6, 1>
}
register! {
    "Access Low Frequency Mode registers (see datasheet for more info)",
    RegOpModeLowFrequencyModeOn<0x01, 3, 1>
}
register! {
    "Device modes (see datasheet for more info)",
    RegOpModeMode<0x01, 0, 3>
}
register! {
    "MSB of RF carrier frequency",
    RegFrMsb<0x06, 0, 8>
}
register! {
    "Mid of RF carrier frequency",
    RegFrMid<0x07, 0, 8>
}
register! {
    "LSB of RF carrier frequency",
    RegFrLsb<0x08, 0, 8>
}
register! {
    "RegPaConfig (see datasheet for more info)",
    RegPaConfig<0x09, 0, 8>
}
register! {
    "SPI interface address pointer in FIFO data buffer",
    RegFifoAddrPtr<0x0D, 0, 8>
}
register! {
    "Write base address in FIFO data buffer for TX modulator",
    RegFifoTxBaseAddr<0x0E, 0, 8>
}
register! {
    "Read base address in FIFO data buffer for RX demodulator",
    RegFifoRxBaseAddr<0x0F, 0, 8>
}
register! {
    "Start address (in data buffer) of last packet received",
    RegFifoRxCurrentAddr<0x10, 0, 8>
}
register! {
    "Timeout interrupt mask: setting this bit masks the corresponding IRQ in RegIrqFlags",
    RegIrqFlagsMaskRxTimeoutMask<0x11, 7, 1>
}
register! {
    "Packet reception complete interrupt mask: setting this bit masks the corresponding IRQ in RegIrqFlags",
    RegIrqFlagsMaskRxDoneMask<0x11, 6, 1>
}
register! {
    "Payload CRC error interrupt mask: setting this bit masks thecorresponding IRQ in RegIrqFlags",
    RegIrqFlagsMaskPayloadCrcErrorMask<0x11, 5, 1>
}
register! {
    "FIFO Payload transmission complete interrupt mask: setting this bit masks the corresponding IRQ in RegIrqFlags",
    RegIrqFlagsMaskTxDoneMask<0x11, 3, 1>
}
register! {
    "Timeout interrupt: writing a 1 clears the IRQ",
    RegIrqFlagsRxTimeout<0x12, 7, 1>
}
register! {
    "Packet reception complete interrupt: writing a 1 clears the IRQ",
    RegIrqFlagsRxDone<0x12, 6, 1>
}
register! {
    "Payload CRC error interrupt: writing a 1 clears the IRQ",
    RegIrqFlagsPayloadCrcError<0x12, 5, 1>
}
register! {
    "FIFO Payload transmission complete interrupt: writing a 1 clears the IRQ",
    RegIrqFlagsTxDone<0x12, 3, 1>
}
register! {
    "Number of payload bytes of latest packet received",
    RegRxNbBytes<0x13, 0, 8>
}
register! {
    "SNR of last packet recieved",
    RegPktSnrValue<0x19, 0, 8>
}
register! {
    "RSSI of last packet recieved",
    RegPktRssiValue<0x1A, 0, 8>
}
register! {
    "Current RSSI value",
    RegRssiValue<0x1B, 0, 8>
}
register! {
    "Signal bandwidth (see datasheet for more info)",
    RegModemConfig1Bw<0x1D, 4, 4>
}
register! {
    "Error coding rate (see datasheet for more info)",
    RegModemConfig1CodingRate<0x1D, 1, 3>
}
register! {
    "0 -> Explicit header mode, 1 -> Implicit header mode",
    RegModemConfig1ImplicitHeaderModeOn<0x1D, 0, 1>
}
register! {
    "SF rate (expressed as a base-2 logarithm, see datasheet for more info)",
    RegModemConfig2SpreadingFactor<0x1E, 4, 4>
}
register! {
    "Enable CRC generation, and check on payload: 0 -> CRC disable, 1 -> CRC enable (see datasheet for more info)",
    RegModemConfig2RxPayloadCrcOn<0x1E, 2, 1>
}
register! {
    "RX Time-Out MSB",
    RegModemConfig2SymbTimeout98<0x1E, 0, 2>
}
register! {
    "RX Time-Out LSB; RX operation time-out value expressed as number of symbols: `TimeOut = SymbTimeout * Ts`",
    RegSymbTimeoutLsb<0x1F, 0, 8>
}
register! {
    "Preamble length MSB (see datasheet for more info)",
    RegPreambleMsb<0x20, 0, 8>
}
register! {
    "Preamble Length LSB (see datasheet for more info)",
    RegPreambleLsb<0x21, 0, 8>
}
register! {
    "Payload length in bytes; the register needs to be set in implicit header mode for the expected packet length (a `0` value is not permitted)",
    RegPayloadLength<0x22, 0, 8>
}
register! {
    "0 -> Disabled, 1 -> Enabled; mandated for when the symbol length exceeds 16ms",
    RegModemConfig3LowDataRateOptimize<0x26, 3, 1>
}
register! {
    "Invert the LoRa I and Q signals; 0 -> normal mode, 1 -> I and Q signals are inverted",
    RegInvertIQ<0x33, 6, 1>
}
register! {
    "LoRa Sync Word; value 0x34 is used for LoRaWAN networks",
    RegSyncWord<0x39, 0, 8>
}
#[cfg(not(feature = "debug"))]
register! {
    "Semtech ID relating the silicon revision",
    RegVersion<0x42, 0, 8>
}

/// The highest reasonable register address for dumping
#[cfg(feature = "debug")]
pub const REGISTER_MAX: u8 = 0x64;
