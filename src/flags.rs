use crate::errors::*;

// Mode Codes are defined by the standard to provide the bus controller with
// data bus management and error handling/recovery capability. The mode
// codes are divided into two groups: those with, and those without, a data
// word.
// 
// The data word that is associated with the mode codes—and only one word
// per mode code is allowed—contains information pertinent to the control of
// the bus and does not generally contain information required by the
// subsystem (the exception may be the Synchronize with Data Word Mode
// Code). The mode codes are defined by bit times 15-19 of the command
// word. The most significant bit (bit 15) can be used to differentiate between
// the two-mode code groups. When a data word is associated with the mode
// code, the T/R bit determines if the data word is transmitted or received by
// the remote terminal. The mode codes are listed in Table 5.
#[repr(u8)]
pub enum ModeCode {
    DynamicBusControl,
    Synchronize,
    TransmitStatusWord,
    TransmitVectorWord,

    TransmitLastCommandWord,
    TransmitBITWord,

    TransmitterShutdown,
    OverrideTransmitterShutdown,

    InhibitTerminalFlagBit,
    OverrideInhibitTerminalFlagBit,

    SelectedTransmitterShutdown,
    OverrideSelectedTransmitterShutdown,

    InitiateSelfTest,
    ResetRemoteTerminal,
}

impl ModeCode {
    pub fn from_data(transmit_receive_bit: u8, mode_code: u8) -> Result<ModeCode> {
        use ModeCode::*;
        match (transmit_receive_bit,mode_code) {
            (1,0b00000) => Ok(DynamicBusControl),
            (1,0b00001) => Ok(Synchronize),
            (0,0b10001) => Ok(Synchronize),
            (1,0b00010) => Ok(TransmitStatusWord),
            (1,0b10000) => Ok(TransmitVectorWord),
            (1,0b10010) => Ok(TransmitLastCommandWord),
            (1,0b10011) => Ok(TransmitBITWord),
            (1,0b00100) => Ok(TransmitterShutdown),
            (1,0b00101) => Ok(OverrideTransmitterShutdown),
            (1,0b00110) => Ok(InhibitTerminalFlagBit),
            (1,0b00111) => Ok(OverrideInhibitTerminalFlagBit),
            (0,0b10100) => Ok(SelectedTransmitterShutdown),
            (0,0b10101) => Ok(OverrideSelectedTransmitterShutdown),
            (1,0b00011) => Ok(InitiateSelfTest),
            (1,0b01000) => Ok(ResetRemoteTerminal),
            (_,_)       => Err(INVALID_CODE),
        }
    }

    pub fn into_data(&self, transmit_receive_bit: u8) -> Result<u8> {
        use ModeCode::*;
        match (transmit_receive_bit,self) {
            (1,DynamicBusControl)                   => Ok(0b00000),
            (1,Synchronize)                         => Ok(0b00001),
            (0,Synchronize)                         => Ok(0b10001),
            (1,TransmitStatusWord)                  => Ok(0b00010),
            (1,TransmitVectorWord)                  => Ok(0b10000),
            (1,TransmitLastCommandWord)             => Ok(0b10010),
            (1,TransmitBITWord)                     => Ok(0b10011),
            (1,TransmitterShutdown)                 => Ok(0b00100),
            (1,OverrideTransmitterShutdown)         => Ok(0b00101),
            (1,InhibitTerminalFlagBit)              => Ok(0b00110),
            (1,OverrideInhibitTerminalFlagBit)      => Ok(0b00111),
            (0,SelectedTransmitterShutdown)         => Ok(0b10100),
            (0,OverrideSelectedTransmitterShutdown) => Ok(0b10101),
            (1,InitiateSelfTest)                    => Ok(0b00011),
            (1,ResetRemoteTerminal)                 => Ok(0b01000),
            (_,_)                                   => Err(INVALID_CODE)
        }
    }
}

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

/// Terminal address (COMMAND WORD)
///    The five bit Terminal Address (TA) field (bit times 4-8) states to which unique 
///    remote terminal the command is intended (no two terminals may have the same address).
///
///    Note:
///        An address of 00000B is a valid address, and an address of 11111B is
///        always reserved as a broadcast address. Additionally, there is no
///        requirement that the bus controller be assigned an address, therefore
///        the maximum number of terminals the data bus can support is 31.
///
/// Subaddress(COMMAND WORD)
///     The next five bits (bits 10-14) make up the Subaddress (SA)/Mode
///     Command bits. Logic 00000B or 11111B within this field is decoded to
///     indicate that the command is a Mode Code Command. All other logic
///     combinations of this field are used to direct the data to different functions
///     within the subsystem. An example might be that 00001B is position and
///     rate data, 00010B is frequency data, 10010B is display information, and
///     10011B is self-test data
#[derive(Debug,PartialEq)]
pub enum Address {
    None,
    Terminal(u8),
    Subsystem(u8),
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