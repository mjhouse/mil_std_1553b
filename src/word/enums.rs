use crate::Word;

use super::{CommandWord, DataWord, StatusWord};

/// Container enum for the different kinds of words
#[derive(Debug, Clone, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wordtype_clone_equal_command() {
        let item1 = WordType::Command(0b1010101010101010.into());
        let item2 = item1.clone();
        assert!(item1 == item2);
    }

    #[test]
    fn test_wordtype_clone_equal_status() {
        let item1 = WordType::Status(0b1010101010101010.into());
        let item2 = item1.clone();
        assert!(item1 == item2);
    }

    #[test]
    fn test_wordtype_clone_equal_data() {
        let item1 = WordType::Data(0b1010101010101010.into());
        let item2 = item1.clone();
        assert!(item1 == item2);
    }

    #[test]
    fn test_wordtype_equal_command() {
        let item1 = WordType::Command(0b1010101010101010.into());
        let item2 = WordType::Command(0b1010101010101010.into());
        assert!(item1 == item2);
    }

    #[test]
    fn test_wordtype_not_equal_command() {
        let item1 = WordType::Command(0b0000000000000000.into());
        let item2 = WordType::Command(0b1010101010101010.into());
        assert!(item1 != item2);
    }

    #[test]
    fn test_wordtype_equal_status() {
        let item1 = WordType::Status(0b1010101010101010.into());
        let item2 = WordType::Status(0b1010101010101010.into());
        assert!(item1 == item2);
    }

    #[test]
    fn test_wordtype_not_equal_status() {
        let item1 = WordType::Status(0b0000000000000000.into());
        let item2 = WordType::Status(0b1010101010101010.into());
        assert!(item1 != item2);
    }

    #[test]
    fn test_wordtype_equal_data() {
        let item1 = WordType::Data(0b1010101010101010.into());
        let item2 = WordType::Data(0b1010101010101010.into());
        assert!(item1 == item2);
    }

    #[test]
    fn test_wordtype_not_equal_data() {
        let item1 = WordType::Data(0b0000000000000000.into());
        let item2 = WordType::Data(0b1010101010101010.into());
        assert!(item1 != item2);
    }

    #[test]
    fn test_wordtype_is_command() {
        let item = WordType::from(CommandWord::new());
        assert!(item.is_command());
    }

    #[test]
    fn test_wordtype_is_status() {
        let item = WordType::from(StatusWord::new());
        assert!(item.is_status());
    }

    #[test]
    fn test_wordtype_is_data() {
        let item = WordType::from(DataWord::new());
        assert!(item.is_data());
    }

    #[test]
    fn test_wordtype_is_some_command() {
        let item = WordType::from(CommandWord::new());
        assert!(item.is_some());
        assert!(!item.is_none());
    }

    #[test]
    fn test_wordtype_is_some_status() {
        let item = WordType::from(StatusWord::new());
        assert!(item.is_some());
        assert!(!item.is_none());
    }

    #[test]
    fn test_wordtype_is_some_data() {
        let item = WordType::from(DataWord::new());
        assert!(item.is_some());
        assert!(!item.is_none());
    }

    #[test]
    fn test_wordtype_is_none() {
        let item = WordType::None;
        assert!(!item.is_some());
        assert!(item.is_none());
    }

    #[test]
    fn test_wordtype_bytes_command() {
        let item = WordType::Command(0b1010101010101010.into());
        assert_eq!(item.bytes(),[0b10101010,0b10101010]);
    }

    #[test]
    fn test_wordtype_bytes_status() {
        let item = WordType::Status(0b1010101010101010.into());
        assert_eq!(item.bytes(),[0b10101010,0b10101010]);
    }

    #[test]
    fn test_wordtype_bytes_data() {
        let item = WordType::Data(0b1010101010101010.into());
        assert_eq!(item.bytes(),[0b10101010,0b10101010]);
    }

    #[test]
    fn test_wordtype_bytes_nont() {
        let item = WordType::None;
        assert_eq!(item.bytes(),[0,0]);
    }

    #[test]
    fn test_wordtype_parity_command() {
        let item = WordType::from(CommandWord::new().with_parity(1));
        assert_eq!(item.parity(),1);
    }

    #[test]
    fn test_wordtype_parity_status() {
        let item = WordType::from(StatusWord::new().with_parity(1));
        assert_eq!(item.parity(),1);
    }

    #[test]
    fn test_wordtype_parity_data() {
        let item = WordType::from(DataWord::new().with_parity(1));
        assert_eq!(item.parity(),1);
    }

    #[test]
    fn test_wordtype_parity_none() {
        let item = WordType::None;
        assert_eq!(item.parity(),0);
    }

}