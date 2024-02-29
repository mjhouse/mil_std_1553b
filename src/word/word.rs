use crate::errors::{MessageError, SubsystemError, TerminalError};
use crate::fields::*;
use crate::flags::*;

use super::Parity;
use super::Word;

impl_word!(CommandWord);
impl_word!(StatusWord);
impl_word!(DataWord);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandWord {
    data: u16,
    parity: Parity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusWord {
    data: u16,
    parity: Parity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataWord {
    data: u16,
    parity: Parity,
}

impl CommandWord {
    /// Create a new command word given the data
    ///
    /// The complete word is made up of 3 sync bits, 16 data
    /// bits, and a single parity bit. The sync bits are
    /// evaluated prior to word construction, the data bits
    /// are passed in here, and the parity is calculated from
    /// the given data.
    ///
    /// If the parity calculated here is not equal to the
    /// parity extracted from an incoming message, then this
    /// word is in error.
    pub fn new(data: u16) -> Self {
        Self {
            data,
            parity: Parity::from(&data),
        }
    }

    /// Get the terminal address of this word
    ///
    /// See [Address](crate::flags::Address) for more information
    /// about this field.
    pub fn address(&self) -> Address {
        Address::from(COMMAND_TERMINAL_ADDRESS_FIELD.get(self.data()))
    }

    /// Set the terminal address of this word
    ///
    /// See [CommandWord::address] for
    /// more information.
    pub fn set_address(&mut self, value: Address) {
        self.data = COMMAND_TERMINAL_ADDRESS_FIELD.set(self.data(), value.into());
    }

    /// Get the subaddress of this word
    ///
    /// Values of 0b00000 and 0b11111 are interpreted as a flag that
    /// indicates that this command is carrying a mode code value.
    ///
    /// See [SubAddress](crate::flags::SubAddress) for more information
    /// about this field.
    pub fn subaddress(&self) -> SubAddress {
        SubAddress::from(COMMAND_SUBADDRESS_FIELD.get(self.data()))
    }

    /// Set the subaddress of this word
    ///
    /// See [CommandWord::subaddress] for
    /// more information.
    pub fn set_subaddress(&mut self, value: SubAddress) {
        self.data = COMMAND_SUBADDRESS_FIELD.set(self.data(), value.into());
    }

    /// Get the direction of transmission
    ///
    /// See [Direction](crate::flags::Direction) for more information
    /// about this field.
    pub fn transmit_receive(&self) -> TransmitReceive {
        TransmitReceive::from(COMMAND_TRANSMIT_RECEIVE_FIELD.get(self.data()))
    }

    /// Set the direction of transmission
    ///
    /// See [CommandWord::transmit_receive] for more information
    /// about this field.
    pub fn set_transmit_receive(&mut self, value: TransmitReceive) {
        self.data = COMMAND_TRANSMIT_RECEIVE_FIELD.set(self.data(), value.into());
    }

    /// Get the mode code of this word
    ///
    /// This field is `None` if the subaddress isn't set to the ModeCode value.
    /// See [ModeCode](crate::flags::ModeCode) for more information about
    /// this field, or [SubAddress](crate::flags::SubAddress) for details about
    /// the ModeCode setting of the subaddress.
    pub fn mode_code(&self) -> Option<ModeCode> {
        if self.is_mode_code() {
            Some(ModeCode::from(COMMAND_MODE_CODE_FIELD.get(self.data())))
        } else {
            None
        }
    }

    /// Set the mode code of this word
    ///
    /// This method will do nothing if the subaddress is not set to the ModeCode
    /// value. See [CommandWord::mode_code] for more information.
    pub fn set_mode_code(&mut self, value: ModeCode) {
        if self.is_mode_code() {
            self.data = COMMAND_MODE_CODE_FIELD.set(self.data(), value.into());
        }
    }

    /// Get the number of data words associated with this word
    ///
    /// This field is `None` if the subaddress is set to the ModeCode value.
    /// See [SubAddress](crate::flags::SubAddress) for details about
    /// the ModeCode setting of the subaddress.
    pub fn word_count(&self) -> Option<u8> {
        if !self.is_mode_code() {
            match COMMAND_WORD_COUNT_FIELD.get(self.data()) {
                0 => Some(32),
                k => Some(k),
            }
        } else {
            None
        }
    }

    /// Set the number of words associated with this word
    ///
    /// This method will do nothing if the subaddress is set to the ModeCode
    /// value. See [CommandWord::word_count] for more information.
    pub fn set_word_count(&mut self, value: u8) {
        if !self.is_mode_code() {
            self.data = COMMAND_WORD_COUNT_FIELD.set(self.data(), value.into());
        }
    }

    /// Check if this word contains a mode code value
    ///
    /// See [CommandWord::mode_code]
    /// for more information.
    pub fn is_mode_code(&self) -> bool {
        self.subaddress().is_mode_code()
    }

    /// Check if this word is being transmitted to a terminal
    ///
    /// See [CommandWord::transmit_receive]
    /// for more information.
    pub fn is_transmit(&self) -> bool {
        self.transmit_receive().is_transmit()
    }

    /// Check if this word is being received by a terminal
    ///
    /// See [CommandWord::transmit_receive]
    /// for more information.
    pub fn is_receive(&self) -> bool {
        self.transmit_receive().is_receive()
    }

    /// Get the word count or 0 if word is a mode code command
    pub fn count(&self) -> usize {
        self.word_count().unwrap_or(0) as usize
    }

    /// Convert the word into a byte array
    pub fn bytes(&self) -> [u8;2] {
        [
            (self.data >> 8) as u8,
            (self.data >> 0) as u8,
        ]
    }
}

impl StatusWord {
    /// Create a new status word given the data
    ///
    /// The complete word is made up of 3 sync bits, 16 data
    /// bits, and a single parity bit. The sync bits are
    /// evaluated prior to word construction, the data bits
    /// are passed in here, and the parity is calculated from
    /// the given data.
    ///
    /// If the parity calculated here is not equal to the
    /// parity extracted from an incoming message, then this
    /// word is in error.
    pub fn new(data: u16) -> Self {
        Self {
            data,
            parity: Parity::from(&data),
        }
    }

    /// Get the terminal address of this word
    ///
    /// See [Address](crate::flags::Address) for more information
    /// about this field.
    pub fn address(&self) -> Address {
        Address::from(STATUS_TERMINAL_ADDRESS_FIELD.get(self.data()))
    }

    /// Set the terminal address of this word
    ///
    /// See [StatusWord::address] for
    /// more information.
    pub fn set_address(&mut self, value: Address) {
        self.data = STATUS_TERMINAL_ADDRESS_FIELD.set(self.data(), value.into());
    }

    /// Get Instrumentation flag of the status word
    ///
    /// See [Instrumentation](crate::flags::Instrumentation) for
    /// more information.
    pub fn instrumentation(&self) -> Instrumentation {
        Instrumentation::from(STATUS_INSTRUMENTATION_FIELD.get(self.data()))
    }

    /// Set Instrumentation flag of the status word
    ///
    /// See [StatusWord::instrumentation] for
    /// more information.
    pub fn set_instrumentation(&mut self, value: Instrumentation) {
        self.data = STATUS_INSTRUMENTATION_FIELD.set(self.data(), value.into());
    }

    /// Get Service Request flag of the status word
    ///
    /// See [ServiceRequest](crate::flags::ServiceRequest) for
    /// more information.
    pub fn service_request(&self) -> ServiceRequest {
        ServiceRequest::from(STATUS_SERVICE_REQUEST_FIELD.get(self.data()))
    }

    /// Set Service Request flag of the status word
    ///
    /// See [StatusWord::service_request] for
    /// more information.
    pub fn set_service_request(&mut self, value: ServiceRequest) {
        self.data = STATUS_SERVICE_REQUEST_FIELD.set(self.data(), value.into());
    }

    /// Get the value of the reserved portion of the status word
    ///
    /// See [Reserved](crate::flags::Reserved) for
    /// more information.
    pub fn reserved(&self) -> Reserved {
        Reserved::from(STATUS_RESERVED_BITS_FIELD.get(self.data()))
    }

    /// Set the value of the reserved portion of the status word
    ///
    /// See [StatusWord::reserved] for
    /// more information.
    pub fn set_reserved(&mut self, value: Reserved) {
        self.data = STATUS_RESERVED_BITS_FIELD.set(self.data(), value.into());
    }

    /// Get the Broadcast Command flag from the status word
    ///
    /// If set, the flag indicates that the terminal has received a valid
    /// broadcast command. See [BroadcastCommand](crate::flags::BroadcastCommand) for
    /// more information.
    pub fn broadcast_received(&self) -> BroadcastCommand {
        BroadcastCommand::from(STATUS_BROADCAST_RECEIVED_FIELD.get(self.data()))
    }

    /// Set the Broadcast Command flag from the status word
    ///
    /// See [StatusWord::broadcast_received] for
    /// more information.
    pub fn set_broadcast_received(&mut self, value: BroadcastCommand) {
        self.data = STATUS_BROADCAST_RECEIVED_FIELD.set(self.data(), value.into());
    }

    /// Get the Busy flag from the status word
    ///
    /// If set, the flag indicates that the terminal is unable to respond to
    /// commands at this time. See [TerminalBusy](crate::flags::TerminalBusy) for
    /// more information.
    pub fn terminal_busy(&self) -> TerminalBusy {
        TerminalBusy::from(STATUS_TERMINAL_BUSY_FIELD.get(self.data()))
    }

    /// Set the Busy flag on the status word
    ///
    /// See [StatusWord::terminal_busy] for
    /// more information.
    pub fn set_terminal_busy(&mut self, value: TerminalBusy) {
        self.data = STATUS_TERMINAL_BUSY_FIELD.set(self.data(), value.into());
    }

    /// Get the Dynamic Bus Control Acceptance flag from the status word
    ///
    /// If set, the flag indicates that the terminal is taking control
    /// of the bus. See [BusControlAccept](crate::flags::BusControlAccept) for
    /// more information.
    pub fn dynamic_bus_acceptance(&self) -> BusControlAccept {
        BusControlAccept::from(STATUS_DYNAMIC_BUS_ACCEPT_FIELD.get(self.data()))
    }

    /// Set the Dynamic Bus Control Acceptance flag on the status word
    ///
    /// See [StatusWord::dynamic_bus_acceptance] for
    /// more information.
    pub fn set_dynamic_bus_acceptance(&mut self, value: BusControlAccept) {
        self.data = STATUS_DYNAMIC_BUS_ACCEPT_FIELD.set(self.data(), value.into());
    }

    /// Check if the message error flag is set
    ///
    /// See [MessageError](crate::errors::MessageError) for more
    /// information.
    pub fn message_error(&self) -> MessageError {
        MessageError::from(STATUS_MESSAGE_ERROR_FIELD.get(self.data()))
    }

    /// Set the message error flag on this word
    ///
    /// See [StatusWord::message_error] for
    /// more information.
    pub fn set_message_error(&mut self, value: MessageError) {
        self.data = STATUS_MESSAGE_ERROR_FIELD.set(self.data(), value.into());
    }

    /// Check if the subsystem error flag is set
    ///
    /// See [SubsystemError](crate::errors::SubsystemError) for more
    /// information.
    pub fn subsystem_error(&self) -> SubsystemError {
        SubsystemError::from(STATUS_SUBSYSTEM_FLAG_FIELD.get(self.data()))
    }

    /// Set the subsystem error flag on this word
    ///
    /// See [StatusWord::subsystem_error] for
    /// more information.
    pub fn set_subsystem_error(&mut self, value: SubsystemError) {
        self.data = STATUS_SUBSYSTEM_FLAG_FIELD.set(self.data(), value.into());
    }

    /// Check if the terminal error flag is set
    ///
    /// See [TerminalError](crate::errors::TerminalError) for more
    /// information.
    pub fn terminal_error(&self) -> TerminalError {
        TerminalError::from(STATUS_TERMINAL_FLAG_FIELD.get(self.data()))
    }

    /// Set the terminal error flag on this word
    ///
    /// See [StatusWord::terminal_error] for
    /// more information.
    pub fn set_terminal_error(&mut self, value: TerminalError) {
        self.data = STATUS_TERMINAL_FLAG_FIELD.set(self.data(), value.into());
    }

    /// Check if any of the various error flags are set
    ///
    /// See [StatusWord::message_error],
    /// [StatusWord::subsystem_error],
    /// or [StatusWord::terminal_error],
    /// for more information.
    pub fn is_error(&self) -> bool {
        self.message_error().is_error()
            || self.subsystem_error().is_error()
            || self.terminal_error().is_error()
    }

    /// Check if the terminal is currently busy
    ///
    /// See [StatusWord::terminal_busy] for
    /// more information.
    pub fn is_busy(&self) -> bool {
        self.terminal_busy().is_busy()
    }

    /// Check if the reserved field is being used
    ///
    /// See [StatusWord::reserved] for
    /// more information.
    pub fn is_valid(&self) -> bool {
        self.reserved().is_none()
    }

    /// Convert the word into a byte array
    pub fn bytes(&self) -> [u8;2] {
        [
            (self.data >> 8) as u8,
            (self.data >> 0) as u8,
        ]
    }
}

impl DataWord {
    /// Create a new data word given the data
    ///
    /// The complete word is made up of 3 sync bits, 16 data
    /// bits, and a single parity bit. The sync bits are
    /// evaluated prior to word construction, the data bits
    /// are passed in here, and the parity is calculated from
    /// the given data.
    ///
    /// If the parity calculated here is not equal to the
    /// parity extracted from an incoming message, then this
    /// word is in error.
    pub fn new(data: u16) -> Self {
        Self {
            data,
            parity: Parity::from(&data),
        }
    }

    /// Convert data to text
    pub fn as_text(&self) -> String {
        String::from_utf8_lossy(&self.bytes()).to_string()
    }

    /// Convert the word into a byte array
    pub fn bytes(&self) -> [u8;2] {
        [
            (self.data >> 8) as u8,
            (self.data >> 0) as u8,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_get_address() {
        let word = CommandWord::new(COMMAND_TERMINAL_ADDRESS);
        assert!(word.address().is_broadcast());
    }

    #[test]
    fn test_command_set_address() {
        let mut word = CommandWord::new(WORD_EMPTY);
        word.set_address(Address::Value(0b10101));
        assert_eq!(word.data(), 0b1010100000000000);
    }

    #[test]
    fn test_command_set_broadcast_address() {
        let mut word = CommandWord::new(WORD_EMPTY);
        word.set_address(Address::Value(0b11111));
        assert!(word.address().is_broadcast());
    }

    #[test]
    fn test_command_get_subaddress() {
        let word = CommandWord::new(COMMAND_SUBADDRESS);
        assert!(word.subaddress().is_mode_code());
    }

    #[test]
    fn test_command_set_subaddress() {
        let mut word = CommandWord::new(WORD_EMPTY);
        word.set_subaddress(SubAddress::Value(0b10101));
        assert_eq!(word.data(), 0b0000001010100000);
    }

    #[test]
    fn test_command_get_transmit_receive() {
        let word = CommandWord::new(WORD_EMPTY);
        assert!(word.transmit_receive().is_receive());

        let word = CommandWord::new(COMMAND_TRANSMIT_RECEIVE);
        assert!(word.transmit_receive().is_transmit());
    }

    #[test]
    fn test_command_set_transmit_receive() {
        let mut word = CommandWord::new(WORD_EMPTY);
        word.set_transmit_receive(TransmitReceive::Transmit);
        assert!(word.transmit_receive().is_transmit());
    }

    #[test]
    fn test_command_get_mode_code() {
        let word = CommandWord::new(WORD_EMPTY);
        assert_eq!(word.mode_code(), Some(ModeCode::DynamicBusControl));

        let word = CommandWord::new(COMMAND_MODE_CODE);
        assert_eq!(word.mode_code(), Some(ModeCode::UnknownModeCode(0b11111)));
    }

    #[test]
    fn test_command_set_mode_code() {
        let mut word = CommandWord::new(WORD_EMPTY);
        word.set_mode_code(ModeCode::OverrideTransmitterShutdown);
        assert_eq!(
            word.mode_code(),
            Some(ModeCode::OverrideTransmitterShutdown)
        );

        word.set_subaddress(SubAddress::Value(0b01010));
        assert!(word.mode_code().is_none());
    }

    #[test]
    fn test_command_get_word_count() {
        let address = SubAddress::Value(0b01010);

        // word count is none because subaddress is ModeCode
        let mut word = CommandWord::new(COMMAND_WORD_COUNT);
        assert!(word.word_count().is_none());

        // word count is 32 after subaddress changed
        word.set_subaddress(address);
        assert!(word.word_count().is_some());
        assert_eq!(word.word_count(), Some(31));
    }

    #[test]
    fn test_command_set_word_count() {
        let mut word = CommandWord::new(WORD_EMPTY);
        word.set_subaddress(SubAddress::Value(0b01010));

        word.set_word_count(0b10101);
        assert_eq!(word.word_count(), Some(0b10101));
    }

    #[test]
    fn test_command_is_mode_code() {
        let address = SubAddress::Value(0b01010);

        let mut word = CommandWord::new(WORD_EMPTY);
        assert!(word.is_mode_code());

        word.set_subaddress(address);
        assert!(!word.is_mode_code());
    }

    #[test]
    fn test_command_is_transmit() {
        let mut word = CommandWord::new(WORD_EMPTY);
        assert!(!word.is_transmit());

        word.set_transmit_receive(TransmitReceive::Transmit);
        assert!(word.is_transmit());
    }

    #[test]
    fn test_command_is_receive() {
        let mut word = CommandWord::new(WORD_EMPTY);
        assert!(word.is_receive());

        word.set_transmit_receive(TransmitReceive::Transmit);
        assert!(!word.is_receive());
    }

    #[test]
    fn test_status_get_address() {
        let word = StatusWord::new(STATUS_TERMINAL_ADDRESS);
        assert!(word.address().is_broadcast());
    }

    #[test]
    fn test_status_set_address() {
        let mut word = StatusWord::new(WORD_EMPTY);
        word.set_address(Address::Value(0b10101));
        assert_eq!(word.data(), 0b1010100000000000);
    }

    #[test]
    fn test_status_set_broadcast_address() {
        let mut word = StatusWord::new(WORD_EMPTY);
        word.set_address(Address::Value(0b11111));
        assert!(word.address().is_broadcast());
    }

    #[test]
    fn test_status_get_instrumentation() {
        let word = StatusWord::new(WORD_EMPTY);
        assert!(word.instrumentation().is_status());

        let word = StatusWord::new(STATUS_INSTRUMENTATION);
        assert!(word.instrumentation().is_command());
    }

    #[test]
    fn test_status_set_instrumentation() {
        let mut word = StatusWord::new(WORD_EMPTY);
        word.set_instrumentation(Instrumentation::Command);
        assert!(word.instrumentation().is_command());
    }

    #[test]
    fn test_status_get_service_request() {
        let word = StatusWord::new(WORD_EMPTY);
        assert!(word.service_request().is_noservice());

        let word = StatusWord::new(STATUS_SERVICE_REQUEST);
        assert!(word.service_request().is_service());
    }

    #[test]
    fn test_status_set_service_request() {
        let mut word = StatusWord::new(WORD_EMPTY);
        word.set_service_request(ServiceRequest::Service);
        assert!(word.service_request().is_service());
    }

    #[test]
    fn test_status_get_reserved() {
        let word = StatusWord::new(WORD_EMPTY);
        assert!(word.reserved().is_none());

        let word = StatusWord::new(STATUS_RESERVED_BITS);
        assert!(word.reserved().is_value());
    }

    #[test]
    fn test_status_set_reserved() {
        let mut word = StatusWord::new(WORD_EMPTY);
        word.set_reserved(Reserved::Value(0b111));
        assert!(word.reserved().is_value());
        assert!(!word.is_valid());
    }

    #[test]
    fn test_status_get_broadcast_received() {
        let word = StatusWord::new(WORD_EMPTY);
        assert!(word.broadcast_received().is_notreceived());

        let word = StatusWord::new(STATUS_BROADCAST_RECEIVED);
        assert!(word.broadcast_received().is_received());
    }

    #[test]
    fn test_status_set_broadcast_received() {
        let mut word = StatusWord::new(WORD_EMPTY);
        word.set_broadcast_received(BroadcastCommand::Received);
        assert!(word.broadcast_received().is_received());
    }

    #[test]
    fn test_status_get_terminal_busy() {
        let word = StatusWord::new(WORD_EMPTY);
        assert!(word.terminal_busy().is_notbusy());

        let word = StatusWord::new(STATUS_TERMINAL_BUSY);
        assert!(word.terminal_busy().is_busy());
        assert!(word.is_busy());
    }

    #[test]
    fn test_status_set_terminal_busy() {
        let mut word = StatusWord::new(WORD_EMPTY);
        word.set_terminal_busy(TerminalBusy::Busy);
        assert!(word.terminal_busy().is_busy());
        assert!(word.is_busy());
    }

    #[test]
    fn test_status_get_dynamic_bus_acceptance() {
        let word = StatusWord::new(WORD_EMPTY);
        assert!(word.dynamic_bus_acceptance().is_notaccepted());

        let word = StatusWord::new(STATUS_DYNAMIC_BUS_ACCEPT);
        assert!(word.dynamic_bus_acceptance().is_accepted());
    }

    #[test]
    fn test_status_set_dynamic_bus_acceptance() {
        let mut word = StatusWord::new(WORD_EMPTY);
        word.set_dynamic_bus_acceptance(BusControlAccept::Accepted);
        assert!(word.dynamic_bus_acceptance().is_accepted());
    }

    #[test]
    fn test_status_get_message_error() {
        let word = StatusWord::new(WORD_EMPTY);
        assert!(word.message_error().is_none());

        let word = StatusWord::new(STATUS_MESSAGE_ERROR);
        assert!(word.message_error().is_error());
        assert!(word.is_error());
    }

    #[test]
    fn test_status_set_message_error() {
        let mut word = StatusWord::new(WORD_EMPTY);
        word.set_message_error(MessageError::Error);
        assert!(word.message_error().is_error());
        assert!(word.is_error());
    }

    #[test]
    fn test_status_get_subsystem_error() {
        let word = StatusWord::new(WORD_EMPTY);
        assert!(word.subsystem_error().is_none());

        let word = StatusWord::new(STATUS_SUBSYSTEM_FLAG);
        assert!(word.subsystem_error().is_error());
        assert!(word.is_error());
    }

    #[test]
    fn test_status_set_subsystem_error() {
        let mut word = StatusWord::new(WORD_EMPTY);
        word.set_subsystem_error(SubsystemError::Error);
        assert!(word.subsystem_error().is_error());
        assert!(word.is_error());
    }

    #[test]
    fn test_status_get_terminal_error() {
        let word = StatusWord::new(WORD_EMPTY);
        assert!(word.terminal_error().is_none());

        let word = StatusWord::new(STATUS_TERMINAL_FLAG);
        assert!(word.terminal_error().is_error());
        assert!(word.is_error());
    }

    #[test]
    fn test_status_set_terminal_error() {
        let mut word = StatusWord::new(WORD_EMPTY);
        word.set_terminal_error(TerminalError::Error);
        assert!(word.terminal_error().is_error());
        assert!(word.is_error());
    }

    #[test]
    fn test_data_convert_text() {
        let word = DataWord::new(0b0110100001101001);
        let text = word.as_text();
        assert_eq!(text,"hi");
    }

    #[test]
    fn test_data_bytes() {
        let word = DataWord::new(0b0110100001101001);
        let data = word.bytes();
        assert_eq!(data,[0b01101000, 0b01101001]);
    }

    #[test]
    fn test_command_bytes() {
        let word = CommandWord::new(0b0110100001101001);
        let data = word.bytes();
        assert_eq!(data,[0b01101000, 0b01101001]);
    }

    #[test]
    fn test_status_bytes() {
        let word = StatusWord::new(0b0110100001101001);
        let data = word.bytes();
        assert_eq!(data,[0b01101000, 0b01101001]);
    }
}
