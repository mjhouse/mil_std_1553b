//! A "word" in the 1553B standard is made up of twenty bits, total. Three sync bits, 
//! 16 bits of data (in one of three different formats), and a trailing parity
//! bit. This means that there are two ways of referencing a particular bit- either with 
//! a bit index offset from the beginning of the *word data* or as a "bit time" offset
//! from the begining of the word, including the sync bits.
//!
//! |------ |-- |-- |-- |-- |-- |-- |-- |-- |-- |--  |--  |--  |--  |--  |--  |--  |--  |--  |--  |-- |
//! | Index | - | - | - | 0 | 1 | 2 | 3 | 4 | 5 |  6 |  7 |  8 |  9 | 10 | 11 | 12 | 13 | 14 | 15 | - |
//! | Time  | - | - | - | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | - |
//!
//! The bit-time reference is used in the standard, but because we're only dealing with 
//! the 16-bit data from each word in this project we'll be using a zero-indexed reference
//! in the actual code.

pub struct Field {
    pub mask: u16,
    pub offset: u8,
}

impl Field {
    pub const fn new(mask: u16, offset: u8) -> Self {
        Self { mask, offset }
    }
}

/// The leading sync pattern for a data word
pub const DATA_SYNC: u8 = 0b00000111;

/// The leading sync pattern for a non-data word
pub const SERV_SYNC: u8 = 0b00111000;

/// Constant for a "full" (all-ones) data value
pub const FULL_WORD: u16                            = 0b1111111111111111;

/// Constant for an "empty" (all-zero) data value
pub const EMPTY_WORD: u16                           = 0b0000000000000000;

/// The five bit Terminal Address (TA) field (bit times 4-8) states to which 
/// unique remote terminal the command is intended (no two terminals may have 
/// the same address).
///
/// NOTE: An address of 00000B is a valid address, and an address of 11111B is
/// always reserved as a broadcast address. Additionally, there is no
/// requirement that the bus controller be assigned an address, therefore
/// the maximum number of terminals the data bus can support is 31.
///
/// See: http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf (p.21/22)
pub const COMMAND_TERMINAL_ADDRESS: u16             = 0b1111100000000000;
pub const COMMAND_TERMINAL_ADDRESS_FIELD: Field     = Field::new(COMMAND_TERMINAL_ADDRESS,11);

/// The next bit (bit time 9) makes up the Transmit/Receive (T/R) bit. This
/// defines the direction of information flow and is always from the point of
/// view of the remote terminal. A transmit command (logic 1) indicates that
/// the remote terminal is to transmit data, while a receive command (logic 0)
/// indicates that the remote terminal is going to receive data. The only
/// exceptions to this rule are associated with mode commands.
///
/// See: http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf (p.22)
pub const COMMAND_TRANSMIT_RECEIVE: u16             = 0b0000010000000000;
pub const COMMAND_TRANSMIT_RECEIVE_FIELD: Field     = Field::new(COMMAND_TRANSMIT_RECEIVE,10);


/// The next five bits (bit times 10-14) make up the Subaddress (SA)/Mode
/// Command bits. Logic 00000B or 11111B within this field is decoded to
/// indicate that the command is a Mode Code Command. All other logic
/// combinations of this field are used to direct the data to different functions
/// within the subsystem. An example might be that 00001B is position and
/// rate data, 00010B is frequency data, 10010B is display information, and
/// 10011B is self-test data.
///
/// See: http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf (p.22)
pub const COMMAND_SUBADDRESS: u16                   = 0b0000001111100000;
pub const COMMAND_SUBADDRESS_FIELD: Field           = Field::new(COMMAND_SUBADDRESS,5);


/// The next five bit positions (bit times 15-19) define the Word Count (WC)
/// or Mode Code to be performed. If the Subaddress/Mode Code field is
/// 00000B or 11111B, then this field defines the mode code to be performed.
/// If not a mode code, then this field defines the number of data words to be
/// received or transmitted depending on the T/R bit. A word count field of
/// 00000B is decoded as 32 data words.
///
/// See: http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf (p.22)
pub const COMMAND_MODE_CODE: u16                    = 0b0000000000011111;
pub const COMMAND_MODE_CODE_FIELD: Field            = Field::new(COMMAND_MODE_CODE,0);

/// The next five bit positions (bit times 15-19) define the Word Count (WC)
/// or Mode Code to be performed. If the Subaddress/Mode Code field is
/// 00000B or 11111B, then this field defines the mode code to be performed.
/// If not a mode code, then this field defines the number of data words to be
/// received or transmitted depending on the T/R bit. A word count field of
/// 00000B is decoded as 32 data words.
///
/// See: http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf (p.22)
pub const COMMAND_WORD_COUNT: u16                   = 0b0000000000011111;
pub const COMMAND_WORD_COUNT_FIELD: Field           = Field::new(COMMAND_WORD_COUNT,0);

/// The first five bits (bit times 4-8) of the information field are the Terminal
/// Address (TA). These five bits should match the corresponding field within
/// the command word that the terminal received. The remote terminal sets
/// these bits to the address to which it has been programmed. The bus
/// controller should examine these bits to insure that the terminal responding
/// with its status word was indeed the terminal to which the command word
/// was addressed.
///
/// See: http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf (p.24)
pub const STATUS_TERMINAL_ADDRESS: u16              = 0b1111100000000000;
pub const STATUS_TERMINAL_ADDRESS_FIELD: Field      = Field::new(STATUS_TERMINAL_ADDRESS,11);

/// The next bit (bit time 9) is the Message Error (ME) bit. This bit is set by
/// the remote terminal upon detection of an error in the message or upon
/// detection of an invalid message (i.e. Illegal Command) to the terminal.
/// The error may occur in any of the data words within the message. When
/// the terminal detects an error and sets this bit, none of the data received
/// within the message is used. In fact, once an error is detected, the terminal
/// need not continue decoding the rest of the message, though most do.
///
/// See: http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf (p.24)
pub const STATUS_MESSAGE_ERROR: u16                 = 0b0000010000000000;
pub const STATUS_MESSAGE_ERROR_FIELD: Field         = Field::new(STATUS_MESSAGE_ERROR,10);

/// The Instrumentation bit (bit time 10) is provided to differentiate between a
/// command word and a status word (remember they both have the same sync
/// pattern). The instrumentation bit in the status word is always set to logic
/// "0". If used, the corresponding bit in the command word is set to a logic
/// "1". This bit in the command word is the most significant bit of the
/// Subaddress field, and therefore, would limit the subaddresses used to
/// 10000 - 11110, hence reducing the number of subaddresses available from
/// 30 to 15. The instrumentation bit is also the reason why there are two
/// mode code identifiers (00000B and 11111B), the latter required when the
/// instrumentation bit is used.
/// 
/// Earlier monitoring systems required the use of the instrumentation bit to
/// differentiate between command and status words. However, the price paid
/// (loss of subaddresses) was too high for modern applications. Most
/// monitors today are capable of following the protocol and message formats
/// to determine which word is which.
///
/// See: http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf (p.24/25)
pub const STATUS_INSTRUMENTATION: u16               = 0b0000001000000000;
pub const STATUS_INSTRUMENTATION_FIELD: Field       = Field::new(STATUS_INSTRUMENTATION,9);

/// The Service Request bit (bit time 11) is provided so that the remote
/// terminal can inform the bus controller that it needs to be serviced. This bit
/// is set to a logic "1" by the subsystem to indicate that servicing is needed.
/// This bit is typically used when the bus controller is "polling" terminals to
/// determine if they require processing.
///
/// See: http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf (p.25)
pub const STATUS_SERVICE_REQUEST: u16               = 0b0000000100000000;
pub const STATUS_SERVICE_REQUEST_FIELD: Field       = Field::new(STATUS_SERVICE_REQUEST,8);

/// Bit times 12-14 are reserved for future growth of the standard and must be
/// set to a logic "0". The bus controller should declare a message in error if
/// the remote terminal responds with any of these bits set in its status word.
///
/// See: http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf (p.25)
pub const STATUS_RESERVED_BITS: u16                 = 0b0000000011100000;
pub const STATUS_RESERVED_BITS_FIELD: Field         = Field::new(STATUS_RESERVED_BITS,5);

/// The Broadcast Command Received bit (bit time 15) indicates that the
/// remote terminal received a valid broadcast command. On receiving a valid
/// broadcast command, the remote terminal sets this bit to logic "1" and
/// suppresses the transmission of its status words. The bus controller may
/// issue a Transmit Status Word or Transmit Last Command Word Mode
/// Code to determine if the terminal received the message properly.
///
/// See: http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf (p.25)
pub const STATUS_BROADCAST_RECEIVED: u16            = 0b0000000000010000;
pub const STATUS_BROADCAST_RECEIVED_FIELD: Field    = Field::new(STATUS_BROADCAST_RECEIVED,4);

/// The Busy bit (bit time 16) is provided as a feedback to the bus controller as
/// to when the remote terminal is unable to move data between the remote
/// terminal electronics and the subsystem in compliance to a command from
/// the bus controller.
///
/// See: http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf (p.25)
pub const STATUS_TERMINAL_BUSY: u16                 = 0b0000000000001000;
pub const STATUS_TERMINAL_BUSY_FIELD: Field         = Field::new(STATUS_TERMINAL_BUSY,3);

/// The Subsystem Flag bit (bit time 17) is used to provide "health" data
/// regarding the subsystems to which the remote terminal is connected.
/// Multiple subsystems may logically "OR" their bits together to form a
/// composite health indicator. 
///
/// This single bit serves only as an indicator to the bus controller and 
/// user of the data that a fault or failure exists.
///
/// See: http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf (p.26)
pub const STATUS_SUBSYSTEM_FLAG: u16                = 0b0000000000000100;
pub const STATUS_SUBSYSTEM_FLAG_FIELD: Field        = Field::new(STATUS_SUBSYSTEM_FLAG,2);

/// The Dynamic Bus Control Acceptance bit (bit time 18) informs the bus
/// controller that the remote terminal has received the Dynamic Bus Control
/// Mode Code and has accepted control of the bus. For the remote terminal,
/// the setting of this bit is controlled by the subsystem and is based on passing
/// some level of built-in-test (i.e., a processor passing its power-up and
/// continuous background tests).
///
/// See: http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf (p.27)
pub const STATUS_BUS_CONTROL_ACCEPT: u16            = 0b0000000000000010;
pub const STATUS_BUS_CONTROL_ACCEPT_FIELD: Field    = Field::new(STATUS_BUS_CONTROL_ACCEPT,1);

/// The Terminal Flag bit (bit time 19) informs the bus controller of a fault or
/// failure within the remote terminal circuitry (only the remote terminal). A
/// logic "1" shall indicate a fault condition.
/// 
/// This bit is used solely to inform the bus controller of a fault or failure.
/// Further information regarding the nature of the failure must be obtained in
/// some other fashion
///
/// See: http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf (p.27)
pub const STATUS_TERMINAL_FLAG: u16                = 0b0000000000000001;
pub const STATUS_TERMINAL_FLAG_FIELD: Field        = Field::new(STATUS_TERMINAL_FLAG,0);