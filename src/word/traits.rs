use crate::{Result, WordType};

/// Common functionality for service words
pub trait Header
where
    Self: Sized + Into<WordType>,
{
    /// The number of data words expected
    fn count(&self) -> Option<usize>;
}

/// Common functionality for all words
pub trait Word
where
    Self: Sized + Into<WordType>,
{
    /// Create an empty word
    fn new() -> Self;

    /// Constructor method to set the word from a u16
    fn with_value(self, data: u16) -> Self;

    /// Constructor method to set the word from bytes
    fn with_bytes(self, data: [u8; 2]) -> Self;

    /// Constructor method to explicitly set the parity
    fn with_parity(self, parity: u8) -> Self;

    /// Constructor method to calculate a parity bit
    fn with_calculated_parity(self) -> Self;

    /// Finish and validate construction of a word
    fn build(self) -> Result<Self>;

    /// Create a word from a u16
    fn from_value(data: u16) -> Self;

    /// Create a word from two bytes
    fn from_bytes(data: [u8; 2]) -> Self;

    /// Get the internal data as a slice
    fn as_bytes(&self) -> [u8; 2];

    /// Get the internal data as u16
    fn as_value(&self) -> u16;

    /// Set the internal data as a slice
    fn set_bytes(&mut self, data: [u8; 2]);

    /// Set the internal data as u16
    fn set_value(&mut self, data: u16);

    /// Get the current parity bit
    fn parity(&self) -> u8;

    /// Set the current parity bit
    fn set_parity(&mut self, parity: u8);

    /// Get a calculated parity bit
    fn calculate_parity(&self) -> u8;

    /// Check if the current parity bit is correct
    fn check_parity(&self) -> bool;
}
