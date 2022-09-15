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

pub enum Word {
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

    // macro_rules! debug {
    //     ( $w: expr ) => {
    //         println!("bits: {:#018b}", $w);
    //     }
    // }

    #[test]
    fn test_status_set_get_terminal_flag() {
        let mut word = StatusWord::new(0);
        word.set_terminal_flag(1);
        let flag = word.get_terminal_flag();
        assert_eq!(flag,1);
    }

    #[test]
    fn test_status_set_get_dynamic_bus_acceptance() {
        let mut word = StatusWord::new(0);
        word.set_dynamic_bus_acceptance(1);
        let flag = word.get_dynamic_bus_acceptance();
        assert_eq!(flag,1);
    }

    #[test]
    fn test_status_set_get_subsystem_flag() {
        let mut word = StatusWord::new(0);
        word.set_subsystem_flag(1);
        let flag = word.get_subsystem_flag();
        assert_eq!(flag,1);
    }

    #[test]
    fn test_status_set_get_busy() {
        let mut word = StatusWord::new(0);
        word.set_busy(1);
        let flag = word.get_busy();
        assert_eq!(flag,1);
    }

    #[test]
    fn test_status_set_get_broadcast_command_received() {
        let mut word = StatusWord::new(0);
        word.set_broadcast_command_received(1);
        let flag = word.get_broadcast_command_received();
        assert_eq!(flag,1);
    }

    #[test]
    fn test_status_set_get_service_request() {
        let mut word = StatusWord::new(0);
        word.set_service_request(1);
        let flag = word.get_service_request();
        assert_eq!(flag,1);
    }

    #[test]
    fn test_status_set_get_instrumentation() {
        let mut word = StatusWord::new(0);
        word.set_instrumentation(1);
        let flag = word.get_instrumentation();
        assert_eq!(flag,1);
    }

    #[test]
    fn test_status_set_get_message_error() {
        let mut word = StatusWord::new(0);
        word.set_message_error(1);
        let flag = word.get_message_error();
        assert_eq!(flag,1);
    }

    #[test]
    fn test_command_set_get_mode_code() {
        let mut word = CommandWord::new(0);
        word.set_mode_code(31);
        let code = word.get_mode_code();
        assert_eq!(code,31);
    }

    #[test]
    fn test_command_set_get_sub_address() {
        let mut word = CommandWord::new(0);
        word.set_sub_address(31);
        let address = word.get_sub_address();
        assert_eq!(address,31);
    }

    #[test]
    fn test_command_set_get_tr() {
        let mut word = CommandWord::new(0);
        word.set_transmit_receive(1);
        let flag = word.get_transmit_receive();
        assert_eq!(flag,1);
    }

    #[test]
    fn test_command_set_get_address() {
        let mut word = CommandWord::new(0);
        word.set_terminal_address(31);
        let address = word.get_terminal_address();
        assert_eq!(address,31);
    }

    #[test]
    fn test_status_set_get_address() {
        let mut word = StatusWord::new(0);
        word.set_terminal_address(31);
        let address = word.get_terminal_address();
        assert_eq!(address,31);
    }

}
