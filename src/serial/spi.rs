//! SPI serial traits.
//!
//! Serial Peripheral Interface.

/// Data transfer begins at
pub enum Phase {
    /// Forward front of the clock signals.
    Forward,
    /// Backward front of the clock signals.
    Backward,
}

/// Clock signals polarity types.
pub enum Polarity {
    /// Idle state is low and pulse is high.
    Low,
    /// Idle state is high and pulse is low.
    High,
}

/// Bit length of data transfer.
pub enum BitLength {
    /// 4 bit
    B4,
    /// 5 bit
    B5,
    /// 6 bit
    B6,
    /// 7 bit
    B7,
    /// 8 bit
    B8,
    /// 9 bit
    B9,
    /// 10 bit
    B10,
    /// 11 bit
    B11,
    /// 12 bit
    B12,
    /// 13 bit
    B13,
    /// 14 bit
    B14,
    /// 15 bit
    B15,
    /// 16 bit
    B16,
}

/// Spi configuration trait.
pub trait SpiConfiguration {
    /// Set port bitrate
    fn set_port_bitrate(&mut self, bitrate: usize) -> &mut Self;

    /// Set transfer phase.
    fn set_phase(&mut self, phase: Phase) -> &mut Self;

    /// Set clock polarity.
    fn set_polarity(&mut self, polarity: Polarity) -> &mut Self;

    /// Set bit length.
    fn set_bit_length(&mut self, bit_length: BitLength) -> &mut Self;
}
