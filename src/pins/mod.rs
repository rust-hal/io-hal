//! ## Pins specific part of library
//!
//! Operations with chip pins.

pub mod alternate;
pub mod analog;
pub mod digital;

/// Common type of Pulls
pub enum Pull {
    /// No pull
    None,
    /// Pull-up - pin pulled up to i/o power rail.
    PullUp,
    /// Pull-down - pin pulled down to i/o ground rail.
    PullDown,
}

/// Common pin modes.
pub enum Mode {
    /// Input mode
    Input,
    /// Output mode
    Output,
    /// Bidirection mode
    Bidirection,
}

/// Common pin types.
pub enum PinType {
    /// Digital type - allowed digital operations e.g. input, output as fixed values between 0 and 1 logic state.
    Digital,
    /// Analog type - allowed digital operations e.g. input and output as floating values between 0 and VCC power.
    Analog,
    /// Alternate functionality - other type of operations, pin connected to internal peripheral.
    Alternate,
}

/// Common pin configuration trait.
pub trait PinConfiguration<PinEnumerateT> {
    /// Create a new pin configuration with defaulted configuration.
    fn new_pin(&self, pin_num: PinEnumerateT) -> Self;

    /// Set Pin type
    fn set_type(&mut self, pin_type: PinType) -> Self;

    /// Set pin mode
    fn set_mode(&mut self, mode: Mode) -> Self;

    /// Set pin pull
    fn set_pull(&mut self, pull: Pull) -> Self;
}

/// Common pin creator trait.
pub trait Pin<PinT> {
    /// Pin error type.
    type Error;

    /// Create pin from configuration.
    fn create_pin(&mut self) -> Result<PinT, Self::Error>;
}
