#![allow(dead_code)]

use core::time::Duration;

use embedded_lora_rfm95::{lora::types::{Bandwidth, CodingRate, CrcMode, HeaderMode, Polarity, PreambleLength, SpreadingFactor, SyncWord}, rfm95::{self, Rfm95Driver}};
use embedded_hal_compat::{eh1_0::delay::DelayNs, Forward, ForwardCompat};
use msp430fr2x5x_hal::{delay::Delay, gpio::{Output, Pin, Pin4}, spi::SpiBus, pac::P4};
use crate::pin_mappings::{RadioCsPin, RadioEusci, RadioResetPin, RadioSpi};

const LORA_FREQ_HZ: u32 = 915_000_000;

pub fn new(spi: RadioSpi, cs_pin: RadioCsPin, reset_pin: RadioResetPin, delay: Delay) -> Radio {
    let mut rfm95 = Rfm95Driver::new(spi.forward(), cs_pin.forward(), reset_pin.forward(), DelayWrapper(delay)).unwrap();

    let lora_config = embedded_lora_rfm95::lora::config::Builder::builder()
        .set_bandwidth(Bandwidth::B7_8) // lowest bandwidth == longest range
        .set_coding_rate(CodingRate::C4_8) // Maximum error correction
        .set_crc_mode(CrcMode::Disabled)
        .set_frequency(LORA_FREQ_HZ.into()) // Hz
        .set_header_mode(HeaderMode::Explicit)
        .set_polarity(Polarity::Normal)
        .set_preamble_length(PreambleLength::L8)
        .set_spreading_factor(SpreadingFactor::S10) // High SF == Best range
        .set_sync_word(SyncWord::PRIVATE);
    rfm95.set_config(&lora_config).unwrap();

    Radio{radio: rfm95}
}

pub type RFM95 = Rfm95Driver<Forward<SpiBus<RadioEusci>>, Forward<Pin<P4, Pin4, Output>, embedded_hal_compat::markers::ForwardOutputPin>>;
/// Top-level interface for the radio module.
pub struct Radio {
    radio: RFM95,
}
impl Radio {
    /// Transmit data and wait until transmission is complete.
    /// 
    /// Panics upon recieving any error from the radio module.
    pub fn blocking_transmit(&mut self, data: &[u8]) {
        self.radio.start_tx(data).unwrap();
        loop {
            match self.radio.complete_tx(){
                Ok(None) => continue,   // Still sending
                Ok(_) => return,        // Sending complete
                Err(e) => panic!("{e}"),
            }
        }
    }
    /// Try to recieve data, and don't return until a packet is recieved.
    /// 
    /// Panics upon recieving any non-timeout error from the radio module.
    pub fn blocking_recieve<'a>(&mut self, buf: &'a mut [u8; rfm95::RFM95_FIFO_SIZE]) -> &'a [u8] {
        let size;
        'outer: loop {
            let max_timeout = self.radio.rx_timeout_max().unwrap();
            self.radio.start_rx(max_timeout).unwrap();
            
            'inner: loop {
                match self.radio.complete_rx(buf) {
                    Ok(Some(n)) => {size = n; break 'outer;},
                    Ok(None) => continue,
                    Err("RX timeout") => break 'inner,
                    Err(e) => panic!("{e}"),
                };
            };
        }
        &buf[0..size]
    }
    /// Begin transmission and return immediately. Check whether the transmission is complete by calling `async_transmit_is_complete()`.
    pub fn async_transmit_start(&mut self, data: &[u8]) {
        self.radio.start_tx(data).unwrap();
    }

    /// Check whether the radio has finished sending.
    /// 
    /// Panics upon recieving any error from the radio module.
    pub fn async_transmit_is_complete(&mut self) -> bool {
        match self.radio.complete_tx(){
            Ok(None) => false,    // Still sending
            Ok(_) => true,        // Sending complete
            Err(e) => panic!("{e}"),
        }
    }
    /// Tell the radio to listen for a packet and return immediately. Check whether anything was recieved by calling `async_recieve_is_complete()`.
    /// 
    /// A timeout value is optional, if none is provided the maximum timeout is used. You should prepare to deal with timeouts.
    pub fn async_recieve_start(&mut self, timeout: Option<Duration>) {
        let timeout = match timeout {
            Some(t) => t,
            None => self.radio.rx_timeout_max().unwrap(),
        };
        self.radio.start_rx(timeout).unwrap();
    }

    /// Check whether the radio has recieved a packet. If so, returns the packet as a slice of bytes.
    /// 
    /// If not, returns either `StillRecieving` or `RxTimeout`. In the timeout case you should call `async_recieve_start()` again.
    /// 
    /// Panics upon recieving any non-timeout error from the radio module.
    pub fn async_recieve_is_complete<'a>(&mut self, buf: &'a mut [u8; rfm95::RFM95_FIFO_SIZE]) -> Result<&'a [u8], RadioRecieveError> {
        let size = match self.radio.complete_rx(buf) {
            Ok(Some(n)) => n,
            Ok(None) => return Err(RadioRecieveError::StillRecieving),
            Err("RX timeout") => return Err(RadioRecieveError::RxTimeout),
            Err(e) => panic!("{e}"),
        };
        Ok(&buf[0..size])
    }
}

pub enum RadioRecieveError {
    RxTimeout,
    StillRecieving,
}

use embedded_hal::blocking::delay::DelayMs;

// The radio library uses a different version of embedded_hal, so we need to write some wrappers.
struct DelayWrapper(Delay);
impl DelayNs for DelayWrapper {
    fn delay_ms(&mut self, ms: u32) {
        if ms < (u16::MAX as u32) {
            self.0.delay_ms(ms as u16);
        }
        else {
            let times = ms/(u16::MAX as u32);

            for _ in 0..times {
                self.0.delay_ms(u16::MAX);
            }
            let remainder = ms - times*(u16::MAX as u32);
            self.0.delay_ms(remainder as u16);
        }
    }
    
    fn delay_ns(&mut self, ns: u32) {
        let ms = ns / 1_000_000;
        self.0.delay_ms(ms as u16);
    }
}