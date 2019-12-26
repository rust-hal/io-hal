//! USART serial traits.
//!
//! Universal Synchronous Asynchronous Receiver Transmitter.

/// Usart speed - BaudRate
pub enum BaudRate {
    /// 110 bod
    B110,
    /// 300 bod
    B300,
    /// 600 bod
    B600,
    /// 1200 bod
    B1200,
    /// 2400 bod
    B2400,
    /// 4800 bod
    B4800,
    /// 9600 bod
    B9600,
    /// 14400 bod
    B14400,
    /// 19200 bod
    B19200,
    /// 38400 bod
    B38400,
    /// 57600 bod
    B57600,
    /// 115200 bod
    B115200,
    /// 128000 bod
    B128000,
    /// 256000 bod
    B256000,
    /// User defined bod
    BUser(usize),
}

/// Usart configuration trait.
pub trait UsartConfiguration {
    /// Set baud rate.
    fn set_baud_rate(&mut self, baud_rate: BaudRate) -> &mut Self;
}
