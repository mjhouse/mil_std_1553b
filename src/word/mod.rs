//! Words transmitted across a bus

#[macro_use]
mod traits;
mod enums;
mod words;

pub use traits::Word;

pub use enums::{Parity, Sync, Type};

pub use words::{CommandWord, DataWord, StatusWord};
