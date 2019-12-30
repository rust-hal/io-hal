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
    Four,
    /// 5 bit
    Five,
    /// 6 bit
    Six,
    /// 7 bit
    Seven,
    /// 8 bit
    Eight,
    /// 9 bit
    Nine,
    /// 10 bit
    Ten,
    /// 11 bit
    Eleven,
    /// 12 bit
    Twelve,
    /// 13 bit
    Thirteen,
    /// 14 bit
    Fourteen,
    /// 15 bit
    Fifteen,
    /// 16 bit
    Sixteen,
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
