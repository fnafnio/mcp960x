// use emb

use crate::{private, Address, Error, Register};
use embedded_hal::i2c::blocking::{Write, WriteRead};

#[derive(Debug, Default)]
pub struct I2cInterface<TWI> {
    pub(crate) twi: TWI,
}

pub trait WriteCommand: private::Sealed {
    /// Error type
    type Error;

    /// Command
    fn write_register(
        &mut self,
        addr: Address,
        register: Register,
        data: u8,
    ) -> Result<(), Self::Error>;
    fn read_register(&mut self, addr: Address, register: Register) -> Result<u16, Self::Error>;

    fn read_all_registers(&mut self, addr: Address, buf: &mut [u8; 29]) -> Result<(), Self::Error>;
}

impl<TWI, CommE> WriteCommand for I2cInterface<TWI>
where
    TWI: WriteRead<u8, Error = CommE> + Write<u8, Error = CommE>,
{
    type Error = Error<CommE>;

    fn write_register(
        &mut self,
        addr: Address,
        register: Register,
        data: u8,
    ) -> Result<(), Self::Error> {
        self.twi
            .write(addr as u8, &[register as _, data])
            .map_err(Error::Comm)?;
        Ok(())
    }

    fn read_register(&mut self, addr: Address, register: Register) -> Result<u16, Self::Error> {
        let mut buf = [0u8; 2];
        let mut reg = [addr as _];
        self.twi
            .write_read(addr as _, &reg, &mut buf)
            .map_err(Error::Comm)?;
        Ok(u16::from_be_bytes(buf))
    }

    fn read_all_registers(&mut self, addr: Address, buf: &mut [u8; 29]) -> Result<(), Self::Error> {
        todo!()
    }
}
