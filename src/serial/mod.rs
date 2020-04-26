//! ## Serial specific part of library
//!
//! Operations with serial I/O peripherals.

pub mod i2c;
pub mod spi;
pub mod usart;

/// Serial port type.
pub enum SerialPortType {
    /// Synchronous serial port - exchange data packets between reciver and transmitter
    /// synchronously with or without syncho clock signals.
    Synchronous,
    /// Asynchronous serial port - exchange data packets between receiver and transmitter
    /// asynchronously.
    Asynchronous,
}

/// Serial port mode.
pub enum SerialPortMode {
    /// Full duplex mode - different lines for receiver and transmitter.
    FullDuplex,
    /// Half duplex mode - one line for receiver and transmitter.
    HalfDuplex,
}

/// Serial role mode.
pub enum SerialRoleMode {
    /// Master role.
    Master,
    /// Slave role.
    Slave,
}

/// Common serial port configuration trait.
pub trait SerialPortConfiguration {
    /// Set port type.
    fn set_type(&mut self, port_type: SerialPortType) -> &mut Self;

    /// Set port mode.
    fn set_mode(&mut self, mode: SerialPortMode) -> &mut Self;

    /// Set role mode.
    fn set_role_mode(&mut self, mode: SerialRoleMode) -> &mut Self;
}
