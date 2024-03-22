//! Words transmitted across a bus

mod enums;
mod traits;
mod words;

pub use enums::WordType;
pub use traits::{Header, Word};
pub use words::{CommandWord, DataWord, StatusWord};
