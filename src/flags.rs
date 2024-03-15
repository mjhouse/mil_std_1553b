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
        match v {
            k if eqs!(k, 0b0001_1111) => Address::Broadcast(k),
            k if fit!(k, 0b0001_1111) => Address::Value(k),
            k => Address::Unknown(k),
        }
    }
}

impl From<Address> for u8 {
    fn from(v: Address) -> u8 {
        match v {
            Address::Value(k) => k,
            Address::Unknown(k) => k,
            Address::Broadcast(k) => k,
        }
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
        match v {
            k if eqs!(k, 0b0000_0000) => SubAddress::ModeCode(k),
            k if eqs!(k, 0b0001_1111) => SubAddress::ModeCode(k),
            k if fit!(k, 0b0001_1111) => SubAddress::Value(k),
            k => SubAddress::Unknown(k),
        }
    }
}

impl From<SubAddress> for u8 {
    fn from(v: SubAddress) -> u8 {
        match v {
            SubAddress::Value(k) => k,
            SubAddress::Unknown(k) => k,
            SubAddress::ModeCode(k) => k,
        }
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
            1 => Self::Received,
            _ => Self::NotReceived,
        }
    }
}

impl From<BroadcastReceived> for u8 {
    fn from(value: BroadcastReceived) -> Self {
        match value {
            BroadcastReceived::Received => 1,
            BroadcastReceived::NotReceived => 0,
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
            1 => Self::Accepted,
            _ => Self::NotAccepted,
        }
    }
}

impl From<DynamicBusAcceptance> for u8 {
    fn from(value: DynamicBusAcceptance) -> Self {
        match value {
            DynamicBusAcceptance::Accepted => 1,
            DynamicBusAcceptance::NotAccepted => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_mode_code_clone() {
        let item1 = ModeCode::InhibitTerminalFlagBit;
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[rstest]
    #[case(ModeCode::DynamicBusControl, 0b00000)]
    #[case(ModeCode::Synchronize, 0b00001)]
    #[case(ModeCode::TransmitStatusWord, 0b00010)]
    #[case(ModeCode::InitiateSelfTest, 0b00011)]
    #[case(ModeCode::TransmitterShutdown, 0b00100)]
    #[case(ModeCode::OverrideTransmitterShutdown, 0b00101)]
    #[case(ModeCode::InhibitTerminalFlagBit, 0b00110)]
    #[case(ModeCode::OverrideInhibitTerminalFlagBit, 0b00111)]
    #[case(ModeCode::ResetRemoteTerminal, 0b01000)]
    #[case(ModeCode::TransmitVectorWord, 0b10000)]
    #[case(ModeCode::SynchronizeWithDataWord, 0b10001)]
    #[case(ModeCode::TransmitLastCommandWord, 0b10010)]
    #[case(ModeCode::TransmitBITWord, 0b10011)]
    #[case(ModeCode::SelectedTransmitterShutdown, 0b10100)]
    #[case(ModeCode::OverrideSelectedTransmitterShutdown, 0b10101)]
    #[case(ModeCode::UnknownModeCode(0b11111), 0b11111)]
    fn test_mode_code_value(#[case] code: ModeCode, #[case] expected: u8) {
        assert_eq!(code.value(), expected);
    }

    #[rstest]
    #[case(ModeCode::DynamicBusControl, true)]
    #[case(ModeCode::Synchronize, false)]
    #[case(ModeCode::TransmitStatusWord, true)]
    #[case(ModeCode::InitiateSelfTest, true)]
    #[case(ModeCode::TransmitterShutdown, true)]
    #[case(ModeCode::OverrideTransmitterShutdown, true)]
    #[case(ModeCode::InhibitTerminalFlagBit, true)]
    #[case(ModeCode::OverrideInhibitTerminalFlagBit, true)]
    #[case(ModeCode::ResetRemoteTerminal, true)]
    #[case(ModeCode::TransmitVectorWord, true)]
    #[case(ModeCode::SynchronizeWithDataWord, true)]
    #[case(ModeCode::TransmitLastCommandWord, true)]
    #[case(ModeCode::TransmitBITWord, true)]
    #[case(ModeCode::SelectedTransmitterShutdown, false)]
    #[case(ModeCode::OverrideSelectedTransmitterShutdown, false)]
    #[case(ModeCode::UnknownModeCode(0b11111), false)]
    fn test_mode_code_is_transmit(#[case] code: ModeCode, #[case] expected: bool) {
        assert_eq!(code.is_transmit(), expected);
    }

    #[rstest]
    #[case(ModeCode::DynamicBusControl, false)]
    #[case(ModeCode::Synchronize, true)]
    #[case(ModeCode::TransmitStatusWord, false)]
    #[case(ModeCode::InitiateSelfTest, false)]
    #[case(ModeCode::TransmitterShutdown, false)]
    #[case(ModeCode::OverrideTransmitterShutdown, false)]
    #[case(ModeCode::InhibitTerminalFlagBit, false)]
    #[case(ModeCode::OverrideInhibitTerminalFlagBit, false)]
    #[case(ModeCode::ResetRemoteTerminal, false)]
    #[case(ModeCode::TransmitVectorWord, false)]
    #[case(ModeCode::SynchronizeWithDataWord, false)]
    #[case(ModeCode::TransmitLastCommandWord, false)]
    #[case(ModeCode::TransmitBITWord, false)]
    #[case(ModeCode::SelectedTransmitterShutdown, true)]
    #[case(ModeCode::OverrideSelectedTransmitterShutdown, true)]
    #[case(ModeCode::UnknownModeCode(0b11111), false)]
    fn test_mode_code_is_receive(#[case] code: ModeCode, #[case] expected: bool) {
        assert_eq!(code.is_receive(), expected);
    }

    #[rstest]
    #[case(ModeCode::DynamicBusControl, false)]
    #[case(ModeCode::Synchronize, true)]
    #[case(ModeCode::TransmitStatusWord, false)]
    #[case(ModeCode::InitiateSelfTest, false)]
    #[case(ModeCode::TransmitterShutdown, false)]
    #[case(ModeCode::OverrideTransmitterShutdown, false)]
    #[case(ModeCode::InhibitTerminalFlagBit, false)]
    #[case(ModeCode::OverrideInhibitTerminalFlagBit, false)]
    #[case(ModeCode::ResetRemoteTerminal, false)]
    #[case(ModeCode::TransmitVectorWord, true)]
    #[case(ModeCode::SynchronizeWithDataWord, false)]
    #[case(ModeCode::TransmitLastCommandWord, true)]
    #[case(ModeCode::TransmitBITWord, true)]
    #[case(ModeCode::SelectedTransmitterShutdown, true)]
    #[case(ModeCode::OverrideSelectedTransmitterShutdown, true)]
    #[case(ModeCode::UnknownModeCode(0b11111), false)]
    fn test_mode_code_has_data(#[case] code: ModeCode, #[case] expected: bool) {
        assert_eq!(code.has_data(), expected);
    }

    #[rstest]
    #[case(ModeCode::DynamicBusControl, false)]
    #[case(ModeCode::Synchronize, true)]
    #[case(ModeCode::TransmitStatusWord, false)]
    #[case(ModeCode::InitiateSelfTest, true)]
    #[case(ModeCode::TransmitterShutdown, true)]
    #[case(ModeCode::OverrideTransmitterShutdown, true)]
    #[case(ModeCode::InhibitTerminalFlagBit, true)]
    #[case(ModeCode::OverrideInhibitTerminalFlagBit, true)]
    #[case(ModeCode::ResetRemoteTerminal, true)]
    #[case(ModeCode::TransmitVectorWord, false)]
    #[case(ModeCode::SynchronizeWithDataWord, true)]
    #[case(ModeCode::TransmitLastCommandWord, false)]
    #[case(ModeCode::TransmitBITWord, false)]
    #[case(ModeCode::SelectedTransmitterShutdown, true)]
    #[case(ModeCode::OverrideSelectedTransmitterShutdown, true)]
    #[case(ModeCode::UnknownModeCode(0b11111), false)]
    fn test_mode_code_is_broadcast(#[case] code: ModeCode, #[case] expected: bool) {
        assert_eq!(code.is_broadcast(), expected);
    }

    #[rstest]
    #[case(ModeCode::DynamicBusControl, false)]
    #[case(ModeCode::Synchronize, false)]
    #[case(ModeCode::TransmitStatusWord, false)]
    #[case(ModeCode::InitiateSelfTest, false)]
    #[case(ModeCode::TransmitterShutdown, false)]
    #[case(ModeCode::OverrideTransmitterShutdown, false)]
    #[case(ModeCode::InhibitTerminalFlagBit, false)]
    #[case(ModeCode::OverrideInhibitTerminalFlagBit, false)]
    #[case(ModeCode::ResetRemoteTerminal, false)]
    #[case(ModeCode::TransmitVectorWord, false)]
    #[case(ModeCode::SynchronizeWithDataWord, false)]
    #[case(ModeCode::TransmitLastCommandWord, false)]
    #[case(ModeCode::TransmitBITWord, false)]
    #[case(ModeCode::SelectedTransmitterShutdown, false)]
    #[case(ModeCode::OverrideSelectedTransmitterShutdown, false)]
    #[case(ModeCode::UnknownModeCode(0b11111), true)]
    fn test_mode_code_is_unknown(#[case] code: ModeCode, #[case] expected: bool) {
        assert_eq!(code.is_unknown(), expected);
    }

    #[rstest]
    #[case(0b00000, ModeCode::DynamicBusControl)]
    #[case(0b00001, ModeCode::Synchronize)]
    #[case(0b00010, ModeCode::TransmitStatusWord)]
    #[case(0b00011, ModeCode::InitiateSelfTest)]
    #[case(0b00100, ModeCode::TransmitterShutdown)]
    #[case(0b00101, ModeCode::OverrideTransmitterShutdown)]
    #[case(0b00110, ModeCode::InhibitTerminalFlagBit)]
    #[case(0b00111, ModeCode::OverrideInhibitTerminalFlagBit)]
    #[case(0b01000, ModeCode::ResetRemoteTerminal)]
    #[case(0b10000, ModeCode::TransmitVectorWord)]
    #[case(0b10001, ModeCode::SynchronizeWithDataWord)]
    #[case(0b10010, ModeCode::TransmitLastCommandWord)]
    #[case(0b10011, ModeCode::TransmitBITWord)]
    #[case(0b10100, ModeCode::SelectedTransmitterShutdown)]
    #[case(0b10101, ModeCode::OverrideSelectedTransmitterShutdown)]
    #[case(0b11111, ModeCode::UnknownModeCode(0b11111))]
    fn test_mode_code_from_u8(#[case] value: u8, #[case] expected: ModeCode) {
        assert_eq!(ModeCode::from(value), expected);
    }

    #[rstest]
    #[case(0b00000, ModeCode::DynamicBusControl)]
    #[case(0b00001, ModeCode::Synchronize)]
    #[case(0b00010, ModeCode::TransmitStatusWord)]
    #[case(0b00011, ModeCode::InitiateSelfTest)]
    #[case(0b00100, ModeCode::TransmitterShutdown)]
    #[case(0b00101, ModeCode::OverrideTransmitterShutdown)]
    #[case(0b00110, ModeCode::InhibitTerminalFlagBit)]
    #[case(0b00111, ModeCode::OverrideInhibitTerminalFlagBit)]
    #[case(0b01000, ModeCode::ResetRemoteTerminal)]
    #[case(0b10000, ModeCode::TransmitVectorWord)]
    #[case(0b10001, ModeCode::SynchronizeWithDataWord)]
    #[case(0b10010, ModeCode::TransmitLastCommandWord)]
    #[case(0b10011, ModeCode::TransmitBITWord)]
    #[case(0b10100, ModeCode::SelectedTransmitterShutdown)]
    #[case(0b10101, ModeCode::OverrideSelectedTransmitterShutdown)]
    #[case(0b11111, ModeCode::UnknownModeCode(0b11111))]
    fn test_mode_code_to_u8(#[case] expected: u8, #[case] code: ModeCode) {
        assert_eq!(u8::from(code), expected);
    }

    #[test]
    fn test_transmit_receive_clone() {
        let item1 = TransmitReceive::Transmit;
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[rstest]
    #[case(TransmitReceive::Receive, false)]
    #[case(TransmitReceive::Transmit, true)]
    fn test_transmit_receive_is_transmit(#[case] code: TransmitReceive, #[case] expected: bool) {
        assert_eq!(code.is_transmit(), expected);
    }

    #[rstest]
    #[case(TransmitReceive::Receive, true)]
    #[case(TransmitReceive::Transmit, false)]
    fn test_transmit_receive_is_receive(#[case] code: TransmitReceive, #[case] expected: bool) {
        assert_eq!(code.is_receive(), expected);
    }

    #[rstest]
    #[case(TransmitReceive::Receive, 0)]
    #[case(TransmitReceive::Transmit, 1)]
    fn test_transmit_receive_from_u8(#[case] expected: TransmitReceive, #[case] input: u8) {
        assert_eq!(TransmitReceive::from(input), expected);
    }

    #[rstest]
    #[case(TransmitReceive::Receive, 0)]
    #[case(TransmitReceive::Transmit, 1)]
    fn test_transmit_receive_to_u8(#[case] input: TransmitReceive, #[case] expected: u8) {
        assert_eq!(u8::from(input), expected);
    }

    #[test]
    fn test_address_clone() {
        let item1 = Address::Broadcast(0b11111);
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[rstest]
    #[case(Address::Value(0b10101), true)]
    #[case(Address::Unknown(0b1111111), false)]
    #[case(Address::Broadcast(0b11111), false)]
    fn test_address_is_value(#[case] input: Address, #[case] expected: bool) {
        assert_eq!(input.is_value(), expected);
    }

    #[rstest]
    #[case(Address::Value(0b10101), false)]
    #[case(Address::Unknown(0b1111111), true)]
    #[case(Address::Broadcast(0b11111), false)]
    fn test_address_is_unknown(#[case] input: Address, #[case] expected: bool) {
        assert_eq!(input.is_unknown(), expected);
    }

    #[rstest]
    #[case(Address::Value(0b10101), false)]
    #[case(Address::Unknown(0b1111111), false)]
    #[case(Address::Broadcast(0b11111), true)]
    fn test_address_is_broadcast(#[case] input: Address, #[case] expected: bool) {
        assert_eq!(input.is_broadcast(), expected);
    }

    #[rstest]
    #[case(Address::Value(0b10101), 0b10101)]
    #[case(Address::Unknown(0b1111111), 0b1111111)]
    #[case(Address::Broadcast(0b11111), 0b11111)]
    fn test_address_to_u8(#[case] input: Address, #[case] expected: u8) {
        assert_eq!(u8::from(input), expected);
    }

    #[rstest]
    #[case(Address::Value(0b10101), 0b10101)]
    #[case(Address::Unknown(0b1111111), 0b1111111)]
    #[case(Address::Broadcast(0b11111), 0b11111)]
    fn test_address_from_u8(#[case] expected: Address, #[case] input: u8) {
        assert_eq!(Address::from(input), expected);
    }

    #[test]
    fn test_subaddress_clone() {
        let item1 = SubAddress::ModeCode(0b11111);
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[rstest]
    #[case(SubAddress::Value(0b10101), true)]
    #[case(SubAddress::Unknown(0b1111111), false)]
    #[case(SubAddress::ModeCode(0b11111), false)]
    #[case(SubAddress::ModeCode(0b00000), false)]
    fn test_subaddress_is_value(#[case] input: SubAddress, #[case] expected: bool) {
        assert_eq!(input.is_value(), expected);
    }

    #[rstest]
    #[case(SubAddress::Value(0b10101), false)]
    #[case(SubAddress::Unknown(0b1111111), true)]
    #[case(SubAddress::ModeCode(0b11111), false)]
    #[case(SubAddress::ModeCode(0b00000), false)]
    fn test_subaddress_is_unknown(#[case] input: SubAddress, #[case] expected: bool) {
        assert_eq!(input.is_unknown(), expected);
    }

    #[rstest]
    #[case(SubAddress::Value(0b10101), false)]
    #[case(SubAddress::Unknown(0b1111111), false)]
    #[case(SubAddress::ModeCode(0b11111), true)]
    #[case(SubAddress::ModeCode(0b00000), true)]
    fn test_subaddress_is_mode_code(#[case] input: SubAddress, #[case] expected: bool) {
        assert_eq!(input.is_mode_code(), expected);
    }

    #[rstest]
    #[case(SubAddress::Value(0b10101), 0b10101)]
    #[case(SubAddress::Unknown(0b1111111), 0b1111111)]
    #[case(SubAddress::ModeCode(0b11111), 0b11111)]
    #[case(SubAddress::ModeCode(0b00000), 0b00000)]
    fn test_subaddress_to_u8(#[case] input: SubAddress, #[case] expected: u8) {
        assert_eq!(u8::from(input), expected);
    }

    #[rstest]
    #[case(SubAddress::Value(0b10101), 0b10101)]
    #[case(SubAddress::Unknown(0b1111111), 0b1111111)]
    #[case(SubAddress::ModeCode(0b11111), 0b11111)]
    #[case(SubAddress::ModeCode(0b00000), 0b00000)]
    fn test_subaddress_from_u8(#[case] expected: SubAddress, #[case] input: u8) {
        assert_eq!(SubAddress::from(input), expected);
    }

    #[test]
    fn test_instrumentation_clone() {
        let item1 = Instrumentation::Command;
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[rstest]
    #[case(Instrumentation::Status, true)]
    #[case(Instrumentation::Command, false)]
    fn test_instrumentation_is_status(#[case] input: Instrumentation, #[case] expected: bool) {
        assert_eq!(input.is_status(), expected);
    }

    #[rstest]
    #[case(Instrumentation::Status, false)]
    #[case(Instrumentation::Command, true)]
    fn test_instrumentation_is_command(#[case] input: Instrumentation, #[case] expected: bool) {
        assert_eq!(input.is_command(), expected);
    }

    #[rstest]
    #[case(Instrumentation::Status, 0)]
    #[case(Instrumentation::Command, 1)]
    fn test_instrumentation_from_u8(#[case] expected: Instrumentation, #[case] input: u8) {
        assert_eq!(Instrumentation::from(input), expected);
    }

    #[rstest]
    #[case(Instrumentation::Status, 0)]
    #[case(Instrumentation::Command, 1)]
    fn test_instrumentation_to_u8(#[case] input: Instrumentation, #[case] expected: u8) {
        assert_eq!(u8::from(input), expected);
    }

    #[test]
    fn test_service_request_clone() {
        let item1 = ServiceRequest::Service;
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[rstest]
    #[case(ServiceRequest::NoService, true)]
    #[case(ServiceRequest::Service, false)]
    fn test_service_request_is_noservice(#[case] input: ServiceRequest, #[case] expected: bool) {
        assert_eq!(input.is_noservice(), expected);
    }

    #[rstest]
    #[case(ServiceRequest::NoService, false)]
    #[case(ServiceRequest::Service, true)]
    fn test_service_request_is_service(#[case] input: ServiceRequest, #[case] expected: bool) {
        assert_eq!(input.is_service(), expected);
    }

    #[rstest]
    #[case(ServiceRequest::NoService, 0)]
    #[case(ServiceRequest::Service, 1)]
    fn test_service_request_from_u8(#[case] expected: ServiceRequest, #[case] input: u8) {
        assert_eq!(ServiceRequest::from(input), expected);
    }

    #[rstest]
    #[case(ServiceRequest::NoService, 0)]
    #[case(ServiceRequest::Service, 1)]
    fn test_service_request_to_u8(#[case] input: ServiceRequest, #[case] expected: u8) {
        assert_eq!(u8::from(input), expected);
    }

    #[test]
    fn test_reserved_clone() {
        let item1 = Reserved::Value(0b111);
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[rstest]
    #[case(Reserved::None, true)]
    #[case(Reserved::Value(0b111), false)]
    fn test_reserved_is_none(#[case] input: Reserved, #[case] expected: bool) {
        assert_eq!(input.is_none(), expected);
    }

    #[rstest]
    #[case(Reserved::None, false)]
    #[case(Reserved::Value(0b111), true)]
    fn test_reserved_is_value(#[case] input: Reserved, #[case] expected: bool) {
        assert_eq!(input.is_value(), expected);
    }

    #[rstest]
    #[case(Reserved::None, 0)]
    #[case(Reserved::Value(0b111), 0b111)]
    fn test_reserved_from_u8(#[case] expected: Reserved, #[case] input: u8) {
        assert_eq!(Reserved::from(input), expected);
    }

    #[rstest]
    #[case(Reserved::None, 0)]
    #[case(Reserved::Value(0b111), 0b111)]
    fn test_reserved_to_u8(#[case] input: Reserved, #[case] expected: u8) {
        assert_eq!(u8::from(input), expected);
    }

    #[test]
    fn test_broadcast_received_clone() {
        let item1 = BroadcastReceived::Received;
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[rstest]
    #[case(BroadcastReceived::NotReceived, true)]
    #[case(BroadcastReceived::Received, false)]
    fn test_broadcast_received_is_notreceived(
        #[case] input: BroadcastReceived,
        #[case] expected: bool,
    ) {
        assert_eq!(input.is_notreceived(), expected);
    }

    #[rstest]
    #[case(BroadcastReceived::NotReceived, false)]
    #[case(BroadcastReceived::Received, true)]
    fn test_broadcast_received_is_received(
        #[case] input: BroadcastReceived,
        #[case] expected: bool,
    ) {
        assert_eq!(input.is_received(), expected);
    }

    #[rstest]
    #[case(BroadcastReceived::NotReceived, 0)]
    #[case(BroadcastReceived::Received, 1)]
    fn test_broadcast_received_from_u8(#[case] expected: BroadcastReceived, #[case] input: u8) {
        assert_eq!(BroadcastReceived::from(input), expected);
    }

    #[rstest]
    #[case(BroadcastReceived::NotReceived, 0)]
    #[case(BroadcastReceived::Received, 1)]
    fn test_broadcast_received_to_u8(#[case] input: BroadcastReceived, #[case] expected: u8) {
        assert_eq!(u8::from(input), expected);
    }

    #[test]
    fn test_terminal_busy_clone() {
        let item1 = TerminalBusy::Busy;
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[rstest]
    #[case(TerminalBusy::NotBusy, true)]
    #[case(TerminalBusy::Busy, false)]
    fn test_terminal_busy_is_notbusy(#[case] input: TerminalBusy, #[case] expected: bool) {
        assert_eq!(input.is_notbusy(), expected);
    }

    #[rstest]
    #[case(TerminalBusy::NotBusy, false)]
    #[case(TerminalBusy::Busy, true)]
    fn test_terminal_busy_is_busy(#[case] input: TerminalBusy, #[case] expected: bool) {
        assert_eq!(input.is_busy(), expected);
    }

    #[rstest]
    #[case(TerminalBusy::NotBusy, 0)]
    #[case(TerminalBusy::Busy, 1)]
    fn test_terminal_busy_from_u8(#[case] expected: TerminalBusy, #[case] input: u8) {
        assert_eq!(TerminalBusy::from(input), expected);
    }

    #[rstest]
    #[case(TerminalBusy::NotBusy, 0)]
    #[case(TerminalBusy::Busy, 1)]
    fn test_terminal_busy_to_u8(#[case] input: TerminalBusy, #[case] expected: u8) {
        assert_eq!(u8::from(input), expected);
    }

    #[test]
    fn test_dynamic_bus_acceptance_clone() {
        let item1 = DynamicBusAcceptance::Accepted;
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[rstest]
    #[case(DynamicBusAcceptance::NotAccepted, true)]
    #[case(DynamicBusAcceptance::Accepted, false)]
    fn test_dynamic_bus_acceptance_is_notaccepted(
        #[case] input: DynamicBusAcceptance,
        #[case] expected: bool,
    ) {
        assert_eq!(input.is_notaccepted(), expected);
    }

    #[rstest]
    #[case(DynamicBusAcceptance::NotAccepted, false)]
    #[case(DynamicBusAcceptance::Accepted, true)]
    fn test_dynamic_bus_acceptance_is_accepted(
        #[case] input: DynamicBusAcceptance,
        #[case] expected: bool,
    ) {
        assert_eq!(input.is_accepted(), expected);
    }

    #[rstest]
    #[case(DynamicBusAcceptance::NotAccepted, 0)]
    #[case(DynamicBusAcceptance::Accepted, 1)]
    fn test_dynamic_bus_acceptance_from_u8(
        #[case] expected: DynamicBusAcceptance,
        #[case] input: u8,
    ) {
        assert_eq!(DynamicBusAcceptance::from(input), expected);
    }

    #[rstest]
    #[case(DynamicBusAcceptance::NotAccepted, 0)]
    #[case(DynamicBusAcceptance::Accepted, 1)]
    fn test_dynamic_bus_acceptance_to_u8(
        #[case] input: DynamicBusAcceptance,
        #[case] expected: u8,
    ) {
        assert_eq!(u8::from(input), expected);
    }
}
