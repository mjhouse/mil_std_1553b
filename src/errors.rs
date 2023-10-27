use num_enum::{IntoPrimitive,FromPrimitive};

pub type Result<T> = core::result::Result<T,Error>;

/// An error deriving from the software itself, rather than a terminal.
/// 
/// These errors occur during parsing or other calculations when the 
/// calculations fail.
#[derive(Debug,Clone,PartialEq,Eq)]
#[repr(u8)]
pub enum Error {
    OutOfBounds,
    InvalidCode,
    NotModeCode,
    MessageFull,
    MessageBad,
    ReservedUsed,
    NotFound,
    UnknownMessage,
    SystemError(SystemError)
}

/// An error deriving from a remote terminal or bus controller.
/// 
/// These errors are generated during runtime by terminals and
/// provided in messages on the bus.
#[derive(Debug,Clone,PartialEq,Eq)]
#[repr(u8)]
pub enum SystemError {
    None,
    Terminal(TerminalError),
    Subsystem(SubsystemError),
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

/// This flag provides health data regarding subsystems of a remote terminal.
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
