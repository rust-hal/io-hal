//! ## Block specific part of library
//!
//! Operations with chip block i/o.

pub mod eeprom;
pub mod flash;

/// Block device configuration trait.
pub trait BlockConfiguration {}

/// Block device reader trait.
pub trait BlockReader<Block: Copy> {
    /// Block read error
    type Error;

    /// Read one block from the device by provided address.
    fn read_block(&mut self, address: *mut Block) -> Result<Block, Self::Error>;

    /// Read many blocks from the device by provided address and buffer.
    fn read_many_blocks(
        &mut self,
        address: *mut Block,
        buffer: &mut [Block],
    ) -> Result<(), Self::Error>;
}

/// Block device writer trait.
pub trait BlockWriter<Block: Copy> {
    /// Block write error.
    type Error;

    /// Write one block to the device by provided address.
    fn write_block(&mut self, address: *mut Block, block: Block) -> Result<(), Self::Error>;

    /// Write many blocks to the device by provided address and buffer.
    fn write_many_blocks(
        &mut self,
        address: *mut Block,
        buffer: &mut [Block],
    ) -> Result<(), Self::Error>;
}
