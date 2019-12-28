//! SPI serial traits.
//!
//! Serial Peripheral Interface.

/// Spi configuration trait.
pub trait SpiConfiguration {
    /// Set port bitrate
    fn set_port_bitrate(&mut self, bitrate: usize) -> &mut Self;
}
