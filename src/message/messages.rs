use crate::word::Type as Word;
use crate::word::{CommandWord, DataWord, StatusWord};
use crate::{errors::*, Packet};

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

    /// Parse a slice of bytes into a command message
    ///
    /// This method interpretes the byte array as a series
    /// of 20-bit long words, beginning with a command word.
    /// The word count of the parsed command word will
    /// determine how many data words are parsed.
    ///
    /// Each word is a triplet containing 3-bit sync, 16-bit word,
    /// and 1-bit parity. It is assumed that the message
    /// being parsed is aligned to the beginning of the slice:
    ///  
    /// aligned:
    ///      | 11111111 | 11111111 | 11110000 |
    /// unaligned:
    ///      | 00001111 | 11111111 | 11111111 |
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use mil_std_1553b::*;
    /// # fn try_main() -> Result<()> {
    ///     let message = Message::parse_command(&[
    ///         0b10000011,
    ///         0b00001100,
    ///         0b01010110,
    ///         0b10000110,
    ///         0b10010000
    ///     ])?;
    ///
    ///     assert!(message.is_full());
    ///     assert!(message.has_command());
    ///     assert_eq!(message.word_count(),2);
    ///     assert_eq!(message.data_count(),1);
    /// # Ok(())
    /// # }
    pub fn parse_command(data: &[u8]) -> Result<Self> {
        // get the first word as a command word
        let mut message = Self::new().with_command(Packet::parse(data, 0)?.to_command()?)?;

        // get the number of data words expected
        let num = message.data_expected();

        let sbit = 20; // starting data bit
        let ebit = 20 * (num + 1); // ending data bit

        // iterate chunks of 20 bits for each word
        for bit in (sbit..ebit).step_by(20) {
            let index = bit / 8; // byte index in the slice
            let offset = bit % 8; // bit index in the last byte

            // get a trimmed slice to parse
            let bytes = &data[index..];

            // parse as a data word and add
            message.add_data(Packet::parse(bytes, offset)?.to_data()?)?;
        }

        Ok(message)
    }

    /// Parse a slice of bytes into a status message
    ///
    /// This method interpretes the byte array as a series
    /// of 20-bit long words, starting with a status word.
    /// Because status words do not have a word count field,
    /// this method will parse data words to the end of the
    /// byte array.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use mil_std_1553b::*;
    /// # fn try_main() -> Result<()> {
    ///     let message = Message::parse_status(&[
    ///         0b10000011,
    ///         0b00001100,
    ///         0b01010110,
    ///         0b10000110,
    ///         0b10010000
    ///     ])?;
    ///
    ///     // the message is not full because we haven't hit
    ///     // the maximum number of words.
    ///     assert!(!message.is_full());
    ///     assert!(message.has_status());
    ///     assert_eq!(message.word_count(),2);
    ///     assert_eq!(message.data_count(),1);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// See [Message::parse_command] for more information.
    pub fn parse_status(data: &[u8]) -> Result<Self> {
        // get the first word as a status word
        let mut message = Self::new().with_status(Packet::parse(data, 0)?.to_status()?)?;

        let bits = data.len() * 8;
        let step = 20;
        let sbit = step;
        let ebit = (bits / step) * step;

        // iterate chunks of 20 bits for each word
        for bit in (sbit..ebit).step_by(step) {
            let index = bit / 8; // byte index in the slice
            let offset = bit % 8; // offset into a byte

            // get a trimmed slice to parse
            let bytes = &data[index..];

            println!("{}:", index);
            for b in bytes {
                println!("{:08b}", b);
            }

            // parse as a data word and add
            message.add_data(Packet::parse(bytes, offset)?.to_data()?)?;
        }

        Ok(message)
    }

    /// Check if the message is full
    #[must_use = "Returned value is not used"]
    pub fn is_full(&self) -> bool {
        if self.has_command() {
            self.data_count() == self.data_expected()
        } else {
            self.count == self.words.len()
        }
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
        self.first()
            .map(Word::data_count)
            .unwrap_or(0)
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
        self.first()
            .map(Word::is_command)
            .unwrap_or(false)
    }

    /// Check if message starts with a status word
    #[must_use = "Returned value is not used"]
    pub fn has_status(&self) -> bool {
        self.first()
            .map(Word::is_status)
            .unwrap_or(false)
    }

    /// Add a word to the message, returning size on success
    pub fn add<T: Into<Word>>(&mut self, word: T) -> Result<usize> {
        match word.into() {
            Word::Data(v) => self.add_data(v),
            Word::Status(v) => self.add_status(v),
            Word::Command(v) => self.add_command(v),
            _ => Err(Error::WordIsInvalid),
        }
    }

    /// Get a data word from the message by index
    ///
    /// An index of 0 will return the first *data word*, not
    /// the leading command or status word.
    pub fn get(&self, index: usize) -> Option<&DataWord> {
        if let Some(Word::Data(w)) = &self.words.get(index + 1) {
            Some(w)
        } else {
            None
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
    fn test_parse_command_three_data_words() {
        let message = Message::parse_command(&[
            0b10000011, 0b00001100, 0b01110010, 0b11010000, 0b11010010, 0b00101111, 0b00101101,
            0b11100010, 0b11001110, 0b11011110,
        ])
        .unwrap();

        assert!(message.is_full());
        assert!(message.has_command());
        assert_eq!(message.word_count(), 4);
        assert_eq!(message.data_count(), 3);

        let word0 = message
            .get(0)
            .and_then(|w| w.as_string().ok())
            .unwrap_or("");
        let word1 = message
            .get(1)
            .and_then(|w| w.as_string().ok())
            .unwrap_or("");
        let word2 = message
            .get(2)
            .and_then(|w| w.as_string().ok())
            .unwrap_or("");

        assert_eq!(word0, "hi");
        assert_eq!(word1, "yo");
        assert_eq!(word2, "go");
    }

    #[test]
    fn test_parse_command_two_data_words() {
        let message = Message::parse_command(&[
            0b10000011, 0b00001100, 0b01000010, 0b11010000, 0b11010010, 0b00101111, 0b00101101,
            0b11100000,
        ])
        .unwrap();

        assert!(message.is_full());
        assert!(message.has_command());
        assert_eq!(message.word_count(), 3);
        assert_eq!(message.data_count(), 2);
    }

    #[test]
    fn test_parse_command_one_data_word() {
        let message =
            Message::parse_command(&[0b10000011, 0b00001100, 0b00100010, 0b11010000, 0b11010010])
                .unwrap();

        assert!(message.is_full());
        assert!(message.has_command());
        assert_eq!(message.word_count(), 2);
        assert_eq!(message.data_count(), 1);
    }

    #[test]
    fn test_parse_status_three_data_words() {
        let message = Message::parse_status(&[
            0b10000011, 0b00001100, 0b01110010, 0b11010000, 0b11010010, 0b00101111, 0b00101101,
            0b11100010, 0b11001110, 0b11011110,
        ])
        .unwrap();

        assert!(!message.is_full());
        assert!(message.has_status());
        assert_eq!(message.word_count(), 4);
        assert_eq!(message.data_count(), 3);

        let word0 = message
            .get(0)
            .and_then(|w| w.as_string().ok())
            .unwrap_or("");
        let word1 = message
            .get(1)
            .and_then(|w| w.as_string().ok())
            .unwrap_or("");
        let word2 = message
            .get(2)
            .and_then(|w| w.as_string().ok())
            .unwrap_or("");

        assert_eq!(word0, "hi");
        assert_eq!(word1, "yo");
        assert_eq!(word2, "go");
    }

    #[test]
    fn test_parse_status_two_data_words() {
        let message = Message::parse_status(&[
            0b10000011, 0b00001100, 0b01000010, 0b11010000, 0b11010010, 0b00101111, 0b00101101,
            0b11100000,
        ])
        .unwrap();

        assert!(!message.is_full());
        assert!(message.has_status());
        assert_eq!(message.word_count(), 3);
        assert_eq!(message.data_count(), 2);
    }

    #[test]
    fn test_parse_status_one_data_word() {
        let message =
            Message::parse_status(&[0b10000011, 0b00001100, 0b00100010, 0b11010000, 0b11010010])
                .unwrap();

        assert!(!message.is_full());
        assert!(message.has_status());
        assert_eq!(message.word_count(), 2);
        assert_eq!(message.data_count(), 1);
    }

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
        message
            .add(word.clone())
            .unwrap();

        assert_eq!(message.word_count(), 1);
        assert_eq!(message.data_count(), 0);
        assert_eq!(message.data_expected(), 2);
    }

    #[test]
    fn test_message_command_add_data() {
        let mut message = Message::new();

        let word = Word::Command(CommandWord::from_data(0b0001100001100010));
        message
            .add(word.clone())
            .unwrap();

        let data = Word::Data(DataWord::from_data(0b0110100001101001));
        message
            .add(data.clone())
            .unwrap();

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
        message
            .add(word.clone())
            .unwrap();

        assert_eq!(message.word_count(), 1);
        assert_eq!(message.data_count(), 0);
        assert_eq!(message.data_expected(), 0);
    }

    #[test]
    fn test_message_status_add_data() {
        let mut message = Message::new();

        let status = Word::Status(StatusWord::from_data(0b0001100000000000));
        message
            .add(status.clone())
            .unwrap();

        let data = Word::Data(DataWord::from_data(0b0110100001101001));
        message
            .add(data.clone())
            .unwrap();

        assert_eq!(message.word_count(), 2);
        assert_eq!(message.data_count(), 1);

        // status words don't have a word count field, and the
        // number of data words following a status word is set
        // by an earlier request.
        assert_eq!(message.data_expected(), 0);
    }
}
