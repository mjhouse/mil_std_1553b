#![no_std]
#![deny(clippy::unwrap_used)]
#![doc = include_str!("../README.md")]

pub mod errors;
pub mod fields;
pub mod flags;
pub mod message;
pub mod word;

pub use message::Message;

pub use errors::{Error, Result};

pub use word::{CommandWord, DataWord, StatusWord};

pub use flags::{
    Address, BroadcastCommand, BusControlAccept, Instrumentation, ModeCode, Reserved,
    ServiceRequest, SubAddress, TerminalBusy, TransmitReceive,
};
