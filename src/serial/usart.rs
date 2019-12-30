//! USART serial traits.
//!
//! Universal Synchronous Asynchronous Receiver Transmitter.

/// Usart speed - BaudRate
pub enum BaudRate {
    /// 110 bod
    Baud110,
    /// 300 bod
    Baud300,
    /// 600 bod
    Baud600,
    /// 1200 bod
    Baud1200,
    /// 2400 bod
    Baud2400,
    /// 4800 bod
    Baud4800,
    /// 9600 bod
    Baud9600,
    /// 14400 bod
    Baud14400,
    /// 19200 bod
    Baud19200,
    /// 38400 bod
    Baud38400,
    /// 57600 bod
    Baud57600,
    /// 115200 bod
    Baud115200,
    /// 128000 bod
    Baud128000,
    /// 256000 bod
    Baud256000,
    /// User defined bod
    BaudUser(usize),
}

/// Parity check mode
pub enum ParityCheck {
    /// No checks
    None,
    /// Odd checks
    Odd,
    /// Even checks
    Even,
}

/// Stop bit count
pub enum StopBitCount {
    /// 1 bit
    One,
    /// 1.5 bit
    OneAndAHalf,
    /// 2 bit
    Two,
}

/// Bit length
pub enum BitLength {
    /// 5 bit
    Five,
    /// 6 bit
    Six,
    /// 7 bit
    Seven,
    /// 8 bit
    Eight,
    /// 9 bit
    Nine,
}

/// Usart configuration trait.
pub trait UsartConfiguration {
    /// Set baud rate.
    fn set_baud_rate(&mut self, baud_rate: BaudRate) -> &mut Self;

    /// Set parity check.
    fn set_parity_check(&mut self, parity: ParityCheck) -> &mut Self;

    /// Set stop bit.
    fn set_stop_bit(&mut self, stop: StopBitCount) -> &mut Self;

    /// Set bit length.
    fn set_bit_length(&mut self, bit_length: BitLength) -> &mut Self;
}
