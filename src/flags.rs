//! Flags parsed from fields

macro_rules! fit {
    ( $v: expr, $p: expr ) => {
        $v <= $p
    };
}

macro_rules! eqs {
    ( $v: expr, $p: expr ) => {
        $v == $p
    };
}

/// A flag used by the bus controller to manage remote terminals on the bus.
///
/// Mode Codes are flags defined by the 1553 standard to provide the Bus
/// Controller (BC) with bus management and error handling/recovery capability.
/// They are divided into two groups- those with and those without an associated
/// data word.
///
/// The data word (of which there can only be one) contains information pertinent
/// to the control of the bus and which is not generally required by the subsystem.
///
/// Mode Codes are defined by bit times 15-19 (index 11-15) of the command word,
/// the most significant bit of which (bit 15/index 11) of which can be used to
/// differentiate between the two mode-code groups. When a data word is associated
/// with the mode code, the T/R bit determines if the data word is transmitted or
/// received by the remote terminal.
///
/// Mode Codes are listed on page 40, in table 5, of the MIL-STD-1553 Tutorial[^1].
///
/// [^1]: [MIL-STD-1553 Tutorial](http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum ModeCode {
    /// Dynamic Bus Control Mode Code is used to pass control of the data
    /// bus between terminals, thus supplying a “round robin” type of control.
    DynamicBusControl = 0b00000,

    /// Synchronize Mode Code is transmitted to an RT as a request that
    /// some predefined synchronization event occur.
    Synchronize = 0b00001,

    /// Transmit Status Word Mode Code requests that the RT transmit the
    /// status word associated with the previous message.
    TransmitStatusWord = 0b00010,

    /// Initiate Self Test Mode Code is used to take the receiving RT offline
    /// in order to perform internal testing.
    InitiateSelfTest = 0b00011,

    /// Transmitter Shutdown Mode Code is used by the BC to kill an RT
    /// which is continuously transmitting on the bus.
    TransmitterShutdown = 0b00100,

    /// Override Transmitter Shutdown Mode Code is used by the BC to restart
    /// an RT previously shutdown with the Transmitter Shutdown Mode Code.
    OverrideTransmitterShutdown = 0b00101,

    /// Inhibit Terminal Flag Bit Mode Code is used by the BC to request an RT
    /// set the Terminal Flag Bit in messages to 0, regardless of the true state.
    InhibitTerminalFlagBit = 0b00110,

    /// Override Inhibit Terminal Flag Bit is used by the BC to request an RT set
    /// the Terminal Flag Bit in messages based on the true state of the RT.
    OverrideInhibitTerminalFlagBit = 0b00111,

    /// Reset Remote Terminal Mode Code is used by the BC to request an RT set
    /// itself back to it's original state, as if it has just powered on.
    ResetRemoteTerminal = 0b01000,

    /// Transmit Vector Word Mode Code is used by the BC to request an RT transmit
    /// its current needs as a data word. Data word parsing is platform specific.
    TransmitVectorWord = 0b10000,

    /// Synchronize With Data Word Mode Code is the same as Synchronize, but with
    /// additional information included in a data word.
    SynchronizeWithDataWord = 0b10001,

    /// Transmit Last Command Word Mode Code is used by the BC to request that an RT
    /// transmit it's last received command word.
    TransmitLastCommandWord = 0b10010,

    /// Transmit Built-In-Test (BIT) Word Mode Code is used by the BC to request
    /// details of the BIT status of the RT.
    TransmitBITWord = 0b10011,

    /// Selected Transmitter Shutdown Mode Code is the same as TransmitterShutdown,
    /// but includes a specific bus (transmitter) in the data word.
    SelectedTransmitterShutdown = 0b10100,

    /// Override Selected Transmitter Shutdown Mode Code is the same as
    /// OverrideTransmitterShutdown but includes a specific bus (transmitter) in the data word.
    OverrideSelectedTransmitterShutdown = 0b10101,

    /// Unknown Mode Code is a catch-all for parsed mode codes that we don't
    /// recognize. This doesn't mean they are invalid, but they may be system-specific.
    UnknownModeCode(u8),
}

impl ModeCode {
    /// Get the actual u8 value of the mode code
    pub fn value(&self) -> u8 {
        (*self).into()
    }

    /// Check if mode code is associated with transmit messages
    ///
    /// If the TR bit is cleared, but this function returns true,
    /// then the message is illegal.
    pub const fn is_transmit(&self) -> bool {
        matches!(
            self,
            Self::DynamicBusControl
                | Self::SynchronizeWithDataWord
                | Self::TransmitStatusWord
                | Self::InitiateSelfTest
                | Self::TransmitterShutdown
                | Self::OverrideTransmitterShutdown
                | Self::InhibitTerminalFlagBit
                | Self::OverrideInhibitTerminalFlagBit
                | Self::ResetRemoteTerminal
                | Self::TransmitVectorWord
                | Self::TransmitLastCommandWord
                | Self::TransmitBITWord
        )
    }

    /// Check if mode code is associated with receive messages
    ///
    /// If the TR bit is set, but this function returns true,
    /// then the message is illegal.
    #[must_use = "Returned value is not used"]
    pub const fn is_receive(&self) -> bool {
        matches!(
            self,
            Self::Synchronize
                | Self::SelectedTransmitterShutdown
                | Self::OverrideSelectedTransmitterShutdown
        )
    }

    /// Check if data is associated with the mode code
    ///
    /// This property is true if the MSB of the mode
    /// code value is 1. TransmitBITWord, for example,
    /// has an actual value of 0b00010011, but the field
    /// in the word is five bits wide, making the value
    /// 0b10011. The MSB in this case is 1.
    ///
    /// For clarity, the enum variants are explicitly
    /// listed here rather than converted to a u8 and
    /// masked to get the bool value.
    #[must_use = "Returned value is not used"]
    pub const fn has_data(&self) -> bool {
        matches!(
            self,
            Self::TransmitVectorWord
                | Self::Synchronize
                | Self::TransmitLastCommandWord
                | Self::TransmitBITWord
                | Self::SelectedTransmitterShutdown
                | Self::OverrideSelectedTransmitterShutdown
        )
    }

    /// Check if mode code can be broadcast to all terminals
    ///
    /// Some mode codes can be sent to all receiving terminals
    /// (RTs) while for other codes, this would be nonsensical.
    /// Even if a mode code *can* be sent to all RTs, it may
    /// have disasterous consequences if done while in flight.
    #[must_use = "Returned value is not used"]
    pub const fn is_broadcast(&self) -> bool {
        matches!(
            self,
            Self::SynchronizeWithDataWord
                | Self::InitiateSelfTest
                | Self::TransmitterShutdown
                | Self::OverrideTransmitterShutdown
                | Self::InhibitTerminalFlagBit
                | Self::OverrideInhibitTerminalFlagBit
                | Self::ResetRemoteTerminal
                | Self::Synchronize
                | Self::SelectedTransmitterShutdown
                | Self::OverrideSelectedTransmitterShutdown
        )
    }

    /// Check if the mode code is unrecognized
    #[must_use = "Returned value is not used"]
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::UnknownModeCode(_))
    }
}

impl From<u8> for ModeCode {
    fn from(value: u8) -> Self {
        use ModeCode::*;
        match value {
            0b00000 => DynamicBusControl,
            0b00001 => Synchronize,
            0b00010 => TransmitStatusWord,
            0b00011 => InitiateSelfTest,
            0b00100 => TransmitterShutdown,
            0b00101 => OverrideTransmitterShutdown,
            0b00110 => InhibitTerminalFlagBit,
            0b00111 => OverrideInhibitTerminalFlagBit,
            0b01000 => ResetRemoteTerminal,
            0b10000 => TransmitVectorWord,
            0b10001 => SynchronizeWithDataWord,
            0b10010 => TransmitLastCommandWord,
            0b10011 => TransmitBITWord,
            0b10100 => SelectedTransmitterShutdown,
            0b10101 => OverrideSelectedTransmitterShutdown,
            v => UnknownModeCode(v),
        }
    }
}

impl From<ModeCode> for u8 {
    fn from(value: ModeCode) -> Self {
        use ModeCode::*;
        match value {
            DynamicBusControl => 0b00000,
            Synchronize => 0b00001,
            TransmitStatusWord => 0b00010,
            InitiateSelfTest => 0b00011,
            TransmitterShutdown => 0b00100,
            OverrideTransmitterShutdown => 0b00101,
            InhibitTerminalFlagBit => 0b00110,
            OverrideInhibitTerminalFlagBit => 0b00111,
            ResetRemoteTerminal => 0b01000,
            TransmitVectorWord => 0b10000,
            SynchronizeWithDataWord => 0b10001,
            TransmitLastCommandWord => 0b10010,
            TransmitBITWord => 0b10011,
            SelectedTransmitterShutdown => 0b10100,
            OverrideSelectedTransmitterShutdown => 0b10101,
            UnknownModeCode(v) => v,
        }
    }
}

/// The direction of message transmission from the point of view of the remote terminal.
///
/// This flag is available in bit 9 (index 5). A transmit bit (logic 1)
/// indicates that the remote terminal is to transmit data, while a receive
/// command (logic 0) indicates that the remote terminal is going to receive
/// data. The only exceptions to this rule are associated with mode commands.
///
/// This flag is called T/R or Transmit/Receive on page 28 of the MIL-STD-1553 Tutorial[^1].
///
/// [^1]: [MIL-STD-1553 Tutorial](http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum TransmitReceive {
    /// The remote terminal is receiving data
    Receive = 0,

    /// The remote terminal is to transmit data
    Transmit = 1,
}

impl TransmitReceive {
    /// Check if this enum is the transmit variant
    #[must_use = "Returned value is not used"]
    pub const fn is_transmit(&self) -> bool {
        matches!(self, Self::Transmit)
    }

    /// Check if this enum is the receive variant
    #[must_use = "Returned value is not used"]
    pub const fn is_receive(&self) -> bool {
        matches!(self, Self::Receive)
    }
}

impl From<u8> for TransmitReceive {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Transmit,
            _ => Self::Receive,
        }
    }
}

impl From<TransmitReceive> for u8 {
    fn from(value: TransmitReceive) -> Self {
        match value {
            TransmitReceive::Transmit => 1,
            TransmitReceive::Receive => 0,
        }
    }
}

/// The address of a remote terminal
///
/// This 5-bit address is found in the Terminal Address (TA) field located at bit times 4-8
/// (index 0-4). A value of 0b11111 is reserved in the TA field as a broadcast address.
///
/// This field is described on page 28 of the MIL-STD-1553 Tutorial[^1].
///
/// [^1]: [MIL-STD-1553 Tutorial](http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Address {
    /// The address references a terminal
    Value(u8),

    /// The address doesn't have a valid value
    Unknown(u8),

    /// The address is a reserved broadcast value
    Broadcast(u8),
}

impl Address {
    /// Create a new address from a u8 value
    pub const fn new(value: u8) -> Self {
        match value {
            k if eqs!(k, 0b0001_1111) => Address::Broadcast(k),
            k if fit!(k, 0b0001_1111) => Address::Value(k),
            k => Address::Unknown(k),
        }
    }

    /// Get the actual u8 value of the address
    #[must_use = "Returned value is not used"]
    pub const fn value(&self) -> u8 {
        match self {
            Self::Value(k) => *k,
            Self::Unknown(k) => *k,
            Self::Broadcast(k) => *k,
        }
    }

    /// Check if this enum contains an address
    #[must_use = "Returned value is not used"]
    pub const fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    /// Check if this enum contains an unknown address
    #[must_use = "Returned value is not used"]
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown(_))
    }

    /// Check if this address is a reserved broadcast value
    #[must_use = "Returned value is not used"]
    pub const fn is_broadcast(&self) -> bool {
        matches!(self, Self::Broadcast(_))
    }
}

impl From<u8> for Address {
    fn from(v: u8) -> Address {
        Address::new(v)
    }
}

impl From<Address> for u8 {
    fn from(v: Address) -> u8 {
        v.value()
    }
}

/// The address of a subsystem within a remote terminal.
///
/// This 5-bit address is found in the Subaddress (SA) field located at bit times
/// 10-14 (index 6-10). If the SA value is 0b00000 or 0b11111, then the field is
/// decoded as a Mode Code command.
///
/// This field is described on page 28 of the MIL-STD-1553 Tutorial[^1].
///
/// [^1]: [MIL-STD-1553 Tutorial](http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SubAddress {
    /// The address references a subsystem
    Value(u8),

    /// The address doesn't have a valid value
    Unknown(u8),

    /// The address is a reserved mode code flag
    ModeCode(u8),
}

impl SubAddress {
    /// Create a new address from a u8 value
    pub const fn new(value: u8) -> Self {
        match value {
            k if eqs!(k, 0b0000_0000) => SubAddress::ModeCode(k),
            k if eqs!(k, 0b0001_1111) => SubAddress::ModeCode(k),
            k if fit!(k, 0b0001_1111) => SubAddress::Value(k),
            k => SubAddress::Unknown(k),
        }
    }

    /// Get the actual u8 value of the address
    pub const fn value(&self) -> u8 {
        match self {
            Self::Value(k) => *k,
            Self::Unknown(k) => *k,
            Self::ModeCode(k) => *k,
        }
    }

    /// Check if this enum contains an address
    pub const fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    /// Check if this enum contains an invalid/unkown address
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown(_))
    }

    /// Check if this address is a reserved mode code value
    pub const fn is_mode_code(&self) -> bool {
        matches!(self, Self::ModeCode(_))
    }
}

impl From<u8> for SubAddress {
    fn from(v: u8) -> SubAddress {
        SubAddress::new(v)
    }
}

impl From<SubAddress> for u8 {
    fn from(v: SubAddress) -> u8 {
        v.value()
    }
}

/// Used to differentiate between a command and status word.
///
/// The instrumentation bit in the status word is always set to a logic 0,
/// and if used, the same bit in a command word is set to logic 1. This bit
/// is the MSB of the Subaddress field, and if used will limit the subaddresses
/// used to 10000-11110, reducing the number of addressable terminals from 30
/// to 15. It is also the reason there are two mode code identifiers
/// (see [SubAddress]).
///
/// **Most systems no longer use this flag, as the cost in reduced subaddress
/// range is too high**.
///
/// This field is described in the status word diagram on page 28 of the
/// MIL-STD-1553 Tutorial[^1].
///
/// [^1]: [MIL-STD-1553 Tutorial](http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum Instrumentation {
    /// The containing word is a status word
    Status = 0,

    /// The containing word is a command word
    Command = 1,
}

impl Instrumentation {
    /// Check if this enum is the Status variant
    #[must_use = "Returned value is not used"]
    pub const fn is_status(&self) -> bool {
        matches!(self, Self::Status)
    }

    /// Check if this enum is the Command variant
    #[must_use = "Returned value is not used"]
    pub const fn is_command(&self) -> bool {
        matches!(self, Self::Command)
    }
}

impl From<u8> for Instrumentation {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Command,
            _ => Self::Status,
        }
    }
}

impl From<Instrumentation> for u8 {
    fn from(value: Instrumentation) -> Self {
        match value {
            Instrumentation::Command => 1,
            Instrumentation::Status => 0,
        }
    }
}

/// Used by a remote terminal to tell the bus controller that it needs to be serviced.
///
/// This flag is located at bit time 11 (index 7) and is typically used when
/// the bus controller is polling terminals to determine if they require
/// processing. The bus controller, on receiving this flag, takes a predetermined
/// action such as issuing a series of messages or requests for further data
/// to the remote terminal.
///
/// This field is described in the status word diagram on page 28 of the
/// MIL-STD-1553 Tutorial[^1].
///
/// [^1]: [MIL-STD-1553 Tutorial](http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum ServiceRequest {
    /// This terminal does not require servicing
    NoService = 0,

    /// This terminal requires servicing
    Service = 1,
}

impl ServiceRequest {
    /// Check if enum is the NoService variant
    #[must_use = "Returned value is not used"]
    pub const fn is_noservice(&self) -> bool {
        matches!(self, Self::NoService)
    }

    /// Check if the enum is the Service variant
    #[must_use = "Returned value is not used"]
    pub const fn is_service(&self) -> bool {
        matches!(self, Self::Service)
    }
}

impl From<u8> for ServiceRequest {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Service,
            _ => Self::NoService,
        }
    }
}

impl From<ServiceRequest> for u8 {
    fn from(value: ServiceRequest) -> Self {
        match value {
            ServiceRequest::Service => 1,
            ServiceRequest::NoService => 0,
        }
    }
}

/// Reserved bits that should always be zero
///
/// Bit times 12-14 (index 8-10) are reserved for future growth of the standard
/// and must be set to a logic “0”. The bus controller should declare a message
/// in error if the remote terminal responds with any of these bits set in its
/// status word.
///
/// This field is described in the status word diagram on page 28 of the
/// MIL-STD-1553 Tutorial[^1].
///
/// [^1]: [MIL-STD-1553 Tutorial](http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum Reserved {
    /// The reserved bits are empty
    None = 0b000,

    /// The reserved bits are not empty
    Value(u8),
}

impl Reserved {
    /// Check if this enum is the None variant
    #[must_use = "Returned value is not used"]
    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Check if this enum is the Value variant
    #[must_use = "Returned value is not used"]
    pub const fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }
}

impl From<u8> for Reserved {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            v => Self::Value(v),
        }
    }
}

impl From<Reserved> for u8 {
    fn from(value: Reserved) -> Self {
        match value {
            Reserved::None => 0,
            Reserved::Value(v) => v,
        }
    }
}

/// Indicates that the remote terminal has received a valid broadcast command.
///
/// On receiving such a command, the remote terminal sets this flag and
/// suppresses transmission of its status words. The bus controller may then
/// issue a Transmit Status word or Transmit Last Command mode code to
/// determine if the terminal received the message properly.
///
/// This field is described in the status word diagram on page 28 of the
/// MIL-STD-1553 Tutorial[^1].
///
/// [^1]: [MIL-STD-1553 Tutorial](http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum BroadcastCommand {
    /// This terminal has not received a broadcast command
    NotReceived = 0,

    /// This termina received a broadcast command
    Received = 1,
}

impl BroadcastCommand {
    /// Check if enum is the NotReceived variant
    #[must_use = "Returned value is not used"]
    pub const fn is_notreceived(&self) -> bool {
        matches!(self, Self::NotReceived)
    }

    /// Check if the enum is the Received variant
    #[must_use = "Returned value is not used"]
    pub const fn is_received(&self) -> bool {
        matches!(self, Self::Received)
    }
}

impl From<u8> for BroadcastCommand {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Received,
            _ => Self::NotReceived,
        }
    }
}

impl From<BroadcastCommand> for u8 {
    fn from(value: BroadcastCommand) -> Self {
        match value {
            BroadcastCommand::Received => 1,
            BroadcastCommand::NotReceived => 0,
        }
    }
}

/// Indicates that the remote terminal is busy
///
/// The Busy bit, located at bit time 16 (index 12) is provided as
/// feedback to the bus controller when the remote terminal is unable to move
/// data between the remote terminal electronics and the subsystem in compliance
/// to a command from the bus controller.
///
/// This field is called "Busy" in the status word diagram on page 28 of the
/// MIL-STD-1553 Tutorial[^1].
///
/// [^1]: [MIL-STD-1553 Tutorial](http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum TerminalBusy {
    /// This terminal is not busy
    NotBusy = 0,

    /// This terminal is busy
    Busy = 1,
}

impl TerminalBusy {
    /// Check if enum is the NotBusy variant
    #[must_use = "Returned value is not used"]
    pub const fn is_notbusy(&self) -> bool {
        matches!(self, Self::NotBusy)
    }

    /// Check if the enum is the Busy variant
    #[must_use = "Returned value is not used"]
    pub const fn is_busy(&self) -> bool {
        matches!(self, Self::Busy)
    }
}

impl From<u8> for TerminalBusy {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Busy,
            _ => Self::NotBusy,
        }
    }
}

impl From<TerminalBusy> for u8 {
    fn from(value: TerminalBusy) -> Self {
        match value {
            TerminalBusy::Busy => 1,
            TerminalBusy::NotBusy => 0,
        }
    }
}

/// Informs the bus controller that the terminal has accepted bus control.
///
/// This flag is set by remote terminals that have received the Dynamic Bus
/// Control Mode Code and have accepted control of the bus. The remote terminal,
/// on transmitting its status word, becomes the bus controller, and the bus
/// controller, on receiving the status word with this flag, ceases to
/// control the bus.
///
/// This field is called "Dynamic Bus Acceptance" in the status word diagram on
/// page 28 of the MIL-STD-1553 Tutorial[^1].
///
/// [^1]: [MIL-STD-1553 Tutorial](http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum BusControlAccept {
    /// This terminal has refused control of the bus
    NotAccepted = 0,

    /// This terminal has accepted control of the bus
    Accepted = 1,
}

impl BusControlAccept {
    /// Check if the enum is the NotAccepted variant
    #[must_use = "Returned value is not used"]
    pub const fn is_notaccepted(&self) -> bool {
        matches!(self, Self::NotAccepted)
    }

    /// Check if the enum is the Accepted variant
    #[must_use = "Returned value is not used"]
    pub const fn is_accepted(&self) -> bool {
        matches!(self, Self::Accepted)
    }
}

impl From<u8> for BusControlAccept {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Accepted,
            _ => Self::NotAccepted,
        }
    }
}

impl From<BusControlAccept> for u8 {
    fn from(value: BusControlAccept) -> Self {
        match value {
            BusControlAccept::Accepted => 1,
            BusControlAccept::NotAccepted => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_unknown() {
        let num = 0b11111111;

        let flag: Address = num.into();
        assert_eq!(flag, Address::Unknown(num));

        let value: u8 = flag.into();
        assert_eq!(value, num);

        let flag: Address = value.into();
        assert_eq!(flag, Address::Unknown(num))
    }
}
