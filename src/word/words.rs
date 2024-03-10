use crate::errors::{parity, Error, MessageError, Result, SubsystemError, TerminalError};
use crate::fields::*;
use crate::flags::*;

/// Common functionality for all words
pub trait Word where Self: Sized{
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
    fn as_bytes(&self) -> &[u8];

    /// Get the internal data as u16
    fn as_value(&self) -> u16;

    /// Set the internal data as a slice
    fn set_bytes(&mut self, data: [u8;2]);

    /// Set the internal data as u16
    fn set_value(&mut self, data: u16);

    /// Get a the number of ones in the word
    fn count_ones(&self) -> u8;

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
///     .with_address(16)
///     .with_subaddress(0) // mode code value
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
    data: [u8;2],

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
///     .with_address(16)
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
    data: [u8;2],

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
    pub fn set_address<T>(&mut self, value: T)
    where
        T: Into<Address>,
    {
        let field = value.into();
        COMMAND_ADDRESS_FIELD.set(self, field.into());
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
    pub fn with_address<T>(mut self, value: T) -> Self
    where
        T: Into<Address>,
    {
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
    pub fn set_subaddress<T>(&mut self, value: T)
    where
        T: Into<SubAddress>,
    {
        let field = value.into();
        COMMAND_SUBADDRESS_FIELD.set(self, field.into());
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
    pub fn with_subaddress<T>(mut self, value: T) -> Self
    where
        T: Into<SubAddress>,
    {
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
    pub fn set_transmit_receive<T>(&mut self, value: T)
    where
        T: Into<TransmitReceive>,
    {
        let field = value.into();
        COMMAND_TRANSMIT_RECEIVE_FIELD.set(self, field.into());
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
    pub fn with_transmit_receive<T>(mut self, value: T) -> Self
    where
        T: Into<TransmitReceive>,
    {
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
    pub fn set_mode_code<T>(&mut self, value: T)
    where
        T: Into<ModeCode>,
    {
        let field = value.into();
        COMMAND_MODE_CODE_FIELD.set(self, field.into());
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
    pub fn with_mode_code<T>(mut self, value: T) -> Self
    where
        T: Into<ModeCode>,
    {
        self.set_mode_code(value);
        self
    }

    /// Get the number of data words associated with this word
    ///
    /// This field is `None` if the subaddress is set to the ModeCode value.
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
    /// This method will do nothing if the subaddress is set to the ModeCode
    /// value. See [CommandWord::word_count] for more information.
    /// 
    /// # Arguments
    ///
    /// * `value` - A word count to set
    ///
    pub fn set_word_count(&mut self, value: u8) {
        COMMAND_WORD_COUNT_FIELD.set(self, value);
    }

    /// Constructor method to set the number of data words
    ///
    /// This method will do nothing if the subaddress is set to the ModeCode
    /// value. See [CommandWord::word_count] for more information.
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
    pub fn set_address<T>(&mut self, value: T)
    where
        T: Into<Address>,
    {
        let field = value.into();
        STATUS_ADDRESS_FIELD.set(self, field.into());
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
    pub fn with_address<T>(mut self, value: T) -> Self
    where
        T: Into<Address>,
    {
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
    pub fn set_instrumentation<T>(&mut self, value: T)
    where
        T: Into<Instrumentation>,
    {
        let field = value.into();
        STATUS_INSTRUMENTATION_FIELD.set(self, field.into());
    }

    /// Constructor metho to set Instrumentation flag
    ///
    /// See [StatusWord::instrumentation] for
    /// more information.
    /// 
    /// # Arguments
    ///
    /// * `value` - An [Instrumentation] flag to set
    ///
    pub fn with_instrumentation<T>(mut self, value: T) -> Self
    where
        T: Into<Instrumentation>,
    {
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
    pub fn set_service_request<T>(&mut self, value: T)
    where
        T: Into<ServiceRequest>,
    {
        let field = value.into();
        STATUS_SERVICE_REQUEST_FIELD.set(self, field.into());
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
    pub fn with_service_request<T>(mut self, value: T) -> Self
    where
        T: Into<ServiceRequest>,
    {
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
    pub fn set_reserved<T>(&mut self, value: T)
    where
        T: Into<Reserved>,
    {
        let field = value.into();
        STATUS_RESERVED_FIELD.set(self, field.into());
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
    pub fn with_reserved<T>(mut self, value: T) -> Self
    where
        T: Into<Reserved>,
    {
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
    pub fn set_broadcast_received<T>(&mut self, value: T)
    where
        T: Into<BroadcastReceived>,
    {
        let field = value.into();
        STATUS_BROADCAST_RECEIVED_FIELD.set(self, field.into());
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
    pub fn with_broadcast_received<T>(mut self, value: T) -> Self
    where
        T: Into<BroadcastReceived>,
    {
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
    pub fn set_terminal_busy<T>(&mut self, value: T)
    where
        T: Into<TerminalBusy>,
    {
        let field = value.into();
        STATUS_TERMINAL_BUSY_FIELD.set(self, field.into());
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
    pub fn with_terminal_busy<T>(mut self, value: T) -> Self
    where
        T: Into<TerminalBusy>,
    {
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
    pub fn set_dynamic_bus_acceptance<T>(&mut self, value: T)
    where
        T: Into<DynamicBusAcceptance>,
    {
        let field = value.into();
        STATUS_DYNAMIC_BUS_ACCEPTANCE_FIELD.set(self, field.into());
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
    pub fn with_dynamic_bus_acceptance<T>(mut self, value: T) -> Self
    where
        T: Into<DynamicBusAcceptance>,
    {
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
    pub fn set_message_error<T>(&mut self, value: T)
    where
        T: Into<MessageError>,
    {
        let field = value.into();
        STATUS_MESSAGE_ERROR_FIELD.set(self, field.into());
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
    pub fn with_message_error<T>(mut self, value: T) -> Self
    where
        T: Into<MessageError>,
    {
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
    pub fn set_subsystem_error<T>(&mut self, value: T)
    where
        T: Into<SubsystemError>,
    {
        let field = value.into();
        STATUS_SUBSYSTEM_ERROR_FIELD.set(self, field.into());
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
    pub fn with_subsystem_error<T>(mut self, value: T) -> Self
    where
        T: Into<SubsystemError>,
    {
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
    pub fn set_terminal_error<T>(&mut self, value: T)
    where
        T: Into<TerminalError>,
    {
        let field = value.into();
        STATUS_TERMINAL_ERROR_FIELD.set(self, field.into());
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
    pub fn with_terminal_error<T>(mut self, value: T) -> Self
    where
        T: Into<TerminalError>,
    {
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
        Ok(())
    }

    /// Get the internal data as a &str
    ///
    /// Fails if the word is not a valid UTF-8 string.
    pub fn as_string(&self) -> Result<&str> {
        self.try_into()
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
        Self::new()
            .with_value(data)
            .with_calculated_parity()
    }

    fn from_bytes(data: [u8; 2]) -> Self {
        Self::new().with_bytes(data)
    }

    fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    fn as_value(&self) -> u16 {
        self.into()
    }

    fn set_value(&mut self, data: u16){
        self.data = data.to_be_bytes();
        self.parity = self.calculate_parity();
    }

    fn set_bytes(&mut self, data: [u8; 2]){
        self.data = data;
        self.parity = self.calculate_parity();
    }

    fn count_ones(&self) -> u8 {
        self.as_value().count_ones() as u8
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

    fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    fn as_value(&self) -> u16 {
        self.into()
    }

    fn set_value(&mut self, data: u16){
        self.data = data.to_be_bytes();
        self.parity = self.calculate_parity();
    }

    fn set_bytes(&mut self, data: [u8; 2]){
        self.data = data;
        self.parity = self.calculate_parity();
    }

    fn count_ones(&self) -> u8 {
        self.as_value().count_ones() as u8
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

    fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    fn as_value(&self) -> u16 {
        self.into()
    }

    fn set_value(&mut self, data: u16){
        self.data = data.to_be_bytes();
        self.parity = self.calculate_parity();
    }

    fn set_bytes(&mut self, data: [u8; 2]){
        self.data = data;
        self.parity = self.calculate_parity();
    }

    fn count_ones(&self) -> u8 {
        self.as_value().count_ones() as u8
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
        core::str::from_utf8(&value.data)
            .or(Err(Error::StringIsInvalid))
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

impl From<[u8;2]> for CommandWord {
    fn from(value: [u8;2]) -> Self {
        Self::from_bytes(value)
    }
}

impl From<[u8;2]> for StatusWord {
    fn from(value: [u8;2]) -> Self {
        Self::from_bytes(value)
    }
}

impl From<[u8;2]> for DataWord {
    fn from(value: [u8;2]) -> Self {
        Self::from_bytes(value)
    }
}

impl From<&mut CommandWord> for [u8; 2] {
    fn from(value: &mut CommandWord) -> Self {
        value.data
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

impl From<&mut StatusWord> for [u8; 2] {
    fn from(value: &mut StatusWord) -> Self {
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

impl From<&mut DataWord> for [u8; 2] {
    fn from(value: &mut DataWord) -> Self {
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

impl From<&mut CommandWord> for u16 {
    fn from(value: &mut CommandWord) -> Self {
        u16::from_be_bytes(value.data)
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

impl From<&mut StatusWord> for u16 {
    fn from(value: &mut StatusWord) -> Self {
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

impl From<&mut DataWord> for u16 {
    fn from(value: &mut DataWord) -> Self {
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

    #[test]
    fn test_data_with_bytes() {
        let word = DataWord::new()
            .with_bytes([0b01001000, 0b01001001])
            .with_calculated_parity()
            .build()
            .unwrap();

        assert_eq!(word.as_bytes(), &[0b01001000, 0b01001001]);
        assert_eq!(word.as_value(), 0b0100100001001001u16);
        assert_eq!(word.as_string(), Ok("HI"));
        assert_eq!(word.parity(), 0);
    }

    #[test]
    fn test_data_with_data() {
        let word = DataWord::new()
            .with_value(0b0100100001001001u16)
            .with_calculated_parity();

        assert_eq!(word.as_bytes(), &[0b01001000, 0b01001001]);
        assert_eq!(word.as_value(), 0b0100100001001001u16);
        assert_eq!(word.as_string(), Ok("HI"));
        assert_eq!(word.parity(), 0);
    }

    #[test]
    fn test_data_with_str() {
        let word = DataWord::new()
            .with_string("HI")
            .unwrap()
            .with_calculated_parity();

        assert_eq!(word.as_bytes(), &[0b01001000, 0b01001001]);
        assert_eq!(word.as_value(), 0b0100100001001001u16);
        assert_eq!(word.as_string(), Ok("HI"));
        assert_eq!(word.parity(), 0);
    }

    #[test]
    fn test_status_with_methods() {
        let word = StatusWord::new()
            .with_address(4)
            .with_terminal_busy(1)
            .with_message_error(1)
            .with_terminal_error(1)
            .with_subsystem_error(1)
            .build()
            .unwrap();

        assert_eq!(word.address(), Address::Value(4));
        assert_eq!(word.terminal_busy(), TerminalBusy::Busy);
        assert_eq!(word.message_error(), MessageError::Error);
        assert_eq!(word.terminal_error(), TerminalError::Error);
        assert_eq!(word.subsystem_error(), SubsystemError::Error);
    }

    #[test]
    fn test_command_with_methods() {
        let word = CommandWord::new()
            .with_address(4)
            .with_subaddress(2)
            .with_transmit_receive(1)
            .with_word_count(3)
            .build()
            .unwrap();

        assert!(!word.is_mode_code());
        assert_eq!(word.word_count(), 3);
        assert_eq!(word.address(), Address::Value(4));
        assert_eq!(word.subaddress(), SubAddress::Value(2));
        assert_eq!(word.transmit_receive(), TransmitReceive::Transmit);
    }

    #[test]
    fn test_command_parity_update() {
        let mut word = CommandWord::from_value(0b0000000000101010);
        assert_eq!(word.parity, 0);

        word.set_address(0b00000001);
        assert_eq!(word.parity, 1);
    }

    #[test]
    fn test_status_parity_update() {
        let mut word = StatusWord::from_value(0b0000000010101010);
        assert_eq!(word.parity, 1);

        word.set_address(0b00000001);
        assert_eq!(word.parity, 0);
    }

    #[test]
    fn test_command_is_valid() {
        let word = CommandWord::from_value(0b0000000000101010);
        assert!(word.parity == 0);
        assert!(word.check_parity());
    }

    #[test]
    fn test_command_is_invalid() {
        let mut word = CommandWord::from_value(0b0000000000101010);
        word.parity = 1; // make parity wrong
        assert!(!word.check_parity());
    }

    #[test]
    fn test_command_set_parity_odd() {
        let word = CommandWord::from_value(0b0000000000101010);
        assert!(word.parity == 0);
    }

    #[test]
    fn test_command_set_parity_even() {
        let word = CommandWord::from_value(0b0000000010101010);
        assert!(word.parity == 1);
    }

    #[test]
    fn test_status_set_parity_odd() {
        let word = StatusWord::from_value(0b0000000000101010);
        assert!(word.parity == 0);
    }

    #[test]
    fn test_status_set_parity_even() {
        let word = StatusWord::from_value(0b0000000010101010);
        assert!(word.parity == 1);
    }

    #[test]
    fn test_command_get_address() {
        let word = CommandWord::new().with_address(0b11111);
        assert!(word.address().is_broadcast());
    }

    #[test]
    fn test_command_set_address() {
        let mut word = CommandWord::new();
        word.set_address(0b10101);
        assert_eq!(word.as_value(), 0b1010100000000000);
    }

    #[test]
    fn test_command_set_broadcast_address() {
        let word = CommandWord::new().with_address(0b11111);
        assert!(word.address().is_broadcast());
    }

    #[test]
    fn test_command_get_subaddress_ones() {
        let word = CommandWord::new().with_subaddress(0b11111);
        assert!(word.subaddress().is_mode_code());
    }

    #[test]
    fn test_command_get_subaddress_zeroes() {
        let word = CommandWord::new().with_subaddress(0b00000);
        assert!(word.subaddress().is_mode_code());
    }

    #[test]
    fn test_command_set_subaddress() {
        let mut word = CommandWord::new();
        word.set_subaddress(0b10101);
        assert_eq!(word.as_value(), 0b0000001010100000);
    }

    #[test]
    fn test_command_get_set_transmit_receive() {
        let mut word = CommandWord::new();
        assert!(word.transmit_receive().is_receive());

        word.set_transmit_receive(TransmitReceive::Transmit);
        assert!(word.transmit_receive().is_transmit());
    }

    #[test]
    fn test_command_get_set_mode_code() {
        let mut word = CommandWord::new();
        assert_eq!(word.mode_code(), ModeCode::DynamicBusControl);

        word.set_mode_code(ModeCode::InitiateSelfTest);
        assert_eq!(word.mode_code(), ModeCode::InitiateSelfTest);
    }

    #[test]
    fn test_command_get_set_word_count() {
        let mut word = CommandWord::new();
        assert_eq!(word.word_count(), 32);

        word.set_word_count(5);
        assert_eq!(word.word_count(), 5);
    }

    #[test]
    fn test_command_is_mode_code() {
        let mut word = CommandWord::new();
        assert!(word.is_mode_code());

        word.set_subaddress(0b01010);
        assert!(!word.is_mode_code());
    }

    #[test]
    fn test_command_is_transmit() {
        let mut word = CommandWord::new();
        assert!(!word.is_transmit());

        word.set_transmit_receive(TransmitReceive::Transmit);
        assert!(word.is_transmit());
    }

    #[test]
    fn test_command_is_receive() {
        let mut word = CommandWord::new();
        assert!(word.is_receive());

        word.set_transmit_receive(TransmitReceive::Transmit);
        assert!(!word.is_receive());
    }

    #[test]
    fn test_status_get_address() {
        let word = StatusWord::new().with_address(0b11111);
        assert!(word.address().is_broadcast());
    }

    #[test]
    fn test_status_get_set_address() {
        let mut word = StatusWord::new();
        assert_eq!(word.address(), Address::Value(0));

        word.set_address(0b10101);
        assert_eq!(word.address(), Address::Value(0b10101));

        word.set_address(0b11111);
        assert_eq!(word.address(), Address::Broadcast(0b11111));
    }

    #[test]
    fn test_status_get_set_instrumentation() {
        let mut word = StatusWord::new();
        assert!(word.instrumentation().is_status());

        word.set_instrumentation(Instrumentation::Command);
        assert!(word.instrumentation().is_command());
    }

    #[test]
    fn test_status_get_set_service_request() {
        let mut word = StatusWord::new();
        assert!(word.service_request().is_noservice());

        word.set_service_request(ServiceRequest::Service);
        assert!(word.service_request().is_service());
    }

    #[test]
    fn test_status_get_set_reserved() {
        let mut word = StatusWord::new();
        assert!(word.reserved().is_none());

        word.set_reserved(Reserved::Value(0b111));
        assert!(word.reserved().is_value());
    }

    #[test]
    fn test_status_get_set_broadcast_received() {
        let mut word = StatusWord::new();
        assert_eq!(word.broadcast_received(), BroadcastReceived::NotReceived);

        word.set_broadcast_received(BroadcastReceived::Received);
        assert_eq!(word.broadcast_received(), BroadcastReceived::Received);
    }

    #[test]
    fn test_status_get_set_terminal_busy() {
        let mut word = StatusWord::new();
        assert_eq!(word.terminal_busy(), TerminalBusy::NotBusy);

        word.set_terminal_busy(TerminalBusy::Busy);
        assert_eq!(word.terminal_busy(), TerminalBusy::Busy);
    }

    #[test]
    fn test_status_get_set_dynamic_bus_acceptance() {
        let mut word = StatusWord::new();
        assert_eq!(word.dynamic_bus_acceptance(), DynamicBusAcceptance::NotAccepted);

        word.set_dynamic_bus_acceptance(DynamicBusAcceptance::Accepted);
        assert_eq!(word.dynamic_bus_acceptance(), DynamicBusAcceptance::Accepted);
    }

    #[test]
    fn test_status_get_set_message_error() {
        let mut word = StatusWord::new();
        assert_eq!(word.message_error(), MessageError::None);

        word.set_message_error(MessageError::Error);
        assert_eq!(word.message_error(), MessageError::Error);
    }

    #[test]
    fn test_status_get_set_subsystem_error() {
        let mut word = StatusWord::new();
        assert_eq!(word.subsystem_error(), SubsystemError::None);

        word.set_subsystem_error(SubsystemError::Error);
        assert_eq!(word.subsystem_error(), SubsystemError::Error);
    }

    #[test]
    fn test_status_get_set_terminal_error() {
        let mut word = StatusWord::new();
        assert_eq!(word.terminal_error(), TerminalError::None);

        word.set_terminal_error(TerminalError::Error);
        assert_eq!(word.terminal_error(), TerminalError::Error);
    }

    #[test]
    fn test_data_bytes() {
        let word = DataWord::from(0b0110100001101001);
        let data: [u8;2] = word.into();
        assert_eq!(data, [0b01101000, 0b01101001]);
    }

    #[test]
    fn test_command_bytes() {
        let word = CommandWord::from_value(0b0110100001101001);
        let data = word.as_bytes();
        assert_eq!(data, [0b01101000, 0b01101001]);
    }

    #[test]
    fn test_status_bytes() {
        let word = StatusWord::from_value(0b0110100001101001);
        let data = word.as_bytes();
        assert_eq!(data, [0b01101000, 0b01101001]);
    }
}
