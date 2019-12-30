//! SPI serial traits.
//!
//! Serial Peripheral Interface.

/// Data transfer begins at
pub enum Phase {
    /// Forward front of the clock signals.
    Forward,
    /// Backward front of the clock signals.
    BaudRate,
}

/// Clock signals polarity types.
pub enum Polarity {
    /// Idle state is low and pulse is high.
    Low,
    /// Idle state is high and pulse is low.
    High,
}

/// Spi configuration trait.
pub trait SpiConfiguration {
    /// Set port bitrate
    fn set_port_bitrate(&mut self, bitrate: usize) -> &mut Self;

    /// Set transfer phase.
    fn set_phase(&mut self, phase: Phase) -> &mut Self;

    /// Set clock polarity.
    fn set_polarity(&mut self, polarity: Polarity) -> &mut Self;
}
