#![no_std]

//! A "word" in the 1553B standard is made up of twenty bits, total. Three sync bits, 
//! 16 bits of data (in one of three different formats), and a trailing parity
//! bit. This means that there are two ways of referencing a particular bit- either with 
//! a bit index offset from the beginning of the *word data* or as a "bit time" offset
//! from the begining of the word, including the sync bits.
//!
//! | Index  | Sync1 | Sync2 | Sync3 |  0 |  1 |  2 |  3 |  4 |  5 |  6 |  7 |  8 |  9 | 10 | 11 | 12 | 13 | 14 | 15 | Parity |
//! |--------|---    |---    |---    |----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|---     |
//! | Time   | -     | -     | -     |  4 |  5 |  6 |  7 |  8 |  9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | -      |
//! | Offset | -     | -     | -     | 15 | 14 | 13 | 12 | 11 | 10 |  9 |  8 |  7 |  6 |  5 |  4 |  3 |  2 |  1 |  0 | -      |
//!
//! The bit-time reference is used in the standard, but because we're only dealing with 
//! the 16-bit data from each word in this project we'll be using a zero-indexed reference
//! in the actual code.

pub mod errors;
pub mod fields;
pub mod flags;
pub mod word;

// mod message;
// mod parse;
// mod packets;
