use crate::errors::{Error, Result};
use crate::word::{CommandWord, DataWord, StatusWord};

/// The leading sync pattern for a data word
const DATA_SYNC: u8 = 0b00000111;

/// The leading sync pattern for a non-data word
const SERV_SYNC: u8 = 0b00111000;

macro_rules! make_count {
    ( $c:expr ) => {{
        $c[0].count_ones() + $c[1].count_ones()
    }};
}

macro_rules! make_parity {
    ( $c:expr ) => {{
        match make_count!($c) % 2 {
            0 => 1,
            _ => 0,
        }
    }};
}

#[derive(Clone, Copy)]
pub struct Packet {
    pub sync: u8,
    pub bytes: [u8; 2],
    pub parity: u8,
}

impl Packet {
    /// Create a new packet from sync, bytes, and parity
    pub fn new(sync: u8, bytes: [u8; 2], parity: u8) -> Self {
        Self {
            sync,
            bytes,
            parity,
        }
    }

    /// Create a new data packet from bytes
    pub fn data(bytes: [u8; 2]) -> Self {
        Self::new(DATA_SYNC, bytes, make_parity!(bytes))
    }

    /// Create a new service packet from bytes
    pub fn service(bytes: [u8; 2]) -> Self {
        Self::new(SERV_SYNC, bytes, make_parity!(bytes))
    }

    /// Check if this packet is a data packet
    #[must_use = "Result of check is never used"]
    pub fn is_data(&self) -> bool {
        self.sync == DATA_SYNC
    }

    /// Check if this packet is a service packet
    #[must_use = "Result of check is never used"]
    pub fn is_service(&self) -> bool {
        self.sync == SERV_SYNC
    }

    /// Check if this packet has correct parity
    #[must_use = "Result of check is never used"]
    pub fn is_valid(&self) -> bool {
        make_parity!(self.bytes) == self.parity
    }

    pub fn first(&self) -> u16 {
        self.bytes[0] as u16
    }

    pub fn second(&self) -> u16 {
        self.bytes[1] as u16
    }

    pub fn value(&self) -> u16 {
        (self.first() << 8) | self.second()
    }

    /// Convert this packet into a data word
    pub fn as_data(&self) -> Result<DataWord> {
        if self.is_valid() & self.is_data() {
            Ok(DataWord::new(self.value()))
        } else {
            Err(Error::PacketIsInvalid)
        }
    }

    /// Convert this packet into a status word
    pub fn as_status(&self) -> Result<StatusWord> {
        if self.is_valid() & self.is_service() {
            Ok(StatusWord::new(self.value()))
        } else {
            Err(Error::PacketIsInvalid)
        }
    }

    /// Convert this packet into a command word
    pub fn as_command(&self) -> Result<CommandWord> {
        if self.is_valid() & self.is_service() {
            Ok(CommandWord::new(self.value()))
        } else {
            Err(Error::PacketIsInvalid)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::flags::{Address, SubAddress};

    use super::*;

    #[test]
    fn test_packet_value() {
        let packet = Packet::data([0b01000000, 0b00100000]);
        let value = packet.value();
        assert_eq!(value, 0b0100000000100000);
    }

    #[test]
    fn test_new_data_packet() {
        let packet = Packet::data([0b00000000, 0b00000000]);
        assert!(packet.is_data());
    }

    #[test]
    fn test_new_service_packet() {
        let packet = Packet::service([0b00000000, 0b00000000]);
        assert!(packet.is_service());
    }

    #[test]
    fn test_packet_parity_even_both() {
        let packet = Packet::data([0b01000000, 0b00100000]);
        assert_eq!(packet.parity, 1);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_even_first() {
        let packet = Packet::data([0b01100000, 0b00000000]);
        assert_eq!(packet.parity, 1);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_even_second() {
        let packet = Packet::data([0b00000000, 0b00110000]);
        assert_eq!(packet.parity, 1);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_odd_both() {
        let packet = Packet::data([0b00110000, 0b00001000]);
        assert_eq!(packet.parity, 0);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_odd_first() {
        let packet = Packet::data([0b00111000, 0b00000000]);
        assert_eq!(packet.parity, 0);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_odd_second() {
        let packet = Packet::data([0b00000000, 0b00111000]);
        assert_eq!(packet.parity, 0);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_bad_parity_odd() {
        let mut packet = Packet::data([0b00000000, 0b00111000]);
        packet.parity = 1;
        assert_eq!(packet.parity, 1);
        assert!(!packet.is_valid());
    }

    #[test]
    fn test_packet_bad_parity_even() {
        let mut packet = Packet::data([0b00000000, 0b00110000]);
        packet.parity = 0;
        assert_eq!(packet.parity, 0);
        assert!(!packet.is_valid());
    }

    #[test]
    fn test_packet_convert_command() {
        let packet = Packet::service([0b00011000, 0b01100010]);
        let word = packet.as_command().unwrap();

        assert_eq!(word.address(), Address::new(3));
        assert_eq!(word.subaddress(), SubAddress::new(3));

        assert!(!word.is_mode_code());
        assert_eq!(word.word_count(), Some(2));
    }
}