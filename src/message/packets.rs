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
/// # fn main() -> Result<()> {
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
    /// * `offset` - The **bit** offset at which to begin parsing
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use mil_std_1553b::*;
    /// # fn main() -> Result<()> {
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
    #[allow(clippy::if_same_then_else)]
    pub fn read(data: &[u8], offset: usize) -> Result<Self> {
        // if the offset won't fit in a u32
        if offset > 12 {
            return Err(Error::OutOfBounds);
        }
        // if the offset requires 4 bytes and
        // they weren't given
        else if offset > 4 && data.len() < 4 {
            return Err(Error::OutOfBounds);
        }
        // if the offset requires 3 bytes and
        // they weren't given
        else if data.len() < 3 {
            return Err(Error::OutOfBounds);
        }

        let mut buf: [u8; 4] = [0, 0, 0, 0];

        buf[0] = data.first().cloned().unwrap_or(0);
        buf[1] = data.get(1).cloned().unwrap_or(0);
        buf[2] = data.get(2).cloned().unwrap_or(0);
        buf[3] = data.get(3).cloned().unwrap_or(0);

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
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of bytes to parse
    /// * `offset` - The **bit** offset at which to write
    ///
    #[allow(clippy::if_same_then_else)]
    pub fn write(&self, data: &mut [u8], offset: usize) -> Result<()> {
        // if the offset requires 4 bytes and
        // they weren't given
        if offset > 4 && data.len() < 4 {
            return Err(Error::OutOfBounds);
        }
        // if the offset requires 3 bytes and
        // they weren't given
        else if data.len() < 3 {
            return Err(Error::OutOfBounds);
        }

        let mut v: u32 = 0;
        let mut m: u32 = 0;
        let o = offset.clamp(0, 12);

        v |= ((self.sync & 0b00000111) as u32) << 29;
        v |= (self.body[0] as u32) << 21;
        v |= (self.body[1] as u32) << 13;
        v |= ((self.parity & 0b00000001) as u32) << 12;

        v >>= o;

        m |= (data[0] as u32) << 24;
        m |= (data[1] as u32) << 16;
        v |= m & !(u32::MAX >> o);

        let e = if offset > 4 { 4 } else { 3 };
        let result = v.to_be_bytes();

        data[..e].copy_from_slice(&result[..e]);

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
                    false => Self::SERV_SYNC,
                },
                word.bytes(),
                word.parity(),
            )),
        }
    }
}

impl TryFrom<WordType> for Packet {
    type Error = Error;

    fn try_from(word: WordType) -> Result<Self> {
        Self::try_from(&word)
    }
}

impl TryFrom<Packet> for CommandWord {
    type Error = Error;

    fn try_from(value: Packet) -> Result<Self> {
        if value.is_service() {
            value.as_word()
        } else {
            Err(Error::InvalidPacket)
        }
    }
}

impl TryFrom<Packet> for StatusWord {
    type Error = Error;

    fn try_from(value: Packet) -> Result<Self> {
        if value.is_service() {
            value.as_word()
        } else {
            Err(Error::InvalidPacket)
        }
    }
}

impl TryFrom<Packet> for DataWord {
    type Error = Error;

    fn try_from(value: Packet) -> Result<Self> {
        if value.is_data() {
            value.as_word()
        } else {
            Err(Error::InvalidPacket)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ----------------------------------------------------------
    // Packet

    fn test_packet_write_success(offset: usize, input: &[u8], expected: &[u8]) {
        let mut buffer = [0; 4];

        // read then write the packet
        let packet = Packet::read(input, offset).unwrap();
        packet.write(&mut buffer, 0).unwrap();

        // compare the output with the original data
        assert_eq!(&buffer[..3], expected);
    }

    #[test]
    fn test_packet_write_success_0() {
        test_packet_write_success(
            0,
            &[0b11110000, 0b00000000, 0b00110000],
            &[0b11110000, 0b00000000, 0b00110000],
        );
    }

    #[test]
    fn test_packet_write_success_1() {
        test_packet_write_success(
            4,
            &[0b00001111, 0b00000000, 0b00000011],
            &[0b11110000, 0b00000000, 0b00110000],
        );
    }

    #[test]
    fn test_packet_write_success_2() {
        test_packet_write_success(
            6,
            &[0b00000011, 0b11000000, 0b00000000, 0b11000000],
            &[0b11110000, 0b00000000, 0b00110000],
        );
    }

    #[test]
    fn test_packet_write_success_3() {
        test_packet_write_success(
            12,
            &[0b00000000, 0b00001111, 0b00000000, 0b00000011],
            &[0b11110000, 0b00000000, 0b00110000],
        );
    }

    fn test_packet_write_fail(offset: usize, input: &[u8], output: &mut [u8], expected: bool) {
        let packet = Packet::read(input, 0).unwrap();
        let result = packet.write(output, offset);
        assert_eq!(result.is_ok(), expected);
    }

    #[test]
    fn test_packet_write_fail_0() {
        test_packet_write_fail(
            5,
            &[0b11110000, 0b00000000, 0b00110000],
            &mut [0, 0, 0],
            false,
        );
    }

    #[test]
    fn test_packet_write_fail_1() {
        test_packet_write_fail(0, &[0b11110000, 0b00000000, 0b00110000], &mut [0, 0], false);
    }

    fn test_packet_read_success(offset: usize, input: &[u8], sync: u8, body: [u8; 2], parity: u8) {
        let packet = Packet::read(input, offset).unwrap();
        assert_eq!(packet.sync, sync);
        assert_eq!(packet.body, body);
        assert_eq!(packet.parity, parity);
    }

    #[test]
    fn test_packet_read_success_0() {
        test_packet_read_success(
            0,
            &[0b11110000, 0b00000000, 0b00110000],
            0b00000111,
            [0b10000000, 0b00000001],
            0b00000001,
        );
    }

    #[test]
    fn test_packet_read_success_1() {
        test_packet_read_success(
            4,
            &[0b00001111, 0b00000000, 0b00000011],
            0b00000111,
            [0b10000000, 0b00000001],
            0b00000001,
        );
    }

    #[test]
    fn test_packet_read_success_2() {
        test_packet_read_success(
            6,
            &[0b00000011, 0b11000000, 0b00000000, 0b11000000],
            0b00000111,
            [0b10000000, 0b00000001],
            0b00000001,
        );
    }

    #[test]
    fn test_packet_read_success_3() {
        test_packet_read_success(
            12,
            &[0b00000000, 0b00001111, 0b00000000, 0b00000011],
            0b00000111,
            [0b10000000, 0b00000001],
            0b00000001,
        );
    }

    fn test_packet_read_fail(offset: usize, input: &[u8], expected: bool) {
        let result = Packet::read(input, offset);
        assert_eq!(result.is_ok(), expected);
    }

    #[test]
    fn test_packet_read_fail_0() {
        test_packet_read_fail(
            13,
            &[0b00000000, 0b00000111, 0b10000000, 0b00000001, 0b10000000],
            false,
        );
    }

    #[test]
    fn test_packet_read_fail_1() {
        test_packet_read_fail(12, &[0b00000000, 0b00000111, 0b10000000], false);
    }

    #[test]
    fn test_packet_read_fail_2() {
        test_packet_read_fail(0, &[0b11110000, 0b00110000], false);
    }

    fn test_packet_check_parity(sync: u8, body: [u8; 2], parity: u8, expected: bool) {
        let packet = Packet::new(sync, body, parity);
        assert_eq!(packet.check_parity(), expected);
    }

    #[test]
    fn test_packet_check_parity_0() {
        test_packet_check_parity(0b100, [0b10000000, 0b00000001], 1, true);
    }

    #[test]
    fn test_packet_check_parity_1() {
        test_packet_check_parity(0b100, [0b10000100, 0b00000001], 0, true);
    }

    #[test]
    fn test_packet_check_parity_2() {
        test_packet_check_parity(0b100, [0b10000100, 0b00001001], 1, true);
    }

    #[test]
    fn test_packet_check_parity_3() {
        test_packet_check_parity(0b100, [0b10000100, 0b10001001], 0, true);
    }

    #[test]
    fn test_packet_check_parity_4() {
        test_packet_check_parity(0b001, [0b10000000, 0b00000001], 1, true);
    }

    #[test]
    fn test_packet_check_parity_5() {
        test_packet_check_parity(0b001, [0b10000100, 0b00000001], 0, true);
    }

    #[test]
    fn test_packet_check_parity_6() {
        test_packet_check_parity(0b001, [0b10000100, 0b00001001], 1, true);
    }

    #[test]
    fn test_packet_check_parity_7() {
        test_packet_check_parity(0b001, [0b10000100, 0b10001001], 0, true);
    }

    #[test]
    fn test_packet_check_parity_8() {
        test_packet_check_parity(0b100, [0b10000000, 0b00000001], 0, false);
    }

    #[test]
    fn test_packet_check_parity_9() {
        test_packet_check_parity(0b100, [0b10000100, 0b00000001], 1, false);
    }

    #[test]
    fn test_packet_check_parity_10() {
        test_packet_check_parity(0b100, [0b10000100, 0b00001001], 0, false);
    }

    #[test]
    fn test_packet_check_parity_11() {
        test_packet_check_parity(0b100, [0b10000100, 0b10001001], 1, false);
    }

    #[test]
    fn test_packet_check_parity_12() {
        test_packet_check_parity(0b001, [0b10000000, 0b00000001], 0, false);
    }

    #[test]
    fn test_packet_check_parity_13() {
        test_packet_check_parity(0b001, [0b10000100, 0b00000001], 1, false);
    }

    #[test]
    fn test_packet_check_parity_14() {
        test_packet_check_parity(0b001, [0b10000100, 0b00001001], 0, false);
    }

    #[test]
    fn test_packet_check_parity_15() {
        test_packet_check_parity(0b001, [0b10000100, 0b10001001], 1, false);
    }

    fn test_packet_check_sync(sync: u8, body: [u8; 2], parity: u8, expected: bool) {
        let packet = Packet::new(sync, body, parity);
        assert_eq!(packet.check_sync(), expected);
    }

    #[test]
    fn test_packet_check_sync_0() {
        test_packet_check_sync(0b100, [0b10000000, 0b00000001], 1, true);
    }

    #[test]
    fn test_packet_check_sync_1() {
        test_packet_check_sync(0b100, [0b10000000, 0b10000001], 0, true);
    }

    #[test]
    fn test_packet_check_sync_2() {
        test_packet_check_sync(0b001, [0b10000000, 0b00000001], 1, true);
    }

    #[test]
    fn test_packet_check_sync_3() {
        test_packet_check_sync(0b001, [0b10000000, 0b10000001], 0, true);
    }

    fn test_packet_is_data(sync: u8, body: [u8; 2], parity: u8, expected: bool) {
        let packet = Packet::new(sync, body, parity);
        assert_eq!(packet.is_data(), expected);
    }

    #[test]
    fn test_packet_is_data_0() {
        test_packet_is_data(0b001, [0b10000000, 0b00000001], 1, true);
    }

    #[test]
    fn test_packet_is_data_1() {
        test_packet_is_data(0b001, [0b10000000, 0b10000001], 0, true);
    }

    #[test]
    fn test_packet_is_data_2() {
        test_packet_is_data(0b011, [0b10000000, 0b00000001], 1, false);
    }

    #[test]
    fn test_packet_is_data_3() {
        test_packet_is_data(0b011, [0b10000000, 0b10000001], 0, false);
    }

    #[test]
    fn test_packet_is_data_4() {
        test_packet_is_data(0b101, [0b10000000, 0b00000001], 1, false);
    }

    #[test]
    fn test_packet_is_data_5() {
        test_packet_is_data(0b101, [0b10000000, 0b10000001], 0, false);
    }

    #[test]
    fn test_packet_is_data_6() {
        test_packet_is_data(0b110, [0b10000000, 0b00000001], 1, false);
    }

    #[test]
    fn test_packet_is_data_7() {
        test_packet_is_data(0b110, [0b10000000, 0b10000001], 0, false);
    }

    #[test]
    fn test_packet_is_data_8() {
        test_packet_is_data(0b111, [0b10000000, 0b00000001], 1, false);
    }

    #[test]
    fn test_packet_is_data_9() {
        test_packet_is_data(0b111, [0b10000000, 0b10000001], 0, false);
    }

    #[test]
    fn test_packet_is_data_10() {
        test_packet_is_data(0b100, [0b10000000, 0b00000001], 1, false);
    }

    #[test]
    fn test_packet_is_data_11() {
        test_packet_is_data(0b100, [0b10000000, 0b10000001], 0, false);
    }

    fn test_packet_is_service(sync: u8, body: [u8; 2], parity: u8, expected: bool) {
        let packet = Packet::new(sync, body, parity);
        assert_eq!(packet.is_service(), expected);
    }

    #[test]
    fn test_packet_is_service_0() {
        test_packet_is_service(0b001, [0b10000000, 0b00000001], 1, false);
    }

    #[test]
    fn test_packet_is_service_1() {
        test_packet_is_service(0b001, [0b10000000, 0b10000001], 0, false);
    }

    #[test]
    fn test_packet_is_service_2() {
        test_packet_is_service(0b011, [0b10000000, 0b00000001], 1, false);
    }

    #[test]
    fn test_packet_is_service_3() {
        test_packet_is_service(0b011, [0b10000000, 0b10000001], 0, false);
    }

    #[test]
    fn test_packet_is_service_4() {
        test_packet_is_service(0b101, [0b10000000, 0b00000001], 1, false);
    }

    #[test]
    fn test_packet_is_service_5() {
        test_packet_is_service(0b101, [0b10000000, 0b10000001], 0, false);
    }

    #[test]
    fn test_packet_is_service_6() {
        test_packet_is_service(0b110, [0b10000000, 0b00000001], 1, false);
    }

    #[test]
    fn test_packet_is_service_7() {
        test_packet_is_service(0b110, [0b10000000, 0b10000001], 0, false);
    }

    #[test]
    fn test_packet_is_service_8() {
        test_packet_is_service(0b111, [0b10000000, 0b00000001], 1, false);
    }

    #[test]
    fn test_packet_is_service_9() {
        test_packet_is_service(0b111, [0b10000000, 0b10000001], 0, false);
    }

    #[test]
    fn test_packet_is_service_10() {
        test_packet_is_service(0b100, [0b10000000, 0b00000001], 1, true);
    }

    #[test]
    fn test_packet_is_service_11() {
        test_packet_is_service(0b100, [0b10000000, 0b10000001], 0, true);
    }

    fn test_packet_is_valid(sync: u8, body: [u8; 2], parity: u8, expected: bool) {
        let packet = Packet::new(sync, body, parity);
        assert_eq!(packet.is_valid(), expected);
    }

    #[test]
    fn test_packet_is_valid_0() {
        test_packet_is_valid(0b001, [0b10000000, 0b00000001], 1, true);
    }

    #[test]
    fn test_packet_is_valid_1() {
        test_packet_is_valid(0b001, [0b10000000, 0b10000001], 0, true);
    }

    #[test]
    fn test_packet_is_valid_2() {
        test_packet_is_valid(0b001, [0b10001000, 0b00000000], 0, false);
    }

    #[test]
    fn test_packet_is_valid_3() {
        test_packet_is_valid(0b001, [0b10000011, 0b10000001], 1, false);
    }

    #[test]
    fn test_packet_is_valid_4() {
        test_packet_is_valid(0b011, [0b10000000, 0b00000001], 1, false);
    }

    #[test]
    fn test_packet_is_valid_5() {
        test_packet_is_valid(0b011, [0b10000000, 0b10000001], 0, false);
    }

    #[test]
    fn test_packet_is_valid_6() {
        test_packet_is_valid(0b101, [0b10000000, 0b00000001], 1, false);
    }

    #[test]
    fn test_packet_is_valid_7() {
        test_packet_is_valid(0b101, [0b10000000, 0b10000001], 0, false);
    }

    #[test]
    fn test_packet_is_valid_8() {
        test_packet_is_valid(0b110, [0b10000000, 0b00000001], 1, false);
    }

    #[test]
    fn test_packet_is_valid_9() {
        test_packet_is_valid(0b110, [0b10000000, 0b10000001], 0, false);
    }

    #[test]
    fn test_packet_is_valid_10() {
        test_packet_is_valid(0b111, [0b10000000, 0b00000001], 1, false);
    }

    #[test]
    fn test_packet_is_valid_11() {
        test_packet_is_valid(0b111, [0b10000000, 0b10000001], 0, false);
    }

    #[test]
    fn test_packet_is_valid_12() {
        test_packet_is_valid(0b100, [0b10000000, 0b00000001], 1, true);
    }

    #[test]
    fn test_packet_is_valid_13() {
        test_packet_is_valid(0b100, [0b10000000, 0b10000001], 0, true);
    }

    #[test]
    fn test_packet_is_valid_14() {
        test_packet_is_valid(0b100, [0b10000000, 0b00100000], 0, false);
    }

    #[test]
    fn test_packet_is_valid_15() {
        test_packet_is_valid(0b100, [0b10000001, 0b10000000], 1, false);
    }

    // ----------------------------------------------------------
    // Derives

    #[test]
    fn test_packet_clone() {
        let word = WordType::Command(CommandWord::new());
        let packet1 = Packet::try_from(word).unwrap();
        let packet2 = packet1.clone();
        assert_eq!(packet1, packet2);
    }

    // ----------------------------------------------------------

    // ----------------------------------------------------------
    // Traits

    fn test_packet_try_from_word(word: WordType, expected: bool) {
        let result = Packet::try_from(word);
        assert_eq!(result.is_ok(), expected);
    }

    #[test]
    fn test_packet_try_from_word_0() {
        test_packet_try_from_word(WordType::Command(CommandWord::new()), true);
    }

    #[test]
    fn test_packet_try_from_word_1() {
        test_packet_try_from_word(WordType::Status(StatusWord::new()), true);
    }

    #[test]
    fn test_packet_try_from_word_2() {
        test_packet_try_from_word(WordType::Data(DataWord::new()), true);
    }

    #[test]
    fn test_packet_try_from_word_3() {
        test_packet_try_from_word(WordType::None, false);
    }

    fn test_command_try_from_packet(input: &[u8], expected: bool) {
        let result = CommandWord::try_from(Packet::read(input, 0).unwrap());
        assert_eq!(result.is_ok(), expected);
    }

    #[test]
    fn test_command_try_from_packet_0() {
        test_command_try_from_packet(&[0b00100000, 0b00000000, 0b00010000], false);
    }

    #[test]
    fn test_command_try_from_packet_1() {
        test_command_try_from_packet(&[0b01000000, 0b00000000, 0b00010000], false);
    }

    #[test]
    fn test_command_try_from_packet_2() {
        test_command_try_from_packet(&[0b01100000, 0b00000000, 0b00010000], false);
    }

    #[test]
    fn test_command_try_from_packet_3() {
        test_command_try_from_packet(&[0b10100000, 0b00000000, 0b00010000], false);
    }

    #[test]
    fn test_command_try_from_packet_4() {
        test_command_try_from_packet(&[0b11000000, 0b00000000, 0b00010000], false);
    }

    #[test]
    fn test_command_try_from_packet_5() {
        test_command_try_from_packet(&[0b10100000, 0b00000000, 0b00010000], false);
    }

    #[test]
    fn test_command_try_from_packet_6() {
        test_command_try_from_packet(&[0b10000000, 0b00000000, 0b00010000], true);
    }

    #[test]
    fn test_command_try_from_packet_7() {
        test_command_try_from_packet(&[0b10000000, 0b00000000, 0b00010000], true);
    }

    fn test_status_try_from_packet(input: &[u8], expected: bool) {
        let result = StatusWord::try_from(Packet::read(input, 0).unwrap());
        assert_eq!(result.is_ok(), expected);
    }

    #[test]
    fn test_status_try_from_packet_0() {
        test_status_try_from_packet(&[0b00100000, 0b00000000, 0b00010000], false);
    }

    #[test]
    fn test_status_try_from_packet_1() {
        test_status_try_from_packet(&[0b01000000, 0b00000000, 0b00010000], false);
    }

    #[test]
    fn test_status_try_from_packet_2() {
        test_status_try_from_packet(&[0b01100000, 0b00000000, 0b00010000], false);
    }

    #[test]
    fn test_status_try_from_packet_3() {
        test_status_try_from_packet(&[0b10100000, 0b00000000, 0b00010000], false);
    }

    #[test]
    fn test_status_try_from_packet_4() {
        test_status_try_from_packet(&[0b11000000, 0b00000000, 0b00010000], false);
    }

    #[test]
    fn test_status_try_from_packet_5() {
        test_status_try_from_packet(&[0b10100000, 0b00000000, 0b00010000], false);
    }

    #[test]
    fn test_status_try_from_packet_6() {
        test_status_try_from_packet(&[0b10000000, 0b00000000, 0b00010000], true);
    }

    #[test]
    fn test_status_try_from_packet_7() {
        test_status_try_from_packet(&[0b10000000, 0b00000000, 0b00010000], true);
    }

    fn test_data_try_from_packet(input: &[u8], expected: bool) {
        let result = DataWord::try_from(Packet::read(input, 0).unwrap());
        assert_eq!(result.is_ok(), expected);
    }

    #[test]
    fn test_data_try_from_packet_0() {
        test_data_try_from_packet(&[0b01000000, 0b00000000, 0b00010000], false);
    }

    #[test]
    fn test_data_try_from_packet_1() {
        test_data_try_from_packet(&[0b01100000, 0b00000000, 0b00010000], false);
    }

    #[test]
    fn test_data_try_from_packet_2() {
        test_data_try_from_packet(&[0b10000000, 0b00000000, 0b00010000], false);
    }

    #[test]
    fn test_data_try_from_packet_3() {
        test_data_try_from_packet(&[0b10100000, 0b00000000, 0b00010000], false);
    }

    #[test]
    fn test_data_try_from_packet_4() {
        test_data_try_from_packet(&[0b11000000, 0b00000000, 0b00010000], false);
    }

    #[test]
    fn test_data_try_from_packet_5() {
        test_data_try_from_packet(&[0b10100000, 0b00000000, 0b00010000], false);
    }

    #[test]
    fn test_data_try_from_packet_6() {
        test_data_try_from_packet(&[0b00100000, 0b00000000, 0b00010000], true);
    }

    #[test]
    fn test_data_try_from_packet_7() {
        test_data_try_from_packet(&[0b00100000, 0b00000000, 0b00010000], true);
    }

    // ----------------------------------------------------------
}
