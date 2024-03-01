//! Fields found in command and status words

/// Represents a field inside of a 16-bit word
///
/// Given a mask and offset, the Field struct can get 
/// or set between 1 and 8 bits in a u16 word.
pub struct Field {
    /// The mask used to isolate the value
    mask: u16,

    /// The offset used to shift the value to a single byte
    offset: u32,
}

impl Field {
    /// Create a new field given a mask
    pub const fn new(mask: u16) -> Self {
        Self {
            mask,
            offset: mask.trailing_zeros(),
        }
    }

    /// Read the value of the field from a data word
    pub const fn get(&self, data: u16) -> u8 {
        ((data & self.mask) >> self.offset) as u8
    }

    /// Write the value of the field to a data word
    pub const fn set(&self, data: u16, value: u8) -> u16 {
        let v = (value as u16) << self.offset;
        (data & !self.mask) | (v & self.mask)
    }
}

/// Mask for an empty field
pub const WORD_EMPTY: u16 = 0b0000000000000000;

/// Mask for parsing the terminal address of a command word.
pub const COMMAND_TERMINAL_ADDRESS: u16 = 0b1111100000000000;

/// Field definition for the terminal address of a command word.
pub const COMMAND_TERMINAL_ADDRESS_FIELD: Field = Field::new(COMMAND_TERMINAL_ADDRESS);

/// Mask for parsing the transmit/receive flag of a command word.
pub const COMMAND_TRANSMIT_RECEIVE: u16 = 0b0000010000000000;

/// Field definition for the transmit/receive flag of a command word.
pub const COMMAND_TRANSMIT_RECEIVE_FIELD: Field = Field::new(COMMAND_TRANSMIT_RECEIVE);

/// Mask for parsing the terminal subaddress of a command word.
pub const COMMAND_SUBADDRESS: u16 = 0b0000001111100000;

/// Field definition for the terminal subaddress of a command word.
pub const COMMAND_SUBADDRESS_FIELD: Field = Field::new(COMMAND_SUBADDRESS);

/// Mask for parsing the mode code of a command word.
pub const COMMAND_MODE_CODE: u16 = 0b0000000000011111;

/// Field definition for the mode code of a command word.
pub const COMMAND_MODE_CODE_FIELD: Field = Field::new(COMMAND_MODE_CODE);

/// Mask for parsing the word count of a command word.
pub const COMMAND_WORD_COUNT: u16 = 0b0000000000011111;

/// Field definition for the word count of a command word.
pub const COMMAND_WORD_COUNT_FIELD: Field = Field::new(COMMAND_WORD_COUNT);

/// Mask for parsing the terminal address of a status word.
pub const STATUS_TERMINAL_ADDRESS: u16 = 0b1111100000000000;

/// Field definition for the terminal address of a status word.
pub const STATUS_TERMINAL_ADDRESS_FIELD: Field = Field::new(STATUS_TERMINAL_ADDRESS);

/// Mask for parsing the error flag of a status word.
pub const STATUS_MESSAGE_ERROR: u16 = 0b0000010000000000;

/// Field definition for the error flag of a status word.
pub const STATUS_MESSAGE_ERROR_FIELD: Field = Field::new(STATUS_MESSAGE_ERROR);

/// Mask for parsing the instrumentation flag of a status word.
pub const STATUS_INSTRUMENTATION: u16 = 0b0000001000000000;

/// Field definition for the instrumentation flag of a status word.
pub const STATUS_INSTRUMENTATION_FIELD: Field = Field::new(STATUS_INSTRUMENTATION);

/// Mask for parsing the service request flag of a status word.
pub const STATUS_SERVICE_REQUEST: u16 = 0b0000000100000000;

/// Field definition for the service request flag of a status word.
pub const STATUS_SERVICE_REQUEST_FIELD: Field = Field::new(STATUS_SERVICE_REQUEST);

/// Mask for parsing the reserved bits of a status word.
pub const STATUS_RESERVED_BITS: u16 = 0b0000000011100000;

/// Field definition for the reserved bits of a status word.
pub const STATUS_RESERVED_BITS_FIELD: Field = Field::new(STATUS_RESERVED_BITS);

/// Mask for parsing the broadcast received flag of a status word.
pub const STATUS_BROADCAST_RECEIVED: u16 = 0b0000000000010000;

/// Field definition for the broadcast received flag of a status word.
pub const STATUS_BROADCAST_RECEIVED_FIELD: Field = Field::new(STATUS_BROADCAST_RECEIVED);

/// Mask for parsing the busy flag of the status word.
pub const STATUS_TERMINAL_BUSY: u16 = 0b0000000000001000;

/// Field definition for the busy flag of the status word.
pub const STATUS_TERMINAL_BUSY_FIELD: Field = Field::new(STATUS_TERMINAL_BUSY);

/// Mask for parsing the subsystem flag of the status word.
pub const STATUS_SUBSYSTEM_FLAG: u16 = 0b0000000000000100;

/// Field definition for the subsystem flag of the status word.
pub const STATUS_SUBSYSTEM_FLAG_FIELD: Field = Field::new(STATUS_SUBSYSTEM_FLAG);

/// Mask for parsing the bus control accept flag of the status word.
pub const STATUS_DYNAMIC_BUS_ACCEPT: u16 = 0b0000000000000010;

/// Field definition for the bus control accept flag of the status word.
pub const STATUS_DYNAMIC_BUS_ACCEPT_FIELD: Field = Field::new(STATUS_DYNAMIC_BUS_ACCEPT);

/// Mask for parsing the terminal flag of the status word.
pub const STATUS_TERMINAL_FLAG: u16 = 0b0000000000000001;

/// Field definition for the terminal flag of the status word.
pub const STATUS_TERMINAL_FLAG_FIELD: Field = Field::new(STATUS_TERMINAL_FLAG);
