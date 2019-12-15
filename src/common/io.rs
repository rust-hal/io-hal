//! ## Input/Output traits.
//!
//! Input and Output connectivity traits.

/// Input provide blocking functionality for reading operations.
pub trait Input<T> {}

/// Output provide blocking functionality for writing operations.
pub trait Output<T> {}
