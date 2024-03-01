//! Words transmitted across a bus

mod enums;
mod words;

pub use enums::{Sync, Type};

pub use words::{CommandWord, DataWord, StatusWord};
