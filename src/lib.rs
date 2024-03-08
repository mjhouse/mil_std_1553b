#![no_std]
#![warn(missing_docs)]
#![deny(
    arithmetic_overflow,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    variant_size_differences
)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod errors;
pub mod fields;
pub mod flags;
pub mod message;
pub mod word;

pub use message::{Message, Packet};

pub use errors::{Error, Result};

pub use word::{CommandWord, DataWord, StatusWord};

pub use flags::{
    Address, BroadcastCommand, BusControlAccept, Instrumentation, ModeCode, Reserved,
    ServiceRequest, SubAddress, TerminalBusy, TransmitReceive,
};
