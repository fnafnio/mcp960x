use crate::{Address, DeviceConfig, Register, SensorConfig};
use embassy_traits::delay::Delay;
use embassy_traits::i2c::I2c;
use fixed::types::I12F4;

#[derive(Debug, Default)]
pub struct AsyncI2cInterface<TWI, DELAY> {
    pub(crate) twi: TWI,
    address: Address,
    delay: DELAY,
    d_conf: DeviceConfig,
}

impl<TWI, DELAY> AsyncI2cInterface<TWI, DELAY>
where
    TWI: I2c,
    DELAY: Delay,
{
    pub type I2cError = <TWI as I2c>::Error;
    async fn write_register(&mut self, register: Register, data: u8) -> Result<(), TWI::Error> {
        self.twi
            .write(self.address as _, &[register as _, data])
            .await
    }

    async fn read_register_u16(&mut self, register: Register) -> Result<u16, TWI::Error> {
        let mut buf = [0; 2];

        self.twi.write(self.address as _, &[register as u8]).await?;
        self.twi.read(self.address as _, &mut buf).await?;
        Ok(u16::from_be_bytes(buf))
    }

    async fn read_register_u8(&mut self, register: Register) -> Result<u8, TWI::Error> {
        let mut buf = [0];
        self.twi.write(self.address as _, &[register as u8]).await?;

        self.twi.read(self.address as _, &mut buf).await?;
        Ok(buf[0])
    }

    pub async fn get_all_temps(&mut self) -> Result<(I12F4, I12F4, I12F4), TWI::Error> {
        // self.twi.write(self.address as _, bytes)
        todo!()
    }

    pub async fn set_sensor_config(&mut self, config: &SensorConfig) -> Result<(), TWI::Error> {
        self.write_register(Register::SensorConfiguration, config.as_byte())
            .await
    }

    pub fn set_device_config(&mut self, config: &DeviceConfig) {
        self.d_conf = *config;
        // self.write_device_config().await
    }

    pub async fn write_device_config(&mut self) -> Result<(), TWI::Error> {
        self.write_register(Register::DeviceConfiguration, self.d_conf.as_byte())
            .await
    }

    pub fn new(twi: TWI, address: Address, delay: DELAY) -> Self {
        Self {
            twi,
            address,
            delay,
            d_conf: Default::default(),
        }
    }

    pub async fn get_temp(&mut self) -> Result<I12F4, TWI::Error> {
        self.read_register_u16(Register::HotJunctionTemp)
            .await
            .map(|val| I12F4::from_be_bytes(val.to_be_bytes()))
    }

    pub async fn get_ambient_temp(&mut self) -> Result<I12F4, TWI::Error> {
        self.read_register_u16(Register::ColdJunctionTemp)
            .await
            .map(|val| I12F4::from_be_bytes(val.to_be_bytes()))
    }

    pub async fn get_temp_delta(&mut self) -> Result<I12F4, TWI::Error> {
        self.read_register_u16(Register::JunctionTempDelta)
            .await
            .map(|val| I12F4::from_be_bytes(val.to_be_bytes()))
    }

    pub async fn get_status(&mut self) -> Result<SensorRegister, TWI::Error> {
        self.read_register_u8(Register::Status)
            .await
            .map(|r| SensorRegister::new(r))
    }

    async fn poll_burst(&mut self) -> Result<bool, TWI::Error> {
        self.get_status().await.map(|s| s.burst_complete() == 1)
    }

    pub async fn get_burst_temp(&mut self) -> Result<I12F4, TWI::Error> {
        self.write_device_config().await?;
        while !self.poll_burst().await? {
            self.delay.delay_ms(150).await
        }
        self.get_temp().await
    }
}

use bitutils::bf;
bf!(pub SensorRegister[u8] {
    alert_1: 0:0, // e.g. field1: 0:3, which would encompass the least significant nibble
    alert_2: 1:1,
    alert_3: 2:2,
    alert_5: 3:3,
    in_range: 4:4,
    sc: 5:5,
    t_update: 6:6,
    burst_complete: 7:7
});
