//! ## Input/Output traits.
//!
//! Input and Output connectivity traits.

/// Input provide blocking functionality for reading operations.
pub trait Input<T: Copy> {}

/// Output provide blocking functionality for writing operations.
pub trait Output<T: Copy> {}

/// Exchange provide blocking functionality for reading and writing operations.
pub trait Exchange<T: Copy> {}
