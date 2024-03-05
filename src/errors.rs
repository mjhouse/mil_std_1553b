//! Error enums and flags

use core::array::TryFromSliceError;

/// A result type which uses the [Error] enum as the error type.
pub type Result<T> = core::result::Result<T, Error>;

/// Calculate a parity bit given a u16 word value
///
/// MIL STD 1553B uses an odd parity bit (1 if the
/// bit count of the data is even, 0 if not)[^1].
///
/// [^1]: [MIL-STD-1553 Tutorial](http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf)
#[inline]
#[must_use = "Returned value is not used"]
pub(crate) const fn parity(v: u16) -> u8 {
    match v.count_ones() % 2 {
        0 => 1,
        _ => 0,
    }
}

/// An error deriving from the software itself, rather than a terminal.
///
/// These errors occur during parsing or other calculations when those
/// calculations fail. The [Error::SystemError] variant
/// contains any errors generated by the 1553 bus.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum Error {
    /// An index was out of bounds for the data structure
    OutOfBounds,

    /// An unexpected or invalid ModeCode was found
    InvalidCode,

    /// This word is not a ModeCode command word
    NotModeCode,

    /// A malformed packet was given while building a message
    PacketIsInvalid,

    /// A word was found to be invalid while building a message
    WordIsInvalid,

    /// An array could not be created from a given slice
    FromSliceError,

    /// A byte array could not be converted to a string
    StringIsInvalid,

    /// An invalid StatusWord was given while building a message
    InvalidStatusWord,

    /// An invalid CommandWord was given while building a message
    InvalidCommandWord,

    /// A message cannot begin with a data word
    FirstWordIsData,

    /// A message cannot add new words because it is full
    MessageIsFull,

    /// A message cannot add a status word if it isn't empty
    StatusWordNotFirst,

    /// A message cannot add a command word if it isn't empty
    CommandWordNotFirst,

    /// The reserved bits of a word were used
    ReservedUsed,

    /// The requested resource was not found
    NotFound,

    /// The message has an unrecognizeable configuration
    InvalidMessage,

    /// An error from a terminal elsewhere in the system (see [SystemError])
    SystemError(SystemError),
}

impl From<TryFromSliceError> for Error {
    fn from(_: TryFromSliceError) -> Self {
        Self::FromSliceError
    }
}

/// An error deriving from a remote terminal or bus controller.
///
/// These errors are generated during runtime by terminals and
/// provided in messages on the bus.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum SystemError {
    /// No error
    None,

    /// A terminal error (see [TerminalError] for more information)
    Terminal(TerminalError),

    /// A subsystem error (see [SubsystemError] for more information)
    Subsystem(SubsystemError),

    /// A message error (see [MessageError] for more information)
    Message(MessageError),
}

/// This flag is to inform the bus controller of faults in a remote terminal
///
/// The error bit flag defined here maps to the Terminal Flag bit at bit
/// time 19 (index 15). It is used to notify the bus controller of a fault
/// or failure within the *entire* remote terminal, rather than only the
/// channel on which the error was received.
///
/// This flag is described on page 35 in the MIL-STD-1553 Tutorial[^1].
///
/// [^1]: [MIL-STD-1553 Tutorial](http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum TerminalError {
    /// No error
    None = 0,

    /// An error has occurred
    Error = 1,
}

impl TerminalError {
    /// Check if the enum is the 'None' variant
    #[must_use = "Returned value is not used"]
    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Check if the enum is the 'Error' variant
    #[must_use = "Returned value is not used"]
    pub const fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}

impl From<u8> for TerminalError {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Error,
            _ => Self::None,
        }
    }
}

impl From<TerminalError> for u8 {
    fn from(value: TerminalError) -> Self {
        match value {
            TerminalError::Error => 1,
            TerminalError::None => 0,
        }
    }
}

/// This flag provides health data regarding subsystems of a remote terminal.
///
/// The Subsystem Flag bit located at bit time 17 (index 13) is used to provide
/// “health” data regarding the subsystems to which the remote terminal is connected.
///
/// Multiple subsystems may logically OR their bits together to form a composite
/// health indicator. This indicator only informs the bus controller that a fault
/// or failure exists, and further information must be obtained in some other fashion.
///
/// This flag is described on page 34 in the MIL-STD-1553 Tutorial[^1].
///
/// [^1]: [MIL-STD-1553 Tutorial](http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum SubsystemError {
    /// No error
    None = 0,

    /// An error has occurred
    Error = 1,
}

impl SubsystemError {
    /// Check if the enum is 'None' variant
    #[must_use = "Returned value is not used"]
    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Check if the enum is 'Error' variant
    #[must_use = "Returned value is not used"]
    pub const fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}

impl From<u8> for SubsystemError {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Error,
            _ => Self::None,
        }
    }
}

impl From<SubsystemError> for u8 {
    fn from(value: SubsystemError) -> Self {
        match value {
            SubsystemError::Error => 1,
            SubsystemError::None => 0,
        }
    }
}

/// This flag is set when a receiving terminal detects an error in a message.
///
/// The error may have occurred in any of the data words within the message, and
/// when a terminal receives this flag in a message, it will ignore all data
/// words in the containing message. If an error is detected within a message
/// and this flag is set, the remote terminal must suppress transmission of the
/// status word. If an illegal command is detected, this flag is set and the
/// status word is transmitted.
///
/// This flag is described on page 32 in the MIL-STD-1553 Tutorial[^1].
///
/// [^1]: [MIL-STD-1553 Tutorial](http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum MessageError {
    /// No error
    None = 0,

    /// An error has occurred
    Error = 1,
}

impl MessageError {
    /// Check if the enum is 'None' variant
    #[must_use = "Returned value is not used"]
    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Check if enum is 'Error' variant
    #[must_use = "Returned value is not used"]
    pub const fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}

impl From<u8> for MessageError {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Error,
            _ => Self::None,
        }
    }
}

impl From<MessageError> for u8 {
    fn from(value: MessageError) -> Self {
        match value {
            MessageError::Error => 1,
            MessageError::None => 0,
        }
    }
}
