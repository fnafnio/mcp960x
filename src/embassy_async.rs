use crate::{Address, DeviceConfig, Mode, Register, SensorConfig};
use embassy_traits::i2c::I2c;
use fixed::types::I12F4;

#[derive(Debug, Default)]
pub struct Mcp960x<TWI> {
    pub(crate) twi: TWI,
    address: Address,
}

impl<TWI> Mcp960x<TWI>
where
    TWI: I2c,
{
    pub type I2cError = <TWI as I2c>::Error;
    
    /// convenience function
    async fn write_register(&mut self, register: Register, data: u8) -> Result<(), TWI::Error> {
        self.twi
            .write(self.address as _, &[register as _, data])
            .await
    }

    
    /// convenience function
    async fn read_register_u16(&mut self, register: Register) -> Result<u16, TWI::Error> {
        let mut buf = [0; 2];

        self.twi.write(self.address as _, &[register as u8]).await?;
        self.twi.read(self.address as _, &mut buf).await?;
        Ok(u16::from_be_bytes(buf))
    }

    /// convenience function
    async fn read_register_u8(&mut self, register: Register) -> Result<u8, TWI::Error> {
        let mut buf = [0];
        self.twi.write(self.address as _, &[register as u8]).await?;

        self.twi.read(self.address as _, &mut buf).await?;
        Ok(buf[0])
    }

    /// still to be written
    pub async fn get_all_temps(&mut self) -> Result<(I12F4, I12F4, I12F4), TWI::Error> {
        // self.twi.write(self.address as _, bytes)
        unimplemented!()
    }

    pub async fn get_sensor_config(&mut self) -> Result<SensorConfig, TWI::Error> {
        self.read_register_u8(Register::SensorConfiguration)
            .await
            .map(|conf| conf.into())
    }

    pub async fn set_sensor_config(&mut self, config: &SensorConfig) -> Result<(), TWI::Error> {
        self.write_register(Register::SensorConfiguration, config.as_byte())
            .await
    }

    pub async fn get_device_config(&mut self) -> Result<DeviceConfig, TWI::Error> {
        self.read_register_u8(Register::DeviceConfiguration)
            .await
            .map(|conf| conf.into())
    }
    pub async fn set_device_config(&mut self, config: &DeviceConfig) -> Result<(), TWI::Error> {
        self.write_register(Register::DeviceConfiguration, config.as_byte())
            .await
    }

    /// Setup the MCP960x
    /// writes the device and sensor configuration
    /// use [new] if you want to leave the configuration untouched
    pub async fn setup(
        twi: TWI,
        address: Address,
        device: &DeviceConfig,
        sensor: &SensorConfig,
    ) -> Result<Self, TWI::Error> {
        let mut this = Self { twi, address };
        this.set_device_config(device).await?;
        this.set_sensor_config(sensor).await?;
        Ok(this)
    }

    /// Create an instance of MCP960x
    /// the device and sensor configuration remain untouched
    /// use [setup] if you need to setup the configuration
    /// or configure it manually with [set_device_config] and [set_sensor_config]
    pub fn new(twi: TWI, address: Address) -> Self {
        Self {
            twi,
            address,
        }
    }

    /// Read the hot-junction temperature
    /// i.e. the temperature at the thermocouple "tip"
    pub async fn get_temp(&mut self) -> Result<I12F4, TWI::Error> {
        self.read_register_u16(Register::HotJunctionTemp)
            .await
            .map(|val| I12F4::from_be_bytes(val.to_be_bytes()))
    }

    /// Read the cold-junction temperature
    /// i.e. the temperature at the IC
    pub async fn get_ambient_temp(&mut self) -> Result<I12F4, TWI::Error> {
        self.read_register_u16(Register::ColdJunctionTemp)
            .await
            .map(|val| I12F4::from_be_bytes(val.to_be_bytes()))
    }

    /// get the delta between hot- and cold-junction from the device
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

    pub async fn set_operating_mode(&mut self, mode: Mode) -> Result<(), TWI::Error> {
        let mut config = self.get_device_config().await?;
        config.shutdown = mode;
        self.set_device_config(&config).await
    }

    // async fn poll_burst(&mut self) -> Result<bool, TWI::Error> {
    //     self.get_status().await.map(|s| s.burst_complete() == 1)
    // }

    // pub async fn get_burst_temp(&mut self) -> Result<I12F4, TWI::Error> {
    //     self.write_device_config().await?;
    //     while !self.poll_burst().await? {
    //         self.delay.delay_ms(150).await
    //     }
    //     self.get_temp().await
    // }
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
