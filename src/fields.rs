//! Fields found in command and status words

use crate::Word;

/// Represents a field inside of a 16-bit word
///
/// Given a mask and offset, the Field struct can get
/// or set between 1 and 8 bits in a u16 word.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Field {
    /// The mask used to isolate the value
    mask: u16,

    /// The offset used to shift the value to a single byte
    offset: u32,
}

impl Field {
    /// Create a new field
    pub const fn new() -> Self {
        Self { mask: 0, offset: 0 }
    }

    /// Constructor method to add a mask to the field
    pub const fn with_mask(mut self, mask: u16) -> Self {
        self.mask = mask;
        self
    }

    /// Constructor method to calculate an offset
    pub const fn with_offset(mut self) -> Self {
        self.offset = self.mask.trailing_zeros();
        self
    }

    /// Create a new field from a mask
    pub const fn from(mask: u16) -> Self {
        Self::new()
            .with_mask(mask)
            .with_offset()
    }

    /// Read the value of the field from a data word
    pub fn get<T: Word>(&self, word: &T) -> u8 {
        let value = word.as_value() & self.mask;
        (value >> self.offset) as u8
    }

    /// Write the value of the field to a data word
    pub fn set<T: Word>(&self, word: &mut T, value: u8) {
        let value = (value as u16) << self.offset;
        let data = word.as_value() & !self.mask;
        word.set_value(data | (value & self.mask));
    }
}

/// Mask for parsing the terminal address of a command word.
pub(crate) const COMMAND_ADDRESS: u16 = 0b1111100000000000;

/// Field definition for the terminal address of a command word.
pub(crate) const COMMAND_ADDRESS_FIELD: Field = Field::from(COMMAND_ADDRESS);

/// Mask for parsing the transmit/receive flag of a command word.
pub(crate) const COMMAND_TRANSMIT_RECEIVE: u16 = 0b0000010000000000;

/// Field definition for the transmit/receive flag of a command word.
pub(crate) const COMMAND_TRANSMIT_RECEIVE_FIELD: Field = Field::from(COMMAND_TRANSMIT_RECEIVE);

/// Mask for parsing the terminal subaddress of a command word.
pub(crate) const COMMAND_SUBADDRESS: u16 = 0b0000001111100000;

/// Field definition for the terminal subaddress of a command word.
pub(crate) const COMMAND_SUBADDRESS_FIELD: Field = Field::from(COMMAND_SUBADDRESS);

/// Mask for parsing the mode code of a command word.
pub(crate) const COMMAND_MODE_CODE: u16 = 0b0000000000011111;

/// Field definition for the mode code of a command word.
pub(crate) const COMMAND_MODE_CODE_FIELD: Field = Field::from(COMMAND_MODE_CODE);

/// Mask for parsing the word count of a command word.
pub(crate) const COMMAND_WORD_COUNT: u16 = 0b0000000000011111;

/// Field definition for the word count of a command word.
pub(crate) const COMMAND_WORD_COUNT_FIELD: Field = Field::from(COMMAND_WORD_COUNT);

/// Mask for parsing the terminal address of a status word.
pub(crate) const STATUS_ADDRESS: u16 = 0b1111100000000000;

/// Field definition for the terminal address of a status word.
pub(crate) const STATUS_ADDRESS_FIELD: Field = Field::from(STATUS_ADDRESS);

/// Mask for parsing the error flag of a status word.
pub(crate) const STATUS_MESSAGE_ERROR: u16 = 0b0000010000000000;

/// Field definition for the error flag of a status word.
pub(crate) const STATUS_MESSAGE_ERROR_FIELD: Field = Field::from(STATUS_MESSAGE_ERROR);

/// Mask for parsing the instrumentation flag of a status word.
pub(crate) const STATUS_INSTRUMENTATION: u16 = 0b0000001000000000;

/// Field definition for the instrumentation flag of a status word.
pub(crate) const STATUS_INSTRUMENTATION_FIELD: Field = Field::from(STATUS_INSTRUMENTATION);

/// Mask for parsing the service request flag of a status word.
pub(crate) const STATUS_SERVICE_REQUEST: u16 = 0b0000000100000000;

/// Field definition for the service request flag of a status word.
pub(crate) const STATUS_SERVICE_REQUEST_FIELD: Field = Field::from(STATUS_SERVICE_REQUEST);

/// Mask for parsing the reserved bits of a status word.
pub(crate) const STATUS_RESERVED: u16 = 0b0000000011100000;

/// Field definition for the reserved bits of a status word.
pub(crate) const STATUS_RESERVED_FIELD: Field = Field::from(STATUS_RESERVED);

/// Mask for parsing the broadcast received flag of a status word.
pub(crate) const STATUS_BROADCAST_RECEIVED: u16 = 0b0000000000010000;

/// Field definition for the broadcast received flag of a status word.
pub(crate) const STATUS_BROADCAST_RECEIVED_FIELD: Field = Field::from(STATUS_BROADCAST_RECEIVED);

/// Mask for parsing the busy flag of the status word.
pub(crate) const STATUS_TERMINAL_BUSY: u16 = 0b0000000000001000;

/// Field definition for the busy flag of the status word.
pub(crate) const STATUS_TERMINAL_BUSY_FIELD: Field = Field::from(STATUS_TERMINAL_BUSY);

/// Mask for parsing the subsystem flag of the status word.
pub(crate) const STATUS_SUBSYSTEM_ERROR: u16 = 0b0000000000000100;

/// Field definition for the subsystem flag of the status word.
pub(crate) const STATUS_SUBSYSTEM_ERROR_FIELD: Field = Field::from(STATUS_SUBSYSTEM_ERROR);

/// Mask for parsing the bus control accept flag of the status word.
pub(crate) const STATUS_DYNAMIC_BUS_ACCEPTANCE: u16 = 0b0000000000000010;

/// Field definition for the bus control accept flag of the status word.
pub(crate) const STATUS_DYNAMIC_BUS_ACCEPTANCE_FIELD: Field =
    Field::from(STATUS_DYNAMIC_BUS_ACCEPTANCE);

/// Mask for parsing the terminal flag of the status word.
pub(crate) const STATUS_TERMINAL_ERROR: u16 = 0b0000000000000001;

/// Field definition for the terminal flag of the status word.
pub(crate) const STATUS_TERMINAL_ERROR_FIELD: Field = Field::from(STATUS_TERMINAL_ERROR);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DataWord;

    #[test]
    fn test_field_clone() {
        let field1 = Field::from(0b1010101010101010);
        let field2 = field1.clone();
        assert_eq!(field1, field2);
    }

    #[test]
    fn test_field_new() {
        let field = Field::new();
        assert_eq!(field.mask, 0);
        assert_eq!(field.offset, 0);
    }

    #[test]
    fn test_field_with_mask() {
        let field = Field::new().with_mask(0b1010101010101010);
        assert_eq!(field.mask, 0b1010101010101010);
        assert_eq!(field.offset, 0);
    }

    #[test]
    fn test_field_with_offset_0() {
        let field = Field::new()
            .with_mask(0b1010101010101010)
            .with_offset();
        assert_eq!(field.offset, 1);
    }

    #[test]
    fn test_field_with_offset_1() {
        let field = Field::new()
            .with_mask(0b1010101010101000)
            .with_offset();
        assert_eq!(field.offset, 3);
    }

    #[test]
    fn test_field_with_offset_2() {
        let field = Field::new()
            .with_mask(0b1010101010100000)
            .with_offset();
        assert_eq!(field.offset, 5);
    }

    #[test]
    fn test_field_with_offset_3() {
        let field = Field::new()
            .with_mask(0b1010101010000000)
            .with_offset();
        assert_eq!(field.offset, 7);
    }

    #[test]
    fn test_field_from_u16_0() {
        let input = 0b1010101010101010;
        let expected = 1;
        let field = Field::from(input);
        assert_eq!(field.mask, input);
        assert_eq!(field.offset, expected);
    }

    #[test]
    fn test_field_from_u16_1() {
        let input = 0b1010101010101000;
        let expected = 3;
        let field = Field::from(input);
        assert_eq!(field.mask, input);
        assert_eq!(field.offset, expected);
    }

    #[test]
    fn test_field_from_u16_2() {
        let input = 0b1010101010100000;
        let expected = 5;
        let field = Field::from(input);
        assert_eq!(field.mask, input);
        assert_eq!(field.offset, expected);
    }

    #[test]
    fn test_field_from_u16_3() {
        let input = 0b1010101010000000;
        let expected = 7;
        let field = Field::from(input);
        assert_eq!(field.mask, input);
        assert_eq!(field.offset, expected);
    }

    #[test]
    fn test_field_get_0() {
        let mask = 0b1110000000000000;
        let input = 0b1010000000000000;
        let expected = 0b101;
        let field = Field::from(mask);
        let word = DataWord::from(input);
        let value = field.get(&word);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_field_get_1() {
        let mask = 0b0011110000000000;
        let input = 0b0010110000000000;
        let expected = 0b1011;
        let field = Field::from(mask);
        let word = DataWord::from(input);
        let value = field.get(&word);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_field_get_2() {
        let mask = 0b0000011111000000;
        let input = 0b0000010101000000;
        let expected = 0b10101;
        let field = Field::from(mask);
        let word = DataWord::from(input);
        let value = field.get(&word);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_field_get_3() {
        let mask = 0b0000000001111110;
        let input = 0b0000000001011010;
        let expected = 0b101101;
        let field = Field::from(mask);
        let word = DataWord::from(input);
        let value = field.get(&word);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_field_get_4() {
        let mask = 0b0000000000000011;
        let input = 0b0000000000000011;
        let expected = 0b0000011;
        let field = Field::from(mask);
        let word = DataWord::from(input);
        let value = field.get(&word);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_field_set_0() {
        let mask = 0b1110000000000000;
        let input = 0b101;
        let expected = 0b1010000000000000;
        let field = Field::from(mask);
        let mut word = DataWord::from(0);

        field.set(&mut word, input);
        assert_eq!(word.as_value(), expected);
    }

    #[test]
    fn test_field_set_1() {
        let mask = 0b0011110000000000;
        let input = 0b1011;
        let expected = 0b0010110000000000;
        let field = Field::from(mask);
        let mut word = DataWord::from(0);

        field.set(&mut word, input);
        assert_eq!(word.as_value(), expected);
    }

    #[test]
    fn test_field_set_2() {
        let mask = 0b0000011111000000;
        let input = 0b10101;
        let expected = 0b0000010101000000;
        let field = Field::from(mask);
        let mut word = DataWord::from(0);

        field.set(&mut word, input);
        assert_eq!(word.as_value(), expected);
    }

    #[test]
    fn test_field_set_3() {
        let mask = 0b0000000001111110;
        let input = 0b101101;
        let expected = 0b0000000001011010;
        let field = Field::from(mask);
        let mut word = DataWord::from(0);

        field.set(&mut word, input);
        assert_eq!(word.as_value(), expected);
    }

    #[test]
    fn test_field_set_4() {
        let mask = 0b0000000000000011;
        let input = 0b0000011;
        let expected = 0b0000000000000011;
        let field = Field::from(mask);
        let mut word = DataWord::from(0);

        field.set(&mut word, input);
        assert_eq!(word.as_value(), expected);
    }
}
