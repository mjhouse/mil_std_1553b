#![no_std]
// #![warn(
//     clippy::all,
//     clippy::restriction,
//     clippy::pedantic,
//     clippy::nursery,
//     clippy::cargo,
// )]
#![doc = include_str!("../README.md")]

pub mod errors;
pub mod fields;
pub mod flags;
pub mod message;
pub mod word;
