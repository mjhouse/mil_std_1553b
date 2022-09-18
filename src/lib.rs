#![no_std]

use core::marker::PhantomData;

mod flags;
mod errors;
mod constants;
mod layout;

use constants::*;
use flags::*;
use errors::*;

macro_rules! set {
    ( $target:expr, $field:ident, $value:expr ) => {
        $target &= !$field.mask;
        $target |= ((($value as u16) << $field.offset) & $field.mask);
    }
}

macro_rules! get {
    ( $target:expr, $field:ident ) => {
        (($target & $field.mask) >> $field.offset) as u8
    }
}

pub fn take(data: &[u8]) -> Word {
    
    // remove and take the first two bytes and the 
    // first nibble of a third (leaving 0000 in place of the nibble)
    
    // put first three bits in a u8
    
    // put last bit in a u8
    
    // reorganize the remaining 16 bits into a u16
    
    // check the parity bit to determine whether the word was 
    // transmitted without error
    
    // check the first 3 bits to determine whether the word is 
    // a status/command or data word
    
    // check the the instrumentation bit to determine if a 
    // word is status or command.
    
    Word::None
}

#[derive(Debug,PartialEq)]
pub enum Word {
    None,
    Command(CommandWord),
    Status(StatusWord),
    Data(DataWord)
}

#[derive(Debug,PartialEq)]
pub struct CommandWord {
    data: u16,
}

#[derive(Debug,PartialEq)]
pub struct StatusWord {
    data: u16,
}

#[derive(Debug,PartialEq)]
pub struct DataWord {
    data: u16,
}

impl CommandWord {

    pub fn new(data: u16) -> Self {
        Self { data }
    }

    pub fn address(&self) -> Address {
        let address = self.get_terminal_address();
        Address::Terminal(address)
    }

    pub fn subaddress(&self) -> Address {
        match self.get_subaddress() {
            // 0 or 31 is a mode code command
            0 | 31  => Address::None,
            address => Address::Subsystem(address),
        }
    }

    pub fn has_mode_code(&self) -> bool {
        self.subaddress() == Address::None
    }

    pub fn mode_code(&self) -> Result<ModeCode> {
        if self.has_mode_code() {
            ModeCode::from_data(
                self.get_transmit_receive(),
                self.get_mode_code()
            )
        } else {
            Err(NOT_MODE_CODE)
        }
    }

    pub fn get_terminal_address(&self) -> u8 {
        get!(self.data,COMMAND_TERMINAL_ADDRESS_FIELD)
    }

    pub fn set_terminal_address(&mut self, value: u8) {
        set!(self.data,COMMAND_TERMINAL_ADDRESS_FIELD,value);
    }

    pub fn get_transmit_receive(&self) -> u8 {
        get!(self.data,COMMAND_TRANSMIT_RECEIVE_FIELD)
    }

    pub fn set_transmit_receive(&mut self, value: u8) {
        set!(self.data,COMMAND_TRANSMIT_RECEIVE_FIELD,value);
    }

    pub fn get_subaddress(&self) -> u8 {
        get!(self.data,COMMAND_SUBADDRESS_FIELD)
    }

    pub fn set_subaddress(&mut self, value: u8) {
        set!(self.data,COMMAND_SUBADDRESS_FIELD,value);
    }

    pub fn get_mode_code(&self) -> u8 {
        get!(self.data,COMMAND_MODE_CODE_FIELD)
    }

    pub fn set_mode_code(&mut self, value: u8) {
        set!(self.data,COMMAND_MODE_CODE_FIELD,value);
    }

    pub fn get_word_count(&self) -> u8 {
        get!(self.data,COMMAND_WORD_COUNT_FIELD)
    }

    pub fn set_word_count(&mut self, value: u8) {
        set!(self.data,COMMAND_WORD_COUNT_FIELD,value);
    }

}

impl StatusWord {

    pub fn new(data: u16) -> Self {
        Self { data }
    }

    pub fn address(&self) -> Address {
        let address = self.get_terminal_address();
        Address::Terminal(address)
    }

    pub fn is_busy(&self) -> bool {
        match self.get_busy() {
            0 => false,
            _ => true,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.get_reserved() == 0
    }

    pub fn get_terminal_address(&self) -> u8 {
        get!(self.data,STATUS_TERMINAL_ADDRESS_FIELD)
    }

    pub fn set_terminal_address(&mut self, value: u8) {
        set!(self.data,STATUS_TERMINAL_ADDRESS_FIELD,value);
    }

    pub fn get_message_error(&self) -> u8 {
        get!(self.data,STATUS_MESSAGE_ERROR_FIELD)
    }

    pub fn set_message_error(&mut self, value: u8) {
        set!(self.data,STATUS_MESSAGE_ERROR_FIELD,value);
    }

    pub fn get_instrumentation(&self) -> u8 {
        get!(self.data,STATUS_INSTRUMENTATION_FIELD)
    }

    pub fn set_instrumentation(&mut self, value: u8) {
        set!(self.data,STATUS_INSTRUMENTATION_FIELD,value);
    }

    pub fn get_service_request(&self) -> u8 {
        get!(self.data,STATUS_SERVICE_REQUEST_FIELD)
    }

    pub fn set_service_request(&mut self, value: u8) {
        set!(self.data,STATUS_SERVICE_REQUEST_FIELD,value);
    }

    pub fn get_reserved(&self) -> u8 {
        get!(self.data,STATUS_RESERVED_BITS_FIELD)
    }

    pub fn set_reserved(&mut self, value: u8) {
        set!(self.data,STATUS_RESERVED_BITS_FIELD,value);
    }

    pub fn get_broadcast_command_received(&self) -> u8 {
        get!(self.data,STATUS_BROADCAST_RECEIVED_FIELD)
    }

    pub fn set_broadcast_command_received(&mut self, value: u8) {
        set!(self.data,STATUS_BROADCAST_RECEIVED_FIELD,value);
    }

    pub fn get_busy(&self) -> u8 {
        get!(self.data,STATUS_TERMINAL_BUSY_FIELD)
    }

    pub fn set_busy(&mut self, value: u8) {
        set!(self.data,STATUS_TERMINAL_BUSY_FIELD,value);
    }

    pub fn get_subsystem_flag(&self) -> u8 {
        get!(self.data,STATUS_SUBSYSTEM_FLAG_FIELD)
    }

    pub fn set_subsystem_flag(&mut self, value: u8) {
        set!(self.data,STATUS_SUBSYSTEM_FLAG_FIELD,value);
    }

    pub fn get_dynamic_bus_acceptance(&self) -> u8 {
        get!(self.data,STATUS_BUS_CONTROL_ACCEPT_FIELD)
    }

    pub fn set_dynamic_bus_acceptance(&mut self, value: u8) {
        set!(self.data,STATUS_BUS_CONTROL_ACCEPT_FIELD,value);
    }

    pub fn get_terminal_flag(&self) -> u8 {
        get!(self.data,STATUS_TERMINAL_FLAG_FIELD)
    }

    pub fn set_terminal_flag(&mut self, value: u8) {
        set!(self.data,STATUS_TERMINAL_FLAG_FIELD,value);
    }

}

impl DataWord {

    pub fn new(data: u16) -> Self {
        Self { data }
    }

    pub fn data(&self) -> u16 {
        self.data
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    
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
