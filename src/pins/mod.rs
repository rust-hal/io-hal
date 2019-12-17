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

/// Common pin configuration.
pub struct PinConfiguration<PinEnumerateT> {
    /// Pin number for custom pin representation.
    pub pin_num: PinEnumerateT,
    /// Currrent pin type.
    pub pin_type: PinType,
    /// Currrent pin mode.
    pub mode: Mode,
    /// Currrent pin pull type.
    pub pull: Pull,
}

impl<PinEnumerateT> PinConfiguration<PinEnumerateT> {
    /// Create a new pin configuration with defaulted configuration.
    pub fn new(pin_num: PinEnumerateT) -> Self {
        Self {
            pin_num,
            pin_type: PinType::Digital,
            mode: Mode::Output,
            pull: Pull::None,
        }
    }

    /// Set Pin type
    pub fn set_type(&mut self, pin_type: PinType) -> &Self {
        self.pin_type = pin_type;
        self
    }

    /// Set pin mode
    pub fn set_mode(&mut self, mode: Mode) -> &Self {
        self.mode = mode;
        self
    }

    /// Set pin pull
    pub fn set_pull(&mut self, pull: Pull) -> &Self {
        self.pull = pull;
        self
    }
}

/// Common pin creator trait.
pub trait Pin<PinT, PinEnumerateT> {
    /// Pin error type.
    type Error;

    /// Create pin from configuration.
    fn create_pin(&self, pin_config: &PinConfiguration<PinEnumerateT>)
        -> Result<PinT, Self::Error>;
}
