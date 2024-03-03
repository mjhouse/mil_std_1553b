use super::{CommandWord, DataWord, StatusWord};

/// The sync waveform preceding a word
///
/// This flag is derived from the 3-bit sync waveform
/// preceding each transmitted word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Sync {
    /// The sync waveform indicates data
    Data = 0b001,

    /// The sync waveform indicates command or status
    Service = 0b100,
}

impl From<u8> for Sync {
    fn from(value: u8) -> Self {
        match value {
            0b100 => Self::Service,
            _ => Self::Data,
        }
    }
}

impl From<Sync> for u8 {
    fn from(value: Sync) -> Self {
        match value {
            Sync::Service => 0b100,
            Sync::Data => 0b001,
        }
    }
}

/// Container enum for the different kinds of words
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    None,
    Command(CommandWord),
    Status(StatusWord),
    Data(DataWord),
}

impl Sync {
    /// Get the value of the enum as a u8
    #[must_use = "Value is created but never used"]
    pub fn value(&self) -> u8 {
        (*self).into()
    }

    /// Check if the enum is the Data variant
    #[must_use = "Result of check is never used"]
    pub const fn is_data(&self) -> bool {
        matches!(self, Self::Data)
    }

    /// Check if the enum is the Service variant
    #[must_use = "Result of check is never used"]
    pub const fn is_service(&self) -> bool {
        matches!(self, Self::Service)
    }
}

impl Type {
    /// Check if contained word is command
    #[must_use = "Result of check is never used"]
    pub fn is_command(&self) -> bool {
        matches!(self, Self::Command(_))
    }

    /// Check if contained word is status
    #[must_use = "Result of check is never used"]
    pub fn is_status(&self) -> bool {
        matches!(self, Self::Status(_))
    }

    /// Check if contained word is data
    #[must_use = "Result of check is never used"]
    pub fn is_data(&self) -> bool {
        matches!(self, Self::Data(_))
    }

    /// Check if there is a contained word
    #[must_use = "Result of check is never used"]
    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    /// Check if there is no contained word
    #[must_use = "Result of check is never used"]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Get the number of associated data words
    pub fn data(&self) -> usize {
        match self {
            Self::Command(c) => c.count(),
            _ => 0,
        }
    }
}
