#![allow(dead_code)]

use core::num::{ParseFloatError, ParseIntError};

use arrayvec::ArrayVec;
use msp430fr2x5x_hal::{
    clock::Smclk, 
    serial::{BitCount, BitOrder, Loopback, Parity, SerialConfig, StopBits}};
use embedded_hal::serial::Read;
use crate::pin_mappings::{GpsEusci, GpsRx, GpsRxPin, GpsTx, GpsTxPin};

const NMEA_MESSAGE_MAX_LEN: usize = 82;

pub struct Gps {
    tx: GpsTx,
    rx: GpsRx,
}
impl Gps {
    pub fn new(eusci_reg: GpsEusci, smclk: &Smclk, tx_pin: GpsTxPin, rx_pin: GpsRxPin) -> Self {
        // Configure UART peripheral
        let (tx, rx) = SerialConfig::new(eusci_reg, 
            BitOrder::LsbFirst, 
            BitCount::EightBits, 
            StopBits::OneStopBit, 
            Parity::NoParity, 
            Loopback::NoLoop, 
            9600)
            .use_smclk(smclk)
            .split(tx_pin, rx_pin);
        Self {tx, rx}
    } 
    /// Get a GPS GGA packet as a `&[u8]`. Useful if you're just sending over the radio.
    pub fn get_raw_gga_packet<'a>(&mut self, buf: &'a mut [AsciiChar; NMEA_MESSAGE_MAX_LEN]) -> &'a [AsciiChar] {
        loop {
            // Wait until start of a message. Messages begin with '$'
            loop {
                if let Ok(b'$') = self.rx.read() { break }
            }
            buf[0] = b'$';

            // Store message ID
            for i in 1..=5 {
                buf[i] = self.rx.read().unwrap_or_else(|_| panic!("Err in serial read"));
            }

            // Check if the message is a GGA-type message. If not, wait for the start of the next message.
            if buf[3..=5] != *b"GGA" { continue; }

            let mut i = 0;
            for (idx, chr) in buf.iter_mut().enumerate().skip(6) {
                i = idx;
                *chr = match self.rx.read() {
                    Ok(b'\n') => break,
                    Ok(c) => c,
                    Err(_) => panic!("Err in serial read"),
                }
            }
            buf[i] = b'\n';
            return &buf[0..=i];
        }
    }
    /// Get a GPS GGA packet as a struct with native fields. Useful for interpreting.
    pub fn try_get_packet_as_struct(&mut self) -> Result<GpsGgaPacket, IntOrFloatParseError> {
        let mut buf = [b'\0'; NMEA_MESSAGE_MAX_LEN];
        let str = self.get_raw_gga_packet(&mut buf);
        AsciiGpsGgaPacket::try_from(str).unwrap().try_into()
    }
}

type AsciiChar = u8;

// A GGA packet in intermediate ASCII form.
#[derive(Default)]
pub struct AsciiGpsGgaPacket {
    utc_time:   [AsciiChar; 10],
    latitude:   [AsciiChar; 9],
    north_south_indicator: AsciiChar,
    longitude:  [AsciiChar; 10],
    east_west_indicator: AsciiChar,
    position_fix_indicator: AsciiChar,
    num_satellites: ArrayVec<AsciiChar, 2>,
    mean_sea_level_altitude: ArrayVec<AsciiChar, 7>,
    altitude_units: AsciiChar,
}

impl TryFrom<&[AsciiChar]> for AsciiGpsGgaPacket {
    type Error = GgaParseError;

    fn try_from(message: &[AsciiChar]) -> Result<Self, Self::Error> {
        let sections = message.split(|&c| c == b',');
        if sections.clone().count() != 15 { return Err(GgaParseError::WrongSectionCount) }
        let mut packet = Self::default();
        for (i, section) in sections.enumerate() {
            match i {
                1  => if section.len() != 10 {return Err(GgaParseError::WrongSectionLength)} else { packet.utc_time.clone_from_slice(section) },
                2  => if section.len() != 9  {return Err(GgaParseError::WrongSectionLength)} else { packet.latitude.clone_from_slice(section) },
                3  => packet.north_south_indicator = section[0],
                4  => if section.len() != 10 {return Err(GgaParseError::WrongSectionLength)} else { packet.longitude.clone_from_slice(section) },
                5  => packet.east_west_indicator = section[0],
                6  => packet.position_fix_indicator = section[0],
                7  => packet.num_satellites.try_extend_from_slice(section).map_err(|_| GgaParseError::WrongSectionLength)?,
                9  => packet.mean_sea_level_altitude.try_extend_from_slice(section).map_err(|_| GgaParseError::WrongSectionLength)?,
                10 => packet.altitude_units = section[0],
                _ => continue,
            };
        } 
        Ok(packet)
    }
}
#[derive(Debug)]
pub enum GgaParseError {
    WrongSectionCount,
    InvalidChecksum,
    WrongSectionLength,
}

// A GGA packet in native form. Useful for interpreting the results on-device.
pub struct GpsGgaPacket {
    utc_time: UtcTime,
    latitude: f32,
    north_south_indicator: AsciiChar,
    longitude: f32,
    east_west_indicator: AsciiChar,
    position_fix_indicator: PositionFixIndicator,
    num_satellites: u8,
    mean_sea_level_altitude: f32,
}
impl TryFrom<AsciiGpsGgaPacket> for GpsGgaPacket {
    type Error = IntOrFloatParseError;
    fn try_from(packet: AsciiGpsGgaPacket) -> Result<Self, Self::Error> {
        let utc_time = UtcTime {
            hours:   bytes_to_u8(&packet.utc_time[0..2])?, 
            minutes: bytes_to_u8(&packet.utc_time[2..4])?, 
            seconds: bytes_to_u8(&packet.utc_time[4..6])?, 
            // Full stop at packet.utc_time[6]
            millis:  bytes_to_u16(&packet.utc_time[7..])?,
        };

        let latitude: f32  = bytes_to_f32(&packet.latitude)?;
        let longitude: f32 = bytes_to_f32(&packet.longitude)?;
        let mean_sea_level_altitude: f32 = bytes_to_f32(&packet.mean_sea_level_altitude)?;
        let position_fix_indicator: PositionFixIndicator = match packet.position_fix_indicator {
            b'0' => PositionFixIndicator::None,
            b'1' => PositionFixIndicator::Gps,
            b'2' => PositionFixIndicator::DifferentialGps,
            _ => unreachable!(), // Should be unreachable
        };
        let north_south_indicator = packet.north_south_indicator;
        let east_west_indicator = packet.east_west_indicator;
        let num_satellites: u8 = bytes_to_u8(&packet.num_satellites)?;

        Ok( GpsGgaPacket{utc_time, latitude, north_south_indicator, longitude, 
                east_west_indicator, position_fix_indicator, num_satellites, mean_sea_level_altitude} )
    }
}

fn bytes_to_u8(bytes: &[u8]) -> Result<u8, IntOrFloatParseError> {
    core::str::from_utf8(bytes).unwrap().parse().map_err(IntOrFloatParseError::IntError)
}
fn bytes_to_u16(bytes: &[u8]) -> Result<u16, IntOrFloatParseError> {
    core::str::from_utf8(bytes).unwrap().parse().map_err(IntOrFloatParseError::IntError)
}
fn bytes_to_f32(bytes: &[u8]) -> Result<f32, IntOrFloatParseError> {
    core::str::from_utf8(bytes).unwrap().parse().map_err(IntOrFloatParseError::FloatError)
}

#[derive(Debug)]
pub enum IntOrFloatParseError {
    IntError(ParseIntError),
    FloatError(ParseFloatError),
}

struct UtcTime {
    hours: u8,
    minutes: u8,
    seconds: u8,
    millis: u16, 
}

enum PositionFixIndicator {
    None = 0,
    Gps = 1,
    DifferentialGps = 2,
}