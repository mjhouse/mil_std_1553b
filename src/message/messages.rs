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
/// 
/// ## Example
/// 
/// ```rust
/// # use mil_std_1553b::*;
/// # fn try_main() -> Result<()> {
///     let message = Message::new()
///         .with_command(CommandWord::new()
///             .with_subaddress(12)
///             .with_subaddress(5)
///             .with_word_count(2)
///             .build()?
///         )?
///         .with_data(DataWord::new())?
///         .with_data(DataWord::new())?;
/// 
///     assert!(message.is_full());
///     assert_eq!(message.word_count(),3);
///     assert_eq!(message.data_count(),2);
///     assert_eq!(message.data_expected(),2);
/// # Ok(())
/// # }
/// ```
/// 
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Message {
    count: usize,
    words: [Word; MAX_WORDS],
}

impl Message {
    /// Create a new message struct
    pub fn new() -> Self {
        Self {
            count: 0,
            words: [Word::None; MAX_WORDS],
        }
    }

    /// Check if the message is full
    #[must_use = "Returned value is not used"]
    pub fn is_full(&self) -> bool {
        self.data_count() == self.data_expected()
    }

    /// Check if the message is empty
    #[must_use = "Returned value is not used"]
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Clear all words from the message
    pub fn clear(&mut self) {
        self.count = 0;
        self.words = [Word::None; MAX_WORDS];
    }

    /// Get the last word in the message
    pub fn last(&self) -> Option<&Word> {
        match self.count {
            0 => None,
            i => self.words.get(i - 1),
        }
    }

    /// Get the first word in the message
    pub fn first(&self) -> Option<&Word> {
        match self.count {
            0 => None,
            _ => self.words.get(0),
        }
    }

    /// Get the number of words
    pub fn word_count(&self) -> usize {
        self.count
    }

    /// Get the number of data words
    pub fn data_count(&self) -> usize {
        self.words
            .iter()
            .take_while(|w| w.is_some())
            .filter(|w| w.is_data())
            .count()
    }

    /// Get the expected number of data words
    pub fn data_expected(&self) -> usize {
        self.first().map(Word::data_count).unwrap_or(0)
    }

    /// Check if message has data words
    #[must_use = "Returned value is not used"]
    pub fn has_data(&self) -> bool {
        self.data_count() > 0
    }

    /// Check if message can contain more data words
    #[must_use = "Returned value is not used"]
    pub fn has_space(&self) -> bool {
        self.data_count() < self.data_expected()
    }

    /// Check if message starts with a command word
    #[must_use = "Returned value is not used"]
    pub fn has_command(&self) -> bool {
        self.first().map(Word::is_command).unwrap_or(false)
    }

    /// Check if message starts with a status word
    #[must_use = "Returned value is not used"]
    pub fn has_status(&self) -> bool {
        self.first().map(Word::is_status).unwrap_or(false)
    }

    /// Add a generic word to the message, returning size on success
    pub fn add<T: Into<Word>>(&mut self, word: T) -> Result<usize> {
        match word.into() {
            Word::Data(v) => self.add_data(v),
            Word::Status(v) => self.add_status(v),
            Word::Command(v) => self.add_command(v),
            _ => Err(Error::WordIsInvalid),
        }
    }

    /// Constructor method to add a word to the message
    pub fn with_word<T: Into<Word>>(mut self, word: T) -> Result<Self> {
        self.add(word)?;
        Ok(self)
    }

    /// Add a data word, returning the size of the message on success
    pub fn add_data(&mut self, word: DataWord) -> Result<usize> {
        if self.is_full() && self.has_command() {
            Err(Error::MessageIsFull)
        } else if self.is_empty() {
            Err(Error::FirstWordIsData)
        } else {
            self.words[self.count] = Word::Data(word);
            self.count += 1;
            Ok(self.count)
        }
    }

    /// Constructor method to add a data word to the message
    pub fn with_data<T: Into<DataWord>>(mut self, word: T) -> Result<Self> {
        self.add_data(word.into())?;
        Ok(self)
    }

    /// Add a status word, returning the size of the message on success
    pub fn add_status(&mut self, word: StatusWord) -> Result<usize> {
        if !self.is_empty() {
            Err(Error::StatusWordNotFirst)
        } else if !word.check_parity() {
            Err(Error::InvalidWord)
        } else {
            self.words[self.count] = Word::Status(word);
            self.count += 1;
            Ok(self.count)
        }
    }

    /// Constructor method to add a status word to the message
    pub fn with_status<T: Into<StatusWord>>(mut self, word: T) -> Result<Self> {
        self.add_status(word.into())?;
        Ok(self)
    }

    /// Add a command word, returning the size of the message on success
    pub fn add_command(&mut self, word: CommandWord) -> Result<usize> {
        if !self.is_empty() {
            Err(Error::CommandWordNotFirst)
        } else if !word.check_parity() {
            Err(Error::InvalidWord)
        } else {
            self.words[self.count] = Word::Command(word);
            self.count += 1;
            Ok(self.count)
        }
    }

    /// Constructor method to add a command word to the message
    pub fn with_command<T: Into<CommandWord>>(mut self, word: T) -> Result<Self> {
        self.add_command(word.into())?;
        Ok(self)
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

        assert_eq!(message.is_full(), true);
        assert_eq!(message.is_empty(), true);
        assert_eq!(message.first(), None);
        assert_eq!(message.last(), None);

        assert_eq!(message.word_count(), 0);
        assert_eq!(message.data_count(), 0);
    }

    #[test]
    fn test_message_command_add() {
        let mut message = Message::new();

        let word = Word::Command(CommandWord::from_data(0b0001100001100010));
        let result = message.add(word.clone());

        assert_eq!(result, Ok(1));
        assert_eq!(message.first(), Some(&word));
        assert_eq!(message.last(), Some(&word));
    }

    #[test]
    fn test_message_command_data() {
        let mut message = Message::new();

        let word = Word::Command(CommandWord::from_data(0b0001100001100010));
        message.add(word.clone()).unwrap();

        assert_eq!(message.word_count(), 1);
        assert_eq!(message.data_count(), 0);
        assert_eq!(message.data_expected(), 2);
    }

    #[test]
    fn test_message_command_add_data() {
        let mut message = Message::new();

        let word = Word::Command(CommandWord::from_data(0b0001100001100010));
        message.add(word.clone()).unwrap();

        let data = Word::Data(DataWord::from_data(0b0110100001101001));
        message.add(data.clone()).unwrap();

        assert_eq!(message.word_count(), 2);
        assert_eq!(message.data_count(), 1);
    }

    #[test]
    fn test_message_status_add() {
        let mut message = Message::new();

        let word = Word::Status(StatusWord::from_data(0b0001100000000010));
        let result = message.add(word.clone());

        assert_eq!(result, Ok(1));
        assert_eq!(message.first(), Some(&word));
        assert_eq!(message.last(), Some(&word));
    }

    #[test]
    fn test_message_status_no_data() {
        let mut message = Message::new();

        let word = Word::Status(StatusWord::from_data(0b0001100000000010));
        message.add(word.clone()).unwrap();

        assert_eq!(message.word_count(), 1);
        assert_eq!(message.data_count(), 0);
        assert_eq!(message.data_expected(), 0);
    }

    #[test]
    fn test_message_status_add_data() {
        let mut message = Message::new();

        let status = Word::Status(StatusWord::from_data(0b0001100000000000));
        message.add(status.clone()).unwrap();

        let data = Word::Data(DataWord::from_data(0b0110100001101001));
        message.add(data.clone()).unwrap();

        assert_eq!(message.word_count(), 2);
        assert_eq!(message.data_count(), 1);

        // status words don't have a word count field, and the
        // number of data words following a status word is set
        // by an earlier request.
        assert_eq!(message.data_expected(), 0);
    }
}
