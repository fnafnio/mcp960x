use crate::embassy_async;
pub trait Sealed {}
// impl<TWI> Sealed for interface::I2cInterface<TWI> {}

#[cfg(feature = "async")]
impl<TWI, DELAY> Sealed for embassy_async::AsyncI2cInterface<TWI, DELAY> {}
