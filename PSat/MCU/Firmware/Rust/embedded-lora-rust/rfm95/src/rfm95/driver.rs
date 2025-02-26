//! RFM95 driver for LoRa operations

use crate::lora::airtime;
use crate::lora::config::Config;
use crate::lora::types::*;
use crate::rfm95::connection::Rfm95Connection;
use crate::rfm95::registers::*;
use crate::rfm95::RFM95_FIFO_SIZE;
use core::cmp;
use core::fmt::{Debug, Formatter};
use core::time::Duration;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::SpiBus;

/// Raw SPI command interface for RFM95
pub struct Rfm95Driver<Bus, Select>
where
    Bus: SpiBus,
    Select: OutputPin,
{
    /// The SPI connection to the RFM95 radio
    spi: Rfm95Connection<Bus, Select>,
}
impl<Bus, Select> Rfm95Driver<Bus, Select>
where
    Bus: SpiBus,
    Select: OutputPin,
{
    /// Supported silicon revisions for compatibility check
    #[cfg(not(feature = "debug"))]
    const SUPPORTED_SILICON_REVISIONS: [u8; 2] = [0x11, 0x12];
    /// The frequency divider to compute the frequency in milli-hertz
    const FREQUENCY_DIVIDER_MILLIHZ: u64 = 61_035;
    /// The threshold for switching between low-frequency mode (below 525 MHz) and high frequency mode (above 779 MHz)
    const HIGH_FREQUENCY_THRESHOLD: Frequency = Frequency::hz(652_000_000);

    /// The register value to put the device to LoRa mode
    const REG_OPMODE_LONGRANGEMODE_LORA: u8 = 0b1;
    /// The register value to set the shared registers to LoRa mode
    const REG_OPMODE_ACCESSSHAREDREG_LORA: u8 = 0b0;
    /// The pre-assembled register value for the operation mode register to put the device to sleep
    const REG_OPMODE_MODE_SLEEP: u8 = 0b000;
    /// The pre-assembled register value for the operation mode register to go into standby during LoRa mode
    const REG_OPMODE_MODE_STANDBY: u8 = 0b001;
    /// The pre-assembled register value for the operation mode register to start a single LoRa TX transmission
    const REG_OPMODE_MODE_TXSINGLE: u8 = 0b011;
    /// The pre-assembled register value for the operation mode register to start a single LoRa RX reception
    const REG_OPMODE_MODE_RXSINGLE: u8 = 0b110;

    /// Creates a new raw SPI command interface for RFM95
    ///
    /// # Blocking
    /// This function blocks for at least `11ms` plus additional time for the modem transactions. If you have tight
    /// scheduling requirements, you probably want to initialize this driver before entering your main event loop.
    ///
    /// # Important
    /// The RFM95 modem is initialized to LoRa-mode and put to standby. All other configurations are left untouched, so
    /// you probably want to configure the modem initially (also see [`Self::set_config`]).
    pub fn new<R, T>(bus: Bus, select: Select, mut reset: R, mut timer: T) -> Result<Self, &'static str>
    where
        R: OutputPin,
        T: DelayNs,
    {
        // Pull reset to low and wait until the reset is triggered
        reset.set_low().map_err(|_| "Failed to pull reset line to low")?;
        timer.delay_ms(1);

        // Pull reset to high again and give the chip some time to boot
        reset.set_high().map_err(|_| "Failed to pull reset line to high")?;
        timer.delay_ms(10);

        // Validate chip revision to assure the protocol matches
        let mut wire = Rfm95Connection::init(bus, select);
        #[cfg(not(feature = "debug"))]
        {
            // Get chip revision
            let silicon_revision = wire.read(RegVersion)?;
            let true = Self::SUPPORTED_SILICON_REVISIONS.contains(&silicon_revision) else {
                // Raise an error here since other revisions may be incompatible
                return Err("Unsupported silicon revision");
            };
        }

        // Go to sleep, switch to LoRa and enter standby
        wire.write(RegOpModeMode, Self::REG_OPMODE_MODE_SLEEP)?;
        wire.write(RegOpModeLongRangeMode, Self::REG_OPMODE_LONGRANGEMODE_LORA)?;
        wire.write(RegOpModeMode, Self::REG_OPMODE_MODE_STANDBY)?;
        wire.write(RegOpModeAccessSharedReg, Self::REG_OPMODE_ACCESSSHAREDREG_LORA)?;

        // Set TX and RX base address to 0 to use the entire available FIFO space, and the power amplifier to max
        wire.write(RegFifoTxBaseAddr, 0x00)?;
        wire.write(RegFifoRxBaseAddr, 0x00)?;
        wire.write(RegPaConfig, 0xFF)?;

        // Init self
        Ok(Self { spi: wire })
    }

    /// Applies the given config (useful for initialization)
    pub fn set_config(&mut self, config: &Config) -> Result<(), &'static str> {
        self.set_spreading_factor(config.spreading_factor())?;
        self.set_bandwidth(config.bandwidth())?;
        self.set_coding_rate(config.coding_rate())?;
        self.set_polarity(config.polarity())?;
        self.set_header_mode(config.header_mode())?;
        self.set_crc_mode(config.crc_mode())?;
        self.set_sync_word(config.sync_word())?;
        self.set_preamble_len(config.preamble_len())?;
        self.set_frequency(config.frequency())?;
        Ok(())
    }

    /// The current spreading factor
    pub fn spreading_factor(&mut self) -> Result<SpreadingFactor, &'static str> {
        let spreading_factor = self.spi.read(RegModemConfig2SpreadingFactor)?;
        SpreadingFactor::try_from(spreading_factor)
    }
    /// Set the spreading factor
    pub fn set_spreading_factor<T>(&mut self, spreading_factor: T) -> Result<(), &'static str>
    where
        T: Into<SpreadingFactor>,
    {
        // Get config to determine the need for LDO
        let spreading_factor = spreading_factor.into();
        let bandwidth = self.bandwidth()?;
        let needs_ldo = airtime::needs_ldo(spreading_factor, bandwidth);

        // Set registers
        self.spi.write(RegModemConfig2SpreadingFactor, spreading_factor as u8)?;
        self.spi.write(RegModemConfig3LowDataRateOptimize, needs_ldo as u8)?;
        Ok(())
    }

    /// The current bandwidth
    pub fn bandwidth(&mut self) -> Result<Bandwidth, &'static str> {
        let bandwidth = self.spi.read(RegModemConfig1Bw)?;
        Bandwidth::try_from(bandwidth)
    }
    /// Sets the bandwidth
    pub fn set_bandwidth<T>(&mut self, bandwidth: T) -> Result<(), &'static str>
    where
        T: Into<Bandwidth>,
    {
        // Get config to determine the need for LDO
        let bandwidth = bandwidth.into();
        let spreading_factor = self.spreading_factor()?;
        let needs_ldo = airtime::needs_ldo(spreading_factor, bandwidth);

        // Set registers
        self.spi.write(RegModemConfig1Bw, bandwidth as u8)?;
        self.spi.write(RegModemConfig3LowDataRateOptimize, needs_ldo as u8)?;
        Ok(())
    }

    /// The current coding rate
    pub fn coding_rate(&mut self) -> Result<CodingRate, &'static str> {
        let coding_rate = self.spi.read(RegModemConfig1CodingRate)?;
        CodingRate::try_from(coding_rate)
    }
    /// Sets the coding rate
    pub fn set_coding_rate<T>(&mut self, coding_rate: T) -> Result<(), &'static str>
    where
        T: Into<CodingRate>,
    {
        let coding_rate = coding_rate.into();
        self.spi.write(RegModemConfig1CodingRate, coding_rate as u8)
    }

    /// The current IQ polarity
    pub fn polarity(&mut self) -> Result<Polarity, &'static str> {
        let polarity = self.spi.read(RegInvertIQ)?;
        Polarity::try_from(polarity)
    }
    /// Sets the IQ polarity
    pub fn set_polarity<T>(&mut self, polarity: T) -> Result<(), &'static str>
    where
        T: Into<Polarity>,
    {
        let polarity = polarity.into();
        self.spi.write(RegInvertIQ, polarity as u8)
    }

    /// The current header mode
    pub fn header_mode(&mut self) -> Result<HeaderMode, &'static str> {
        let header_mode = self.spi.read(RegModemConfig1ImplicitHeaderModeOn)?;
        HeaderMode::try_from(header_mode)
    }
    /// Sets the header mode
    pub fn set_header_mode<T>(&mut self, header_mode: T) -> Result<(), &'static str>
    where
        T: Into<HeaderMode>,
    {
        let header_mode = header_mode.into();
        self.spi.write(RegModemConfig1ImplicitHeaderModeOn, header_mode as u8)
    }

    /// The current CRC mode
    pub fn crc_mode(&mut self) -> Result<CrcMode, &'static str> {
        let crc_mode = self.spi.read(RegModemConfig2RxPayloadCrcOn)?;
        CrcMode::try_from(crc_mode)
    }
    /// Sets the CRC mode
    pub fn set_crc_mode<T>(&mut self, crc: T) -> Result<(), &'static str>
    where
        T: Into<CrcMode>,
    {
        let crc = crc.into();
        self.spi.write(RegModemConfig2RxPayloadCrcOn, crc as u8)
    }

    /// The current sync word
    pub fn sync_word(&mut self) -> Result<SyncWord, &'static str> {
        let sync_word = self.spi.read(RegSyncWord)?;
        Ok(SyncWord::new(sync_word))
    }
    /// Sets the sync word
    pub fn set_sync_word<T>(&mut self, sync_word: T) -> Result<(), &'static str>
    where
        T: Into<SyncWord>,
    {
        let sync_word = sync_word.into();
        self.spi.write(RegSyncWord, sync_word.into())
    }

    /// The current preamble length
    pub fn preamble_len(&mut self) -> Result<PreambleLength, &'static str> {
        // Read registers
        let preamble_len_msb = self.spi.read(RegPreambleMsb)?;
        let preamble_len_lsb = self.spi.read(RegPreambleLsb)?;

        // Create preamble length
        let preamble_len = u16::from_be_bytes([preamble_len_msb, preamble_len_lsb]);
        Ok(PreambleLength::new(preamble_len))
    }
    /// Sets the preamble length
    pub fn set_preamble_len<T>(&mut self, len: T) -> Result<(), &'static str>
    where
        T: Into<PreambleLength>,
    {
        let [preamble_len_msb, preamble_len_lsb] = u16::from(len.into()).to_be_bytes();
        self.spi.write(RegPreambleMsb, preamble_len_msb)?;
        self.spi.write(RegPreambleLsb, preamble_len_lsb)
    }

    /// The current frequency
    pub fn frequency(&mut self) -> Result<Frequency, &'static str> {
        // Read frequency from registers
        let frequency_msb = self.spi.read(RegFrMsb)?;
        let frequency_mid = self.spi.read(RegFrMid)?;
        let frequency_lsb = self.spi.read(RegFrLsb)?;
        let frequency_raw = u64::from_be_bytes([0, 0, 0, 0, 0, frequency_msb, frequency_mid, frequency_lsb]);

        // Translate crystal native frequency into Hz
        #[allow(clippy::arithmetic_side_effects, reason = "Can never overflow")]
        let frequency_khz = frequency_raw * Self::FREQUENCY_DIVIDER_MILLIHZ;
        let frequency = (frequency_khz / 1000) as u32;
        Ok(Frequency::hz(frequency))
    }
    /// Sets the frequency
    pub fn set_frequency<T>(&mut self, frequency: T) -> Result<(), &'static str>
    where
        T: Into<Frequency>,
    {
        // Set the modem to high- or low-frequency mode (low-frequency is `1`)
        let frequency = frequency.into();
        let frequency_mode = (frequency < Self::HIGH_FREQUENCY_THRESHOLD) as u8;
        self.spi.write(RegOpModeLowFrequencyModeOn, frequency_mode)?;

        // Translate the frequency into the crystal native frequency
        // Note: We go via kHz/mHz to keep higher precision without floats
        #[allow(clippy::arithmetic_side_effects, reason = "Can never overflow")]
        let frequency_khz = u32::from(frequency) as u64 * 1000;
        let [_, _, _, _, _, frequency_msb, frequency_mid, frequency_lsb] =
            (frequency_khz / Self::FREQUENCY_DIVIDER_MILLIHZ).to_be_bytes();

        // Write the frequency to the registers
        self.spi.write(RegFrMsb, frequency_msb)?;
        self.spi.write(RegFrMid, frequency_mid)?;
        self.spi.write(RegFrLsb, frequency_lsb)?;
        Ok(())
    }

    /// Schedules a single TX operation with the given data and returns immediately
    ///
    /// # Non-Blocking
    /// This functions schedules the TX operation and returns immediately. To check if the TX operation is done, use
    /// [`Self::complete_tx`].
    pub fn start_tx(&mut self, data: &[u8]) -> Result<(), &'static str> {
        // Validate input length
        let 1..=RFM95_FIFO_SIZE = data.len() else {
            // The message is empty or too long
            return Err("Invalid TX data length");
        };

        // Copy packet into FIFO...
        for (index, byte) in data.iter().enumerate() {
            // Set destination address and write byte
            self.spi.write(RegFifoAddrPtr, index as u8)?;
            self.spi.write(RegFifo, *byte)?;
        }
        // ... and set packet length
        self.spi.write(RegPayloadLength, data.len() as u8)?;

        // Enable and reset possible old interrupt
        self.spi.write(RegIrqFlagsMaskTxDoneMask, 0)?;
        self.spi.write(RegIrqFlagsTxDone, 1)?;

        // Start TX
        self.spi.write(RegOpModeMode, Self::REG_OPMODE_MODE_TXSINGLE)?;
        Ok(())
    }
    /// Checks if a single TX operation has completed, and returns the amount of bytes sent
    ///
    /// # Non-Blocking
    /// This function is non-blocking. If the TX operation is not done yet, it returns `Ok(None)`.
    pub fn complete_tx(&mut self) -> Result<Option<usize>, &'static str> {
        // Check for TX done
        let 0b1 = self.spi.read(RegIrqFlagsTxDone)? else {
            // The TX operation has not been completed yet
            return Ok(None);
        };

        // Get and return the amount of bytes sent
        let written = self.spi.read(RegPayloadLength)?;
        Ok(Some(written as usize))
    }

    /// Computes the maximum RX timeout for the current configured spreading factor and bandwidth
    ///
    /// # Maximum Timeout
    /// The maximum timeout is dependent on the symbol length, which in turn depends on the configured spreading factor
    /// and bandwidth. This means that computed timeout is only valid for a given spreading-factor+bandwidth combination
    /// and must be recomputed if those parameters change.
    ///
    /// # Implementation details
    /// The RFM95 timeout counter works by counting symbols, and supports a maximum timeout of 1023 symbols. To compute
    /// the maximum timeout, we take the configured [`Self::spreading_factor`] and [`Self::bandwidth`], and get the
    /// duration of a single symbol via [`crate::lora::airtime::symbol_airtime`]. The maximum timeout is the duration of
    /// a single symbol, multiplied with `1023`.
    pub fn rx_timeout_max(&mut self) -> Result<Duration, &'static str> {
        // Get current config
        let spreading_factor = self.spreading_factor()?;
        let bandwidth = self.bandwidth()?;

        // Compute timeout
        let airtime_symbol = airtime::symbol_airtime(spreading_factor, bandwidth);
        Ok(airtime_symbol.saturating_mul(1023))
    }
    /// Schedules a single RX operation and returns immediately
    ///
    /// # Non-Blocking
    /// This functions schedules the RX operation and returns immediately. To check if the TX operation is done and to
    /// get the received data, use [`Self::complete_tx`].
    ///
    /// # Maximum Timeout
    /// The RFM95 timeout counter works by counting symbols, and is thus dependent on the configured spreading factor
    /// and bandwidth. See also [`Self::rx_timeout_max`].
    pub fn start_rx(&mut self, timeout: Duration) -> Result<(), &'static str> {
        // Get the current symbol airtime in microseconds
        let spreading_factor = self.spreading_factor()?;
        let bandwidth = self.bandwidth()?;
        let symbol_airtime = airtime::symbol_airtime(spreading_factor, bandwidth);
        let symbol_airtime_micros = symbol_airtime.as_micros() as i32;

        // Compute the raw timeout
        let timeout_micros = i32::try_from(timeout.as_micros()).map_err(|_| "Timeout is too long")?;
        let timeout_symbols @ 0..1024 = airtime::ceildiv(timeout_micros, symbol_airtime_micros) as u32 else {
            // This timeout is too large to be configured
            return Err("Effective timeout is too large");
        };

        // Configure the timeout and reset the address pointer
        self.spi.write(RegModemConfig2SymbTimeout98, (timeout_symbols >> 8) as u8)?;
        self.spi.write(RegSymbTimeoutLsb, timeout_symbols as u8)?;
        self.spi.write(RegFifoAddrPtr, 0x00)?;

        // Enable interrupts
        self.spi.write(RegIrqFlagsMaskRxDoneMask, 0)?;
        self.spi.write(RegIrqFlagsMaskRxTimeoutMask, 0)?;
        self.spi.write(RegIrqFlagsMaskPayloadCrcErrorMask, 0)?;

        // Reset possible old interrupts
        self.spi.write(RegIrqFlagsRxDone, 1)?;
        self.spi.write(RegIrqFlagsRxTimeout, 1)?;
        self.spi.write(RegIrqFlagsPayloadCrcError, 1)?;

        // Start RX
        self.spi.write(RegOpModeMode, Self::REG_OPMODE_MODE_RXSINGLE)?;
        Ok(())
    }
    /// Checks if a single RX operation has completed, copies the message into `buf` and returns the amount of bytes
    /// received
    ///
    /// # Non-Blocking
    /// This function is non-blocking. If the RX operation is not done yet, it returns `Ok(None)`.
    ///
    /// # Timeout or CRC errors
    /// If the receive operation times out or the received message is corrupt,
    #[allow(clippy::missing_panics_doc, reason = "The panic should never occur during regular operation")]
    pub fn complete_rx(&mut self, buf: &mut [u8]) -> Result<Option<usize>, &'static str> {
        // Check for errors
        let 0b0 = self.spi.read(RegIrqFlagsRxTimeout)? else {
            // The RX operation has timeouted
            return Err("RX timeout");
        };
        let 0b0 = self.spi.read(RegIrqFlagsPayloadCrcError)? else {
            // The RX operation has failed
            return Err("RX CRC error");
        };

        // Check for RX done
        let 0b1 = self.spi.read(RegIrqFlagsRxDone)? else {
            // The RX operation has not been completed yet
            return Ok(None);
        };

        // Get packet begin and length
        let start = self.spi.read(RegFifoRxCurrentAddr)?;
        let len = self.spi.read(RegRxNbBytes)?;
        let to_copy = cmp::min(len as usize, buf.len());

        // Copy data from FIFO
        for (index, slot) in buf.iter_mut().take(to_copy).enumerate() {
            // Validate the index
            #[allow(clippy::expect_used, reason = "The values from the modem should be always valid")]
            let offset = start.checked_add(index as u8).expect("FIFO out of bound access");

            // Set source address and read byte
            self.spi.write(RegFifoAddrPtr, offset)?;
            *slot = self.spi.read(RegFifo)?;
        }

        // Return the amount of bytes copied
        Ok(Some(len as usize))
    }

    /// When operating in the high frequency range the RSSI register values are offset by this much. 
    const HF_RSSI_OFFSET: i16 = -157;

    /// When operating in the low frequency range the RSSI register values are offset by this much. 
    const LF_RSSI_OFFSET: i16 = -164;
    
    /// The cutoff frequency that separates the high and low frequency ranges. Anything below this value is LF.
    const HF_LF_BOUNDARY_FREQ: Frequency = Frequency::hz(779_000_000);

    /// Get the signal strength of the last recieved packet.
    /// 
    /// Unlike RSSI, this accounts for LoRa's ability to recieve packets below the noise floor.
    pub fn get_packet_strength(&mut self) -> Result<i16, &'static str> {
        let offset = if self.frequency()? >= Self::HF_LF_BOUNDARY_FREQ {Self::HF_RSSI_OFFSET} else {Self::LF_RSSI_OFFSET};
        let snr = self.get_packet_snr()?;
        if snr >= 0 {
            Ok( self.spi.read(RegRssiValue)? as i16 + offset )
        }
        else {
            Ok( self.spi.read(RegPktRssiValue)? as i16 + snr as i16 + offset )
        }
    }

    /// Get a Relative Signal Strength Indicator (RSSI) of the last recieved packet.
    pub fn get_rssi(&mut self) -> Result<i16, &'static str> {
        let offset = if self.frequency()? > Self::HF_LF_BOUNDARY_FREQ {Self::HF_RSSI_OFFSET} else {Self::LF_RSSI_OFFSET};
        if self.get_packet_snr()? >= 0 {
            Ok( self.spi.read(RegPktRssiValue)? as i16 * 16/15 + offset )
        }
        else {
            Ok( self.spi.read(RegRssiValue)? as i16 + offset )
        }
    }

    /// Get the Signal to Noise Ratio (SNR) of the last recieved packet.
    pub fn get_packet_snr(&mut self) -> Result<i8, &'static str> {
        // The value is stored in two's complement form in the register, so the cast to i8 is fine
        Ok( (self.spi.read(RegPktSnrValue)? as i8 ) / 4)
    }

    /// Dumps all used registers; usefule for debugging purposes
    #[cfg(feature = "debug")]
    pub fn dump_registers(&mut self) -> Result<[u8; REGISTER_MAX as usize + 1], &'static str> {
        // A dynamic register for dumping purposes
        struct DynamicRegister(u8);
        impl Register for DynamicRegister {
            fn address(&self) -> u8 {
                self.0
            }
        }

        // Dump all registers
        let mut dump = [0; REGISTER_MAX as usize + 1];
        for (register, slot) in dump.iter_mut().enumerate() {
            // Read register
            let register = DynamicRegister(register as u8);
            *slot = self.spi.read(register)?;
        }
        Ok(dump)
    }
    /// Dumps the entire FIFO contents
    #[cfg(feature = "debug")]
    pub fn dump_fifo(&mut self) -> Result<[u8; RFM95_FIFO_SIZE], &'static str> {
        // Save FIFO position
        let fifo_position = self.spi.read(RegFifoAddrPtr)?;

        // Dump all registers
        let mut dump = [0; RFM95_FIFO_SIZE];
        for (index, slot) in dump.iter_mut().enumerate() {
            // Read register
            self.spi.write(RegFifoAddrPtr, index as u8)?;
            *slot = self.spi.read(RegFifo)?;
        }

        // Re-apply old FIFO position
        self.spi.write(RegFifoAddrPtr, fifo_position)?;
        Ok(dump)
    }
}
impl<Bus, Select> Debug for Rfm95Driver<Bus, Select>
where
    Bus: SpiBus,
    Select: OutputPin,
{
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        f.debug_struct("Rfm95Driver").field("spi", &self.spi).finish()
    }
}
