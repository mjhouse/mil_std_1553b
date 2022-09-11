#![no_std]

mod flags;
mod errors;

use errors::{Error,Result};

pub struct Word(u16);

pub struct Command(Word);

pub struct Status(Word);

pub struct Data(Word);

impl Word {

    pub fn full() -> Self {
        Self(0b1111111111111111)
    }

    pub fn empty() -> Self {
        Self(0b0000000000000000)
    }

    pub fn new(value: u16) -> Self {
        Self(value)
    }

    /// Gets the value of a particular bit in the word
    ///
    /// # Arguments
    ///
    /// * `index` - Left-to-left index position
    ///
    /// # Examples
    ///
    /// ```
    /// use mil_std_1553b::Word;
    /// 
    /// let word = Word::full();
    /// let value = word.get(0).unwrap();
    /// 
    /// assert!(value);
    /// ```
    pub fn get(&self, index: usize) -> Result<bool> {
        if index < 16 { 
            Ok((self.0 & ((1 << 15) >> index)) != 0)
        }
        else {
            Err(Error::OutOfBounds)
        }
    }

    /// Sets the value of a particular bit in the word
    ///
    /// # Arguments
    ///
    /// * `index` - Left-to-left index position
    /// * `state` - Boolean state to set (true = 1, false = 0)
    ///
    /// # Examples
    ///
    /// ```
    /// use mil_std_1553b::Word;
    /// 
    /// let mut word = Word::full();
    /// word.set(0,false).unwrap();
    /// 
    /// let value = word.get(0).unwrap();
    /// 
    /// assert!(!value);
    /// ```
    pub fn set(&mut self, index: usize, state: bool) -> Result<bool> {
        if index < 16 {
            if state {
                self.0 |= (1 << 15) >> index;
            }
            else {
                self.0 &= !((1 << 15) >> index);
            }
            Ok(state)
        }
        else {
            Err(Error::OutOfBounds)
        }
    }

}

impl Command {

    pub fn full() -> Self {
        Self(Word::full())
    }

    pub fn empty() -> Self {
        Self(Word::empty())
    }

    pub fn new(value: u16) -> Self {
        Self(Word(value))
    }

    pub fn get_terminal_address(&self) -> u8 {
        ((self.0.0 & 0b1111100000000000) >> 11) as u8
    }

    pub fn set_terminal_address(&mut self, address: u8) {
        self.0.0 |= (address as u16) << 11;
    }

    pub fn get_transmit_receive(&self) -> u8 {
        ((self.0.0 & 0b0000010000000000) >> 10) as u8
    }

    pub fn set_transmit_receive(&mut self, flag: u8) {
        self.0.0 |= (flag as u16) << 10;
    }

    pub fn get_sub_address(&self) -> u8 {
        ((self.0.0 & 0b0000001111100000) >> 5) as u8
    }

    pub fn set_sub_address(&mut self, address: u8) {
        self.0.0 |= (address as u16) << 5;
    }

    pub fn get_mode_code(&self) -> u8 {
        (self.0.0 & 0b0000000000011111) as u8
    }

    pub fn set_mode_code(&mut self, code: u8) {
        self.0.0 |= code as u16;
    }

    pub fn get_word_count(&self) -> u8 {
        self.get_mode_code()
    }

    pub fn set_word_count(&mut self, count: u8) {
        self.set_mode_code(count);
    }

}

impl Status {

    pub fn full() -> Self {
        Self(Word::full())
    }

    pub fn empty() -> Self {
        Self(Word::empty())
    }

    pub fn new(value: u16) -> Self {
        Self(Word(value))
    }

    pub fn get_terminal_address(&self) -> u8 {
        ((self.0.0 & 0b1111100000000000) >> 11) as u8
    }

    pub fn set_terminal_address(&mut self, address: u8) {
        self.0.0 |= (address as u16) << 11;
    }

    pub fn get_message_error(&self) -> u8 {
        ((self.0.0 & 0b0000010000000000) >> 10) as u8
    }

    pub fn set_message_error(&mut self, flag: u8) {
        self.0.0 |= (flag as u16) << 10;
    }

    pub fn get_instrumentation(&self) -> u8 {
        ((self.0.0 & 0b0000001000000000) >> 9) as u8
    }

    pub fn set_instrumentation(&mut self, flag: u8) {
        self.0.0 |= (flag as u16) << 9;
    }

    pub fn get_service_request(&self) -> u8 {
        ((self.0.0 & 0b0000000100000000) >> 8) as u8
    }

    pub fn set_service_request(&mut self, flag: u8) {
        self.0.0 |= (flag as u16) << 8;
    }

    pub fn get_broadcast_command_received(&self) -> u8 {
        ((self.0.0 & 0b0000000000010000) >> 4) as u8
    }

    pub fn set_broadcast_command_received(&mut self, flag: u8) {
        self.0.0 |= (flag as u16) << 4;
    }

    pub fn get_busy(&self) -> u8 {
        ((self.0.0 & 0b0000000000001000) >> 3) as u8
    }

    pub fn set_busy(&mut self, flag: u8) {
        self.0.0 |= (flag as u16) << 3;
    }

    pub fn get_subsystem_flag(&self) -> u8 {
        ((self.0.0 & 0b0000000000000100) >> 2) as u8
    }

    pub fn set_subsystem_flag(&mut self, flag: u8) {
        self.0.0 |= (flag as u16) << 2;
    }

    pub fn get_dynamic_bus_acceptance(&self) -> u8 {
        ((self.0.0 & 0b0000000000000010) >> 1) as u8
    }

    pub fn set_dynamic_bus_acceptance(&mut self, flag: u8) {
        self.0.0 |= (flag as u16) << 1;
    }

    pub fn get_terminal_flag(&self) -> u8 {
        ((self.0.0 & 0b0000000000000001) >> 0) as u8
    }

    pub fn set_terminal_flag(&mut self, flag: u8) {
        self.0.0 |= (flag as u16) << 0;
    }

}

impl Data {

    pub fn full() -> Self {
        Self(Word::full())
    }

    pub fn empty() -> Self {
        Self(Word::empty())
    }

    pub fn new(value: u16) -> Self {
        Self(Word(value))
    }

    pub fn data(&self) -> &u16 {
        &self.0.0
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! debug {
        ( $w: expr ) => {
            println!("bits: {:#018b}", $w);
        }
    }

    #[test]
    fn test_status_set_get_terminal_flag() {
        let mut word = Status::empty();
        word.set_terminal_flag(1);
        let flag = word.get_terminal_flag();
        assert_eq!(flag,1);
    }

    #[test]
    fn test_status_set_get_dynamic_bus_acceptance() {
        let mut word = Status::empty();
        word.set_dynamic_bus_acceptance(1);
        let flag = word.get_dynamic_bus_acceptance();
        assert_eq!(flag,1);
    }

    #[test]
    fn test_status_set_get_subsystem_flag() {
        let mut word = Status::empty();
        word.set_subsystem_flag(1);
        let flag = word.get_subsystem_flag();
        assert_eq!(flag,1);
    }

    #[test]
    fn test_status_set_get_busy() {
        let mut word = Status::empty();
        word.set_busy(1);
        let flag = word.get_busy();
        assert_eq!(flag,1);
    }

    #[test]
    fn test_status_set_get_broadcast_command_received() {
        let mut word = Status::empty();
        word.set_broadcast_command_received(1);
        let flag = word.get_broadcast_command_received();
        assert_eq!(flag,1);
    }

    #[test]
    fn test_status_set_get_service_request() {
        let mut word = Status::empty();
        word.set_service_request(1);
        let flag = word.get_service_request();
        assert_eq!(flag,1);
    }

    #[test]
    fn test_status_set_get_instrumentation() {
        let mut word = Status::empty();
        word.set_instrumentation(1);
        let flag = word.get_instrumentation();
        assert_eq!(flag,1);
    }

    #[test]
    fn test_status_set_get_message_error() {
        let mut word = Status::empty();
        word.set_message_error(1);
        let flag = word.get_message_error();
        assert_eq!(flag,1);
    }

    #[test]
    fn test_command_set_get_mode_code() {
        let mut word = Command::empty();
        word.set_mode_code(31);
        let code = word.get_mode_code();
        assert_eq!(code,31);
    }

    #[test]
    fn test_command_set_get_sub_address() {
        let mut word = Command::empty();
        word.set_sub_address(31);
        let address = word.get_sub_address();
        assert_eq!(address,31);
    }

    #[test]
    fn test_command_set_get_tr() {
        let mut word = Command::empty();
        word.set_transmit_receive(1);
        let flag = word.get_transmit_receive();
        assert_eq!(flag,1);
    }

    #[test]
    fn test_command_set_get_address() {
        let mut word = Command::empty();
        word.set_terminal_address(31);
        let address = word.get_terminal_address();
        assert_eq!(address,31);
    }

    #[test]
    fn test_status_set_get_address() {
        let mut word = Status::empty();
        word.set_terminal_address(31);
        let address = word.get_terminal_address();
        assert_eq!(address,31);
    }

    #[test]
    fn test_set_bit_index_positive() {
        let mut word = Word::empty();
        word.set(0,true).unwrap();
        word.set(9,true).unwrap();
        word.set(15,true).unwrap();
        let err = word.set(21,true).is_err();

        let b0 = word.get(0).unwrap();
        let b9 = word.get(9).unwrap();
        let b15 = word.get(15).unwrap();
        let b21 = word.get(21).is_err();

        assert!(b0);
        assert!(b9);
        assert!(b15);
        assert!(b21);
        assert!(err);
    }

    #[test]
    fn test_set_bit_index_negative() {
        let mut word = Word::full();
        word.set(0,false).unwrap();
        word.set(9,false).unwrap();
        word.set(15,false).unwrap();
        let err = word.set(21,false).is_err();

        let b0 = word.get(0).unwrap();
        let b9 = word.get(9).unwrap();
        let b15 = word.get(15).unwrap();
        let b21 = word.get(21).is_err();

        assert!(!b0);
        assert!(!b9);
        assert!(!b15);
        assert!(b21);
        assert!(err);
    }
}
