use crate::Word;

use super::{CommandWord, DataWord, StatusWord};

/// Container enum for the different kinds of words
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordType {
    /// No contained word
    None,

    /// Command word
    Command(CommandWord),

    /// Status word
    Status(StatusWord),

    /// Data word
    Data(DataWord),
}

impl WordType {
    /// Check if contained word is command
    #[must_use = "Returned value is not used"]
    pub fn is_command(&self) -> bool {
        matches!(self, Self::Command(_))
    }

    /// Check if contained word is status
    #[must_use = "Returned value is not used"]
    pub fn is_status(&self) -> bool {
        matches!(self, Self::Status(_))
    }

    /// Check if contained word is data
    #[must_use = "Returned value is not used"]
    pub fn is_data(&self) -> bool {
        matches!(self, Self::Data(_))
    }

    /// Check if there is a contained word
    #[must_use = "Returned value is not used"]
    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    /// Check if there is no contained word
    #[must_use = "Returned value is not used"]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Get the word as a byte array
    pub fn bytes(&self) -> [u8; 2] {
        match self {
            Self::Command(w) => w.into(),
            Self::Status(w) => w.into(),
            Self::Data(w) => w.into(),
            _ => [0, 0],
        }
    }

    /// Get the parity bit of the word
    pub fn parity(&self) -> u8 {
        match self {
            Self::Command(w) => w.parity(),
            Self::Status(w) => w.parity(),
            Self::Data(w) => w.parity(),
            _ => 0,
        }
    }
}

impl From<CommandWord> for WordType {
    fn from(value: CommandWord) -> Self {
        WordType::Command(value)
    }
}

impl From<StatusWord> for WordType {
    fn from(value: StatusWord) -> Self {
        WordType::Status(value)
    }
}

impl From<DataWord> for WordType {
    fn from(value: DataWord) -> Self {
        WordType::Data(value)
    }
}
