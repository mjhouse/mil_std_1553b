use crate::fields::*;
use crate::flags::*;
use crate::errors::*;

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Word {
    None,
    Command(CommandWord),
    Status(StatusWord),
    Data(DataWord)
}

impl Default for Word {
    fn default() -> Self {
        Self::None
    }
}

/// The Command Word (CW) specifies the function that a remote terminal is
/// to perform. Only the active bus controller transmits this word
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct CommandWord {
    data: u16,
}

/// A remote terminal in response to a valid message transmits only the status
/// word (SW). The status word is used to convey to the bus controller
/// whether a message was properly received or to convey the state of the
/// remote terminal (i.e., service request, busy, etc.)
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct StatusWord {
    data: u16,
}

/// The Data Word (DW) contains the actual information that is being
/// transferred within a message.
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct DataWord {
    data: u16,
}

impl Word {

    pub fn command(data: [u8;2]) -> Self {
        Word::Command(CommandWord::combine(data))
    }

    pub fn status(data: [u8;2]) -> Self {
        Word::Status(StatusWord::combine(data))
    }

    pub fn data(data: [u8;2]) -> Self {
        Word::Data(DataWord::combine(data))
    }

    pub fn mode_code_command(data: [u8;2]) -> Result<Self> {
        Some(CommandWord::combine(data))
            .filter(|c| c.is_mode_code())
            .map(|c| Word::Command(c))
            .ok_or(Error::MessageBad)
    }

    pub fn transmit_command(data: [u8;2]) -> Result<Self> {
        Some(CommandWord::combine(data))
            .filter(|c| c.is_transmit())
            .map(|c| Word::Command(c))
            .ok_or(Error::MessageBad)
    }

    pub fn receive_command(data: [u8;2]) -> Result<Self> {
        Some(CommandWord::combine(data))
            .filter(|c| c.is_receive())
            .map(|c| Word::Command(c))
            .ok_or(Error::MessageBad)
    }

    pub fn is_command(&self) -> bool {
        match self {
            Self::Command(_) => true,
            _ => false,
        }
    }

    pub fn is_status(&self) -> bool {
        match self {
            Self::Status(_) => true,
            _ => false,
        }
    }

    pub fn is_data(&self) -> bool {
        match self {
            Self::Data(_) => true,
            _ => false,
        }
    }


    /// Returns `true` if the word is [`None`].
    ///
    /// [`None`]: Word::None
    #[must_use]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

impl CommandWord {

    pub fn new(data: u16) -> Self {
        Self { data }
    }

    pub fn combine(data: [u8;2]) -> Self {
        let content = ((data[0] as u16) << 8) | data[1] as u16;
        Self::new(((data[0] as u16) << 8) | data[1] as u16)
    }

    /// Extract the address field
    pub fn address(&self) -> Address {
        let address = self.get_terminal_address();
        Address::Terminal(address)
    }

    /// Extract the subaddress field
    /// 
    /// If the SUBADDRESS field is 0b00000 or 0b11111, then this is a mode code 
    /// command and the SUBADRESS field should be decoded as Address::None.
    pub fn subaddress(&self) -> Address {
        match self.get_subaddress() {
            // 0 or 31 is a mode code command
            0 | 31  => Address::None,
            address => Address::Subsystem(address),
        }
    }

    /// Check if this is a mode code command
    /// 
    /// If the Subaddress field is None (0b00000 or 0b11111), then this is a mode code
    /// command.
    pub fn is_mode_code(&self) -> bool {
        self.subaddress() == Address::None
    }

    /// Decode the MODE_CODE field as a value
    /// 
    /// If the Subaddress field is NOT 0b00000 or 0b11111, then the MODE_CODE/WORD_COUNT
    /// field should be interpreted as a mode code enum. The ModeCode value is decoded
    /// from the T/R bit (TRANSMIT_RECEIVE field) and the MODE_CODE field values.
    pub fn mode_code(&self) -> Result<ModeCode> {
        if self.is_mode_code() {
            self.get_mode_code()
                .try_into()
                .map_err(|_| Error::InvalidCode)
        } else {
            Err(Error::NotModeCode)
        }
    }

    /// Decode the WORD_COUNT field as a value
    /// 
    /// If the Subaddress field is 0b00000 or 0b11111, then the MODE_CODE/WORD_COUNT
    /// field defines the number of data words to be received or transmitted (depending
    /// on the TRANSMIT_RECEIVE flag). A word count of 0b00000 here is decoded as 32 
    /// data words.
    pub fn word_count(&self) -> u8 {
        match self.get_word_count() {
            0 => 32,
            v => v,
        }
    }

    pub fn is_receive(&self) -> bool {
        self.get_transmit_receive() == 0
    }

    pub fn is_transmit(&self) -> bool {
        self.get_transmit_receive() == 1
    }

    /// Get terminal_address field value
    pub fn get_terminal_address(&self) -> u8 {
        COMMAND_TERMINAL_ADDRESS_FIELD.get(self.data)
    }

    /// Set terminal_address field value
    pub fn set_terminal_address(&mut self, value: u8) {
        self.data = COMMAND_TERMINAL_ADDRESS_FIELD.set(self.data,value);
    }

    /// Get transmit_receive field value
    pub fn get_transmit_receive(&self) -> u8 {
        COMMAND_TRANSMIT_RECEIVE_FIELD.get(self.data)
    }

    /// Set transmit_receive field value
    pub fn set_transmit_receive(&mut self, value: u8) {
        self.data = COMMAND_TRANSMIT_RECEIVE_FIELD.set(self.data,value);
    }

    /// Get subaddress field value
    pub fn get_subaddress(&self) -> u8 {
        COMMAND_SUBADDRESS_FIELD.get(self.data)
    }

    /// Set subaddress field value
    pub fn set_subaddress(&mut self, value: u8) {
        self.data = COMMAND_SUBADDRESS_FIELD.set(self.data,value);
    }

    /// Get mode_code field value
    pub fn get_mode_code(&self) -> u8 {
        COMMAND_MODE_CODE_FIELD.get(self.data)
    }

    /// Set mode_code field value
    pub fn set_mode_code(&mut self, value: u8) {
        self.data = COMMAND_MODE_CODE_FIELD.set(self.data,value);
    }

    /// Get word_count field value
    pub fn get_word_count(&self) -> u8 {
        COMMAND_WORD_COUNT_FIELD.get(self.data)
    }

    /// Set word_count field value
    pub fn set_word_count(&mut self, value: u8) {
        self.data = COMMAND_WORD_COUNT_FIELD.set(self.data,value);
    }

}

impl StatusWord {

    pub fn new(data: u16) -> Self {
        Self { data }
    }

    pub fn combine(data: [u8;2]) -> Self {
        Self::new(((data[0] as u16) << 8) + data[1] as u16)
    }

    /// Extract the address field
    pub fn address(&self) -> Address {
        let address = self.get_terminal_address();
        Address::Terminal(address)
    }

    /// Get terminal busy status
    ///
    /// The busy bit is provided as a feedback to the bus controller as
    /// to when the remote terminal is unable to move data between the remote
    /// terminal electronics and the subsystem in compliance to a command from
    /// the bus controller.
    pub fn is_busy(&self) -> bool {
        match self.get_busy() {
            0 => false,
            _ => true,
        }
    }

    /// Check if the reserved field has been used
    ///
    /// Currently, no other checks are performed on status words- data in the 
    /// reserved field is the only way a status word can be in an internally
    /// invalid state.
    pub fn is_valid(&self) -> bool {
        self.get_reserved() == 0
    }

    /// Get terminal_address field value
    pub fn get_terminal_address(&self) -> u8 {
        STATUS_TERMINAL_ADDRESS_FIELD.get(self.data)
    }

    /// Set terminal_address field value
    pub fn set_terminal_address(&mut self, value: u8) {
        self.data = STATUS_TERMINAL_ADDRESS_FIELD.set(self.data,value);
    }

    /// Get message_error field value
    pub fn get_message_error(&self) -> u8 {
        STATUS_MESSAGE_ERROR_FIELD.get(self.data)
    }

    /// Set message_error field value
    pub fn set_message_error(&mut self, value: u8) {
        self.data = STATUS_MESSAGE_ERROR_FIELD.set(self.data,value);
    }

    /// Get instrumentation field value
    pub fn get_instrumentation(&self) -> u8 {
        STATUS_INSTRUMENTATION_FIELD.get(self.data)
    }

    /// Set instrumentation field value
    pub fn set_instrumentation(&mut self, value: u8) {
        self.data = STATUS_INSTRUMENTATION_FIELD.set(self.data,value);
    }

    /// Get service_request field value
    pub fn get_service_request(&self) -> u8 {
        STATUS_SERVICE_REQUEST_FIELD.get(self.data)
    }

    /// Set service_request field value
    pub fn set_service_request(&mut self, value: u8) {
        self.data = STATUS_SERVICE_REQUEST_FIELD.set(self.data,value);
    }

    /// Get reserved field value
    pub fn get_reserved(&self) -> u8 {
        STATUS_RESERVED_BITS_FIELD.get(self.data)
    }

    /// Set reserved field value
    pub fn set_reserved(&mut self, value: u8) {
        self.data = STATUS_RESERVED_BITS_FIELD.set(self.data,value);
    }

    /// Get broadcast_command_received field value
    pub fn get_broadcast_command_received(&self) -> u8 {
        STATUS_BROADCAST_RECEIVED_FIELD.get(self.data)
    }

    /// Set broadcast_command_received field value
    pub fn set_broadcast_command_received(&mut self, value: u8) {
        self.data = STATUS_BROADCAST_RECEIVED_FIELD.set(self.data,value);
    }

    /// Get busy field value
    pub fn get_busy(&self) -> u8 {
        STATUS_TERMINAL_BUSY_FIELD.get(self.data)
    }

    /// Set busy field value
    pub fn set_busy(&mut self, value: u8) {
        self.data = STATUS_TERMINAL_BUSY_FIELD.set(self.data,value);
    }

    /// Get subsystem_flag field value
    pub fn get_subsystem_flag(&self) -> u8 {
        STATUS_SUBSYSTEM_FLAG_FIELD.get(self.data)
    }

    /// Set subsystem_flag field value
    pub fn set_subsystem_flag(&mut self, value: u8) {
        self.data = STATUS_SUBSYSTEM_FLAG_FIELD.set(self.data,value);
    }

    /// Get dynamic_bus_acceptance field value
    pub fn get_dynamic_bus_acceptance(&self) -> u8 {
        STATUS_BUS_CONTROL_ACCEPT_FIELD.get(self.data)
    }

    /// Set dynamic_bus_acceptance field value
    pub fn set_dynamic_bus_acceptance(&mut self, value: u8) {
        self.data = STATUS_BUS_CONTROL_ACCEPT_FIELD.set(self.data,value);
    }

    /// Get terminal_flag field value
    pub fn get_terminal_flag(&self) -> u8 {
        STATUS_TERMINAL_FLAG_FIELD.get(self.data)
    }

    /// Set terminal_flag field value
    pub fn set_terminal_flag(&mut self, value: u8) {
        self.data = STATUS_TERMINAL_FLAG_FIELD.set(self.data,value);
    }

}

impl DataWord {

    pub fn new(data: u16) -> Self {
        Self { data }
    }

    pub fn combine(data: [u8;2]) -> Self {
        Self::new(((data[0] as u16) << 8) + data[1] as u16)
    }

    pub fn data(&self) -> u16 {
        self.data
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packets::*;
    
    // only useable with no_std commented out
    macro_rules! debug_bytes {
        ( $w: expr ) => {
            println!("bits: {:#018b}", $w);
        }
    }

    macro_rules! verify {
        ( get_ones: $word:ty, $function:ident, $final:expr ) => {
            let mut word = <$word>::new(FULL_WORD);
            assert_eq!(<$word>::$function(&mut word),$final);
        };
        ( set_ones: $word:ty, $function:ident, $final:expr ) => {
            let mut word = <$word>::new(EMPTY_WORD);
            <$word>::$function(&mut word, 0b11111111);
            assert_eq!(word.data,$final);
        };
        ( set_zeros: $word:ty, $function:ident, $final:expr ) => {
            let mut word = <$word>::new(FULL_WORD);
            <$word>::$function(&mut word, 0b00000000);
            assert_eq!(word.data,$final);
        }
    }

    // #[test]
    // fn test_combine_bytes() {
    //     let byte1 = 0b01010101;
    //     let byte2 = 0b01010101;

    //     let result = combine([byte1,byte2]);
    //     assert_eq!(result,0b0101010101010101);
    // }

    #[test]
    fn test_status_set_ones_terminal_flag() {
        verify!(set_ones: StatusWord,set_terminal_flag,0b0000000000000001);
    }

    #[test]
    fn test_status_set_zeros_terminal_flag() {
        verify!(set_zeros: StatusWord,set_terminal_flag,0b1111111111111110);
    }

    #[test]
    fn test_status_get_ones_terminal_flag() {
        verify!(get_ones: StatusWord,get_terminal_flag,0b00000001);
    }

    #[test]
    fn test_status_set_ones_dynamic_bus_acceptance() {
        verify!(set_ones: StatusWord,set_dynamic_bus_acceptance,0b0000000000000010);
    }

    #[test]
    fn test_status_set_zeros_dynamic_bus_acceptance() {
        verify!(set_zeros: StatusWord,set_dynamic_bus_acceptance,0b1111111111111101);
    }

    #[test]
    fn test_status_get_ones_dynamic_bus_acceptance() {
        verify!(get_ones: StatusWord,get_dynamic_bus_acceptance,0b00000001);
    }

    #[test]
    fn test_status_set_ones_subsystem_flag() {
        verify!(set_ones: StatusWord,set_subsystem_flag,0b0000000000000100);
    }

    #[test]
    fn test_status_set_zeros_subsystem_flag() {
        verify!(set_zeros: StatusWord,set_subsystem_flag,0b1111111111111011);
    }

    #[test]
    fn test_status_get_ones_subsystem_flag() {
        verify!(get_ones: StatusWord,get_subsystem_flag,0b00000001);
    }

    #[test]
    fn test_status_set_ones_busy() {
        verify!(set_ones: StatusWord,set_busy,0b0000000000001000);
    }

    #[test]
    fn test_status_set_zeros_busy() {
        verify!(set_zeros: StatusWord,set_busy,0b1111111111110111);
    }

    #[test]
    fn test_status_get_ones_busy() {
        verify!(get_ones: StatusWord,get_busy,0b00000001);
    }

    #[test]
    fn test_status_set_ones_broadcast_command_received() {
        verify!(set_ones: StatusWord,set_broadcast_command_received,0b0000000000010000);
    }

    #[test]
    fn test_status_set_zeros_broadcast_command_received() {
        verify!(set_zeros: StatusWord,set_broadcast_command_received,0b1111111111101111);
    }

    #[test]
    fn test_status_get_ones_broadcast_command_received() {
        verify!(get_ones: StatusWord,get_broadcast_command_received,0b00000001);
    }

    #[test]
    fn test_status_set_ones_service_request() {
        verify!(set_ones: StatusWord,set_service_request,0b0000000100000000);
    }

    #[test]
    fn test_status_set_zeros_service_request() {
        verify!(set_zeros: StatusWord,set_service_request,0b1111111011111111);
    }

    #[test]
    fn test_status_get_ones_service_request() {
        verify!(get_ones: StatusWord,get_service_request,0b00000001);
    }

    #[test]
    fn test_status_set_ones_reserved() {
        verify!(set_ones: StatusWord,set_reserved,0b0000000011100000);
    }

    #[test]
    fn test_status_set_zeros_reserved() {
        verify!(set_zeros: StatusWord,set_reserved,0b1111111100011111);
    }

    #[test]
    fn test_status_get_ones_reserved() {
        verify!(get_ones: StatusWord,get_reserved,0b00000111);
    }

    #[test]
    fn test_status_set_ones_instrumentation() {
        verify!(set_ones: StatusWord,set_instrumentation,0b0000001000000000);
    }

    #[test]
    fn test_status_set_zeros_instrumentation() {
        verify!(set_zeros: StatusWord,set_instrumentation,0b1111110111111111);
    }

    #[test]
    fn test_status_get_ones_instrumentation() {
        verify!(get_ones: StatusWord,get_instrumentation,0b00000001);
    }

    #[test]
    fn test_status_set_ones_message_error() {
        verify!(set_ones: StatusWord,set_message_error,0b0000010000000000);
    }

    #[test]
    fn test_status_set_zeros_message_error() {
        verify!(set_zeros: StatusWord,set_message_error,0b1111101111111111);
    }

    #[test]
    fn test_status_get_ones_message_error() {
        verify!(get_ones: StatusWord,get_message_error,0b00000001);
    }

    #[test]
    fn test_command_set_ones_mode_code() {
        verify!(set_ones: CommandWord,set_mode_code,0b0000000000011111);
    }

    #[test]
    fn test_command_set_zeros_mode_code() {
        verify!(set_zeros: CommandWord,set_mode_code,0b1111111111100000);
    }

    #[test]
    fn test_command_get_ones_mode_code() {
        verify!(get_ones: CommandWord,get_mode_code,0b00011111);
    }

    #[test]
    fn test_command_set_ones_subaddress() {
        verify!(set_ones: CommandWord,set_subaddress,0b0000001111100000);
    }

    #[test]
    fn test_command_set_zeros_subaddress() {
        verify!(set_zeros: CommandWord,set_subaddress,0b1111110000011111);
    }

    #[test]
    fn test_command_get_ones_subaddress() {
        verify!(get_ones: CommandWord,get_subaddress,0b00011111);
    }

    #[test]
    fn test_command_set_ones_transmit_receive() {
        verify!(set_ones: CommandWord,set_transmit_receive,0b0000010000000000);
    }

    #[test]
    fn test_command_set_zeros_transmit_receive() {
        verify!(set_zeros: CommandWord,set_transmit_receive,0b1111101111111111);
    }

    #[test]
    fn test_command_get_ones_transmit_receive() {
        verify!(get_ones: CommandWord,get_transmit_receive,0b00000001);
    }

    #[test]
    fn test_command_set_ones_terminal_address() {
        verify!(set_ones: CommandWord,set_terminal_address,0b1111100000000000);
    }

    #[test]
    fn test_command_set_zeros_terminal_address() {
        verify!(set_zeros: CommandWord,set_terminal_address,0b0000011111111111);
    }

    #[test]
    fn test_command_get_ones_terminal_address() {
        verify!(get_ones: CommandWord,get_terminal_address,0b00011111);
    }

    #[test]
    fn test_status_set_ones_terminal_address() {
        verify!(set_ones: StatusWord,set_terminal_address,0b1111100000000000);
    }

    #[test]
    fn test_status_set_zeros_terminal_address() {
        verify!(set_zeros: StatusWord,set_terminal_address,0b0000011111111111);
    }

    #[test]
    fn test_status_get_ones_terminal_address() {
        verify!(get_ones: StatusWord,get_terminal_address,0b00011111);
    }

    #[test]
    fn test_command_set_address() {
        let mut word = CommandWord::new(EMPTY_WORD);
        word.set_terminal_address(0b11111);
        assert_eq!(word.address(),Address::Terminal(0b11111));
    }

    #[test]
    fn test_status_set_address() {
        let mut word = StatusWord::new(EMPTY_WORD);
        word.set_terminal_address(0b11111);
        assert_eq!(word.address(),Address::Terminal(0b11111));
    }

    #[test]
    fn test_command_set_subaddress_zeros() {
        let mut word = CommandWord::new(EMPTY_WORD);
        word.set_subaddress(0b00000);
        assert_eq!(word.subaddress(),Address::None);
    }

    #[test]
    fn test_command_set_subaddress_ones() {
        let mut word = CommandWord::new(EMPTY_WORD);
        word.set_subaddress(0b00011111);
        assert_eq!(word.subaddress(),Address::None);
    }

    #[test]
    fn test_command_set_subaddress_other() {
        let mut word = CommandWord::new(EMPTY_WORD);
        word.set_subaddress(0b10101);
        assert_eq!(word.subaddress(),Address::Subsystem(0b10101));
    }

    #[test]
    fn test_status_is_valid_true() {
        let mut word = StatusWord::new(EMPTY_WORD);
        word.set_reserved(0b111);
        assert!(!word.is_valid());
    }

    #[test]
    fn test_status_is_valid_false() {
        let mut word = StatusWord::new(EMPTY_WORD);
        assert!(word.is_valid());
    }

}