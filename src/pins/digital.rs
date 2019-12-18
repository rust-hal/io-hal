//! ## Digital pins traits

/// Digital pin state.
pub enum DigitalState {
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
    fn input(&self) -> Result<DigitalState, Self::Error>;
}

/// Output pin trait.
pub trait DigitalOutput {
    /// Pin error type.
    type Error;

    /// Returns output state - high or low.
    fn output(&self) -> Result<DigitalState, Self::Error>;

    /// Set pin output state to high or low.
    fn set_output(&mut self, state: DigitalState) -> Result<(), Self::Error>;

    /// Toggle output state to be the opposite of the current state, returns new state.
    fn toggle_output(&mut self) -> Result<DigitalState, Self::Error>;
}
