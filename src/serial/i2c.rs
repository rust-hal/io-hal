//! I2C serial traits.
//!
//! Inter intgrated Circuit.

/// I2c configuration trait.
pub trait I2cConfiguration {
    /// Set port bitrate
    fn set_port_bitrate(&mut self, bitrate: usize) -> &mut Self;

    /// Set device address
    fn set_device_address(&mut self, address: usize) -> &mut Self;
}
