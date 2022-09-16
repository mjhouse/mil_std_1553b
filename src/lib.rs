#![no_std]

use core::marker::PhantomData;

mod flags;
mod errors;
mod constants;
mod layout;

use constants::*;

macro_rules! set {
    ( $target:expr, $field:ident, $value:expr ) => {
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

pub enum Word {
    None,
    Command(CommandWord),
    Status(StatusWord),
    Data(DataWord)
}

pub struct CommandWord {
    data: u16,
}

pub struct StatusWord {
    data: u16,
}

pub struct DataWord {
    data: u16,
}

impl CommandWord {

    pub fn new(data: u16) -> Self {
        Self { data }
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

    pub fn get_sub_address(&self) -> u8 {
        get!(self.data,COMMAND_SUBADDRESS_FIELD)
    }

    pub fn set_sub_address(&mut self, value: u8) {
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
    
    macro_rules! verify_set {
        ( $word:ty, $function:ident, $result:expr ) => {
            // create a new instance of the word type
            let mut word = <$word>::new(0);

            // call the function on the word 
            <$word>::$function(&mut word, 0b11111111);

            // assert that the u16 data is correct
            assert_eq!(word.data,$result);
        }
    }

    macro_rules! verify_get {
        ( $word:ty, $function:ident, $result:expr ) => {
            // create a new instance of the word type
            let mut word = <$word>::new(FULL_WORD);

            // call the function on the word 
            let value = <$word>::$function(&mut word);

            // assert that the u8 result is correct
            assert_eq!(value,$result);
        }
    }

    #[test]
    fn test_status_set_terminal_flag() {
        verify_set!(StatusWord,set_terminal_flag,0b0000000000000001);
    }

    #[test]
    fn test_status_get_terminal_flag() {
        verify_get!(StatusWord,get_terminal_flag,0b00000001);
    }

    #[test]
    fn test_status_set_dynamic_bus_acceptance() {
        verify_set!(StatusWord,set_dynamic_bus_acceptance,0b0000000000000010);
    }

    #[test]
    fn test_status_get_dynamic_bus_acceptance() {
        verify_get!(StatusWord,get_dynamic_bus_acceptance,0b00000001);
    }

    #[test]
    fn test_status_set_subsystem_flag() {
        verify_set!(StatusWord,set_subsystem_flag,0b0000000000000100);
    }

    #[test]
    fn test_status_get_subsystem_flag() {
        verify_get!(StatusWord,get_subsystem_flag,0b00000001);
    }

    #[test]
    fn test_status_set_busy() {
        verify_set!(StatusWord,set_busy,0b0000000000001000);
    }

    #[test]
    fn test_status_get_busy() {
        verify_get!(StatusWord,get_busy,0b00000001);
    }

    #[test]
    fn test_status_set_broadcast_command_received() {
        verify_set!(StatusWord,set_broadcast_command_received,0b0000000000010000);
    }

    #[test]
    fn test_status_get_broadcast_command_received() {
        verify_get!(StatusWord,get_broadcast_command_received,0b00000001);
    }

    #[test]
    fn test_status_set_service_request() {
        verify_set!(StatusWord,set_service_request,0b0000000100000000);
    }

    #[test]
    fn test_status_get_service_request() {
        verify_get!(StatusWord,get_service_request,0b00000001);
    }

    #[test]
    fn test_status_set_instrumentation() {
        verify_set!(StatusWord,set_instrumentation,0b0000001000000000);
    }

    #[test]
    fn test_status_get_instrumentation() {
        verify_get!(StatusWord,get_instrumentation,0b00000001);
    }

    #[test]
    fn test_status_set_message_error() {
        verify_set!(StatusWord,set_message_error,0b0000010000000000);
    }

    #[test]
    fn test_status_get_message_error() {
        verify_get!(StatusWord,get_message_error,0b00000001);
    }

    #[test]
    fn test_command_set_mode_code() {
        verify_set!(CommandWord,set_mode_code,0b0000000000011111);
    }

    #[test]
    fn test_command_get_mode_code() {
        verify_get!(CommandWord,get_mode_code,0b00011111);
    }

    #[test]
    fn test_command_set_sub_address() {
        verify_set!(CommandWord,set_sub_address,0b0000001111100000);
    }

    #[test]
    fn test_command_get_sub_address() {
        verify_get!(CommandWord,get_sub_address,0b00011111);
    }

    #[test]
    fn test_command_set_transmit_receive() {
        verify_set!(CommandWord,set_transmit_receive,0b0000010000000000);
    }

    #[test]
    fn test_command_get_transmit_receive() {
        verify_get!(CommandWord,get_transmit_receive,0b00000001);
    }

    #[test]
    fn test_command_set_terminal_address() {
        verify_set!(CommandWord,set_terminal_address,0b1111100000000000);
    }

    #[test]
    fn test_command_get_terminal_address() {
        verify_get!(CommandWord,get_terminal_address,0b00011111);
    }

    #[test]
    fn test_status_set_terminal_address() {
        verify_set!(StatusWord,set_terminal_address,0b1111100000000000);
    }

    #[test]
    fn test_status_get_terminal_address() {
        verify_get!(StatusWord,get_terminal_address,0b00011111);
    }

}
