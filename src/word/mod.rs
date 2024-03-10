//! Words transmitted across a bus

mod enums;
mod words;

pub use enums::WordType;
pub use words::{CommandWord, DataWord, StatusWord, Word};
