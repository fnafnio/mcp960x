#![no_std]

use core::{convert::TryFrom, result::Result};
use embedded_hal::{self as hal, digital::blocking::OutputPin, i2c::blocking::*};
use fixed::types::I12F4;

use core::marker::PhantomData;
// use embedded_hal::blocking::i2c::*;
use embedded_hal::i2c::blocking::*;

use embedded_hal::i2c::SevenBitAddress;
/// All possible errors in this crate
#[derive(Debug, PartialEq)]
pub enum Error<CommE> {
    /// Communication error
    Comm(CommE),
}

#[derive(Debug, defmt::Format, Clone, Copy)]
pub enum Address {
    Addr0 = 0x66,
    Addr1 = 0x67,
}

impl Into<SevenBitAddress> for Address {
    fn into(self) -> SevenBitAddress {
        self as u8
    }
}
pub struct MCP9600<DI> {
    iface: DI,
    addr: Address,
}

impl<DI> MCP9600<DI>
where
    DI: interface::WriteCommand,
{
    pub fn new(iface: DI, addr: Address) -> Self {
        Self { iface, addr }
    }

    pub fn set_sensor_config(&mut self, config: SensorConfig) {
        self.iface
            .write_register(self.addr, Register::SensorConfiguration, config.into());
    }

    pub fn set_device_config(&mut self, config: DeviceConfig) {
        self.iface
            .write_register(self.addr, Register::DeviceConfiguration, config.into());
    }

    pub fn get_temp(&mut self) -> Result<I12F4, ()> {
        let val = self.iface.read_register(self.addr, Register::HotJunctionTemp)?;
        Ok(I12F4::from_be_bytes(val.to_be_bytes()))
    }
}

#[derive(Debug, defmt::Format, Clone, Copy, FromPrimitive, ToPrimitive)]
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

#[derive(Debug, defmt::Format, Clone, Copy, FromPrimitive, ToPrimitive)]
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

#[derive(Debug, defmt::Format, Clone, Copy, FromPrimitive, ToPrimitive)]
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

pub struct DeviceConfig {
    resolution: Resolution,
    adc_resolution: AdcResolution,
    burst_mode: BurstMode,
    shutdown: Shutdown,
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

impl Into<u8> for DeviceConfig {
    fn into(self) -> u8 {
        (self.resolution as u8) << 7
            | (self.adc_resolution as u8) << 5
            | (self.burst_mode as u8) << 2
            | self.shutdown as u8
    }
}

#[derive(Debug, defmt::Format, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum Resolution {
    High = 0,
    Low = 1,
}

#[derive(Debug, defmt::Format, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum AdcResolution {
    R18 = 0,
    R16 = 1,
    R14 = 2,
    R12 = 3,
}

#[derive(Debug, defmt::Format, Clone, Copy, FromPrimitive, ToPrimitive)]
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

#[derive(Debug, defmt::Format, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum Shutdown {
    Normal = 0,
    Shutdown = 1,
    Burst = 2,
}
pub struct SensorConfig {
    t_type: Type,
    filter: Filter,
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
        (self.t_type as u8) << 4 | self.filter as u8
    }
}

mod interface;

mod private;
