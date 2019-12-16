//! ## Input/Output non-blocking traits.
//!
//! Input and Output non-blocking connectivity traits.

/// Non-blocking result
pub enum NBResult<T, Context> {
    // Ready for getting a result
    Ready(T),
    // Pending to getting a result, returning an current context for rerun sequence.
    Pending(Context),
}

/// Input provides non-blocking functionality for reading operations.
pub trait Input<T: Copy> {
    /// Input error.
    type Error;
    /// Context for pending rerun.
    type NBContext;

    /// Read a single word from the input interface.
    fn read(&mut self) -> NBResult<Result<T, Self::Error>, Self::NBContext>;

    /// Read multiple words from the input interface to provided buffer.
    fn read_many(&mut self, buffer: &mut [T])
        -> NBResult<Result<(), Self::Error>, Self::NBContext>;
}

/// Output provides non-blocking functionality for writing operations.
pub trait Output<T: Copy> {
    /// Output error.
    type Error;
    /// Context for pending rerun.
    type NBContext;

    /// Write a single word to the output interface.
    fn write(&mut self, value: T) -> NBResult<Result<(), Self::Error>, Self::NBContext>;

    /// Write multiple words to the output interface from provided buffer.
    fn write_many(&mut self, buffer: &[T]) -> NBResult<Result<(), Self::Error>, Self::NBContext>;
}

/// Exchange provides non-blocking functionality for reading and writing operations.
pub trait Exchange<T: Copy> {
    /// Exchange error.
    type Error;
    /// Context for pending rerun.
    type NBContext;

    /// Exchange a single word through the exchange interface.
    fn exchange(&mut self, value: &mut T) -> NBResult<Result<(), Self::Error>, Self::NBContext>;

    /// Exchange multiple words through the exchange interface from the provided buffer.
    fn exchange_many(
        &mut self,
        buffer: &mut [T],
    ) -> NBResult<Result<(), Self::Error>, Self::NBContext>;
}
