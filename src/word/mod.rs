#[macro_use]
mod traits;
mod enums;
mod word;

pub use traits::Word;

pub use enums::{Parity, Sync, Type};

pub use word::{CommandWord, DataWord, StatusWord};
