//! ## Block specific part of library
//!
//! Operations with chip block i/o.

pub mod eeprom;
pub mod flash;

/// Block device configuration trait.
pub trait BlockConfiguration {}

/// Block device reader trait.
pub trait BlockReader<T: Copy> {
    /// Block read error
    type Error;

    /// Read one block from the device by provided address.
    fn read_block(&mut self, address: usize) -> Result<T, Self::Error>;

    /// Read many blocks from the device by provided address and buffer.
    fn read_many_blocks(&mut self, address: usize, buffer: &mut [T]) -> Result<(), Self::Error>;
}

/// Block device writer trait.
pub trait BlockWriter<T: Copy> {
    /// Block write error.
    type Error;

    /// Write one block to the device by provided address.
    fn write_block(&mut self, address: usize, block: T) -> Result<(), Self::Error>;

    /// Write many blocks to the device by provided address and buffer.
    fn write_many_blocks(&mut self, address: usize, buffer: &mut [T]) -> Result<(), Self::Error>;
}
