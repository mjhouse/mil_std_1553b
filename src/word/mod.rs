//! Words transmitted across a bus

mod enums;
mod words;

pub use enums::Type;
pub use words::{CommandWord, DataWord, StatusWord};
