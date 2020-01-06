//! ## Digital pins traits

/// Digital pin level.
pub enum Level {
    /// Low digital level.
    Low,
    /// High digital level.
    High,
}

/// Input pin trait.
pub trait DigitalInput {
    /// Pin error type.
    type Error;

    /// Returns input state - high or low.
    fn input(&self) -> Result<Level, Self::Error>;
}

/// Output pin trait.
pub trait DigitalOutput {
    /// Pin error type.
    type Error;

    /// Returns output state - high or low.
    fn output(&self) -> Result<Level, Self::Error>;

    /// Set pin output state to high or low.
    fn set_output(&mut self, state: Level) -> Result<(), Self::Error>;

    /// Toggle output state to be the opposite of the current state, returns new state.
    fn toggle_output(&mut self) -> Result<Level, Self::Error>;
}
