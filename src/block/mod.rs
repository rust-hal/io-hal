//! ## Block specific part of library
//!
//! Operations with chip block i/o.

pub mod eeprom;
pub mod flash;

/// Block device configuration trait.
pub trait BlockConfiguration {}
