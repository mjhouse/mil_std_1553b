use crate::fields::*;
use crate::flags::{Address, Instrumentation, ServiceRequest, BroadcastCommand};
use crate::errors::{Result,Error, MessageError};

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct CommandWord {
    sync: u8,
    data: u16,
    parity: u8
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct StatusWord {
    sync: u8,
    data: u16,
    parity: u8
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct DataWord {
    sync: u8,
    data: u16,
    parity: u8
}

pub trait Word {

    fn sync(&self) -> u8;

    fn data(&self) -> u16;

    fn parity(&self) -> u8;

}

impl CommandWord {

    pub fn address(&self) -> Address {
        Address::Terminal(COMMAND_TERMINAL_ADDRESS_FIELD.get(self.data()))
    }

    pub fn subaddress(&self) -> Address {
        Address::Subsystem(COMMAND_SUBADDRESS_FIELD.get(self.data()))
    }

}

impl StatusWord {

    pub fn address(&self) -> Address {
        Address::Terminal(STATUS_TERMINAL_ADDRESS_FIELD.get(self.data()))
    }

    pub fn error(&self) -> MessageError {
        MessageError::from(STATUS_MESSAGE_ERROR_FIELD.get(self.data()))
    }

    pub fn instrumentation(&self) -> Instrumentation {
        Instrumentation::from(STATUS_INSTRUMENTATION_FIELD.get(self.data()))
    }

    pub fn service(&self) -> ServiceRequest {
        ServiceRequest::from(STATUS_SERVICE_REQUEST_FIELD.get(self.data()))
    }

    pub fn reserved(&self) -> u8 {
        STATUS_RESERVED_BITS_FIELD.get(self.data())
    }

    pub fn received(&self) -> BroadcastCommand {
        BroadcastCommand::from(STATUS_BROADCAST_RECEIVED_FIELD.get(self.data()))
    }

    pub fn is_busy(&self) -> bool {
        STATUS_TERMINAL_BUSY_FIELD.get(self.data()) > 0
    }

    pub fn subsystem(&self) -> u8 {
        STATUS_SUBSYSTEM_FLAG_FIELD.get(self.data())
    }

    pub fn acceptance(&self) -> bool {
        STATUS_DYNAMIC_BUS_ACCEPT_FIELD.get(self.data()) > 0
    }

    pub fn terminal(&self) -> u8 {
        STATUS_TERMINAL_FLAG_FIELD.get(self.data())
    }

}

impl Word for CommandWord {
    
    fn sync(&self) -> u8 {
        self.sync
    }

    fn data(&self) -> u16 {
        self.data
    }

    fn parity(&self) -> u8 {
        self.parity
    }

}

impl Word for StatusWord {
    
    fn sync(&self) -> u8 {
        self.sync
    }

    fn data(&self) -> u16 {
        self.data
    }

    fn parity(&self) -> u8 {
        self.parity
    }

}

impl Word for DataWord {
    
    fn sync(&self) -> u8 {
        self.sync
    }

    fn data(&self) -> u16 {
        self.data
    }

    fn parity(&self) -> u8 {
        self.parity
    }

}