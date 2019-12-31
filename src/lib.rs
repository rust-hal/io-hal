//! ## IO framework for microcontrollers
//!
//! Gracefully path of embedded abstraction.

#![no_std]
#![warn(missing_docs)]

pub mod block;
pub mod common;
pub mod pins;
pub mod serial;

#[cfg(test)]
mod tests {
    #[test]
    fn none() {}
}
