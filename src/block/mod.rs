//! ## Block specific part of library
//!
//! Operations with chip block i/o.

pub mod eeprom;
pub mod flash;

/// Block device configuration trait.
pub trait BlockConfiguration {}

/// Write protection
pub enum WriteProtect {
    /// Write protection is enabled.
    Enabled,
    /// Write protection is disabled.
    Disabled,
}

/// Block device control trait.
pub trait BlockControl<Block: Copy> {
    /// Block control error.
    type Error;

    /// Status of write protection.
    fn get_write_protection_status(&self) -> Result<WriteProtect, Self::Error>;

    /// Enable/Disable write protection.
    fn set_write_protection_state(
        &mut self,
        write_protection: WriteProtect,
    ) -> Result<(), Self::Error>;
}

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

/// Block device erase trait.
pub trait BlockEraser<Block: Copy> {
    /// Block erase error.
    type Error;

    /// Erase one block from the device by provided address.
    fn erase_block(&mut self, address: *mut Block) -> Result<(), Self::Error>;

    /// Erase many blocks from the device by provided address.
    fn erase_many_blocks(&mut self, address: *mut Block, count: usize) -> Result<(), Self::Error>;
}
