use crate::errors::{parity, Error, Result};
use crate::word::{CommandWord, DataWord, StatusWord, Word};
use crate::WordType;

/// A packet of data parsed from binary
///
/// Incoming data is parsed into a triplet of (sync,data,parity)
/// using this struct, and then may be further parsed as an
/// explicit command, status, or data word.
///
/// ## Example
///
/// ```rust
/// # use mil_std_1553b::*;
/// # fn try_main() -> Result<()> {
///     let packet = Packet::new(
///         0b100,
///         [0b01000000, 0b00100000],
///         1
///     );
///     assert!(packet.is_service());
/// # Ok(())
/// # }
/// ```
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Packet {

    /// The 3-bit sync pattern of a word
    pub sync: u8,

    /// The 16-bit body of a word
    pub body: [u8; 2],

    /// The 1-bit parity of a word
    pub parity: u8,

}

impl Packet {
    /// The leading sync pattern for a data word
    pub const DATA_SYNC: u8 = 0b001;

    /// The leading sync pattern for a non-data word
    pub const SERV_SYNC: u8 = 0b100;

    /// Create a new packet from sync, bytes, and parity
    ///
    /// # Arguments
    ///
    /// * `sync` - The leading 3 bit sync field as a u8
    /// * `body` - Two bytes of data following sync
    /// * `parity` - One bit parity field for the data as u8
    ///
    pub fn new(sync: u8, body: [u8; 2], parity: u8) -> Self {
        Self { sync, body, parity }
    }

    /// Parse a slice of bytes into sync, body, and parity
    ///
    /// This method interpretes the first 20 bits of the byte
    /// array as a triplet: 3-bit sync, 16-bit body, and 1-bit
    /// parity, given a bit offset at which to parse.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of bytes to parse
    /// * `offset` - The **bit** offset at which to begin parsing.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use mil_std_1553b::*;
    /// # fn try_main() -> Result<()> {
    ///    let packet = Packet::read(&[
    ///        0b00000000,
    ///        0b00001111,
    ///        0b00000000,
    ///        0b00000011
    ///    ],12)?;
    ///
    ///    assert_eq!(packet.sync, 0b00000111);
    ///    assert_eq!(packet.body, [0b10000000,0b00000001]);
    ///    assert_eq!(packet.parity, 0b00000001);
    /// # Ok(())
    /// # }
    pub fn read(data: &[u8], offset: usize) -> Result<Self> {
        if offset > 12 {
            return Err(Error::OutOfBounds);
        }

        let buf: [u8; 4] = match data.len() {
            3 => [data[0], data[1], data[2], 0],
            i if i > 3 => [data[0], data[1], data[2], data[3]],
            _ => return Err(Error::OutOfBounds),
        };

        let mut v: u32 = u32::from_be_bytes(buf);

        v <<= offset;
        v >>= 12;

        let s = ((v & 0b11100000000000000000) >> 17) as u8;
        let w1 = ((v & 0b00011111111000000000) >> 9) as u8;
        let w2 = ((v & 0b00000000000111111110) >> 1) as u8;
        let p = (v & 0b00000000000000000001) as u8;

        Ok(Self::new(s, [w1, w2], p))
    }

    /// Write the packet to a byte array
    pub fn write(&self, bytes: &mut [u8], offset: usize) -> Result<()> {
        let mut v: u32 = 0;
        let mut m: u32 = 0;
        let o = offset.clamp(0, 12);

        v |= ((self.sync & 0b00000111) as u32) << 29;
        v |= (self.body[0] as u32) << 21;
        v |= (self.body[1] as u32) << 13;
        v |= ((self.parity & 0b00000001) as u32) << 12;

        v >>= o;

        m |= (bytes[0] as u32) << 24;
        m |= (bytes[1] as u32) << 16;
        v |= m & !(u32::MAX >> o);

        let e = if offset > 4 { 4 } else { 3 };

        if bytes.len() < e {
            return Err(Error::OutOfBounds);
        }

        let result = v.to_be_bytes();
        bytes[..e].copy_from_slice(&result[..e]);

        Ok(())
    }

    /// Check the parity flag is correct
    #[must_use = "Result of check is never used"]
    pub fn check_parity(&self) -> bool {
        parity(u16::from_be_bytes(self.body)) == self.parity
    }

    /// Check the sync flag is correct
    #[must_use = "Result of check is never used"]
    pub fn check_sync(&self) -> bool {
        self.sync == Self::DATA_SYNC || self.sync == Self::SERV_SYNC
    }

    /// Check if this packet is a data packet
    #[must_use = "Result of check is never used"]
    pub fn is_data(&self) -> bool {
        self.sync == Self::DATA_SYNC
    }

    /// Check if this packet is a service packet
    #[must_use = "Result of check is never used"]
    pub fn is_service(&self) -> bool {
        self.sync == Self::SERV_SYNC
    }

    /// Check if this packet has correct parity and sync
    #[must_use = "Result of check is never used"]
    pub fn is_valid(&self) -> bool {
        self.check_parity() && self.check_sync()
    }

    /// Convert this packet into a word
    pub fn as_word<T: Word>(&self) -> Result<T> {
        T::new()
            .with_bytes(self.body)
            .with_parity(self.parity)
            .build()
    }
}

impl TryFrom<&WordType> for Packet {
    type Error = Error;

    fn try_from(word: &WordType) -> Result<Self> {
        match word {
            WordType::None => Err(Error::InvalidWord),
            _ => Ok(Self::new(
                match word.is_data() {
                    true => Self::DATA_SYNC,
                    false => Self::SERV_SYNC
                },
                word.bytes(), 
                word.parity()
            ))
        }
    }
}

impl TryFrom<WordType> for Packet {
    type Error = Error;

    fn try_from(word: WordType) -> Result<Self> {
        Self::try_from(&word)
    }
}

impl TryFrom<&Packet> for CommandWord {
    type Error = Error;

    fn try_from(value: &Packet) -> Result<Self> {
        if value.is_service() {
            value.as_word()
        } else {
            Err(Error::PacketIsInvalid)
        }
    }
}

impl TryFrom<Packet> for CommandWord {
    type Error = Error;

    fn try_from(value: Packet) -> Result<Self> {
        CommandWord::try_from(&value)
    }
}

impl TryFrom<&Packet> for StatusWord {
    type Error = Error;

    fn try_from(value: &Packet) -> Result<Self> {
        if value.is_service() {
            value.as_word()
        } else {
            Err(Error::PacketIsInvalid)
        }
    }
}

impl TryFrom<Packet> for StatusWord {
    type Error = Error;

    fn try_from(value: Packet) -> Result<Self> {
        StatusWord::try_from(&value)
    }
}

impl TryFrom<&Packet> for DataWord {
    type Error = Error;

    fn try_from(value: &Packet) -> Result<Self> {
        if value.is_data() {
            value.as_word()
        } else {
            Err(Error::PacketIsInvalid)
        }
    }
}

impl TryFrom<Packet> for DataWord {
    type Error = Error;

    fn try_from(value: Packet) -> Result<Self> {
        DataWord::try_from(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flags::{Address, BroadcastReceived, SubAddress};

    #[test]
    fn test_packet_parse_offset_13() {
        let result = Packet::read(
            &[0b00000000, 0b00000111, 0b10000000, 0b00000001, 0b10000000],
            13,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_packet_parse_offset_12() {
        let packet = Packet::read(&[0b00000000, 0b00001111, 0b00000000, 0b00000011], 12).unwrap();

        assert_eq!(packet.sync, 0b00000111);
        assert_eq!(packet.body, [0b10000000, 0b00000001]);
        assert_eq!(packet.parity, 0b00000001);
    }

    #[test]
    fn test_packet_parse_offset_6() {
        let packet = Packet::read(&[0b00000011, 0b11000000, 0b00000000, 0b11000000], 6).unwrap();

        assert_eq!(packet.sync, 0b00000111);
        assert_eq!(packet.body, [0b10000000, 0b00000001]);
        assert_eq!(packet.parity, 0b00000001);
    }

    #[test]
    fn test_packet_parse_offset_4() {
        let packet = Packet::read(&[0b00001111, 0b00000000, 0b00000011], 4).unwrap();

        assert_eq!(packet.sync, 0b00000111);
        assert_eq!(packet.body, [0b10000000, 0b00000001]);
        assert_eq!(packet.parity, 0b00000001);
    }

    #[test]
    fn test_packet_parse_offset_0() {
        let packet = Packet::read(&[0b11110000, 0b00000000, 0b00110000], 0).unwrap();

        assert_eq!(packet.sync, 0b00000111);
        assert_eq!(packet.body, [0b10000000, 0b00000001]);
        assert_eq!(packet.parity, 0b00000001);
    }

    #[test]
    fn test_packet_write_offset_12() {
        let packet = Packet::read(&[0b11110000, 0b00000000, 0b00110000], 0).unwrap();

        let mut buffer = [0, 0, 0, 0];
        let result = packet.write(&mut buffer, 12);

        assert!(result.is_ok());
        assert_eq!(buffer, [0b00000000, 0b00001111, 0b00000000, 0b00000011]);
    }

    #[test]
    fn test_packet_write_offset_7() {
        let packet = Packet::read(&[0b11110000, 0b00000000, 0b00110000], 0).unwrap();

        let mut buffer = [0, 0, 0, 0];
        let result = packet.write(&mut buffer, 7);

        assert!(result.is_ok());
        assert_eq!(buffer, [0b00000001, 0b11100000, 0b00000000, 0b01100000]);
    }

    #[test]
    fn test_packet_write_offset_5() {
        let packet = Packet::read(&[0b11110000, 0b00000000, 0b00110000], 0).unwrap();

        let mut buffer = [0b10101000, 0, 0, 0];
        let result = packet.write(&mut buffer, 5);

        assert!(result.is_ok());
        assert_eq!(buffer, [0b10101111, 0b10000000, 0b00000001, 0b10000000]);
    }

    #[test]
    fn test_packet_write_offset_4() {
        let packet = Packet::read(&[0b11110000, 0b00000000, 0b00110000], 0).unwrap();

        let mut buffer = [0b10100000, 0, 0, 0];
        let result = packet.write(&mut buffer, 4);

        assert!(result.is_ok());
        assert_eq!(buffer, [0b10101111, 0b00000000, 0b00000011, 0b00000000]);
    }

    #[test]
    fn test_packet_write_offset_0() {
        let packet = Packet::read(&[0b11110000, 0b00000000, 0b00110000], 0).unwrap();

        let mut buffer = [0, 0, 0];
        let result = packet.write(&mut buffer, 0);

        assert!(result.is_ok());
        assert_eq!(buffer, [0b11110000, 0b00000000, 0b00110000]);
    }

    #[test]
    fn test_new_data_packet() {
        let packet = Packet::new(Packet::DATA_SYNC, [0b00000000, 0b00000000], 1);
        assert!(packet.is_data());
    }

    #[test]
    fn test_new_service_packet() {
        let packet = Packet::new(Packet::SERV_SYNC, [0b01000000, 0b00100000], 1);
        assert!(packet.is_service());
    }

    #[test]
    fn test_packet_parity_even_both() {
        let packet = Packet::new(Packet::DATA_SYNC, [0b01000000, 0b00100000], 1);
        assert_eq!(packet.parity, 1);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_even_first() {
        let packet = Packet::new(Packet::DATA_SYNC, [0b01100000, 0b00000000], 1);
        assert_eq!(packet.parity, 1);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_even_second() {
        let packet = Packet::new(Packet::DATA_SYNC, [0b00000000, 0b00110000], 1);
        assert_eq!(packet.parity, 1);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_odd_both() {
        let packet = Packet::new(Packet::DATA_SYNC, [0b01100000, 0b00100000], 0);
        assert_eq!(packet.parity, 0);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_odd_first() {
        let packet = Packet::new(Packet::DATA_SYNC, [0b01110000, 0b00000000], 0);
        assert_eq!(packet.parity, 0);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_odd_second() {
        let packet = Packet::new(Packet::DATA_SYNC, [0b00000000, 0b00111000], 0);
        assert_eq!(packet.parity, 0);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_bad_parity_odd() {
        let packet = Packet::new(Packet::DATA_SYNC, [0b00000000, 0b00111000], 1);
        assert_eq!(packet.parity, 1);
        assert!(!packet.is_valid());
    }

    #[test]
    fn test_packet_bad_parity_even() {
        let packet = Packet::new(Packet::DATA_SYNC, [0b00000000, 0b00110000], 0);
        assert_eq!(packet.parity, 0);
        assert!(!packet.is_valid());
    }

    #[test]
    fn test_packet_convert_command() {
        let packet = Packet::new(Packet::SERV_SYNC, [0b00011000, 0b01100010], 0);
        let word = CommandWord::try_from(packet).unwrap();

        assert_eq!(word.address(), Address::new(3));
        assert_eq!(word.subaddress(), SubAddress::new(3));

        assert!(!word.is_mode_code());
        assert_eq!(word.word_count(), 2);
    }

    #[test]
    fn test_packet_convert_status() {
        let packet = Packet::new(Packet::SERV_SYNC, [0b00011000, 0b00010000], 0);
        let word = StatusWord::try_from(packet).unwrap();

        assert_eq!(word.address(), Address::new(3));
        assert_eq!(word.broadcast_received(), BroadcastReceived::Received);
    }

    #[test]
    fn test_packet_parse_word_alternate() {
        let packet = Packet::read(&[0b00010101, 0b01010101, 0b01000000], 0).unwrap();

        assert_eq!(packet.sync, 0b00000000);
        assert_eq!(packet.body, [0b10101010, 0b10101010]);
        assert_eq!(packet.parity, 0b00000000);
    }

    #[test]
    fn test_packet_parse_word_ones() {
        let packet = Packet::read(&[0b00011111, 0b11111111, 0b11100000], 0).unwrap();

        assert_eq!(packet.sync, 0b00000000);
        assert_eq!(packet.body, [0b11111111, 0b11111111]);
        assert_eq!(packet.parity, 0b00000000);
    }

    #[test]
    fn test_packet_parse_word_zeroes() {
        let packet = Packet::read(&[0b11100000, 0b00000000, 0b00010000], 0).unwrap();

        assert_eq!(packet.sync, 0b00000111);
        assert_eq!(packet.body, [0b00000000, 0b00000000]);
        assert_eq!(packet.parity, 0b00000001);
    }

    #[test]
    fn test_packet_parse_sync_zeroes() {
        let packet = Packet::read(&[0b00011111, 0b11111111, 0b11111111], 0).unwrap();

        assert_eq!(packet.sync, 0b00000000);
        assert_eq!(packet.body, [0b11111111, 0b11111111]);
        assert_eq!(packet.parity, 0b00000001);
    }

    #[test]
    fn test_packet_parse_sync_ones() {
        let packet = Packet::read(&[0b11100000, 0b00000000, 0b00000000], 0).unwrap();

        assert_eq!(packet.sync, 0b00000111);
        assert_eq!(packet.body, [0b00000000, 0b00000000]);
        assert_eq!(packet.parity, 0b00000000);
    }

    #[test]
    fn test_packet_parse_parity_one() {
        let packet = Packet::read(
            &[
                0b00000000, 0b00000000, 0b00010000, // 20th
            ],
            0,
        )
        .unwrap();

        assert_eq!(packet.sync, 0b00000000);
        assert_eq!(packet.body, [0b00000000, 0b00000000]);
        assert_eq!(packet.parity, 0b00000001);
    }

    #[test]
    fn test_packet_parse_parity_one_right() {
        let packet = Packet::read(
            &[
                0b00000000, 0b00000000, 0b00001000, // 21st
            ],
            0,
        )
        .unwrap();

        assert_eq!(packet.sync, 0b00000000);
        assert_eq!(packet.body, [0b00000000, 0b00000000]);
        assert_eq!(packet.parity, 0b00000000);
    }

    #[test]
    fn test_packet_parse_parity_one_left() {
        let packet = Packet::read(
            &[
                0b00000000, 0b00000000, 0b00100000, // 19th
            ],
            0,
        )
        .unwrap();

        assert_eq!(packet.sync, 0b00000000);
        assert_eq!(packet.body, [0b00000000, 0b00000001]);
        assert_eq!(packet.parity, 0b00000000);
    }

    #[test]
    fn test_packet_parse_parity_zero() {
        let packet = Packet::read(
            &[
                0b11111111, 0b11111111, 0b11101111, // 20th
            ],
            0,
        )
        .unwrap();

        assert_eq!(packet.sync, 0b00000111);
        assert_eq!(packet.body, [0b11111111, 0b11111111]);
        assert_eq!(packet.parity, 0b00000000);
    }
}
