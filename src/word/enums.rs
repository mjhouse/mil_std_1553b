use super::{CommandWord, DataWord, StatusWord};

/// Container enum for the different kinds of words
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    /// No contained word
    None,

    /// Command word
    Command(CommandWord),

    /// Status word
    Status(StatusWord),

    /// Data word
    Data(DataWord),
}

impl Type {
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

    /// Get the number of associated data words
    pub fn data_count(&self) -> usize {
        match self {
            Self::Command(c) => c.count(),
            _ => 0,
        }
    }
}

impl From<CommandWord> for Type {
    fn from(value: CommandWord) -> Self {
        Type::Command(value)
    }
}

impl From<StatusWord> for Type {
    fn from(value: StatusWord) -> Self {
        Type::Status(value)
    }
}

impl From<DataWord> for Type {
    fn from(value: DataWord) -> Self {
        Type::Data(value)
    }
}
