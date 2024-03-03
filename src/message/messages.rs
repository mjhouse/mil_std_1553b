use crate::errors::*;
use crate::word::Type as Word;
use crate::word::{CommandWord, DataWord, StatusWord};

/// a message can only contain 32 words
const MAX_WORDS: usize = 33;

/// A message sent between two terminals on the bus
///
/// The Message object does very minimal message validation
/// for the message structure:
///
/// * Command or status words are always the first word.
/// * Data words are limited based on the command word count.
/// * Messages can't exceed max message size.
///
/// It does not validate larger messaging formats that
/// require context about previous messages or terminal type.
pub struct Message {
    words: [Word; MAX_WORDS],
}

impl Message {
    /// Create a new message struct
    pub fn new() -> Self {
        Self {
            words: [Word::None; MAX_WORDS],
        }
    }

    /// Check if the message is full
    pub fn is_full(&self) -> bool {
        self.words.iter().filter(|w| w.is_none()).count() == 0
    }

    /// Check if the message is empty
    pub fn is_empty(&self) -> bool {
        self.words.iter().filter(|w| w.is_some()).count() == 0
    }

    /// Clear all words from the message
    pub fn clear(&mut self) {
        self.words = [Word::None; MAX_WORDS];
    }

    /// Get the last word in the message
    pub fn last(&self) -> Option<&Word> {
        self.words.iter().rev().find(|w| w.is_some())
    }

    /// Get the first word in the message
    pub fn first(&self) -> Option<&Word> {
        match self.words.first() {
            Some(Word::None) | None => None,
            Some(v) => Some(v)
        }
    }

    /// Get the number of words
    pub fn word_count(&self) -> usize {
        self.words.iter().filter(|w| w.is_some()).count()
    }

    /// Get the number of data words
    pub fn data_count(&self) -> usize {
        self.words.iter().filter(|w| w.is_data()).count()
    }

    /// Get the expected number of data words
    pub fn data_expected(&self) -> usize {
        self.first().map(Word::data).unwrap_or(0)
    }

    /// Check if message has data words
    pub fn has_data(&self) -> bool {
        self.data_count() > 0
    }

    /// Check if message can contain more data words
    pub fn has_space(&self) -> bool {
        self.data_count() < self.data_expected()
    }

    /// Check if message starts with a command word
    pub fn has_command(&self) -> bool {
        self.first().map(Word::is_command).unwrap_or(false)
    }

    /// Check if message starts with a status word
    pub fn has_status(&self) -> bool {
        self.first().map(Word::is_status).unwrap_or(false)
    }

    /// Add a generic word to the message, returning size on success
    pub fn add(&mut self, word: Word) -> Result<usize> {
        match word {
            Word::Data(v) => self.add_data(v),
            Word::Status(v) => self.add_status(v),
            Word::Command(v) => self.add_command(v),
            _ => Err(Error::WordIsInvalid),
        }
    }

    /// Add a data word, returning the size of the message on success
    pub fn add_data(&mut self, word: DataWord) -> Result<usize> {
        if self.is_full() {
            Err(Error::MessageIsFull)
        } else if self.is_empty() {
            Err(Error::FirstWordIsData)
        } else {
            let index = self.word_count();
            self.words[index] = Word::Data(word);
            Ok(index + 1)
        }
    }

    /// Add a status word, returning the size of the message on success
    pub fn add_status(&mut self, word: StatusWord) -> Result<usize> {
        if !self.is_empty() {
            Err(Error::StatusWordNotFirst)
        } else if !word.is_valid() {
            Err(Error::InvalidStatusWord)
        } else {
            let index = self.word_count();
            self.words[index] = Word::Status(word);
            Ok(index + 1)
        }
    }

    /// Add a command word, returning the size of the message on success
    pub fn add_command(&mut self, word: CommandWord) -> Result<usize> {
        if !self.is_empty() {
            Err(Error::CommandWordNotFirst)
        } else {
            let index = self.word_count();
            self.words[index] = Word::Command(word);
            Ok(index + 1)
        }
    }
}

impl Default for Message {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_message() {
        let message = Message::new();

        assert_eq!(message.is_full(), false);
        assert_eq!(message.is_empty(), true);
        assert_eq!(message.first(), None);
        assert_eq!(message.last(), None);

        assert_eq!(message.word_count(), 0);
        assert_eq!(message.data_count(), 0);
    }

    #[test]
    fn test_message_command_add() {
        let mut message = Message::new();

        let word = Word::Command(CommandWord::new(0b0001100001100010));
        let result = message.add(word.clone());

        assert_eq!(result, Ok(1));
        assert_eq!(message.first(), Some(&word));
        assert_eq!(message.last(), Some(&word));
    }

    #[test]
    fn test_message_command_data() {
        let mut message = Message::new();

        let word = Word::Command(CommandWord::new(0b0001100001100010));
        message.add(word.clone()).unwrap();

        assert_eq!(message.word_count(), 1);
        assert_eq!(message.data_count(), 0);
        assert_eq!(message.data_expected(), 2);
    }

    #[test]
    fn test_message_command_add_data() {
        let mut message = Message::new();

        let word = Word::Command(CommandWord::new(0b0001100001100010));
        message.add(word.clone()).unwrap();

        let data = Word::Data(DataWord::new(0b0110100001101001));
        message.add(data.clone()).unwrap();

        assert_eq!(message.word_count(), 2);
        assert_eq!(message.data_count(), 1);
    }

    #[test]
    fn test_message_status_add() {
        let mut message = Message::new();

        let word = Word::Status(StatusWord::new(0b0001100000000010));
        let result = message.add(word.clone());

        assert_eq!(result, Ok(1));
        assert_eq!(message.first(), Some(&word));
        assert_eq!(message.last(), Some(&word));
    }

    #[test]
    fn test_message_status_add_invalid() {
        let mut message = Message::new();

        // word is using the reserved bits (0b0000000011100000)
        let word = Word::Status(StatusWord::new(0b0000000011100000));
        let result = message.add(word.clone());
        assert!(result.is_err());
    }

    #[test]
    fn test_message_status_no_data() {
        let mut message = Message::new();

        let word = Word::Status(StatusWord::new(0b0001100000000010));
        message.add(word.clone()).unwrap();

        assert_eq!(message.word_count(), 1);
        assert_eq!(message.data_count(), 0);
        assert_eq!(message.data_expected(), 0);
    }

    #[test]
    fn test_message_status_add_data() {
        let mut message = Message::new();

        let status = Word::Status(StatusWord::new(0b0001100000000000));
        message.add(status.clone()).unwrap();

        let data = Word::Data(DataWord::new(0b0110100001101001));
        message.add(data.clone()).unwrap();

        assert_eq!(message.word_count(), 2);
        assert_eq!(message.data_count(), 1);

        // status words don't have a word count field, and the
        // number of data words following a status word is set
        // by an earlier request.
        assert_eq!(message.data_expected(), 0);
    }
}
