/// (COMMAND WORD)
/// Bit 9 is the Transmit/Receive (T/R) bit. This defines the direction of 
/// information flow and is always from the point of view of the remote 
/// terminal. A transmit command (logic 1) indicates that the remote terminal 
/// is to transmit data, while a receive command (logic 0) indicates that 
/// the remote terminal is going to receive data. The only exceptions to 
/// this rule are associated with mode commands.
pub enum Direction {
    Transmit,
    Receive
}

/// (COMMAND WORD)
/// The next five bits (bits 10-14) make up the Subaddress (SA)/Mode
/// Command bits. Logic 00000B or 11111B within this field is decoded to
/// indicate that the command is a Mode Code Command. All other logic
/// combinations of this field are used to direct the data to different functions
/// within the subsystem. An example might be that 00001B is position and
/// rate data, 00010B is frequency data, 10010B is display information, and
/// 10011B is self-test data
pub enum Subaddress {
    Command, // 0 or 31
    Value(u8)
}

/// (COMMAND WORD)
/// The next five bit positions (bits 15-19) define the Word Count (WC)
/// or Mode Code to be performed. If the Subaddress/Mode Code field is
/// 00000B or 11111B, then this field defines the mode code to be performed.
/// If not a mode code, then this field defines the number of data words to be
/// received or transmitted depending on the T/R bit. A word count field of
/// 00000B is decoded as 32 data words.
pub enum Mode {
    WordCount(u8),
    ModeCode(u8)
}

/// (STATUS WORD)
/// The Instrumentation bit (bit 10) is provided to differentiate between a
/// command word and a status word (remember they both have the same sync
/// pattern). The instrumentation bit in the status word is always set to logic
/// “0”. If used, the corresponding bit in the command word is set to a logic
/// “1”. This bit in the command word is the most significant bit of the
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
pub enum Instrumentation {
    Status,  // 0
    Command, // 1
}

/// (STATUS WORD)
/// The Service Request bit (bit time 11) is provided so that the remote
/// terminal can inform the bus controller that it needs to be serviced. This bit
/// is set to a logic “1” by the subsystem to indicate that servicing is needed.
/// This bit is typically used when the bus controller is “polling” terminals to
/// determine if they require processing.
/// 
/// The bus controller, on receiving this bit set to a logic “1”, takes a predeter-
/// mined action such as issuing a series of messages or requests further data
/// from the remote terminal. The later approach can be accomplished by
/// requesting the terminal to transmit data from a defined Subaddress or by
/// using the Transit Vector Word Mode Code.
pub enum ServiceRequest {
    NoService, // 0
    Service    // 1
}

/// (STATUS WORD)
/// The Broadcast Command Received bit (bit 15) indicates that the
/// remote terminal received a valid broadcast command. On receiving a valid
/// broadcast command, the remote terminal sets this bit to logic “1” and
/// suppresses the transmission of its status words. The bus controller may
/// issue a Transmit Status Word or Transmit Last Command Word Mode
/// Code to determine if the terminal received the message properly.
pub enum BroadcastCommand {
    NotReceived, // 0
    Received     // 1
}

/// (STATUS WORD)
/// The Busy bit (bit 16) is provided as a feedback to the bus controller as
/// to when the remote terminal is unable to move data between the remote
/// terminal electronics and the subsystem in compliance to a command from
/// the bus controller.
/// 
/// In the earlier days of 1553, the busy bit was required because many of the
/// subsystem interfaces (analogs, synchros, etc.) were much slower compared
/// to the speed of the multiplex data bus. Some terminals were not able to
/// move the data fast enough. So instead of loosing data, a terminal was able
/// to set the busy bit, indicating to the bus controller cannot handle new data
/// at this time, and for the bus controller to try again later. As new systems
/// have been developed, the need for the busy bit has been reduced. There
/// are, however, still systems that need and have a valid use for the busy bit.
/// Examples of these are radios where the bus controller issues a command to
/// the radio to tune to a certain frequency. It may take the radio several
/// seconds to accomplish this, and while it is tuning, it may set the busy bit to
/// inform the bus controller that it is doing as told. Another example is a tape
/// or disk drive, which may take from milliseconds to seconds to store or
/// retrieve a particular piece of data.
/// 
/// When a terminal is busy, it does not need to respond to commands in the
/// “normal” way. For received commands, the terminal collects the data, but
/// doesn’t have to pass the data to the subsystem. For transmit commands,
/// the terminal transmits its status word only. Therefore, while a terminal is
/// busy, the data it supplies to the rest of the system is not available. This can
/// have an overall effect upon the flow of data within the system and may
/// increase the data latency within time critical systems (e.g., flight controls).
/// Some terminals used the busy bit to overcome design problems, setting the
/// busy bit when needed. Notice 2 to the standard “strongly discourages” the
/// use of the busy bit. However, there are still valid needs for its use.
/// Therefore, if used, Notice 2 now requires that the busy bit may be set only
/// as the result of a particular command received from the bus controller and
/// not due to an internal periodic or processing function. By following this
/// requirement, the bus controller, with prior knowledge of the remote
/// terminal's characteristics, can determine what will cause a terminal to go
/// busy and minimize the effects on data latency throughout the system.
pub enum BusyFlag {
    NotBusy, // 0
    Busy,    // 1
}

/// (STATUS WORD)
/// The Dynamic Bus Control Acceptance bit (bit time 18) informs the bus
/// controller that the remote terminal has received the Dynamic Bus Control
/// Mode Code and has accepted control of the bus. For the remote terminal,
/// the setting of this bit is controlled by the subsystem and is based on passing
/// some level of built-in-test (i.e., a processor passing its power-up and
/// continuous background tests).
/// 
/// The remote terminal, on transmitting its status word, becomes the bus
/// controller. The bus controller, on receiving the status word from the
/// remote terminal with this bit set, ceases to function as the bus controller
/// and may become a remote terminal or bus monitor.
pub enum BusControl {
    NotAccepted, // 0
    Accepted,    // 1
}

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
pub enum Fault {
    None,
    Terminal,  // Terminal Flag bit
    Subsystem, // Subsystem Flag bit
    Message,   // MessageError bit
}