use fixed::types::I12F4;

use crate::{Address, DeviceConfig, Register, SensorConfig, interface};

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

    pub fn get_temp(&mut self) -> Result<I12F4, DI::Error> {
        let val = self
            .iface
            .read_register(self.addr, Register::HotJunctionTemp)?;
        Ok(I12F4::from_be_bytes(val.to_be_bytes()))
    }
}
