use crate::word::WordType;
use crate::word::{CommandWord, DataWord, StatusWord};
use crate::{errors::*, Packet};

/// A message sent between two terminals on the bus
///
/// The Message struct does very minimal message validation
/// for the message structure:
///
/// * Command or status words are always the first word.
/// * Data words are limited based on the command word count.
/// * Messages can't exceed [max message size][Message::MAX_WORDS].
///
/// Messages do not validate larger messaging patterns that
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
    words: [WordType; Self::MAX_WORDS],
}

impl Message {
    /// The maximum number of words that a message can hold
    ///
    /// For messages which begin with a [StatusWord], this value
    /// is equal to **one more than** the number of [DataWords][DataWord]
    /// that the message will accept before it is full. For messages
    /// which begin with a [CommandWord], this value is the maximum
    /// number of words which may be returned by the
    /// [word_count][CommandWord::word_count] method.
    pub const MAX_WORDS: usize = 33;

    /// Create a new message struct
    pub fn new() -> Self {
        Self {
            count: 0,
            words: [WordType::None; Self::MAX_WORDS],
        }
    }

    /// Parse a slice of bytes into a command message
    ///
    /// This method interpretes the byte array as a series
    /// of 20-bit long words, beginning with a command word.
    /// The word count of the parsed command will determine
    /// how many data words are parsed.
    ///
    /// Each word is a triplet containing 3-bit sync, 16-bit word,
    /// and 1-bit parity. It is assumed that the message
    /// being parsed is aligned to the beginning of the slice.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of bytes to parse
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
    /// byte array. Slice the input data to avoid parsing
    /// any unwanted words.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of bytes to parse
    ///
    /// See [parse_command][Message::parse_command] for more information.
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

            // parse as a data word and add
            message.add_data(Packet::parse(bytes, offset)?.to_data()?)?;
        }

        Ok(message)
    }

    /// Check if the message is full
    ///
    /// This method will return false for status messages
    /// until the [maximum number of data words][Message::MAX_WORDS]
    /// has been added.
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
        self.words = [WordType::None; Self::MAX_WORDS];
    }

    /// Get the last word in the message
    pub fn last(&self) -> Option<&WordType> {
        match self.count {
            0 => None,
            i => self.words.get(i - 1),
        }
    }

    /// Get the first word in the message
    pub fn first(&self) -> Option<&WordType> {
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
            .map(WordType::data_count)
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
            .map(WordType::is_command)
            .unwrap_or(false)
    }

    /// Check if message starts with a status word
    #[must_use = "Returned value is not used"]
    pub fn has_status(&self) -> bool {
        self.first()
            .map(WordType::is_status)
            .unwrap_or(false)
    }

    /// Add a word to the message, returning size on success
    ///
    /// # Arguments
    ///
    /// * `word` - A word to add
    ///
    pub fn add<T: Into<WordType>>(&mut self, word: T) -> Result<usize> {
        match word.into() {
            WordType::Data(v) => self.add_data(v),
            WordType::Status(v) => self.add_status(v),
            WordType::Command(v) => self.add_command(v),
            _ => Err(Error::WordIsInvalid),
        }
    }

    /// Get a data word from the message by index
    ///
    /// An index of 0 will return the first *data word*, not
    /// the leading command or status word.
    ///
    /// # Arguments
    ///
    /// * `index` - An index
    ///
    pub fn get(&self, index: usize) -> Option<&DataWord> {
        if let Some(WordType::Data(w)) = &self.words.get(index + 1) {
            Some(w)
        } else {
            None
        }
    }

    /// Constructor method to add a word to the message
    ///
    /// # Arguments
    ///
    /// * `word` - A word to add
    ///
    pub fn with_word<T: Into<WordType>>(mut self, word: T) -> Result<Self> {
        self.add(word)?;
        Ok(self)
    }

    /// Add a data word, returning the size of the message on success
    ///
    /// # Arguments
    ///
    /// * `word` - A word to add
    ///
    pub fn add_data(&mut self, word: DataWord) -> Result<usize> {
        if self.is_full() && self.has_command() {
            Err(Error::MessageIsFull)
        } else if self.is_empty() {
            Err(Error::FirstWordIsData)
        } else {
            self.words[self.count] = WordType::Data(word);
            self.count += 1;
            Ok(self.count)
        }
    }

    /// Constructor method to add a data word to the message
    ///
    /// # Arguments
    ///
    /// * `word` - A word to add
    ///
    pub fn with_data<T: Into<DataWord>>(mut self, word: T) -> Result<Self> {
        self.add_data(word.into())?;
        Ok(self)
    }

    /// Add a status word, returning the size of the message on success
    ///
    /// # Arguments
    ///
    /// * `word` - A word to add
    ///
    pub fn add_status(&mut self, word: StatusWord) -> Result<usize> {
        if !self.is_empty() {
            Err(Error::StatusWordNotFirst)
        } else if !word.check_parity() {
            Err(Error::InvalidWord)
        } else {
            self.words[self.count] = WordType::Status(word);
            self.count += 1;
            Ok(self.count)
        }
    }

    /// Constructor method to add a status word to the message
    ///
    /// # Arguments
    ///
    /// * `word` - A word to add
    ///
    pub fn with_status<T: Into<StatusWord>>(mut self, word: T) -> Result<Self> {
        self.add_status(word.into())?;
        Ok(self)
    }

    /// Add a command word, returning the size of the message on success
    ///
    /// # Arguments
    ///
    /// * `word` - A word to add
    ///
    pub fn add_command(&mut self, word: CommandWord) -> Result<usize> {
        if !self.is_empty() {
            Err(Error::CommandWordNotFirst)
        } else if !word.check_parity() {
            Err(Error::InvalidWord)
        } else {
            self.words[self.count] = WordType::Command(word);
            self.count += 1;
            Ok(self.count)
        }
    }

    /// Constructor method to add a command word to the message
    ///
    /// # Arguments
    ///
    /// * `word` - A word to add
    ///
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
    fn test_message_command_data() {
        let mut message = Message::new();

        message
            .add(CommandWord::from_data(0b0001100001100010))
            .unwrap();

        assert_eq!(message.word_count(), 1);
        assert_eq!(message.data_count(), 0);
        assert_eq!(message.data_expected(), 2);
    }

    #[test]
    fn test_message_command_add_data() {
        let mut message = Message::new();

        message
            .add(CommandWord::from_data(0b0001100001100010))
            .unwrap();

        message
            .add(DataWord::from_data(0b0110100001101001))
            .unwrap();

        assert_eq!(message.word_count(), 2);
        assert_eq!(message.data_count(), 1);
    }

    #[test]
    fn test_message_status_no_data() {
        let mut message = Message::new();

        message
            .add(StatusWord::from_data(0b0001100000000010))
            .unwrap();

        assert_eq!(message.word_count(), 1);
        assert_eq!(message.data_count(), 0);
        assert_eq!(message.data_expected(), 0);
    }

    #[test]
    fn test_message_status_add_data() {
        let mut message = Message::new();

        message
            .add(StatusWord::from_data(0b0001100000000000))
            .unwrap();

        message
            .add(DataWord::from_data(0b0110100001101001))
            .unwrap();

        assert_eq!(message.word_count(), 2);
        assert_eq!(message.data_count(), 1);

        // status words don't have a word count field, and the
        // number of data words following a status word is set
        // by an earlier request.
        assert_eq!(message.data_expected(), 0);
    }
}
