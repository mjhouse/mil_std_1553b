#![no_std]
#![deny(clippy::unwrap_used)]
#![doc = include_str!("../README.md")]

pub mod errors;
pub mod fields;
pub mod flags;
pub mod message;
pub mod word;

pub use message::Message;

pub use errors::{
    Result,
    Error
};

pub use word::{
    CommandWord,
    StatusWord,
    DataWord
};

pub use flags::{
    ModeCode,
    TransmitReceive,
    Address,
    SubAddress,
    Instrumentation,
    ServiceRequest,
    Reserved,
    BroadcastCommand,
    TerminalBusy,
    BusControlAccept,
};