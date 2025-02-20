use embedded_lora_rfm95::{lora::types::{Bandwidth, CodingRate, CrcMode, HeaderMode, Polarity, PreambleLength, SpreadingFactor, SyncWord}, rfm95::Rfm95Driver};
use embedded_hal_compat::{eh1_0::delay::DelayNs, Forward, ForwardCompat};
use msp430fr2355::{E_USCI_B1, P4, P5};
use msp430fr2x5x_hal::{delay::Delay, gpio::{Output, Pin, Pin2, Pin4}, spi::SpiBus};

pub type RFM95 = Rfm95Driver<Forward<SpiBus<E_USCI_B1>>, Forward<Pin<P4, Pin4, Output>, embedded_hal_compat::markers::ForwardOutputPin>>;
pub fn prepare_radio(spi: SpiBus<E_USCI_B1>, cs_pin: Pin<P4, Pin4, Output>, reset_pin:  Pin<P5, Pin2, Output>, delay: Delay) -> RFM95 {
    let mut rfm95 = Rfm95Driver::new(spi.forward(), cs_pin.forward(), reset_pin.forward(), DelayWrapper(delay)).unwrap();

    let lora_config = embedded_lora_rfm95::lora::config::Builder::builder()
        .set_bandwidth(Bandwidth::B7_8) // lowest bandwidth == longest range
        .set_coding_rate(CodingRate::C4_8) // Maximum error correction
        .set_crc_mode(CrcMode::Disabled)
        .set_frequency(915_000_000.into()) // Hz
        .set_header_mode(HeaderMode::Explicit)
        .set_polarity(Polarity::Normal)
        .set_preamble_length(PreambleLength::L8)
        .set_spreading_factor(SpreadingFactor::S10) // High SF == Best range
        .set_sync_word(SyncWord::PRIVATE);
    rfm95.set_config(&lora_config).unwrap();
    rfm95
}

use embedded_hal::blocking::delay::DelayMs;
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