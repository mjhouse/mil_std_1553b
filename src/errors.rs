use num_enum::{IntoPrimitive,FromPrimitive};

pub type Result<T> = core::result::Result<T,Error>;

/// An error deriving from the software itself, rather than a terminal.
/// 
/// These errors occur during parsing or other calculations when the 
/// calculations fail.
#[derive(Debug,Clone,PartialEq,Eq)]
#[repr(u8)]
pub enum Error {

    /// An index was out of bounds for the index structure
    OutOfBounds,

    /// An unexpected or invalid ModeCode was found
    InvalidCode,

    /// This word is not a ModeCode command word
    NotModeCode,

    /// Cannot add additional words to the current message
    MessageFull,

    /// The message is malformed 
    MessageBad,

    /// The reserved bits of a word were used
    ReservedUsed,

    /// The requested resource was not found
    NotFound,

    /// The message has an unrecognizeable configuration
    UnknownMessage,

    /// An error from a terminal elsewhere in the system (see [SystemError]) 
    SystemError(SystemError)
}

/// An error deriving from a remote terminal or bus controller.
/// 
/// These errors are generated during runtime by terminals and
/// provided in messages on the bus.
#[derive(Debug,Clone,PartialEq,Eq)]
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
#[derive(Debug,Clone,PartialEq,Eq,IntoPrimitive,FromPrimitive)]
#[repr(u8)]
pub enum TerminalError {
    #[default]
    None  = 0,
    Error = 1,
}

impl TerminalError {

    /// Check if error enum is None variant
    pub const fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false
        }
    }

    /// Check if error enum is Error variant
    pub const fn is_error(&self) -> bool {
        match self {
            Self::Error => true,
            _ => false
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
#[derive(Debug,Clone,PartialEq,Eq,IntoPrimitive,FromPrimitive)]
#[repr(u8)]
pub enum SubsystemError {
    #[default]
    None  = 0,
    Error = 1,
}

impl SubsystemError {

    /// Check if error enum is None variant
    pub const fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false
        }
    }

    /// Check if error enum is Error variant
    pub const fn is_error(&self) -> bool {
        match self {
            Self::Error => true,
            _ => false
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
#[derive(Debug,Clone,PartialEq,Eq,IntoPrimitive,FromPrimitive)]
#[repr(u8)]
pub enum MessageError {
    #[default]
    None  = 0,
    Error = 1,
}

impl MessageError {

    /// Check if error enum is None variant
    pub const fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false
        }
    }

    /// Check if error enum is Error variant
    pub const fn is_error(&self) -> bool {
        match self {
            Self::Error => true,
            _ => false
        }
    }

}