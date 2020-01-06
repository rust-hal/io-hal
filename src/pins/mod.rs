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
    Up,
    /// Pull-down - pin pulled down to i/o ground rail.
    Down,
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

/// Append modes for pin.
pub enum AppendMode {
    /// Push-pull mode.
    PushPull,
    /// Open drain mode.
    OpenDrain,
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
pub trait PinConfiguration {
    /// Set Pin type
    fn set_type(&mut self, pin_type: PinType) -> &mut Self;

    /// Set pin mode
    fn set_mode(&mut self, mode: Mode) -> &mut Self;

    /// Set append mode
    fn set_append_mode(&mut self, mode: AppendMode) -> &mut Self;

    /// Set pin pull
    fn set_pull(&mut self, pull: Pull) -> &mut Self;
}

/// Common pin creator trait.
pub trait Pin<PinEnumerateT, PinT> {
    /// Pin error type.
    type Error;

    /// Create pin from selector with defaulted configuration..
    fn create_pin(&mut self, pin_num: PinEnumerateT) -> Result<PinT, Self::Error>;
}
