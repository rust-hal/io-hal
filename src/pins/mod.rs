//! ## Pins specific part of library
//!
//! Operations with chip pins.

/// Common pin types.
pub enum PinType {
    /// Digital type - allowed digital operations e.g. input, output as fixed values between 0 and 1 logic state.
    Digital,
    /// Analog type - allowed digital operations e.g. input and output as floating values between 0 and VCC power.
    Analog,
    /// Alternate functionality - other type of operations, pin connected to internal peripheral.
    Alternate,
}
