//! ## Pins specific part of library
//!
//! Operations with chip pins.

pub mod alternate;
pub mod analog;
pub mod digital;

/// Common type of Pulls
pub enum Pull {
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
