use crate::errors::{parity, Error, MessageError, Result, SubsystemError, TerminalError};
use crate::flags::*;
use crate::{fields::*, WordType};

/// Common functionality for service words
pub trait Header
where
    Self: Sized + Into<WordType>,
{
    /// The number of data words expected
    fn count(&self) -> Option<usize>;
}

/// Common functionality for all words
pub trait Word
where
    Self: Sized + Into<WordType>,
{
    /// Create an empty word
    fn new() -> Self;

    /// Constructor method to set the word from a u16
    fn with_value(self, data: u16) -> Self;

    /// Constructor method to set the word from bytes
    fn with_bytes(self, data: [u8; 2]) -> Self;

    /// Constructor method to explicitly set the parity
    fn with_parity(self, parity: u8) -> Self;

    /// Constructor method to calculate a parity bit
    fn with_calculated_parity(self) -> Self;

    /// Finish and validate construction of a word
    fn build(self) -> Result<Self>;

    /// Create a word from a u16
    fn from_value(data: u16) -> Self;

    /// Create a word from two bytes
    fn from_bytes(data: [u8; 2]) -> Self;

    /// Get the internal data as a slice
    fn as_bytes(&self) -> [u8; 2];

    /// Get the internal data as u16
    fn as_value(&self) -> u16;

    /// Set the internal data as a slice
    fn set_bytes(&mut self, data: [u8; 2]);

    /// Set the internal data as u16
    fn set_value(&mut self, data: u16);

    /// Get the current parity bit
    fn parity(&self) -> u8;

    /// Set the current parity bit
    fn set_parity(&mut self, parity: u8);

    /// Get a calculated parity bit
    fn calculate_parity(&self) -> u8;

    /// Check if the current parity bit is correct
    fn check_parity(&self) -> bool;
}

/// Specifies the function that a remote terminal is to perform
///
/// This word is parsed from a packet that includes an initial service
/// sync flag. Only the active bus controller emits this word.[^1]
///
/// ## Example
///
/// ```rust
/// # use mil_std_1553b::*;
/// # fn try_main() -> Result<()> {
/// let word = CommandWord::new()
///     .with_address(Address::Value(16))
///     .with_subaddress(SubAddress::ModeCode(0))
///     .with_transmit_receive(TransmitReceive::Receive)
///     .with_mode_code(ModeCode::TransmitterShutdown)
///     .with_calculated_parity()
///     .build()?;
///
/// assert_eq!(word.subaddress(),SubAddress::ModeCode(0));
/// assert_eq!(word.mode_code(),ModeCode::TransmitterShutdown);
/// # Ok(())
/// # }
/// ```
///
/// [^1]: p30 [MIL-STD-1553 Tutorial](http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandWord {
    /// Data of the word
    data: [u8; 2],

    /// Parity of the word
    parity: u8,
}

/// Sent in response to a valid message from the bus controller
///
/// This word is parsed from a packet that includes an initial service
/// sync flag. Status words are only transmitted by a remote terminal
/// in response to a message from the bus controller.[^1]
///
/// ## Example
///
/// ```rust
/// # use mil_std_1553b::*;
/// # fn try_main() -> Result<()> {
/// let word = StatusWord::new()
///     .with_address(Address::Value(16))
///     .with_service_request(ServiceRequest::Service)
///     .with_broadcast_received(BroadcastReceived::Received)
///     .with_calculated_parity()
///     .build()?;
///
/// assert_eq!(word.broadcast_received(),BroadcastReceived::Received);
/// assert_eq!(word.service_request(),ServiceRequest::Service);
/// # Ok(())
/// # }
/// ```
///
/// [^1]: p31 [MIL-STD-1553 Tutorial](http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusWord {
    /// Data of the word
    data: [u8; 2],

    /// Parity of the word
    parity: u8,
}

/// Contains data that is being transmitted in a message.
///
/// This word is parsed from a packet that includes an initial data
/// sync flag. Data words can be transmitted by either a remote terminal
/// (transmit command) or a bus controller (receive command) and contain
/// two bytes of data each.[^1]
///
/// ## Example
///
/// ```rust
/// # use mil_std_1553b::*;
/// # fn try_main() -> Result<()> {
/// let word = DataWord::new()
///     .with_value(0b0100100001001001u16)
///     .with_calculated_parity()
///     .build()?;
///
/// assert_eq!(word.as_string(),Ok("HI"));
/// # Ok(())
/// # }
/// ```
///
/// [^1]: p31 [MIL-STD-1553 Tutorial](http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataWord {
    /// Data of the word
    data: [u8; 2],

    /// Parity of the word
    parity: u8,
}

impl CommandWord {
    /// Get the terminal address of this word
    ///
    /// See [Address](crate::flags::Address) for more information
    /// about this field.
    pub fn address(&self) -> Address {
        COMMAND_ADDRESS_FIELD.get(self).into()
    }

    /// Set the terminal address of this word
    ///
    /// See [CommandWord::address] for
    /// more information.
    ///
    /// # Arguments
    ///
    /// * `value` - An [Address] to set
    ///
    pub fn set_address(&mut self, value: Address) {
        COMMAND_ADDRESS_FIELD.set(self, value.into());
    }

    /// Constructor method to set the terminal address
    ///
    /// See [CommandWord::address] for
    /// more information.
    ///
    /// # Arguments
    ///
    /// * `value` - An [Address] to set
    ///
    pub fn with_address(mut self, value: Address) -> Self {
        self.set_address(value);
        self
    }

    /// Get the subaddress of this word
    ///
    /// Values of 0b00000 and 0b11111 are interpreted as a flag that
    /// indicates that this command is carrying a mode code value.
    ///
    /// See [SubAddress](crate::flags::SubAddress) for more information
    /// about this field.
    pub fn subaddress(&self) -> SubAddress {
        COMMAND_SUBADDRESS_FIELD.get(self).into()
    }

    /// Set the subaddress of this word
    ///
    /// See [CommandWord::subaddress] for
    /// more information.
    ///
    /// # Arguments
    ///
    /// * `value` - A [SubAddress] to set
    ///
    pub fn set_subaddress(&mut self, value: SubAddress) {
        COMMAND_SUBADDRESS_FIELD.set(self, value.into());
    }

    /// Constructor method to set the subaddress
    ///
    /// See [CommandWord::subaddress] for
    /// more information.
    ///
    /// # Arguments
    ///
    /// * `value` - A [SubAddress] to set
    ///
    pub fn with_subaddress(mut self, value: SubAddress) -> Self {
        self.set_subaddress(value);
        self
    }

    /// Get the direction of transmission
    ///
    /// See [TransmitReceive](crate::flags::TransmitReceive) enum for
    /// more information about this field.
    pub fn transmit_receive(&self) -> TransmitReceive {
        COMMAND_TRANSMIT_RECEIVE_FIELD.get(self).into()
    }

    /// Set the direction of transmission
    ///
    /// See [TransmitReceive](crate::flags::TransmitReceive) enum for
    /// more information about this field.
    ///
    /// # Arguments
    ///
    /// * `value` - A [TransmitReceive] flag to set
    ///
    pub fn set_transmit_receive(&mut self, value: TransmitReceive) {
        COMMAND_TRANSMIT_RECEIVE_FIELD.set(self, value.into());
    }

    /// Constructor method to set the direction of transmission
    ///
    /// See [TransmitReceive](crate::flags::TransmitReceive) enum for
    /// more information about this field.
    ///
    /// # Arguments
    ///
    /// * `value` - A [TransmitReceive] flag to set
    ///
    pub fn with_transmit_receive(mut self, value: TransmitReceive) -> Self {
        self.set_transmit_receive(value);
        self
    }

    /// Get the mode code of this word
    ///
    /// This field should not be used if the subaddress isn't set to
    /// a ModeCode value. See [ModeCode](crate::flags::ModeCode) for
    /// more information about this field, or [SubAddress](crate::flags::SubAddress)
    /// for details about the ModeCode setting of the subaddress.
    pub fn mode_code(&self) -> ModeCode {
        COMMAND_MODE_CODE_FIELD.get(self).into()
    }

    /// Set the mode code of this word
    ///
    /// This method sets the same bits that are used by the word count field.
    /// In order to be valid, users must also set the subaddress to a valid
    /// mode code value. See [CommandWord::mode_code] for more information.
    ///
    /// # Arguments
    ///
    /// * `value` - A [ModeCode] flag to set
    ///
    pub fn set_mode_code(&mut self, value: ModeCode) {
        COMMAND_MODE_CODE_FIELD.set(self, value.into());
    }

    /// Constructor method to set the mode code
    ///
    /// This method sets the same bits that are used by the word count field.
    /// In order to be valid, users must also set the subaddress to a valid
    /// mode code value. See [CommandWord::mode_code] for more information.
    ///
    /// # Arguments
    ///
    /// * `value` - A [ModeCode] flag to set
    ///
    pub fn with_mode_code(mut self, value: ModeCode) -> Self {
        self.set_mode_code(value);
        self
    }

    /// Get the number of data words associated with this word
    ///
    /// The word count is a value between 0 and 32 in command words
    /// that are not mode code commands. Mode code commands are 
    /// identified using particular values of the subaddress field.
    /// 
    /// See [SubAddress](crate::flags::SubAddress) for details about
    /// the ModeCode setting of the subaddress.
    pub fn word_count(&self) -> u8 {
        match COMMAND_WORD_COUNT_FIELD.get(self) {
            0 => 32,
            k => k,
        }
    }

    /// Set the number of data words associated with this command
    ///
    /// The word count field is stored in the same bit position as the
    /// mode code value, and should only be used if the sub address is 
    /// *not* set to a ModeCode value. See [CommandWord::word_count] 
    /// for more information.
    ///
    /// Commands that are not mode code require some number of data
    /// words, and setting this field to `0` will indicate 32 data 
    /// words. Any value given greater than 31 will be converted to
    /// `0`.
    /// 
    /// # Arguments
    ///
    /// * `value` - A word count to set
    ///
    pub fn set_word_count(&mut self, mut value: u8) {
        if value > 31 { value = 0; }
        COMMAND_WORD_COUNT_FIELD.set(self, value);
    }

    /// Constructor method to set the number of data words
    ///
    /// See [CommandWord::set_word_count] for more information.
    ///
    /// # Arguments
    ///
    /// * `value` - A word count to set
    ///
    pub fn with_word_count(mut self, value: u8) -> Self {
        self.set_word_count(value);
        self
    }

    /// Check if this word contains a mode code value
    ///
    /// See [CommandWord::mode_code]
    /// for more information.
    #[must_use = "Returned value is not used"]
    pub fn is_mode_code(&self) -> bool {
        self.subaddress().is_mode_code()
    }

    /// Check if this word is being transmitted to a terminal
    ///
    /// See [CommandWord::transmit_receive]
    /// for more information.
    #[must_use = "Returned value is not used"]
    pub fn is_transmit(&self) -> bool {
        self.transmit_receive().is_transmit()
    }

    /// Check if this word is being received by a terminal
    ///
    /// See [CommandWord::transmit_receive]
    /// for more information.
    #[must_use = "Returned value is not used"]
    pub fn is_receive(&self) -> bool {
        self.transmit_receive().is_receive()
    }

    /// Get the data word count of the command word
    #[must_use = "Returned value is not used"]
    pub fn count(&self) -> usize {
        self.word_count() as usize
    }
}

impl StatusWord {
    /// Get the terminal address of this word
    ///
    /// See [Address](crate::flags::Address) for more information
    /// about this field.
    pub fn address(&self) -> Address {
        STATUS_ADDRESS_FIELD.get(self).into()
    }

    /// Set the terminal address of this word
    ///
    /// See [StatusWord::address] for
    /// more information.
    ///
    /// # Arguments
    ///
    /// * `value` - An [Address] to set
    ///
    pub fn set_address(&mut self, value: Address) {
        STATUS_ADDRESS_FIELD.set(self, value.into());
    }

    /// Constructor method to set the terminal address
    ///
    /// See [StatusWord::address] for
    /// more information.
    ///
    /// # Arguments
    ///
    /// * `value` - An [Address] to set
    ///
    pub fn with_address(mut self, value: Address) -> Self {
        self.set_address(value);
        self
    }

    /// Get Instrumentation flag of the status word
    ///
    /// **Most systems no longer use this flag, as the cost in reduced subaddress
    /// range is too high**.
    ///
    /// See [Instrumentation](crate::flags::Instrumentation) for
    /// more information.
    pub fn instrumentation(&self) -> Instrumentation {
        STATUS_INSTRUMENTATION_FIELD.get(self).into()
    }

    /// Set Instrumentation flag of the status word
    ///
    /// See [StatusWord::instrumentation] for
    /// more information.
    ///
    /// # Arguments
    ///
    /// * `value` - An [Instrumentation] flag to set
    ///
    pub fn set_instrumentation(&mut self, value: Instrumentation) {
        STATUS_INSTRUMENTATION_FIELD.set(self, value.into());
    }

    /// Constructor method to set Instrumentation flag
    ///
    /// See [StatusWord::instrumentation] for
    /// more information.
    ///
    /// # Arguments
    ///
    /// * `value` - An [Instrumentation] flag to set
    ///
    pub fn with_instrumentation(mut self, value: Instrumentation) -> Self {
        self.set_instrumentation(value);
        self
    }

    /// Get Service Request flag of the status word
    ///
    /// See [ServiceRequest](crate::flags::ServiceRequest) for
    /// more information.
    pub fn service_request(&self) -> ServiceRequest {
        STATUS_SERVICE_REQUEST_FIELD.get(self).into()
    }

    /// Set Service Request flag of the status word
    ///
    /// See [StatusWord::service_request] for
    /// more information.
    ///
    /// # Arguments
    ///
    /// * `value` - A [ServiceRequest] flag to set
    ///
    pub fn set_service_request(&mut self, value: ServiceRequest) {
        STATUS_SERVICE_REQUEST_FIELD.set(self, value.into());
    }

    /// Constructor method to set the Service Request flag
    ///
    /// See [StatusWord::service_request] for
    /// more information.
    ///
    /// # Arguments
    ///
    /// * `value` - A [ServiceRequest] flag to set
    ///
    pub fn with_service_request(mut self, value: ServiceRequest) -> Self {
        self.set_service_request(value);
        self
    }

    /// Get the value of the reserved portion of the status word
    ///
    /// See [Reserved](crate::flags::Reserved) for
    /// more information.
    pub fn reserved(&self) -> Reserved {
        STATUS_RESERVED_FIELD.get(self).into()
    }

    /// Set the value of the reserved portion of the status word
    ///
    /// See [StatusWord::reserved] for
    /// more information.
    ///
    /// # Arguments
    ///
    /// * `value` - A [Reserved] value to set
    ///
    pub fn set_reserved(&mut self, value: Reserved) {
        STATUS_RESERVED_FIELD.set(self, value.into());
    }

    /// Constructor method to set the value of the reserved field
    ///
    /// See [StatusWord::reserved] for
    /// more information.
    ///
    /// # Arguments
    ///
    /// * `value` - A [Reserved] value to set
    ///
    pub fn with_reserved(mut self, value: Reserved) -> Self {
        self.set_reserved(value);
        self
    }

    /// Get the Broadcast Command flag from the status word
    ///
    /// If set, the flag indicates that the terminal has received a valid
    /// broadcast command. See [BroadcastReceived](crate::flags::BroadcastReceived) for
    /// more information.
    pub fn broadcast_received(&self) -> BroadcastReceived {
        STATUS_BROADCAST_RECEIVED_FIELD.get(self).into()
    }

    /// Set the Broadcast Command flag of the status word
    ///
    /// See [StatusWord::broadcast_received] for
    /// more information.
    ///
    /// # Arguments
    ///
    /// * `value` - A [BroadcastReceived] flag to set
    ///
    pub fn set_broadcast_received(&mut self, value: BroadcastReceived) {
        STATUS_BROADCAST_RECEIVED_FIELD.set(self, value.into());
    }

    /// Constructor method to set the Broadcast Command flag
    ///
    /// See [StatusWord::broadcast_received] for
    /// more information.
    ///
    /// # Arguments
    ///
    /// * `value` - A [BroadcastReceived] flag to set
    ///
    pub fn with_broadcast_received(mut self, value: BroadcastReceived) -> Self {
        self.set_broadcast_received(value);
        self
    }

    /// Get the Busy flag from the status word
    ///
    /// If set, the flag indicates that the terminal is unable to respond to
    /// commands at this time. See [TerminalBusy](crate::flags::TerminalBusy) for
    /// more information.
    pub fn terminal_busy(&self) -> TerminalBusy {
        STATUS_TERMINAL_BUSY_FIELD.get(self).into()
    }

    /// Set the Busy flag on the status word
    ///
    /// See [StatusWord::terminal_busy] for
    /// more information.
    ///
    /// # Arguments
    ///
    /// * `value` - A [TerminalBusy] flag to set
    ///
    pub fn set_terminal_busy(&mut self, value: TerminalBusy) {
        STATUS_TERMINAL_BUSY_FIELD.set(self, value.into());
    }

    /// Constructor method to set the Busy flag
    ///
    /// See [StatusWord::terminal_busy] for
    /// more information.
    ///
    /// # Arguments
    ///
    /// * `value` - A [TerminalBusy] flag to set
    ///
    pub fn with_terminal_busy(mut self, value: TerminalBusy) -> Self {
        self.set_terminal_busy(value);
        self
    }

    /// Get the Dynamic Bus Control Acceptance flag from the status word
    ///
    /// If set, the flag indicates that the terminal is taking control
    /// of the bus. See [DynamicBusAcceptance](crate::flags::DynamicBusAcceptance) for
    /// more information.
    pub fn dynamic_bus_acceptance(&self) -> DynamicBusAcceptance {
        STATUS_DYNAMIC_BUS_ACCEPTANCE_FIELD.get(self).into()
    }

    /// Set the Dynamic Bus Control Acceptance flag on the status word
    ///
    /// See [StatusWord::dynamic_bus_acceptance] for
    /// more information.
    ///
    /// # Arguments
    ///
    /// * `value` - A [DynamicBusAcceptance] flag to set
    ///
    pub fn set_dynamic_bus_acceptance(&mut self, value: DynamicBusAcceptance) {
        STATUS_DYNAMIC_BUS_ACCEPTANCE_FIELD.set(self, value.into());
    }

    /// Constructor method to set the Dynamic Bus Control Acceptance flag
    ///
    /// See [StatusWord::dynamic_bus_acceptance] for
    /// more information.
    ///
    /// # Arguments
    ///
    /// * `value` - A [DynamicBusAcceptance] flag to set
    ///
    pub fn with_dynamic_bus_acceptance(mut self, value: DynamicBusAcceptance) -> Self {
        self.set_dynamic_bus_acceptance(value);
        self
    }

    /// Check if the message error flag is set
    ///
    /// See [MessageError] for more
    /// information.
    pub fn message_error(&self) -> MessageError {
        STATUS_MESSAGE_ERROR_FIELD.get(self).into()
    }

    /// Set the message error flag on this word
    ///
    /// See [StatusWord::message_error] for
    /// more information.
    ///
    /// # Arguments
    ///
    /// * `value` - A [MessageError] flag to set
    ///
    pub fn set_message_error(&mut self, value: MessageError) {
        STATUS_MESSAGE_ERROR_FIELD.set(self, value.into());
    }

    /// Constructor method to set the message error flag
    ///
    /// See [StatusWord::message_error] for
    /// more information.
    ///
    /// # Arguments
    ///
    /// * `value` - A [MessageError] flag to set
    ///
    pub fn with_message_error(mut self, value: MessageError) -> Self {
        self.set_message_error(value);
        self
    }

    /// Check if the subsystem error flag is set
    ///
    /// See [SubsystemError](crate::errors::SubsystemError) for more
    /// information.
    pub fn subsystem_error(&self) -> SubsystemError {
        STATUS_SUBSYSTEM_ERROR_FIELD.get(self).into()
    }

    /// Set the subsystem error flag on this word
    ///
    /// See [StatusWord::subsystem_error] for
    /// more information.
    ///
    /// # Arguments
    ///
    /// * `value` - A [SubsystemError] flag to set
    ///
    pub fn set_subsystem_error(&mut self, value: SubsystemError) {
        STATUS_SUBSYSTEM_ERROR_FIELD.set(self, value.into());
    }

    /// Constructor method to set the subsystem error flag
    ///
    /// See [StatusWord::subsystem_error] for
    /// more information.
    ///
    /// # Arguments
    ///
    /// * `value` - A [SubsystemError] flag to set
    ///
    pub fn with_subsystem_error(mut self, value: SubsystemError) -> Self {
        self.set_subsystem_error(value);
        self
    }

    /// Check if the terminal error flag is set
    ///
    /// See [`TerminalError`](crate::errors::TerminalError) for more
    /// information.
    pub fn terminal_error(&self) -> TerminalError {
        STATUS_TERMINAL_ERROR_FIELD.get(self).into()
    }

    /// Set the terminal error flag on this word
    ///
    /// See [`StatusWord::terminal_error`][StatusWord::terminal_error] for
    /// more information.
    ///
    /// # Arguments
    ///
    /// * `value` - A [TerminalError] flag to set
    ///
    pub fn set_terminal_error(&mut self, value: TerminalError) {
        STATUS_TERMINAL_ERROR_FIELD.set(self, value.into());
    }

    /// Constructor set the terminal error flag
    ///
    /// See [`StatusWord::terminal_error`][StatusWord::terminal_error] for
    /// more information.
    ///
    /// # Arguments
    ///
    /// * `value` - A [TerminalError] flag to set
    ///
    pub fn with_terminal_error(mut self, value: TerminalError) -> Self {
        self.set_terminal_error(value);
        self
    }

    /// Check if any of the various error flags are set
    ///
    /// See [StatusWord::message_error], [StatusWord::subsystem_error],
    /// or [StatusWord::terminal_error] for more information.
    #[must_use = "Returned value is not used"]
    pub fn is_error(&self) -> bool {
        self.message_error().is_error()
            || self.subsystem_error().is_error()
            || self.terminal_error().is_error()
    }

    /// Check if the terminal is currently busy
    ///
    /// See [StatusWord::terminal_busy] for
    /// more information.
    #[must_use = "Returned value is not used"]
    pub fn is_busy(&self) -> bool {
        self.terminal_busy().is_busy()
    }
}

impl DataWord {
    /// Constructor method to set the word from a string
    ///
    /// Fails if the given string is more than two
    /// bytes long.
    ///
    /// # Arguments
    ///
    /// * `data` - A &str to set as data
    ///
    pub fn with_string(mut self, data: &str) -> Result<Self> {
        self.set_string(data)?;
        Ok(self)
    }

    /// Set the word from a string
    ///
    /// Fails if the given string is more than two
    /// bytes long.
    ///
    /// # Arguments
    ///
    /// * `data` - A &str to set as data
    ///
    pub fn set_string(&mut self, data: &str) -> Result<()> {
        self.data = data.as_bytes().try_into()?;
        self.parity = self.calculate_parity();
        Ok(())
    }

    /// Get the internal data as a &str
    ///
    /// Fails if the word is not a valid UTF-8 string.
    pub fn as_string(&self) -> Result<&str> {
        self.try_into()
    }
}

impl Header for CommandWord {
    fn count(&self) -> Option<usize> {
        Some(self.word_count() as usize)
    }
}

impl Header for StatusWord {
    fn count(&self) -> Option<usize> {
        None
    }
}

impl Word for CommandWord {
    fn new() -> Self {
        Self {
            data: [0, 0],
            parity: 1,
        }
    }

    fn with_value(mut self, data: u16) -> Self {
        self.set_value(data);
        self
    }

    fn with_bytes(mut self, data: [u8; 2]) -> Self {
        self.set_bytes(data);
        self
    }

    fn with_parity(mut self, parity: u8) -> Self {
        self.set_parity(parity);
        self
    }

    fn with_calculated_parity(mut self) -> Self {
        self.parity = self.calculate_parity();
        self
    }

    fn build(self) -> Result<Self> {
        if self.check_parity() {
            Ok(self)
        } else {
            Err(Error::WordIsInvalid)
        }
    }

    fn from_value(data: u16) -> Self {
        Self::new().with_value(data).with_calculated_parity()
    }

    fn from_bytes(data: [u8; 2]) -> Self {
        Self::new().with_bytes(data)
    }

    fn as_bytes(&self) -> [u8; 2] {
        self.data
    }

    fn as_value(&self) -> u16 {
        self.into()
    }

    fn set_value(&mut self, data: u16) {
        self.data = data.to_be_bytes();
        self.parity = self.calculate_parity();
    }

    fn set_bytes(&mut self, data: [u8; 2]) {
        self.data = data;
        self.parity = self.calculate_parity();
    }

    fn parity(&self) -> u8 {
        self.parity
    }

    fn set_parity(&mut self, parity: u8) {
        self.parity = parity;
    }

    fn calculate_parity(&self) -> u8 {
        parity(self.as_value())
    }

    fn check_parity(&self) -> bool {
        self.parity() == self.calculate_parity()
    }
}

impl Word for StatusWord {
    fn new() -> Self {
        Self {
            data: [0, 0],
            parity: 1,
        }
    }

    fn with_value(mut self, data: u16) -> Self {
        self.set_value(data);
        self
    }

    fn with_bytes(mut self, data: [u8; 2]) -> Self {
        self.set_bytes(data);
        self
    }

    fn with_parity(mut self, parity: u8) -> Self {
        self.set_parity(parity);
        self
    }

    fn with_calculated_parity(mut self) -> Self {
        self.parity = self.calculate_parity();
        self
    }

    fn build(self) -> Result<Self> {
        if self.check_parity() {
            Ok(self)
        } else {
            Err(Error::WordIsInvalid)
        }
    }

    fn from_value(data: u16) -> Self {
        Self::new().with_value(data)
    }

    fn from_bytes(data: [u8; 2]) -> Self {
        Self::new().with_bytes(data)
    }

    fn as_bytes(&self) -> [u8; 2] {
        self.data
    }

    fn as_value(&self) -> u16 {
        self.into()
    }

    fn set_value(&mut self, data: u16) {
        self.data = data.to_be_bytes();
        self.parity = self.calculate_parity();
    }

    fn set_bytes(&mut self, data: [u8; 2]) {
        self.data = data;
        self.parity = self.calculate_parity();
    }

    fn parity(&self) -> u8 {
        self.parity
    }

    fn set_parity(&mut self, parity: u8) {
        self.parity = parity;
    }

    fn calculate_parity(&self) -> u8 {
        parity(self.as_value())
    }

    fn check_parity(&self) -> bool {
        self.parity() == self.calculate_parity()
    }
}

impl Word for DataWord {
    fn new() -> Self {
        Self {
            data: [0, 0],
            parity: 1,
        }
    }

    fn with_value(mut self, data: u16) -> Self {
        self.set_value(data);
        self
    }

    fn with_bytes(mut self, data: [u8; 2]) -> Self {
        self.set_bytes(data);
        self
    }

    fn with_parity(mut self, parity: u8) -> Self {
        self.set_parity(parity);
        self
    }

    fn with_calculated_parity(mut self) -> Self {
        self.parity = self.calculate_parity();
        self
    }

    fn build(self) -> Result<Self> {
        if self.check_parity() {
            Ok(self)
        } else {
            Err(Error::WordIsInvalid)
        }
    }

    fn from_value(data: u16) -> Self {
        Self::new().with_value(data)
    }

    fn from_bytes(data: [u8; 2]) -> Self {
        Self::new().with_bytes(data)
    }

    fn as_bytes(&self) -> [u8; 2] {
        self.data
    }

    fn as_value(&self) -> u16 {
        self.into()
    }

    fn set_value(&mut self, data: u16) {
        self.data = data.to_be_bytes();
        self.parity = self.calculate_parity();
    }

    fn set_bytes(&mut self, data: [u8; 2]) {
        self.data = data;
        self.parity = self.calculate_parity();
    }

    fn parity(&self) -> u8 {
        self.parity
    }

    fn set_parity(&mut self, parity: u8) {
        self.parity = parity;
    }

    fn calculate_parity(&self) -> u8 {
        parity(self.as_value())
    }

    fn check_parity(&self) -> bool {
        self.parity() == self.calculate_parity()
    }
}

impl Default for CommandWord {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for StatusWord {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for DataWord {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for DataWord {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::new()
            .with_string(value)
            .map(|w| w.with_calculated_parity())
    }
}

impl<'a> TryFrom<&'a DataWord> for &'a str {
    type Error = Error;

    fn try_from(value: &'a DataWord) -> Result<Self> {
        core::str::from_utf8(&value.data).or(Err(Error::StringIsInvalid))
    }
}

impl From<u16> for CommandWord {
    fn from(value: u16) -> Self {
        Self::from_value(value)
    }
}

impl From<u16> for StatusWord {
    fn from(value: u16) -> Self {
        Self::from_value(value)
    }
}

impl From<u16> for DataWord {
    fn from(value: u16) -> Self {
        Self::from_value(value)
    }
}

impl From<[u8; 2]> for CommandWord {
    fn from(value: [u8; 2]) -> Self {
        Self::from_bytes(value)
    }
}

impl From<[u8; 2]> for StatusWord {
    fn from(value: [u8; 2]) -> Self {
        Self::from_bytes(value)
    }
}

impl From<[u8; 2]> for DataWord {
    fn from(value: [u8; 2]) -> Self {
        Self::from_bytes(value)
    }
}

impl From<&CommandWord> for [u8; 2] {
    fn from(value: &CommandWord) -> Self {
        value.data
    }
}

impl From<CommandWord> for [u8; 2] {
    fn from(value: CommandWord) -> Self {
        value.data
    }
}

impl From<&StatusWord> for [u8; 2] {
    fn from(value: &StatusWord) -> Self {
        value.data
    }
}

impl From<StatusWord> for [u8; 2] {
    fn from(value: StatusWord) -> Self {
        value.data
    }
}

impl From<&DataWord> for [u8; 2] {
    fn from(value: &DataWord) -> Self {
        value.data
    }
}

impl From<DataWord> for [u8; 2] {
    fn from(value: DataWord) -> Self {
        value.data
    }
}

impl From<&CommandWord> for u16 {
    fn from(value: &CommandWord) -> Self {
        u16::from_be_bytes(value.data)
    }
}

impl From<CommandWord> for u16 {
    fn from(value: CommandWord) -> Self {
        u16::from_be_bytes(value.data)
    }
}

impl From<&StatusWord> for u16 {
    fn from(value: &StatusWord) -> Self {
        u16::from_be_bytes(value.data)
    }
}

impl From<StatusWord> for u16 {
    fn from(value: StatusWord) -> Self {
        u16::from_be_bytes(value.data)
    }
}

impl From<&DataWord> for u16 {
    fn from(value: &DataWord) -> Self {
        u16::from_be_bytes(value.data)
    }
}

impl From<DataWord> for u16 {
    fn from(value: DataWord) -> Self {
        u16::from_be_bytes(value.data)
    }
}

impl From<DataWord> for i16 {
    fn from(value: DataWord) -> Self {
        u16::from(value) as i16
    }
}

impl From<DataWord> for u32 {
    fn from(value: DataWord) -> Self {
        u16::from(value) as u32
    }
}

impl From<DataWord> for i32 {
    fn from(value: DataWord) -> Self {
        u16::from(value) as i32
    }
}

impl From<DataWord> for u64 {
    fn from(value: DataWord) -> Self {
        u16::from(value) as u64
    }
}

impl From<DataWord> for i64 {
    fn from(value: DataWord) -> Self {
        u16::from(value) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ----------------------------------------------------------
    // CommandWord

    #[test]
    fn test_command_get_set_address() {
        let value1 = Address::Broadcast(0b11111);
        let value2 = Address::Value(0b10101);


        let mut word = CommandWord::new().with_address(value1);
        assert_eq!(word.address(),value1);

        word.set_address(value2);
        assert_eq!(word.address(),value2);
        assert_eq!(word.as_value(), 0b1010100000000000);
    }

    #[test]
    fn test_command_get_set_subaddress() {
        let value1 = SubAddress::ModeCode(0b11111);
        let value2 = SubAddress::Value(0b10101);


        let mut word = CommandWord::new().with_subaddress(value1);
        assert_eq!(word.subaddress(),value1);

        word.set_subaddress(value2);
        assert_eq!(word.subaddress(),value2);
        assert_eq!(word.as_value(), 0b0000001010100000);
    }

    #[test]
    fn test_command_get_set_transmit_receive() {
        let mut word = CommandWord::new().with_transmit_receive(TransmitReceive::Receive);
        assert!(word.transmit_receive().is_receive());

        word.set_transmit_receive(TransmitReceive::Transmit);
        assert!(word.transmit_receive().is_transmit());
    }

    #[test]
    fn test_command_get_set_mode_code() {
        let mut word = CommandWord::new().with_mode_code(ModeCode::TransmitVectorWord);
        assert_eq!(word.mode_code(), ModeCode::TransmitVectorWord);

        word.set_mode_code(ModeCode::InitiateSelfTest);
        assert_eq!(word.mode_code(), ModeCode::InitiateSelfTest);
    }

    #[test]
    fn test_command_get_set_word_count() {
        let mut word = CommandWord::new().with_word_count(33);
        assert_eq!(word.word_count(), 32);

        word.set_word_count(5);
        assert_eq!(word.word_count(), 5);
    }

    #[test]
    fn test_command_is_mode_code() {
        let mut word = CommandWord::new();

        word.set_subaddress(SubAddress::ModeCode(0b00000));
        assert!(word.is_mode_code());

        word.set_subaddress(SubAddress::Value(0b01010));
        assert!(!word.is_mode_code());

        word.set_subaddress(SubAddress::ModeCode(0b11111));
        assert!(word.is_mode_code());
    }

    #[test]
    fn test_command_is_transmit() {
        let mut word = CommandWord::new();

        word.set_transmit_receive(TransmitReceive::Transmit);
        assert!(word.is_transmit());

        word.set_transmit_receive(TransmitReceive::Receive);
        assert!(!word.is_transmit());
    }

    #[test]
    fn test_command_is_receive() {
        let mut word = CommandWord::new();

        word.set_transmit_receive(TransmitReceive::Receive);
        assert!(word.is_receive());

        word.set_transmit_receive(TransmitReceive::Transmit);
        assert!(!word.is_receive());
    }

    #[test]
    fn test_command_count() {
        let mut word = CommandWord::new().with_word_count(33);
        assert_eq!(word.count(), 32);

        word.set_word_count(5);
        assert_eq!(word.count(), 5);
    }

    #[test]
    fn test_command_roundtrip() {
        let word1 = CommandWord::from_value(0b0110100001101001);
        let data1 = word1.as_bytes();

        let word2 = CommandWord::from_bytes(data1);
        let data2 = word2.as_bytes();

        assert_eq!(data1, [0b01101000, 0b01101001]);
        assert_eq!(data2, [0b01101000, 0b01101001]);
        assert_eq!(word1, word2);
    }

    // ----------------------------------------------------------

    // ----------------------------------------------------------
    // CommandWord: Header

    #[test]
    fn test_command_header_count() {
        let mut word = CommandWord::new().with_word_count(33);
        assert_eq!(Header::count(&word), Some(32));

        word.set_word_count(5);
        assert_eq!(Header::count(&word), Some(5));
    }

    // ----------------------------------------------------------

    // ----------------------------------------------------------
    // CommandWord: Word

    #[test]
    fn test_command_word_new() {
        let word = CommandWord::new();
        assert_eq!(word.data, [0,0]);
        assert_eq!(word.parity, 1);
    }

    #[test]
    fn test_command_word_value() {
        let mut word = CommandWord::new().with_value(0b0101010101010101);
        assert_eq!(word.data, [0b01010101, 0b01010101]);
        assert_eq!(word.as_value(), 0b0101010101010101);

        word.set_value(0b1010101010101010);
        assert_eq!(word.data, [0b10101010,0b10101010]);
        assert_eq!(word.as_value(), 0b1010101010101010);

        word = CommandWord::from_value(0b0101010101010101);
        assert_eq!(word.data, [0b01010101, 0b01010101]);
        assert_eq!(word.as_value(), 0b0101010101010101);
    }

    #[test]
    fn test_command_word_bytes() {
        let mut word = CommandWord::new().with_bytes([0b01010101, 0b01010101]);
        assert_eq!(word.data, [0b01010101, 0b01010101]);
        assert_eq!(word.as_bytes(), [0b01010101, 0b01010101]);

        word.set_bytes([0b10101010,0b10101010]);
        assert_eq!(word.data, [0b10101010,0b10101010]);
        assert_eq!(word.as_bytes(), [0b10101010,0b10101010]);

        word = CommandWord::from_bytes([0b01010101, 0b01010101]);
        assert_eq!(word.data, [0b01010101, 0b01010101]);
        assert_eq!(word.as_bytes(), [0b01010101, 0b01010101]);
    }

    #[test]
    fn test_command_word_parity() {
        let mut word = CommandWord::from(0).with_parity(0);
        assert_eq!(word.parity, 0);
        assert_eq!(word.parity(), 0);
        assert_eq!(word.check_parity(),false);

        word.set_parity(1);
        assert_eq!(word.parity, 1);
        assert_eq!(word.parity(), 1);
        assert_eq!(word.check_parity(),true);

        word = CommandWord::from(1).with_calculated_parity();
        assert_eq!(word.parity, 0);
        assert_eq!(word.parity(), 0);
        assert_eq!(word.check_parity(),true);
    }

    #[test]
    fn test_command_word_build_success() {
        let word = CommandWord::new()
            .with_calculated_parity()
            .build();
        assert!(word.is_ok());
    }

    #[test]
    fn test_command_word_build_fail() {
        let word = CommandWord::new()
            .with_parity(0)
            .build();
        assert!(word.is_err());
    }

    // ----------------------------------------------------------

    #[test]
    fn test_command_default() {
        let word = CommandWord::default();
        assert_eq!(word.data, [0,0]);
        assert_eq!(word.parity, 1);
    }

    #[test]
    fn test_command_from_value() {
        let word: CommandWord = 0b1010101010101010.into();
        assert_eq!(word.as_value(),0b1010101010101010);
    }

    #[test]
    fn test_command_from_bytes() {
        let word: CommandWord = [0b10101010,0b10101010].into();
        assert_eq!(word.as_value(),0b1010101010101010);
    }

    #[test]
    fn test_command_into_value() {
        let word = CommandWord::from(0b1010101010101010);
        let value = u16::from(word);
        assert_eq!(value,0b1010101010101010);
    }

    #[test]
    fn test_command_into_bytes() {
        let word = CommandWord::from(0b1010101010101010);
        let bytes = <[u8;2]>::from(word);
        assert_eq!(bytes,[0b10101010, 0b10101010]);
    }

    // ----------------------------------------------------------
    // StatusWord

    #[test]
    fn test_status_get_set_address() {
        let item1 = Address::Broadcast(0b11111);
        let item2 = Address::Value(0b10101);

        let mut word = StatusWord::new().with_address(item1);
        assert_eq!(word.address(),item1);

        word.set_address(item2);
        assert_eq!(word.address(),item2);
        assert_eq!(word.as_value(), 0b1010100000000000);
    }

    #[test]
    fn test_status_get_set_instrumentation() {
        let item1 = Instrumentation::Status;
        let item2 = Instrumentation::Command;

        let mut word = StatusWord::new().with_instrumentation(item1);
        assert_eq!(word.instrumentation(),item1);

        word.set_instrumentation(item2);
        assert_eq!(word.instrumentation(),item2);
    }

    #[test]
    fn test_status_get_set_service_request() {
        let item1 = ServiceRequest::NoService;
        let item2 = ServiceRequest::Service;

        let mut word = StatusWord::new().with_service_request(item1);
        assert_eq!(word.service_request(),item1);

        word.set_service_request(item2);
        assert_eq!(word.service_request(),item2);
    }

    #[test]
    fn test_status_get_set_reserved() {
        let item1 = Reserved::None;
        let item2 = Reserved::Value(0b111);

        let mut word = StatusWord::new().with_reserved(item1);
        assert_eq!(word.reserved(),item1);

        word.set_reserved(item2);
        assert_eq!(word.reserved(),item2);
    }

    #[test]
    fn test_status_get_set_broadcast_received() {
        let item1 = BroadcastReceived::NotReceived;
        let item2 = BroadcastReceived::Received;

        let mut word = StatusWord::new().with_broadcast_received(item1);
        assert_eq!(word.broadcast_received(),item1);

        word.set_broadcast_received(item2);
        assert_eq!(word.broadcast_received(),item2);
    }

    #[test]
    fn test_status_get_set_terminal_busy() {
        let item1 = TerminalBusy::NotBusy;
        let item2 = TerminalBusy::Busy;

        let mut word = StatusWord::new().with_terminal_busy(item1);
        assert_eq!(word.is_busy(),false);
        assert_eq!(word.terminal_busy(),item1);

        word.set_terminal_busy(item2);
        assert_eq!(word.is_busy(),true);
        assert_eq!(word.terminal_busy(),item2);
    }

    #[test]
    fn test_status_get_set_dynamic_bus_acceptance() {
        let item1 = DynamicBusAcceptance::NotAccepted;
        let item2 = DynamicBusAcceptance::Accepted;

        let mut word = StatusWord::new().with_dynamic_bus_acceptance(item1);
        assert_eq!(word.dynamic_bus_acceptance(),item1);

        word.set_dynamic_bus_acceptance(item2);
        assert_eq!(word.dynamic_bus_acceptance(),item2);
    }

    #[test]
    fn test_status_get_set_message_error() {
        let item1 = MessageError::None;
        let item2 = MessageError::Error;

        let mut word = StatusWord::new().with_message_error(item1);
        assert_eq!(word.message_error(),item1);

        word.set_message_error(item2);
        assert_eq!(word.message_error(),item2);
    }

    #[test]
    fn test_status_get_set_subsystem_error() {
        let item1 = SubsystemError::None;
        let item2 = SubsystemError::Error;

        let mut word = StatusWord::new().with_subsystem_error(item1);
        assert_eq!(word.subsystem_error(),item1);

        word.set_subsystem_error(item2);
        assert_eq!(word.subsystem_error(),item2);
    }

    #[test]
    fn test_status_get_set_terminal_error() {
        let item1 = TerminalError::None;
        let item2 = TerminalError::Error;

        let mut word = StatusWord::new().with_terminal_error(item1);
        assert_eq!(word.terminal_error(),item1);

        word.set_terminal_error(item2);
        assert_eq!(word.terminal_error(),item2);
    }

    #[test]
    fn test_status_is_error() {
        let item1 = TerminalError::Error;
        let item2 = SubsystemError::Error;
        let item3 = MessageError::Error;

        let mut word1 = StatusWord::new();
        let mut word2 = StatusWord::new();
        let mut word3 = StatusWord::new();

        assert!(!word1.is_error());
        assert!(!word2.is_error());
        assert!(!word3.is_error());

        word1.set_terminal_error(item1);
        word2.set_subsystem_error(item2);
        word3.set_message_error(item3);

        assert!(word1.is_error());
        assert!(word2.is_error());
        assert!(word3.is_error());
    }

    #[test]
    fn test_status_roundtrip() {
        let word1 = StatusWord::from_value(0b0110100001101001);
        let data1 = word1.as_bytes();

        let word2 = StatusWord::from_bytes(data1);
        let data2 = word2.as_bytes();

        assert_eq!(data1, [0b01101000, 0b01101001]);
        assert_eq!(data2, [0b01101000, 0b01101001]);
        assert_eq!(word1, word2);
    }

    // ----------------------------------------------------------

    // ----------------------------------------------------------
    // StatusWord: Header

    #[test]
    fn test_status_header_count() {
        let word = StatusWord::new();
        assert_eq!(Header::count(&word), None);
    }

    // ----------------------------------------------------------

    // ----------------------------------------------------------
    // StatusWord: Word

    #[test]
    fn test_status_word_new() {
        let word = StatusWord::new();
        assert_eq!(word.data, [0,0]);
        assert_eq!(word.parity, 1);
    }

    #[test]
    fn test_status_word_value() {
        let mut word = StatusWord::new().with_value(0b0101010101010101);
        assert_eq!(word.data, [0b01010101, 0b01010101]);
        assert_eq!(word.as_value(), 0b0101010101010101);

        word.set_value(0b1010101010101010);
        assert_eq!(word.data, [0b10101010,0b10101010]);
        assert_eq!(word.as_value(), 0b1010101010101010);

        word = StatusWord::from_value(0b0101010101010101);
        assert_eq!(word.data, [0b01010101, 0b01010101]);
        assert_eq!(word.as_value(), 0b0101010101010101);
    }

    #[test]
    fn test_status_word_bytes() {
        let mut word = StatusWord::new().with_bytes([0b01010101, 0b01010101]);
        assert_eq!(word.data, [0b01010101, 0b01010101]);
        assert_eq!(word.as_bytes(), [0b01010101, 0b01010101]);

        word.set_bytes([0b10101010,0b10101010]);
        assert_eq!(word.data, [0b10101010,0b10101010]);
        assert_eq!(word.as_bytes(), [0b10101010,0b10101010]);

        word = StatusWord::from_bytes([0b01010101, 0b01010101]);
        assert_eq!(word.data, [0b01010101, 0b01010101]);
        assert_eq!(word.as_bytes(), [0b01010101, 0b01010101]);
    }

    #[test]
    fn test_status_word_parity() {
        let mut word = StatusWord::from(0).with_parity(0);
        assert_eq!(word.parity, 0);
        assert_eq!(word.parity(), 0);
        assert_eq!(word.check_parity(),false);

        word.set_parity(1);
        assert_eq!(word.parity, 1);
        assert_eq!(word.parity(), 1);
        assert_eq!(word.check_parity(),true);

        word = StatusWord::from(1).with_calculated_parity();
        assert_eq!(word.parity, 0);
        assert_eq!(word.parity(), 0);
        assert_eq!(word.check_parity(),true);
    }

    #[test]
    fn test_status_word_build_success() {
        let word = StatusWord::new()
            .with_calculated_parity()
            .build();
        assert!(word.is_ok());
    }

    #[test]
    fn test_status_word_build_fail() {
        let word = StatusWord::new()
            .with_parity(0)
            .build();
        assert!(word.is_err());
    }

    // ----------------------------------------------------------


    #[test]
    fn test_status_default() {
        let word = StatusWord::default();
        assert_eq!(word.data, [0,0]);
        assert_eq!(word.parity, 1);
    }

    #[test]
    fn test_status_from_value() {
        let word: StatusWord = 0b1010101010101010.into();
        assert_eq!(word.as_value(),0b1010101010101010);
    }

    #[test]
    fn test_status_from_bytes() {
        let word: StatusWord = [0b10101010,0b10101010].into();
        assert_eq!(word.as_value(),0b1010101010101010);
    }

    #[test]
    fn test_status_into_value() {
        let word = StatusWord::from(0b1010101010101010);
        let value = u16::from(word);
        assert_eq!(value,0b1010101010101010);
    }

    #[test]
    fn test_status_into_bytes() {
        let word = StatusWord::from(0b1010101010101010);
        let bytes = <[u8;2]>::from(word);
        assert_eq!(bytes,[0b10101010, 0b10101010]);
    }

    // ----------------------------------------------------------
    // DataWord

    #[test]
    fn test_data_get_set_string_success() {
        let mut word = DataWord::new().with_string("HI").unwrap();
        assert_eq!(word.as_bytes(), [0b01001000, 0b01001001]);
        assert_eq!(word.as_value(), 0b0100100001001001u16);
        assert_eq!(word.as_string(), Ok("HI"));
        assert_eq!(word.parity(), 0);

        word.set_string("NO").unwrap();
        assert_eq!(word.as_bytes(), [0b01001110, 0b01001111]);
        assert_eq!(word.as_value(), 0b0100111001001111);
        assert_eq!(word.as_string(), Ok("NO"));
        assert_eq!(word.parity(), 0);
    } 

    #[test]
    fn test_data_get_set_string_failure() {
        let mut word = DataWord::new().with_string("HI").unwrap();

        // fails if string is too long without modifying data
        let result = word.set_string("TOO LONG");
        assert_eq!(word.as_string(), Ok("HI"));
        assert_eq!(word.parity(), 0);
        assert!(result.is_err());

        // fails to return a string if not valid utf-8
        word.set_value(0b1111111111111111);
        assert_eq!(word.as_value(), 0b1111111111111111);
        assert!(word.as_string().is_err());
        assert_eq!(word.parity(), 1);
    } 

    #[test]
    fn test_data_roundtrip() {
        let word1 = DataWord::from_value(0b0110100001101001);
        let data1 = word1.as_bytes();

        let word2 = DataWord::from_bytes(data1);
        let data2 = word2.as_bytes();

        assert_eq!(data1, [0b01101000, 0b01101001]);
        assert_eq!(data2, [0b01101000, 0b01101001]);
        assert_eq!(word1, word2);
    }

    // ----------------------------------------------------------
    // DataWord: Word

    #[test]
    fn test_data_word_new() {
        let word = StatusWord::new();
        assert_eq!(word.data, [0,0]);
        assert_eq!(word.parity, 1);
    }

    #[test]
    fn test_data_word_value() {
        let mut word = DataWord::new().with_value(0b0101010101010101);
        assert_eq!(word.data, [0b01010101, 0b01010101]);
        assert_eq!(word.as_value(), 0b0101010101010101);

        word.set_value(0b1010101010101010);
        assert_eq!(word.data, [0b10101010,0b10101010]);
        assert_eq!(word.as_value(), 0b1010101010101010);

        word = DataWord::from_value(0b0101010101010101);
        assert_eq!(word.data, [0b01010101, 0b01010101]);
        assert_eq!(word.as_value(), 0b0101010101010101);
    }

    #[test]
    fn test_data_word_bytes() {
        let mut word = DataWord::new().with_bytes([0b01010101, 0b01010101]);
        assert_eq!(word.data, [0b01010101, 0b01010101]);
        assert_eq!(word.as_bytes(), [0b01010101, 0b01010101]);

        word.set_bytes([0b10101010,0b10101010]);
        assert_eq!(word.data, [0b10101010,0b10101010]);
        assert_eq!(word.as_bytes(), [0b10101010,0b10101010]);

        word = DataWord::from_bytes([0b01010101, 0b01010101]);
        assert_eq!(word.data, [0b01010101, 0b01010101]);
        assert_eq!(word.as_bytes(), [0b01010101, 0b01010101]);
    }

    #[test]
    fn test_data_word_parity() {
        let mut word = DataWord::from(0).with_parity(0);
        assert_eq!(word.parity, 0);
        assert_eq!(word.parity(), 0);
        assert_eq!(word.check_parity(),false);

        word.set_parity(1);
        assert_eq!(word.parity, 1);
        assert_eq!(word.parity(), 1);
        assert_eq!(word.check_parity(),true);

        word = DataWord::from(1).with_calculated_parity();
        assert_eq!(word.parity, 0);
        assert_eq!(word.parity(), 0);
        assert_eq!(word.check_parity(),true);
    }

    #[test]
    fn test_data_build_success() {
        let word = DataWord::new()
            .with_calculated_parity()
            .build();
        assert!(word.is_ok());
    }

    #[test]
    fn test_data_build_fail() {
        let word = DataWord::new()
            .with_parity(0)
            .build();
        assert!(word.is_err());
    }

    // ----------------------------------------------------------

    #[test]
    fn test_data_default() {
        let word = DataWord::default();
        assert_eq!(word.data, [0,0]);
        assert_eq!(word.parity, 1);
    }

    #[test]
    fn test_data_try_from_string_success() {
        let result = DataWord::try_from("HI");
        assert!(result.is_ok());
    }

    #[test]
    fn test_data_try_from_string_fail() {
        let result = DataWord::try_from("TOO LONG");
        assert!(result.is_err());
    }

    #[test]
    fn test_data_from_value() {
        let word: DataWord = 0b1010101010101010.into();
        assert_eq!(word.as_value(),0b1010101010101010);
    }

    #[test]
    fn test_data_from_bytes() {
        let word: DataWord = [0b10101010,0b10101010].into();
        assert_eq!(word.as_value(),0b1010101010101010);
    }

    #[test]
    fn test_data_into_value() {
        let word = DataWord::from(0b1010101010101010);
        let value = u16::from(word);
        assert_eq!(value,0b1010101010101010);
    }

    #[test]
    fn test_data_into_bytes() {
        let word = DataWord::from(0b1010101010101010);
        let bytes = <[u8;2]>::from(word);
        assert_eq!(bytes,[0b10101010, 0b10101010]);
    }

    #[test]
    fn test_data_into_u32() {
        let word = DataWord::from(0b0000000010101010);
        let value = u32::from(word);
        assert_eq!(value,170); // decimal
    }

    #[test]
    fn test_data_into_u64() {
        let word = DataWord::from(0b0000000010101010);
        let value = u64::from(word);
        assert_eq!(value,170); // decimal
    }

    #[test]
    fn test_data_into_i16() {
        let word = DataWord::from(0b0000000010101010);
        let value = i16::from(word);
        assert_eq!(value,170); // decimal
    }

    #[test]
    fn test_data_into_i32() {
        let word = DataWord::from(0b0000000010101010);
        let value = i32::from(word);
        assert_eq!(value,170); // decimal
    }

    #[test]
    fn test_data_into_i64() {
        let word = DataWord::from(0b0000000010101010);
        let value = i64::from(word);
        assert_eq!(value,170); // decimal
    }

}
