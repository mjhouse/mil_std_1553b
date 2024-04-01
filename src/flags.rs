//! Flags parsed from fields

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
    #[must_use = "Returned value is not used"]
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

impl From<u16> for ModeCode {
    fn from(value: u16) -> Self {
        Self::from(value as u8)
    }
}

impl From<ModeCode> for u16 {
    fn from(value: ModeCode) -> Self {
        u8::from(value) as u16
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
    /// Check if this enum is the receive variant
    #[must_use = "Returned value is not used"]
    pub const fn is_receive(&self) -> bool {
        matches!(self, Self::Receive)
    }

    /// Check if this enum is the transmit variant
    #[must_use = "Returned value is not used"]
    pub const fn is_transmit(&self) -> bool {
        matches!(self, Self::Transmit)
    }
}

impl From<u8> for TransmitReceive {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Receive,
            _ => Self::Transmit,
        }
    }
}

impl From<TransmitReceive> for u8 {
    fn from(value: TransmitReceive) -> Self {
        match value {
            TransmitReceive::Receive => 0,
            TransmitReceive::Transmit => 1,
        }
    }
}

impl From<u16> for TransmitReceive {
    fn from(value: u16) -> Self {
        Self::from(value as u8)
    }
}

impl From<TransmitReceive> for u16 {
    fn from(value: TransmitReceive) -> Self {
        u8::from(value) as u16
    }
}

/// The address of a remote terminal
///
/// This 5-bit address is found in the Terminal Address (TA) field located at bit times 4-8
/// (index 0-4). A value of 0b11111 is reserved in the TA field as a broadcast address. If
/// a value larger than 0b11111 is parsed, it will be truncated to 0b11111.
///
/// This field is described on page 28 of the MIL-STD-1553 Tutorial[^1].
///
/// [^1]: [MIL-STD-1553 Tutorial](http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Address {
    /// The address references a terminal
    Value(u8),

    /// The address is a reserved broadcast value
    Broadcast(u8),
}

impl Address {
    /// The maximum real value that can be addressed
    const MAX: u8 = 0b11111;

    /// Check if this enum contains an address
    #[must_use = "Returned value is not used"]
    pub const fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    /// Check if this address is a reserved broadcast value
    #[must_use = "Returned value is not used"]
    pub const fn is_broadcast(&self) -> bool {
        matches!(self, Self::Broadcast(_))
    }
}

impl From<u8> for Address {
    fn from(v: u8) -> Address {
        match v {
            k if k < Self::MAX => Address::Value(k),
            k => Address::Broadcast(k & Self::MAX),
        }
    }
}

impl From<Address> for u8 {
    fn from(v: Address) -> u8 {
        match v {
            Address::Value(k) => k,
            Address::Broadcast(k) => k,
        }
    }
}

impl From<u16> for Address {
    fn from(value: u16) -> Self {
        Self::from(value as u8)
    }
}

impl From<Address> for u16 {
    fn from(value: Address) -> Self {
        u8::from(value) as u16
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

    /// The address is a reserved mode code flag
    ModeCode(u8),
}

impl SubAddress {
    /// The maximum real value that can be addressed
    const MAX: u8 = 0b11111;

    /// Check if this enum contains an address
    #[must_use = "Returned value is not used"]
    pub const fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    /// Check if this address is a reserved mode code value
    #[must_use = "Returned value is not used"]
    pub const fn is_mode_code(&self) -> bool {
        matches!(self, Self::ModeCode(_))
    }
}

impl From<u8> for SubAddress {
    fn from(v: u8) -> SubAddress {
        match v {
            k if k < Self::MAX && k > 0 => SubAddress::Value(k),
            k => SubAddress::ModeCode(k & Self::MAX),
        }
    }
}

impl From<SubAddress> for u8 {
    fn from(v: SubAddress) -> u8 {
        match v {
            SubAddress::Value(k) => k,
            SubAddress::ModeCode(k) => k,
        }
    }
}

impl From<u16> for SubAddress {
    fn from(value: u16) -> Self {
        Self::from(value as u8)
    }
}

impl From<SubAddress> for u16 {
    fn from(value: SubAddress) -> Self {
        u8::from(value) as u16
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
            0 => Self::Status,
            _ => Self::Command,
        }
    }
}

impl From<Instrumentation> for u8 {
    fn from(value: Instrumentation) -> Self {
        match value {
            Instrumentation::Status => 0,
            Instrumentation::Command => 1,
        }
    }
}

impl From<u16> for Instrumentation {
    fn from(value: u16) -> Self {
        Self::from(value as u8)
    }
}

impl From<Instrumentation> for u16 {
    fn from(value: Instrumentation) -> Self {
        u8::from(value) as u16
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
            0 => Self::NoService,
            _ => Self::Service,
        }
    }
}

impl From<ServiceRequest> for u8 {
    fn from(value: ServiceRequest) -> Self {
        match value {
            ServiceRequest::NoService => 0,
            ServiceRequest::Service => 1,
        }
    }
}

impl From<u16> for ServiceRequest {
    fn from(value: u16) -> Self {
        Self::from(value as u8)
    }
}

impl From<ServiceRequest> for u16 {
    fn from(value: ServiceRequest) -> Self {
        u8::from(value) as u16
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

impl From<u16> for Reserved {
    fn from(value: u16) -> Self {
        Self::from(value as u8)
    }
}

impl From<Reserved> for u16 {
    fn from(value: Reserved) -> Self {
        u8::from(value) as u16
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
pub enum BroadcastReceived {
    /// This terminal has not received a broadcast command
    NotReceived = 0,

    /// This termina received a broadcast command
    Received = 1,
}

impl BroadcastReceived {
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

impl From<u8> for BroadcastReceived {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::NotReceived,
            _ => Self::Received,
        }
    }
}

impl From<BroadcastReceived> for u8 {
    fn from(value: BroadcastReceived) -> Self {
        match value {
            BroadcastReceived::NotReceived => 0,
            BroadcastReceived::Received => 1,
        }
    }
}

impl From<u16> for BroadcastReceived {
    fn from(value: u16) -> Self {
        Self::from(value as u8)
    }
}

impl From<BroadcastReceived> for u16 {
    fn from(value: BroadcastReceived) -> Self {
        u8::from(value) as u16
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
            0 => Self::NotBusy,
            _ => Self::Busy,
        }
    }
}

impl From<TerminalBusy> for u8 {
    fn from(value: TerminalBusy) -> Self {
        match value {
            TerminalBusy::NotBusy => 0,
            TerminalBusy::Busy => 1,
        }
    }
}

impl From<u16> for TerminalBusy {
    fn from(value: u16) -> Self {
        Self::from(value as u8)
    }
}

impl From<TerminalBusy> for u16 {
    fn from(value: TerminalBusy) -> Self {
        u8::from(value) as u16
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
pub enum DynamicBusAcceptance {
    /// This terminal has refused control of the bus
    NotAccepted = 0,

    /// This terminal has accepted control of the bus
    Accepted = 1,
}

impl DynamicBusAcceptance {
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

impl From<u8> for DynamicBusAcceptance {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::NotAccepted,
            _ => Self::Accepted,
        }
    }
}

impl From<DynamicBusAcceptance> for u8 {
    fn from(value: DynamicBusAcceptance) -> Self {
        match value {
            DynamicBusAcceptance::NotAccepted => 0,
            DynamicBusAcceptance::Accepted => 1,
        }
    }
}

impl From<u16> for DynamicBusAcceptance {
    fn from(value: u16) -> Self {
        Self::from(value as u8)
    }
}

impl From<DynamicBusAcceptance> for u16 {
    fn from(value: DynamicBusAcceptance) -> Self {
        u8::from(value) as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_code_clone() {
        let item1 = ModeCode::InhibitTerminalFlagBit;
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[test]
    fn test_mode_code_value_0() {
        assert_eq!(ModeCode::DynamicBusControl.value(), 0b00000u8);
    }

    #[test]
    fn test_mode_code_value_1() {
        assert_eq!(ModeCode::Synchronize.value(), 0b00001u8);
    }

    #[test]
    fn test_mode_code_value_2() {
        assert_eq!(ModeCode::TransmitStatusWord.value(), 0b00010u8);
    }

    #[test]
    fn test_mode_code_value_3() {
        assert_eq!(ModeCode::InitiateSelfTest.value(), 0b00011u8);
    }

    #[test]
    fn test_mode_code_value_4() {
        assert_eq!(ModeCode::TransmitterShutdown.value(), 0b00100u8);
    }

    #[test]
    fn test_mode_code_value_5() {
        assert_eq!(ModeCode::OverrideTransmitterShutdown.value(), 0b00101u8);
    }

    #[test]
    fn test_mode_code_value_6() {
        assert_eq!(ModeCode::InhibitTerminalFlagBit.value(), 0b00110u8);
    }

    #[test]
    fn test_mode_code_value_7() {
        assert_eq!(ModeCode::OverrideInhibitTerminalFlagBit.value(), 0b00111u8);
    }

    #[test]
    fn test_mode_code_value_8() {
        assert_eq!(ModeCode::ResetRemoteTerminal.value(), 0b01000u8);
    }

    #[test]
    fn test_mode_code_value_9() {
        assert_eq!(ModeCode::TransmitVectorWord.value(), 0b10000u8);
    }

    #[test]
    fn test_mode_code_value_10() {
        assert_eq!(ModeCode::SynchronizeWithDataWord.value(), 0b10001u8);
    }

    #[test]
    fn test_mode_code_value_11() {
        assert_eq!(ModeCode::TransmitLastCommandWord.value(), 0b10010u8);
    }

    #[test]
    fn test_mode_code_value_12() {
        assert_eq!(ModeCode::TransmitBITWord.value(), 0b10011u8);
    }

    #[test]
    fn test_mode_code_value_13() {
        assert_eq!(ModeCode::SelectedTransmitterShutdown.value(), 0b10100u8);
    }

    #[test]
    fn test_mode_code_value_14() {
        assert_eq!(
            ModeCode::OverrideSelectedTransmitterShutdown.value(),
            0b10101u8
        );
    }

    #[test]
    fn test_mode_code_value_15() {
        assert_eq!(ModeCode::UnknownModeCode(0b11111u8).value(), 0b11111u8);
    }

    #[test]
    fn test_mode_code_is_transmit_0() {
        assert_eq!(ModeCode::DynamicBusControl.is_transmit(), true);
    }

    #[test]
    fn test_mode_code_is_transmit_1() {
        assert_eq!(ModeCode::Synchronize.is_transmit(), false);
    }

    #[test]
    fn test_mode_code_is_transmit_2() {
        assert_eq!(ModeCode::TransmitStatusWord.is_transmit(), true);
    }

    #[test]
    fn test_mode_code_is_transmit_3() {
        assert_eq!(ModeCode::InitiateSelfTest.is_transmit(), true);
    }

    #[test]
    fn test_mode_code_is_transmit_4() {
        assert_eq!(ModeCode::TransmitterShutdown.is_transmit(), true);
    }

    #[test]
    fn test_mode_code_is_transmit_5() {
        assert_eq!(ModeCode::OverrideTransmitterShutdown.is_transmit(), true);
    }

    #[test]
    fn test_mode_code_is_transmit_6() {
        assert_eq!(ModeCode::InhibitTerminalFlagBit.is_transmit(), true);
    }

    #[test]
    fn test_mode_code_is_transmit_7() {
        assert_eq!(ModeCode::OverrideInhibitTerminalFlagBit.is_transmit(), true);
    }

    #[test]
    fn test_mode_code_is_transmit_8() {
        assert_eq!(ModeCode::ResetRemoteTerminal.is_transmit(), true);
    }

    #[test]
    fn test_mode_code_is_transmit_9() {
        assert_eq!(ModeCode::TransmitVectorWord.is_transmit(), true);
    }

    #[test]
    fn test_mode_code_is_transmit_10() {
        assert_eq!(ModeCode::SynchronizeWithDataWord.is_transmit(), true);
    }

    #[test]
    fn test_mode_code_is_transmit_11() {
        assert_eq!(ModeCode::TransmitLastCommandWord.is_transmit(), true);
    }

    #[test]
    fn test_mode_code_is_transmit_12() {
        assert_eq!(ModeCode::TransmitBITWord.is_transmit(), true);
    }

    #[test]
    fn test_mode_code_is_transmit_13() {
        assert_eq!(ModeCode::SelectedTransmitterShutdown.is_transmit(), false);
    }

    #[test]
    fn test_mode_code_is_transmit_14() {
        assert_eq!(
            ModeCode::OverrideSelectedTransmitterShutdown.is_transmit(),
            false
        );
    }

    #[test]
    fn test_mode_code_is_transmit_15() {
        assert_eq!(ModeCode::UnknownModeCode(0b11111u8).is_transmit(), false);
    }

    #[test]
    fn test_mode_code_is_receive_0() {
        assert_eq!(ModeCode::DynamicBusControl.is_receive(), false);
    }

    #[test]
    fn test_mode_code_is_receive_1() {
        assert_eq!(ModeCode::Synchronize.is_receive(), true);
    }

    #[test]
    fn test_mode_code_is_receive_2() {
        assert_eq!(ModeCode::TransmitStatusWord.is_receive(), false);
    }

    #[test]
    fn test_mode_code_is_receive_3() {
        assert_eq!(ModeCode::InitiateSelfTest.is_receive(), false);
    }

    #[test]
    fn test_mode_code_is_receive_4() {
        assert_eq!(ModeCode::TransmitterShutdown.is_receive(), false);
    }

    #[test]
    fn test_mode_code_is_receive_5() {
        assert_eq!(ModeCode::OverrideTransmitterShutdown.is_receive(), false);
    }

    #[test]
    fn test_mode_code_is_receive_6() {
        assert_eq!(ModeCode::InhibitTerminalFlagBit.is_receive(), false);
    }

    #[test]
    fn test_mode_code_is_receive_7() {
        assert_eq!(ModeCode::OverrideInhibitTerminalFlagBit.is_receive(), false);
    }

    #[test]
    fn test_mode_code_is_receive_8() {
        assert_eq!(ModeCode::ResetRemoteTerminal.is_receive(), false);
    }

    #[test]
    fn test_mode_code_is_receive_9() {
        assert_eq!(ModeCode::TransmitVectorWord.is_receive(), false);
    }

    #[test]
    fn test_mode_code_is_receive_10() {
        assert_eq!(ModeCode::SynchronizeWithDataWord.is_receive(), false);
    }

    #[test]
    fn test_mode_code_is_receive_11() {
        assert_eq!(ModeCode::TransmitLastCommandWord.is_receive(), false);
    }

    #[test]
    fn test_mode_code_is_receive_12() {
        assert_eq!(ModeCode::TransmitBITWord.is_receive(), false);
    }

    #[test]
    fn test_mode_code_is_receive_13() {
        assert_eq!(ModeCode::SelectedTransmitterShutdown.is_receive(), true);
    }

    #[test]
    fn test_mode_code_is_receive_14() {
        assert_eq!(
            ModeCode::OverrideSelectedTransmitterShutdown.is_receive(),
            true
        );
    }

    #[test]
    fn test_mode_code_is_receive_15() {
        assert_eq!(ModeCode::UnknownModeCode(0b11111u8).is_receive(), false);
    }

    #[test]
    fn test_mode_code_has_data_0() {
        assert_eq!(ModeCode::DynamicBusControl.has_data(), false);
    }

    #[test]
    fn test_mode_code_has_data_1() {
        assert_eq!(ModeCode::Synchronize.has_data(), true);
    }

    #[test]
    fn test_mode_code_has_data_2() {
        assert_eq!(ModeCode::TransmitStatusWord.has_data(), false);
    }

    #[test]
    fn test_mode_code_has_data_3() {
        assert_eq!(ModeCode::InitiateSelfTest.has_data(), false);
    }

    #[test]
    fn test_mode_code_has_data_4() {
        assert_eq!(ModeCode::TransmitterShutdown.has_data(), false);
    }

    #[test]
    fn test_mode_code_has_data_5() {
        assert_eq!(ModeCode::OverrideTransmitterShutdown.has_data(), false);
    }

    #[test]
    fn test_mode_code_has_data_6() {
        assert_eq!(ModeCode::InhibitTerminalFlagBit.has_data(), false);
    }

    #[test]
    fn test_mode_code_has_data_7() {
        assert_eq!(ModeCode::OverrideInhibitTerminalFlagBit.has_data(), false);
    }

    #[test]
    fn test_mode_code_has_data_8() {
        assert_eq!(ModeCode::ResetRemoteTerminal.has_data(), false);
    }

    #[test]
    fn test_mode_code_has_data_9() {
        assert_eq!(ModeCode::TransmitVectorWord.has_data(), true);
    }

    #[test]
    fn test_mode_code_has_data_10() {
        assert_eq!(ModeCode::SynchronizeWithDataWord.has_data(), false);
    }

    #[test]
    fn test_mode_code_has_data_11() {
        assert_eq!(ModeCode::TransmitLastCommandWord.has_data(), true);
    }

    #[test]
    fn test_mode_code_has_data_12() {
        assert_eq!(ModeCode::TransmitBITWord.has_data(), true);
    }

    #[test]
    fn test_mode_code_has_data_13() {
        assert_eq!(ModeCode::SelectedTransmitterShutdown.has_data(), true);
    }

    #[test]
    fn test_mode_code_has_data_14() {
        assert_eq!(
            ModeCode::OverrideSelectedTransmitterShutdown.has_data(),
            true
        );
    }

    #[test]
    fn test_mode_code_has_data_15() {
        assert_eq!(ModeCode::UnknownModeCode(0b11111u8).has_data(), false);
    }

    #[test]
    fn test_mode_code_is_broadcast_0() {
        assert_eq!(ModeCode::DynamicBusControl.is_broadcast(), false);
    }

    #[test]
    fn test_mode_code_is_broadcast_1() {
        assert_eq!(ModeCode::Synchronize.is_broadcast(), true);
    }

    #[test]
    fn test_mode_code_is_broadcast_2() {
        assert_eq!(ModeCode::TransmitStatusWord.is_broadcast(), false);
    }

    #[test]
    fn test_mode_code_is_broadcast_3() {
        assert_eq!(ModeCode::InitiateSelfTest.is_broadcast(), true);
    }

    #[test]
    fn test_mode_code_is_broadcast_4() {
        assert_eq!(ModeCode::TransmitterShutdown.is_broadcast(), true);
    }

    #[test]
    fn test_mode_code_is_broadcast_5() {
        assert_eq!(ModeCode::OverrideTransmitterShutdown.is_broadcast(), true);
    }

    #[test]
    fn test_mode_code_is_broadcast_6() {
        assert_eq!(ModeCode::InhibitTerminalFlagBit.is_broadcast(), true);
    }

    #[test]
    fn test_mode_code_is_broadcast_7() {
        assert_eq!(
            ModeCode::OverrideInhibitTerminalFlagBit.is_broadcast(),
            true
        );
    }

    #[test]
    fn test_mode_code_is_broadcast_8() {
        assert_eq!(ModeCode::ResetRemoteTerminal.is_broadcast(), true);
    }

    #[test]
    fn test_mode_code_is_broadcast_9() {
        assert_eq!(ModeCode::TransmitVectorWord.is_broadcast(), false);
    }

    #[test]
    fn test_mode_code_is_broadcast_10() {
        assert_eq!(ModeCode::SynchronizeWithDataWord.is_broadcast(), true);
    }

    #[test]
    fn test_mode_code_is_broadcast_11() {
        assert_eq!(ModeCode::TransmitLastCommandWord.is_broadcast(), false);
    }

    #[test]
    fn test_mode_code_is_broadcast_12() {
        assert_eq!(ModeCode::TransmitBITWord.is_broadcast(), false);
    }

    #[test]
    fn test_mode_code_is_broadcast_13() {
        assert_eq!(ModeCode::SelectedTransmitterShutdown.is_broadcast(), true);
    }

    #[test]
    fn test_mode_code_is_broadcast_14() {
        assert_eq!(
            ModeCode::OverrideSelectedTransmitterShutdown.is_broadcast(),
            true
        );
    }

    #[test]
    fn test_mode_code_is_broadcast_15() {
        assert_eq!(ModeCode::UnknownModeCode(0b11111u8).is_broadcast(), false);
    }

    #[test]
    fn test_mode_code_is_unknown_0() {
        assert_eq!(ModeCode::DynamicBusControl.is_unknown(), false);
    }

    #[test]
    fn test_mode_code_is_unknown_1() {
        assert_eq!(ModeCode::Synchronize.is_unknown(), false);
    }

    #[test]
    fn test_mode_code_is_unknown_2() {
        assert_eq!(ModeCode::TransmitStatusWord.is_unknown(), false);
    }

    #[test]
    fn test_mode_code_is_unknown_3() {
        assert_eq!(ModeCode::InitiateSelfTest.is_unknown(), false);
    }

    #[test]
    fn test_mode_code_is_unknown_4() {
        assert_eq!(ModeCode::TransmitterShutdown.is_unknown(), false);
    }

    #[test]
    fn test_mode_code_is_unknown_5() {
        assert_eq!(ModeCode::OverrideTransmitterShutdown.is_unknown(), false);
    }

    #[test]
    fn test_mode_code_is_unknown_6() {
        assert_eq!(ModeCode::InhibitTerminalFlagBit.is_unknown(), false);
    }

    #[test]
    fn test_mode_code_is_unknown_7() {
        assert_eq!(ModeCode::OverrideInhibitTerminalFlagBit.is_unknown(), false);
    }

    #[test]
    fn test_mode_code_is_unknown_8() {
        assert_eq!(ModeCode::ResetRemoteTerminal.is_unknown(), false);
    }

    #[test]
    fn test_mode_code_is_unknown_9() {
        assert_eq!(ModeCode::TransmitVectorWord.is_unknown(), false);
    }

    #[test]
    fn test_mode_code_is_unknown_10() {
        assert_eq!(ModeCode::SynchronizeWithDataWord.is_unknown(), false);
    }

    #[test]
    fn test_mode_code_is_unknown_11() {
        assert_eq!(ModeCode::TransmitLastCommandWord.is_unknown(), false);
    }

    #[test]
    fn test_mode_code_is_unknown_12() {
        assert_eq!(ModeCode::TransmitBITWord.is_unknown(), false);
    }

    #[test]
    fn test_mode_code_is_unknown_13() {
        assert_eq!(ModeCode::SelectedTransmitterShutdown.is_unknown(), false);
    }

    #[test]
    fn test_mode_code_is_unknown_14() {
        assert_eq!(
            ModeCode::OverrideSelectedTransmitterShutdown.is_unknown(),
            false
        );
    }

    #[test]
    fn test_mode_code_is_unknown_15() {
        assert_eq!(ModeCode::UnknownModeCode(0b11111u8).is_unknown(), true);
    }

    #[test]
    fn test_mode_code_from_u8_0() {
        assert_eq!(ModeCode::from(0b00000u8), ModeCode::DynamicBusControl);
    }

    #[test]
    fn test_mode_code_from_u8_1() {
        assert_eq!(ModeCode::from(0b00001u8), ModeCode::Synchronize);
    }

    #[test]
    fn test_mode_code_from_u8_2() {
        assert_eq!(ModeCode::from(0b00010u8), ModeCode::TransmitStatusWord);
    }

    #[test]
    fn test_mode_code_from_u8_3() {
        assert_eq!(ModeCode::from(0b00011u8), ModeCode::InitiateSelfTest);
    }

    #[test]
    fn test_mode_code_from_u8_4() {
        assert_eq!(ModeCode::from(0b00100u8), ModeCode::TransmitterShutdown);
    }

    #[test]
    fn test_mode_code_from_u8_5() {
        assert_eq!(
            ModeCode::from(0b00101u8),
            ModeCode::OverrideTransmitterShutdown
        );
    }

    #[test]
    fn test_mode_code_from_u8_6() {
        assert_eq!(ModeCode::from(0b00110u8), ModeCode::InhibitTerminalFlagBit);
    }

    #[test]
    fn test_mode_code_from_u8_7() {
        assert_eq!(
            ModeCode::from(0b00111u8),
            ModeCode::OverrideInhibitTerminalFlagBit
        );
    }

    #[test]
    fn test_mode_code_from_u8_8() {
        assert_eq!(ModeCode::from(0b01000u8), ModeCode::ResetRemoteTerminal);
    }

    #[test]
    fn test_mode_code_from_u8_9() {
        assert_eq!(ModeCode::from(0b10000u8), ModeCode::TransmitVectorWord);
    }

    #[test]
    fn test_mode_code_from_u8_10() {
        assert_eq!(ModeCode::from(0b10001u8), ModeCode::SynchronizeWithDataWord);
    }

    #[test]
    fn test_mode_code_from_u8_11() {
        assert_eq!(ModeCode::from(0b10010u8), ModeCode::TransmitLastCommandWord);
    }

    #[test]
    fn test_mode_code_from_u8_12() {
        assert_eq!(ModeCode::from(0b10011u8), ModeCode::TransmitBITWord);
    }

    #[test]
    fn test_mode_code_from_u8_13() {
        assert_eq!(
            ModeCode::from(0b10100u8),
            ModeCode::SelectedTransmitterShutdown
        );
    }

    #[test]
    fn test_mode_code_from_u8_14() {
        assert_eq!(
            ModeCode::from(0b10101u8),
            ModeCode::OverrideSelectedTransmitterShutdown
        );
    }

    #[test]
    fn test_mode_code_from_u8_15() {
        assert_eq!(
            ModeCode::from(0b11111u8),
            ModeCode::UnknownModeCode(0b11111u8)
        );
    }

    #[test]
    fn test_mode_code_from_u16_0() {
        assert_eq!(ModeCode::from(0b00000u16), ModeCode::DynamicBusControl);
    }

    #[test]
    fn test_mode_code_from_u16_1() {
        assert_eq!(ModeCode::from(0b00001u16), ModeCode::Synchronize);
    }

    #[test]
    fn test_mode_code_from_u16_2() {
        assert_eq!(ModeCode::from(0b00010u16), ModeCode::TransmitStatusWord);
    }

    #[test]
    fn test_mode_code_from_u16_3() {
        assert_eq!(ModeCode::from(0b00011u16), ModeCode::InitiateSelfTest);
    }

    #[test]
    fn test_mode_code_from_u16_4() {
        assert_eq!(ModeCode::from(0b00100u16), ModeCode::TransmitterShutdown);
    }

    #[test]
    fn test_mode_code_from_u16_5() {
        assert_eq!(
            ModeCode::from(0b00101u16),
            ModeCode::OverrideTransmitterShutdown
        );
    }

    #[test]
    fn test_mode_code_from_u16_6() {
        assert_eq!(ModeCode::from(0b00110u16), ModeCode::InhibitTerminalFlagBit);
    }

    #[test]
    fn test_mode_code_from_u16_7() {
        assert_eq!(
            ModeCode::from(0b00111u16),
            ModeCode::OverrideInhibitTerminalFlagBit
        );
    }

    #[test]
    fn test_mode_code_from_u16_8() {
        assert_eq!(ModeCode::from(0b01000u16), ModeCode::ResetRemoteTerminal);
    }

    #[test]
    fn test_mode_code_from_u16_9() {
        assert_eq!(ModeCode::from(0b10000u16), ModeCode::TransmitVectorWord);
    }

    #[test]
    fn test_mode_code_from_u16_10() {
        assert_eq!(
            ModeCode::from(0b10001u16),
            ModeCode::SynchronizeWithDataWord
        );
    }

    #[test]
    fn test_mode_code_from_u16_11() {
        assert_eq!(
            ModeCode::from(0b10010u16),
            ModeCode::TransmitLastCommandWord
        );
    }

    #[test]
    fn test_mode_code_from_u16_12() {
        assert_eq!(ModeCode::from(0b10011u16), ModeCode::TransmitBITWord);
    }

    #[test]
    fn test_mode_code_from_u16_13() {
        assert_eq!(
            ModeCode::from(0b10100u16),
            ModeCode::SelectedTransmitterShutdown
        );
    }

    #[test]
    fn test_mode_code_from_u16_14() {
        assert_eq!(
            ModeCode::from(0b10101u16),
            ModeCode::OverrideSelectedTransmitterShutdown
        );
    }

    #[test]
    fn test_mode_code_from_u16_15() {
        assert_eq!(
            ModeCode::from(0b101000_11111u16),
            ModeCode::UnknownModeCode(0b11111u8)
        );
    }

    #[test]
    fn test_mode_code_to_u8_0() {
        assert_eq!(u8::from(ModeCode::DynamicBusControl), 0b00000u8);
    }

    #[test]
    fn test_mode_code_to_u8_1() {
        assert_eq!(u8::from(ModeCode::Synchronize), 0b00001u8);
    }

    #[test]
    fn test_mode_code_to_u8_2() {
        assert_eq!(u8::from(ModeCode::TransmitStatusWord), 0b00010u8);
    }

    #[test]
    fn test_mode_code_to_u8_3() {
        assert_eq!(u8::from(ModeCode::InitiateSelfTest), 0b00011u8);
    }

    #[test]
    fn test_mode_code_to_u8_4() {
        assert_eq!(u8::from(ModeCode::TransmitterShutdown), 0b00100u8);
    }

    #[test]
    fn test_mode_code_to_u8_5() {
        assert_eq!(u8::from(ModeCode::OverrideTransmitterShutdown), 0b00101u8);
    }

    #[test]
    fn test_mode_code_to_u8_6() {
        assert_eq!(u8::from(ModeCode::InhibitTerminalFlagBit), 0b00110u8);
    }

    #[test]
    fn test_mode_code_to_u8_7() {
        assert_eq!(
            u8::from(ModeCode::OverrideInhibitTerminalFlagBit),
            0b00111u8
        );
    }

    #[test]
    fn test_mode_code_to_u8_8() {
        assert_eq!(u8::from(ModeCode::ResetRemoteTerminal), 0b01000u8);
    }

    #[test]
    fn test_mode_code_to_u8_9() {
        assert_eq!(u8::from(ModeCode::TransmitVectorWord), 0b10000u8);
    }

    #[test]
    fn test_mode_code_to_u8_10() {
        assert_eq!(u8::from(ModeCode::SynchronizeWithDataWord), 0b10001u8);
    }

    #[test]
    fn test_mode_code_to_u8_11() {
        assert_eq!(u8::from(ModeCode::TransmitLastCommandWord), 0b10010u8);
    }

    #[test]
    fn test_mode_code_to_u8_12() {
        assert_eq!(u8::from(ModeCode::TransmitBITWord), 0b10011u8);
    }

    #[test]
    fn test_mode_code_to_u8_13() {
        assert_eq!(u8::from(ModeCode::SelectedTransmitterShutdown), 0b10100u8);
    }

    #[test]
    fn test_mode_code_to_u8_14() {
        assert_eq!(
            u8::from(ModeCode::OverrideSelectedTransmitterShutdown),
            0b10101u8
        );
    }

    #[test]
    fn test_mode_code_to_u8_15() {
        assert_eq!(u8::from(ModeCode::UnknownModeCode(0b11111u8)), 0b11111u8);
    }

    #[test]
    fn test_mode_code_to_u16_0() {
        assert_eq!(u16::from(ModeCode::DynamicBusControl), 0b00000u16);
    }

    #[test]
    fn test_mode_code_to_u16_1() {
        assert_eq!(u16::from(ModeCode::Synchronize), 0b00001u16);
    }

    #[test]
    fn test_mode_code_to_u16_2() {
        assert_eq!(u16::from(ModeCode::TransmitStatusWord), 0b00010u16);
    }

    #[test]
    fn test_mode_code_to_u16_3() {
        assert_eq!(u16::from(ModeCode::InitiateSelfTest), 0b00011u16);
    }

    #[test]
    fn test_mode_code_to_u16_4() {
        assert_eq!(u16::from(ModeCode::TransmitterShutdown), 0b00100u16);
    }

    #[test]
    fn test_mode_code_to_u16_5() {
        assert_eq!(u16::from(ModeCode::OverrideTransmitterShutdown), 0b00101u16);
    }

    #[test]
    fn test_mode_code_to_u16_6() {
        assert_eq!(u16::from(ModeCode::InhibitTerminalFlagBit), 0b00110u16);
    }

    #[test]
    fn test_mode_code_to_u16_7() {
        assert_eq!(
            u16::from(ModeCode::OverrideInhibitTerminalFlagBit),
            0b00111u16
        );
    }

    #[test]
    fn test_mode_code_to_u16_8() {
        assert_eq!(u16::from(ModeCode::ResetRemoteTerminal), 0b01000u16);
    }

    #[test]
    fn test_mode_code_to_u16_9() {
        assert_eq!(u16::from(ModeCode::TransmitVectorWord), 0b10000u16);
    }

    #[test]
    fn test_mode_code_to_u16_10() {
        assert_eq!(u16::from(ModeCode::SynchronizeWithDataWord), 0b10001u16);
    }

    #[test]
    fn test_mode_code_to_u16_11() {
        assert_eq!(u16::from(ModeCode::TransmitLastCommandWord), 0b10010u16);
    }

    #[test]
    fn test_mode_code_to_u16_12() {
        assert_eq!(u16::from(ModeCode::TransmitBITWord), 0b10011u16);
    }

    #[test]
    fn test_mode_code_to_u16_13() {
        assert_eq!(u16::from(ModeCode::SelectedTransmitterShutdown), 0b10100u16);
    }

    #[test]
    fn test_mode_code_to_u16_14() {
        assert_eq!(
            u16::from(ModeCode::OverrideSelectedTransmitterShutdown),
            0b10101u16
        );
    }

    #[test]
    fn test_mode_code_to_u16_15() {
        assert_eq!(u16::from(ModeCode::UnknownModeCode(0b11111u8)), 0b11111u16);
    }

    #[test]
    fn test_transmit_receive_clone() {
        let item1 = TransmitReceive::Transmit;
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[test]
    fn test_transmit_receive_is_transmit_0() {
        assert_eq!(TransmitReceive::Receive.is_transmit(), false);
    }

    #[test]
    fn test_transmit_receive_is_transmit_1() {
        assert_eq!(TransmitReceive::Transmit.is_transmit(), true);
    }

    #[test]
    fn test_transmit_receive_is_receive_0() {
        assert_eq!(TransmitReceive::Receive.is_receive(), true);
    }

    #[test]
    fn test_transmit_receive_is_receive_1() {
        assert_eq!(TransmitReceive::Transmit.is_receive(), false);
    }

    #[test]
    fn test_transmit_receive_from_u8_0() {
        assert_eq!(TransmitReceive::from(0u8), TransmitReceive::Receive);
    }

    #[test]
    fn test_transmit_receive_from_u8_1() {
        assert_eq!(TransmitReceive::from(1u8), TransmitReceive::Transmit);
    }

    #[test]
    fn test_transmit_receive_from_u8_2() {
        assert_eq!(TransmitReceive::from(2u8), TransmitReceive::Transmit);
    }

    #[test]
    fn test_transmit_receive_from_u16_0() {
        assert_eq!(TransmitReceive::from(0u16), TransmitReceive::Receive);
    }

    #[test]
    fn test_transmit_receive_from_u16_1() {
        assert_eq!(TransmitReceive::from(1u16), TransmitReceive::Transmit);
    }

    #[test]
    fn test_transmit_receive_from_u16_2() {
        assert_eq!(TransmitReceive::from(2u16), TransmitReceive::Transmit);
    }

    #[test]
    fn test_transmit_receive_to_u8_0() {
        assert_eq!(u8::from(TransmitReceive::Receive), 0);
    }

    #[test]
    fn test_transmit_receive_to_u8_1() {
        assert_eq!(u8::from(TransmitReceive::Transmit), 1);
    }

    #[test]
    fn test_transmit_receive_to_u16_0() {
        assert_eq!(u16::from(TransmitReceive::Receive), 0);
    }

    #[test]
    fn test_transmit_receive_to_u16_1() {
        assert_eq!(u16::from(TransmitReceive::Transmit), 1);
    }

    #[test]
    fn test_address_clone() {
        let item1 = Address::Broadcast(0b11111u8);
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[test]
    fn test_address_is_value_0() {
        assert_eq!(Address::Value(0b10101u8).is_value(), true);
    }

    #[test]
    fn test_address_is_value_2() {
        assert_eq!(Address::Broadcast(0b11111u8).is_value(), false);
    }

    #[test]
    fn test_address_is_broadcast_0() {
        assert_eq!(Address::Value(0b10101u8).is_broadcast(), false);
    }

    #[test]
    fn test_address_is_broadcast_2() {
        assert_eq!(Address::Broadcast(0b11111u8).is_broadcast(), true);
    }

    #[test]
    fn test_address_from_u8_0() {
        assert_eq!(Address::from(0b10101u8), Address::Value(0b10101u8));
    }

    #[test]
    fn test_address_from_u8_2() {
        assert_eq!(Address::from(0b11111u8), Address::Broadcast(0b11111u8));
    }

    #[test]
    fn test_address_from_u8_3() {
        assert_eq!(Address::from(0b11111111u8), Address::Broadcast(0b11111u8));
    }

    #[test]
    fn test_address_from_u16_0() {
        assert_eq!(Address::from(0b10101u16), Address::Value(0b10101u8));
    }

    #[test]
    fn test_address_from_u16_2() {
        assert_eq!(Address::from(0b11111u16), Address::Broadcast(0b11111u8));
    }

    #[test]
    fn test_address_from_u16_3() {
        assert_eq!(Address::from(0b11111111u16), Address::Broadcast(0b11111u8));
    }

    #[test]
    fn test_address_to_u8_0() {
        assert_eq!(u8::from(Address::Value(0b10101u8)), 0b10101u8);
    }

    #[test]
    fn test_address_to_u8_2() {
        assert_eq!(u8::from(Address::Broadcast(0b11111u8)), 0b11111u8);
    }

    #[test]
    fn test_address_to_u16_0() {
        assert_eq!(u16::from(Address::Value(0b10101u8)), 0b10101u16);
    }

    #[test]
    fn test_address_to_u16_2() {
        assert_eq!(u16::from(Address::Broadcast(0b11111u8)), 0b11111u16);
    }

    #[test]
    fn test_subaddress_clone() {
        let item1 = SubAddress::ModeCode(0b11111u8);
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[test]
    fn test_subaddress_is_value_0() {
        assert_eq!(SubAddress::Value(0b10101u8).is_value(), true);
    }

    #[test]
    fn test_subaddress_is_value_2() {
        assert_eq!(SubAddress::ModeCode(0b11111u8).is_value(), false);
    }

    #[test]
    fn test_subaddress_is_value_3() {
        assert_eq!(SubAddress::ModeCode(0b00000u8).is_value(), false);
    }

    #[test]
    fn test_subaddress_is_mode_code_0() {
        assert_eq!(SubAddress::Value(0b10101u8).is_mode_code(), false);
    }

    #[test]
    fn test_subaddress_is_mode_code_2() {
        assert_eq!(SubAddress::ModeCode(0b11111u8).is_mode_code(), true);
    }

    #[test]
    fn test_subaddress_is_mode_code_3() {
        assert_eq!(SubAddress::ModeCode(0b00000u8).is_mode_code(), true);
    }

    #[test]
    fn test_subaddress_to_u8_0() {
        assert_eq!(u8::from(SubAddress::Value(0b10101u8)), 0b10101u8);
    }

    #[test]
    fn test_subaddress_to_u8_2() {
        assert_eq!(u8::from(SubAddress::ModeCode(0b11111u8)), 0b11111u8);
    }

    #[test]
    fn test_subaddress_to_u8_3() {
        assert_eq!(u8::from(SubAddress::ModeCode(0b00000u8)), 0b00000u8);
    }

    #[test]
    fn test_subaddress_to_u16_0() {
        assert_eq!(u16::from(SubAddress::Value(0b10101u8)), 0b10101u16);
    }

    #[test]
    fn test_subaddress_to_u16_2() {
        assert_eq!(u16::from(SubAddress::ModeCode(0b11111u8)), 0b11111u16);
    }

    #[test]
    fn test_subaddress_to_u16_3() {
        assert_eq!(u16::from(SubAddress::ModeCode(0b00000u8)), 0b00000u16);
    }

    #[test]
    fn test_subaddress_from_u8_0() {
        assert_eq!(SubAddress::from(0b10101u8), SubAddress::Value(0b10101u8));
    }

    #[test]
    fn test_subaddress_from_u8_2() {
        assert_eq!(SubAddress::from(0b00000u8), SubAddress::ModeCode(0b00000u8));
    }

    #[test]
    fn test_subaddress_from_u8_3() {
        assert_eq!(SubAddress::from(0b11111u8), SubAddress::ModeCode(0b11111u8));
    }

    #[test]
    fn test_subaddress_from_u16_0() {
        assert_eq!(SubAddress::from(0b10101u16), SubAddress::Value(0b10101u8));
    }

    #[test]
    fn test_subaddress_from_u16_2() {
        assert_eq!(
            SubAddress::from(0b00000u16),
            SubAddress::ModeCode(0b00000u8)
        );
    }

    #[test]
    fn test_subaddress_from_u16_3() {
        assert_eq!(
            SubAddress::from(0b11111u16),
            SubAddress::ModeCode(0b11111u8)
        );
    }

    #[test]
    fn test_subaddress_from_u8_4() {
        assert_eq!(
            SubAddress::from(0b11111111u8),
            SubAddress::ModeCode(0b11111u8)
        );
    }

    #[test]
    fn test_instrumentation_clone() {
        let item1 = Instrumentation::Command;
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[test]
    fn test_instrumentation_is_status_0() {
        assert_eq!(Instrumentation::Status.is_status(), true);
    }

    #[test]
    fn test_instrumentation_is_status_1() {
        assert_eq!(Instrumentation::Command.is_status(), false);
    }

    #[test]
    fn test_instrumentation_is_command_0() {
        assert_eq!(Instrumentation::Status.is_command(), false);
    }

    #[test]
    fn test_instrumentation_is_command_1() {
        assert_eq!(Instrumentation::Command.is_command(), true);
    }

    #[test]
    fn test_instrumentation_from_u8_0() {
        assert_eq!(Instrumentation::from(0u8), Instrumentation::Status);
    }

    #[test]
    fn test_instrumentation_from_u8_1() {
        assert_eq!(Instrumentation::from(1u8), Instrumentation::Command);
    }

    #[test]
    fn test_instrumentation_from_u8_2() {
        assert_eq!(Instrumentation::from(2u8), Instrumentation::Command);
    }

    #[test]
    fn test_instrumentation_to_u8_0() {
        assert_eq!(u8::from(Instrumentation::Status), 0);
    }

    #[test]
    fn test_instrumentation_to_u8_1() {
        assert_eq!(u8::from(Instrumentation::Command), 1);
    }

    #[test]
    fn test_instrumentation_from_u16_0() {
        assert_eq!(Instrumentation::from(0u16), Instrumentation::Status);
    }

    #[test]
    fn test_instrumentation_from_u16_1() {
        assert_eq!(Instrumentation::from(1u16), Instrumentation::Command);
    }

    #[test]
    fn test_instrumentation_from_u16_2() {
        assert_eq!(Instrumentation::from(2u16), Instrumentation::Command);
    }

    #[test]
    fn test_instrumentation_to_u16_0() {
        assert_eq!(u16::from(Instrumentation::Status), 0);
    }

    #[test]
    fn test_instrumentation_to_u16_1() {
        assert_eq!(u16::from(Instrumentation::Command), 1);
    }

    #[test]
    fn test_service_request_clone() {
        let item1 = ServiceRequest::Service;
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[test]
    fn test_service_request_is_noservice_0() {
        assert_eq!(ServiceRequest::NoService.is_noservice(), true);
    }

    #[test]
    fn test_service_request_is_noservice_1() {
        assert_eq!(ServiceRequest::Service.is_noservice(), false);
    }

    #[test]
    fn test_service_request_is_service_0() {
        assert_eq!(ServiceRequest::NoService.is_service(), false);
    }

    #[test]
    fn test_service_request_is_service_1() {
        assert_eq!(ServiceRequest::Service.is_service(), true);
    }

    #[test]
    fn test_service_request_from_u8_0() {
        assert_eq!(ServiceRequest::from(0u8), ServiceRequest::NoService);
    }

    #[test]
    fn test_service_request_from_u8_1() {
        assert_eq!(ServiceRequest::from(1u8), ServiceRequest::Service);
    }

    #[test]
    fn test_service_request_from_u8_2() {
        assert_eq!(ServiceRequest::from(2u8), ServiceRequest::Service);
    }

    #[test]
    fn test_service_request_to_u8_0() {
        assert_eq!(u8::from(ServiceRequest::NoService), 0);
    }

    #[test]
    fn test_service_request_to_u8_1() {
        assert_eq!(u8::from(ServiceRequest::Service), 1);
    }

    #[test]
    fn test_service_request_from_u16_0() {
        assert_eq!(ServiceRequest::from(0u16), ServiceRequest::NoService);
    }

    #[test]
    fn test_service_request_from_u16_1() {
        assert_eq!(ServiceRequest::from(1u16), ServiceRequest::Service);
    }

    #[test]
    fn test_service_request_from_u16_2() {
        assert_eq!(ServiceRequest::from(2u16), ServiceRequest::Service);
    }

    #[test]
    fn test_service_request_to_u16_0() {
        assert_eq!(u16::from(ServiceRequest::NoService), 0);
    }

    #[test]
    fn test_service_request_to_u16_1() {
        assert_eq!(u16::from(ServiceRequest::Service), 1);
    }

    #[test]
    fn test_reserved_clone() {
        let item1 = Reserved::Value(0b111u8);
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[test]
    fn test_reserved_is_none_0() {
        assert_eq!(Reserved::None.is_none(), true);
    }

    #[test]
    fn test_reserved_is_none_1() {
        assert_eq!(Reserved::Value(0b111u8).is_none(), false);
    }

    #[test]
    fn test_reserved_is_value_0() {
        assert_eq!(Reserved::None.is_value(), false);
    }

    #[test]
    fn test_reserved_is_value_1() {
        assert_eq!(Reserved::Value(0b111u8).is_value(), true);
    }

    #[test]
    fn test_reserved_from_u8_0() {
        assert_eq!(Reserved::from(0u8), Reserved::None);
    }

    #[test]
    fn test_reserved_from_u8_1() {
        assert_eq!(Reserved::from(0b111u8), Reserved::Value(0b111u8));
    }

    #[test]
    fn test_reserved_to_u8_0() {
        assert_eq!(u8::from(Reserved::None), 0);
    }

    #[test]
    fn test_reserved_to_u8_1() {
        assert_eq!(u8::from(Reserved::Value(0b111u8)), 0b111u8);
    }

    #[test]
    fn test_reserved_from_u16_0() {
        assert_eq!(Reserved::from(0u16), Reserved::None);
    }

    #[test]
    fn test_reserved_from_u16_1() {
        assert_eq!(Reserved::from(0b111u16), Reserved::Value(0b111u8));
    }

    #[test]
    fn test_reserved_to_u16_0() {
        assert_eq!(u16::from(Reserved::None), 0);
    }

    #[test]
    fn test_reserved_to_u16_1() {
        assert_eq!(u16::from(Reserved::Value(0b111u8)), 0b111u16);
    }

    #[test]
    fn test_broadcast_received_clone() {
        let item1 = BroadcastReceived::Received;
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[test]
    fn test_broadcast_received_is_notreceived_0() {
        assert_eq!(BroadcastReceived::NotReceived.is_notreceived(), true);
    }

    #[test]
    fn test_broadcast_received_is_notreceived_1() {
        assert_eq!(BroadcastReceived::Received.is_notreceived(), false);
    }

    #[test]
    fn test_broadcast_received_is_received_0() {
        assert_eq!(BroadcastReceived::NotReceived.is_received(), false);
    }

    #[test]
    fn test_broadcast_received_is_received_1() {
        assert_eq!(BroadcastReceived::Received.is_received(), true);
    }

    #[test]
    fn test_broadcast_received_from_u8_0() {
        assert_eq!(BroadcastReceived::from(0u8), BroadcastReceived::NotReceived);
    }

    #[test]
    fn test_broadcast_received_from_u8_1() {
        assert_eq!(BroadcastReceived::from(1u8), BroadcastReceived::Received);
    }

    #[test]
    fn test_broadcast_received_from_u8_2() {
        assert_eq!(BroadcastReceived::from(2u8), BroadcastReceived::Received);
    }

    #[test]
    fn test_broadcast_received_to_u8_0() {
        assert_eq!(u8::from(BroadcastReceived::NotReceived), 0);
    }

    #[test]
    fn test_broadcast_received_to_u8_1() {
        assert_eq!(u8::from(BroadcastReceived::Received), 1);
    }

    #[test]
    fn test_broadcast_received_from_u16_0() {
        assert_eq!(
            BroadcastReceived::from(0u16),
            BroadcastReceived::NotReceived
        );
    }

    #[test]
    fn test_broadcast_received_from_u16_1() {
        assert_eq!(BroadcastReceived::from(1u16), BroadcastReceived::Received);
    }

    #[test]
    fn test_broadcast_received_from_u16_2() {
        assert_eq!(BroadcastReceived::from(2u16), BroadcastReceived::Received);
    }

    #[test]
    fn test_broadcast_received_to_u16_0() {
        assert_eq!(u16::from(BroadcastReceived::NotReceived), 0);
    }

    #[test]
    fn test_broadcast_received_to_u16_1() {
        assert_eq!(u16::from(BroadcastReceived::Received), 1);
    }

    #[test]
    fn test_terminal_busy_clone() {
        let item1 = TerminalBusy::Busy;
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[test]
    fn test_terminal_busy_is_notbusy_0() {
        assert_eq!(TerminalBusy::NotBusy.is_notbusy(), true);
    }

    #[test]
    fn test_terminal_busy_is_notbusy_1() {
        assert_eq!(TerminalBusy::Busy.is_notbusy(), false);
    }

    #[test]
    fn test_terminal_busy_is_busy_0() {
        assert_eq!(TerminalBusy::NotBusy.is_busy(), false);
    }

    #[test]
    fn test_terminal_busy_is_busy_1() {
        assert_eq!(TerminalBusy::Busy.is_busy(), true);
    }

    #[test]
    fn test_terminal_busy_from_u8_0() {
        assert_eq!(TerminalBusy::from(0u8), TerminalBusy::NotBusy);
    }

    #[test]
    fn test_terminal_busy_from_u8_1() {
        assert_eq!(TerminalBusy::from(1u8), TerminalBusy::Busy);
    }

    #[test]
    fn test_terminal_busy_from_u8_2() {
        assert_eq!(TerminalBusy::from(2u8), TerminalBusy::Busy);
    }

    #[test]
    fn test_terminal_busy_to_u8_0() {
        assert_eq!(u8::from(TerminalBusy::NotBusy), 0);
    }

    #[test]
    fn test_terminal_busy_to_u8_1() {
        assert_eq!(u8::from(TerminalBusy::Busy), 1);
    }

    #[test]
    fn test_terminal_busy_from_u16_0() {
        assert_eq!(TerminalBusy::from(0u16), TerminalBusy::NotBusy);
    }

    #[test]
    fn test_terminal_busy_from_u16_1() {
        assert_eq!(TerminalBusy::from(1u16), TerminalBusy::Busy);
    }

    #[test]
    fn test_terminal_busy_from_u16_2() {
        assert_eq!(TerminalBusy::from(2u16), TerminalBusy::Busy);
    }

    #[test]
    fn test_terminal_busy_to_u16_0() {
        assert_eq!(u16::from(TerminalBusy::NotBusy), 0);
    }

    #[test]
    fn test_terminal_busy_to_u16_1() {
        assert_eq!(u16::from(TerminalBusy::Busy), 1);
    }

    #[test]
    fn test_dynamic_bus_acceptance_clone() {
        let item1 = DynamicBusAcceptance::Accepted;
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[test]
    fn test_dynamic_bus_acceptance_is_notaccepted_0() {
        assert_eq!(DynamicBusAcceptance::NotAccepted.is_notaccepted(), true);
    }

    #[test]
    fn test_dynamic_bus_acceptance_is_notaccepted_1() {
        assert_eq!(DynamicBusAcceptance::Accepted.is_notaccepted(), false);
    }

    #[test]
    fn test_dynamic_bus_acceptance_is_accepted_0() {
        assert_eq!(DynamicBusAcceptance::NotAccepted.is_accepted(), false);
    }

    #[test]
    fn test_dynamic_bus_acceptance_is_accepted_1() {
        assert_eq!(DynamicBusAcceptance::Accepted.is_accepted(), true);
    }

    #[test]
    fn test_dynamic_bus_acceptance_from_u8_0() {
        assert_eq!(
            DynamicBusAcceptance::from(0u8),
            DynamicBusAcceptance::NotAccepted
        );
    }

    #[test]
    fn test_dynamic_bus_acceptance_from_u8_1() {
        assert_eq!(
            DynamicBusAcceptance::from(1u8),
            DynamicBusAcceptance::Accepted
        );
    }

    #[test]
    fn test_dynamic_bus_acceptance_from_u8_2() {
        assert_eq!(
            DynamicBusAcceptance::from(2u8),
            DynamicBusAcceptance::Accepted
        );
    }

    #[test]
    fn test_dynamic_bus_acceptance_to_u8_0() {
        assert_eq!(u8::from(DynamicBusAcceptance::NotAccepted), 0);
    }

    #[test]
    fn test_dynamic_bus_acceptance_to_u8_1() {
        assert_eq!(u8::from(DynamicBusAcceptance::Accepted), 1);
    }

    #[test]
    fn test_dynamic_bus_acceptance_from_u16_0() {
        assert_eq!(
            DynamicBusAcceptance::from(0u16),
            DynamicBusAcceptance::NotAccepted
        );
    }

    #[test]
    fn test_dynamic_bus_acceptance_from_u16_1() {
        assert_eq!(
            DynamicBusAcceptance::from(1u16),
            DynamicBusAcceptance::Accepted
        );
    }

    #[test]
    fn test_dynamic_bus_acceptance_from_u16_2() {
        assert_eq!(
            DynamicBusAcceptance::from(2u16),
            DynamicBusAcceptance::Accepted
        );
    }

    #[test]
    fn test_dynamic_bus_acceptance_to_u16_0() {
        assert_eq!(u16::from(DynamicBusAcceptance::NotAccepted), 0);
    }

    #[test]
    fn test_dynamic_bus_acceptance_to_u16_1() {
        assert_eq!(u16::from(DynamicBusAcceptance::Accepted), 1);
    }
}
