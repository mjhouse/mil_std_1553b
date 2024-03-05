use crate::errors::{parity, Error, Result};
use crate::word::Sync;
use crate::word::{CommandWord, DataWord, StatusWord};

/// A packet of data parsed from binary
///
/// Incoming data is parsed into a triplet of (sync,data,parity)
/// using this struct, and then may be further parsed as an
/// explicit command, status, or data word.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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

    /// Check if this packet is a data packet
    #[must_use = "Result of check is never used"]
    pub fn is_data(&self) -> bool {
        self.sync == Sync::Data.into()
    }

    /// Check if this packet is a service packet
    #[must_use = "Result of check is never used"]
    pub fn is_service(&self) -> bool {
        self.sync == Sync::Service.into()
    }

    /// Check if this packet has correct parity
    #[must_use = "Result of check is never used"]
    pub fn is_valid(&self) -> bool {
        self.parity() == self.parity
    }

    /// Get the first byte as a u16
    pub fn first(&self) -> u16 {
        self.bytes[0] as u16
    }

    /// Get the second byte as a u16
    pub fn second(&self) -> u16 {
        self.bytes[1] as u16
    }

    /// Get the data of the packet as u16
    pub fn value(&self) -> u16 {
        (self.first() << 8) | self.second()
    }

    /// Calculate the parity bit for the packet
    pub fn parity(&self) -> u8 {
        parity(self.value())
    }

    /// Convert this packet into a data word
    pub fn to_data(&self) -> Result<DataWord> {
        if self.is_valid() & self.is_data() {
            Ok(DataWord::from_data(self.value()))
        } else {
            Err(Error::PacketIsInvalid)
        }
    }

    /// Convert this packet into a status word
    pub fn to_status(&self) -> Result<StatusWord> {
        if self.is_valid() & self.is_service() {
            Ok(StatusWord::from(self.value()))
        } else {
            Err(Error::PacketIsInvalid)
        }
    }

    /// Convert this packet into a command word
    pub fn to_command(&self) -> Result<CommandWord> {
        if self.is_valid() & self.is_service() {
            Ok(CommandWord::from(self.value()))
        } else {
            Err(Error::PacketIsInvalid)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flags::{Address, BroadcastCommand, SubAddress};

    /// The leading sync pattern for a data word
    const DATA_SYNC: u8 = 0b001;

    /// The leading sync pattern for a non-data word
    const SERV_SYNC: u8 = 0b100;

    #[test]
    fn test_packet_value() {
        let packet = Packet::new(DATA_SYNC, [0b01000000, 0b00100000], 1);
        let value = packet.value();
        assert_eq!(value, 0b0100000000100000);
    }

    #[test]
    fn test_new_data_packet() {
        let packet = Packet::new(DATA_SYNC, [0b00000000, 0b00000000], 1);
        assert!(packet.is_data());
    }

    #[test]
    fn test_new_service_packet() {
        let packet = Packet::new(SERV_SYNC, [0b01000000, 0b00100000], 1);
        assert!(packet.is_service());
    }

    #[test]
    fn test_packet_parity_even_both() {
        let packet = Packet::new(DATA_SYNC, [0b01000000, 0b00100000], 1);
        assert_eq!(packet.parity, 1);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_even_first() {
        let packet = Packet::new(DATA_SYNC, [0b01100000, 0b00000000], 1);
        assert_eq!(packet.parity, 1);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_even_second() {
        let packet = Packet::new(DATA_SYNC, [0b00000000, 0b00110000], 1);
        assert_eq!(packet.parity, 1);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_odd_both() {
        let packet = Packet::new(DATA_SYNC, [0b01100000, 0b00100000], 0);
        assert_eq!(packet.parity, 0);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_odd_first() {
        let packet = Packet::new(DATA_SYNC, [0b01110000, 0b00000000], 0);
        assert_eq!(packet.parity, 0);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_odd_second() {
        let packet = Packet::new(DATA_SYNC, [0b00000000, 0b00111000], 0);
        assert_eq!(packet.parity, 0);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_bad_parity_odd() {
        let packet = Packet::new(DATA_SYNC, [0b00000000, 0b00111000], 1);
        assert_eq!(packet.parity, 1);
        assert!(!packet.is_valid());
    }

    #[test]
    fn test_packet_bad_parity_even() {
        let packet = Packet::new(DATA_SYNC, [0b00000000, 0b00110000], 0);
        assert_eq!(packet.parity, 0);
        assert!(!packet.is_valid());
    }

    #[test]
    fn test_packet_convert_command() {
        let packet = Packet::new(SERV_SYNC, [0b00011000, 0b01100010], 0);
        let word = packet.to_command().unwrap();

        assert_eq!(word.address(), Address::new(3));
        assert_eq!(word.subaddress(), SubAddress::new(3));

        assert!(!word.is_mode_code());
        assert_eq!(word.word_count(), Some(2));
    }

    #[test]
    fn test_packet_convert_status() {
        let packet = Packet::new(SERV_SYNC, [0b00011000, 0b00010000], 0);
        let word = packet.to_status().unwrap();

        assert_eq!(word.address(), Address::new(3));
        assert_eq!(word.broadcast_received(), BroadcastCommand::Received);
    }
}
