//! ## Input/Output traits.
//!
//! Input and Output connectivity traits.

/// Input provide blocking functionality for reading operations.
pub trait Input<T: Copy> {
    /// Input error.
    type Error;

    /// Read a single word from the input interface.
    fn read(&mut self) -> Result<T, Self::Error>;

    /// Read multiple words from the input interface to provided buffer.
    fn reads(&mut self, buffer: &mut [T]) -> Result<(), Self::Error>;
}

/// Output provide blocking functionality for writing operations.
pub trait Output<T: Copy> {
    /// Output error.
    type Error;

    /// Write a single word to the output interface.
    fn write(&mut self, value: T) -> Result<(), Self::Error>;

    /// Write multiple words to the output interface from provided buffer.
    fn writes(&mut self, buffer: &[T]) -> Result<(), Self::Error>;
}

/// Exchange provide blocking functionality for reading and writing operations.
pub trait Exchange<T: Copy> {
    /// Exchange error.
    type Error;

    /// Exchange a single word through the exchange interface.
    fn exchange(&mut self, value: &mut T) -> Result<(), Self::Error>;

    /// Exchange multiple words through the exchange interface from the provided buffer.
    fn exchanges(&mut self, buffer: &mut [T]) -> Result<(), Self::Error>;
}
