
#[macro_use]
mod traits;
mod enums;
mod word;

pub use traits::Word;

pub use enums::{
    Parity,
    Type,
    Sync,
};

pub use word::{
    CommandWord,
    StatusWord,
    DataWord
};