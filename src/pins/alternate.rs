//! ## Alternate pins traits.

/// Alternate selector for the pin in alternate mode.
pub enum AlternateMode {
    /// Inter-intgrated circuit.
    I2c(usize),
    /// Serial peripheral interface.
    Spi(usize),
    /// Universal Synchronous/Asynchronous Receiver/Transmitter.
    Usart(usize),
    /// System timers.
    Timer(usize),
    /// Analog to digital converter.
    Adc(usize),
    /// Digital to analog converter.
    Dac(usize),
}

/// Alternate mode configuration trait for pin.
pub trait AlternateModePin {
    /// set alternate mode.
    fn set_alternate_mode(&mut self, alt_mode: AlternateMode) -> &mut Self;
}

/// Input pin trait.
pub trait AlternateInput {}

/// Output pin trait.
pub trait AlternateOutput {}
