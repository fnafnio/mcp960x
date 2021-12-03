#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]
#![feature(inherent_associated_types)]

// This mod MUST go first, so that the others see its macros.
pub(crate) mod fmt;

#[cfg(feature = "async")]
pub mod embassy_async;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Address {
    Addr0 = 0x66,
    Addr1 = 0x67,
}

impl Default for Address {
    fn default() -> Self {
        Self::Addr0
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum Register {
    HotJunctionTemp = 0b000,
    JunctionTempDelta = 0b001,
    ColdJunctionTemp = 0b010,
    AdcRawData = 0b011,
    Status = 0b100,
    SensorConfiguration = 0b101,
    DeviceConfiguration = 0b110,
    Alert1Config = 0b1000,
    Alert2Config = 0b1001,
    Alert3Config = 0b1010,
    Alert4Config = 0b1011,
    Alert1Hysteresis = 0b1100,
    Alert2Hysteresis = 0b1101,
    Alert3Hysteresis = 0b1110,
    Alert4Hysteresis = 0b1111,
    Alert1Limit = 0b10000,
    Alert2Limit = 0b10001,
    Alert3Limit = 0b10010,
    Alert4Limit = 0b10011,
    DeviceId = 0b100000,
}

use num_derive::{FromPrimitive, ToPrimitive};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum Type {
    K = 0b000,
    J = 0b001,
    T = 0b010,
    N = 0b011,
    S = 0b100,
    E = 0b101,
    B = 0b110,
    R = 0b111,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum Filter {
    N0 = 0b000,
    N1 = 0b001,
    N2 = 0b010,
    N3 = 0b011,
    N4 = 0b100,
    N5 = 0b101,
    N6 = 0b110,
    N7 = 0b111,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug)]
pub struct DeviceConfig {
    pub resolution: Resolution,
    pub adc_resolution: AdcResolution,
    pub burst_mode: BurstMode,
    pub shutdown: Shutdown,
}

impl From<u8> for DeviceConfig {
    fn from(val: u8) -> Self {
        let shutdown = num_traits::FromPrimitive::from_u8(val & 0b11).unwrap();
        let burst_mode = num_traits::FromPrimitive::from_u8((val >> 2) & 0b111).unwrap();
        let adc_resolution = num_traits::FromPrimitive::from_u8((val >> 5) & 0b11).unwrap();
        let resolution = num_traits::FromPrimitive::from_u8((val >> 7) & 0b1).unwrap();

        Self {
            resolution,
            adc_resolution,
            burst_mode,
            shutdown,
        }
    }
}

impl Default for DeviceConfig {
    fn default() -> Self {
        Self {
            resolution: Resolution::High,
            adc_resolution: AdcResolution::R14,
            burst_mode: BurstMode::S8,
            shutdown: Shutdown::Burst,
        }
    }
}

impl DeviceConfig {
    pub fn as_byte(&self) -> u8 {
        (self.resolution as u8) << 7
            | (self.adc_resolution as u8) << 5
            | (self.burst_mode as u8) << 2
            | self.shutdown as u8
    }
}

impl Into<u8> for DeviceConfig {
    fn into(self) -> u8 {
        self.as_byte()
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum Resolution {
    High = 0,
    Low = 1,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum AdcResolution {
    R18 = 0,
    R16 = 1,
    R14 = 2,
    R12 = 3,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum BurstMode {
    S1 = 0b000,
    S2 = 0b001,
    S4 = 0b010,
    S8 = 0b011,
    S16 = 0b100,
    S32 = 0b101,
    S64 = 0b110,
    S128 = 0b111,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum Shutdown {
    Normal = 0,
    Shutdown = 1,
    Burst = 2,
}
pub struct SensorConfig {
    t_type: Type,
    filter: Filter,
}

impl SensorConfig {
    pub fn as_byte(&self) -> u8 {
        (self.t_type as u8) << 4 | self.filter as u8
    }
}

impl Default for SensorConfig {
    fn default() -> Self {
        Self {
            t_type: Type::K,
            filter: Filter::N3,
        }
    }
}

impl From<u8> for SensorConfig {
    fn from(val: u8) -> Self {
        let filter: Filter = num_traits::FromPrimitive::from_u8(val & 0b111).unwrap();
        let t_type: Type = num_traits::FromPrimitive::from_u8((val >> 4) & 0b111).unwrap();
        Self { t_type, filter }
    }
}

impl Into<u8> for SensorConfig {
    fn into(self) -> u8 {
        self.as_byte()
    }
}

// mod interface;

mod private;
