pub type Result<T> = core::result::Result<T,Error>;

// shortcuts for errors
pub const OUT_OF_BOUNDS: Error = Error::Logic(LogicError::OutOfBounds);

/// Terminal Flag Bit (STATUS WORD)
///    The Terminal Flag bit (bit time 19) informs the bus controller of a fault or
///    failure within the remote terminal circuitry (only the remote terminal). A
///    logic “1” shall indicate a fault condition.
///    
///    This bit is used solely to inform the bus controller of a fault or failure.
///    Further information regarding the nature of the failure must be obtained in
///    some other fashion. Typically, a Subaddress (often 30) is reserved for BIT
///    information, or the bus controller may issue a Transmit BIT Word Mode
///    Code.
///    
///    It should be noted that the standard states that this bit is for the remote
///    terminal, meaning the entire remote terminal and not just the channel one
///    which the command was received. In recent years, some component
///    manufacturers have developed “chips” that are designed for single bus
///    applications. Terminal designers have applied these chips to remote
///    terminals indented for dual redundant applications. When these chips are
///    used in these dual redundant applications, the status word, and
///    subsequently the Terminal Flag bit, reflects only the status of the channel
///    and not the entire remote terminal. (A dual redundant standby terminal has
///    two channels: A and B). While not necessarily within the intent of the
///    standard, there exist terminals that contain these chips and the bus
///    controller needs to be aware of how the terminal defines the Terminal Flag
///    (i.e. terminal or channel).
///
/// Subsystem Flag Bit (STATUS WORD)
///    The Subsystem Flag bit (bit time 17) is used to provide “health” data
///    regarding the subsystems to which the remote terminal is connected.
///    Multiple subsystems may logically “OR” their bits together to form a
///    composite health indicator. This single bit serves only as an indicator to
///    the bus controller and user of the data that a fault or failure exists. Further
///    information regarding the nature of the failure must be obtained in some
///    other fashion. Typically, a Subaddress is reserved for BIT information,
///    with one or two words devoted to subsystem status data.
///
/// Message Error Bit (STATUS WORD)
///    The next bit (bit 9) is the Message Error (ME) bit. This bit is set by
///    the remote terminal upon detection of an error in the message or upon
///    detection of an invalid message (i.e. Illegal Command) to the terminal.
///    The error may occur in any of the data words within the message. When
///    the terminal detects an error and sets this bit, none of the data received
///    within the message is used. In fact, once an error is detected, the terminal
///    need not continue decoding the rest of the message, though most do.
///    
///    If an error is detected within a message and the ME bit is set, the remote
///    terminal must suppress the transmission of the status word. If the terminal
///    detected an Illegal Command, the ME bit is set and the status word is
///    transmitted. A logic “1” indicates an error. All remote terminals must
///    implement the ME bit in the status word
#[derive(Debug)]
pub enum SystemError {
    None,
    Terminal,  // Terminal Flag bit
    Subsystem, // Subsystem Flag bit
    Message,   // MessageError bit
}

#[derive(Debug)]
pub enum LogicError {
    None,
    OutOfBounds,
}

#[derive(Debug)]
pub enum Error {
    System(SystemError),
    Logic(LogicError),
}
