use crate::interface;
pub trait Sealed {}
impl<TWI> Sealed for interface::I2cInterface<TWI> {}
