use crate::word::WordType;
use crate::word::{CommandWord, DataWord, StatusWord};
use crate::{errors::*, Header, Packet, Word};

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
///     let message: Message = Message::new()
///         .with_command(CommandWord::new()
///             .with_address(Address::Value(12))
///             .with_subaddress(SubAddress::Value(5))
///             .with_word_count(2)
///             .build()?
///         )?
///         .with_data(DataWord::new())?
///         .with_data(DataWord::new())?;
///
///     assert!(message.is_full());
///     assert_eq!(message.length(),3);
///     assert_eq!(message.count(),2);
/// # Ok(())
/// # }
/// ```
///
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Message<const WORDS: usize = 33> {
    count: usize,
    words: [WordType; WORDS],
}

impl<const WORDS: usize> Message<WORDS> {
    /// Create a new message struct
    pub fn new() -> Self {
        Self {
            count: 0,
            words: [WordType::None; WORDS],
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

    /// Constructor method to add a word to the message
    ///
    /// # Arguments
    ///
    /// * `word` - A word to add
    ///
    pub fn with_word<T: Word>(mut self, word: T) -> Result<Self> {
        self.add(word)?;
        Ok(self)
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
    ///     let message: Message = Message::read_command(&[
    ///         0b10000011,
    ///         0b00001100,
    ///         0b01010110,
    ///         0b10000110,
    ///         0b10010000
    ///     ])?;
    ///
    ///     assert!(message.is_full());
    ///     assert!(message.is_command());
    ///     assert_eq!(message.length(),2);
    ///     assert_eq!(message.count(),1);
    /// # Ok(())
    /// # }
    pub fn read_command(data: &[u8]) -> Result<Self> {
        Self::read::<CommandWord>(data)
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
    /// See [read_command][Message::read_command] for more information.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use mil_std_1553b::*;
    /// # fn try_main() -> Result<()> {
    ///     let message: Message = Message::read_status(&[
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
    ///     assert!(message.is_status());
    ///     assert_eq!(message.length(),2);
    ///     assert_eq!(message.count(),1);
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_status(data: &[u8]) -> Result<Self> {
        Self::read::<StatusWord>(data)
    }

    /// Get the command word from the message
    ///
    /// Returns `None` if this message doesn't
    /// have a command word.
    ///
    /// # Arguments
    ///
    /// * `index` - An index
    ///
    pub fn command(&self) -> Option<&CommandWord> {
        if let Some(WordType::Command(w)) = &self.words.get(0) {
            Some(w)
        } else {
            None
        }
    }

    /// Get the status word from the message
    ///
    /// Returns `None` if this message doesn't
    /// have a status word.
    ///
    /// # Arguments
    ///
    /// * `index` - An index
    ///
    pub fn status(&self) -> Option<&StatusWord> {
        if let Some(WordType::Status(w)) = &self.words.get(0) {
            Some(w)
        } else {
            None
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

    /// Add a word to the message
    ///
    /// # Arguments
    ///
    /// * `word` - A word to add
    ///
    pub fn add<T: Word>(&mut self, word: T) -> Result<()> {
        match word.into() {
            WordType::Data(v) => self.add_data(v),
            WordType::Status(v) => self.add_status(v),
            WordType::Command(v) => self.add_command(v),
            _ => Err(Error::WordIsInvalid),
        }
    }

    /// Add a data word
    ///
    /// Performs basic checks for message validity before
    /// Adding the data word. This method will return an error
    /// if the status word-
    ///
    /// * Is the first word in the message.
    /// * If the message is full.
    /// * If the parity bit on the word is wrong.
    ///
    /// # Arguments
    ///
    /// * `word` - A word to add
    ///
    fn add_data(&mut self, word: DataWord) -> Result<()> {
        if self.is_full() && self.is_command() {
            Err(Error::MessageIsFull)
        } else if self.is_empty() {
            Err(Error::FirstWordIsData)
        } else if self.words.len() <= self.count {
            Err(Error::MessageIsFull)
        } else {
            self.words[self.count] = word.into();
            self.count += 1;
            Ok(())
        }
    }

    /// Add a status word
    ///
    /// Performs basic checks for message validity before
    /// Adding the status word. This method will return an error
    /// if the status word-
    ///
    /// * Is not the first word in the message.
    /// * If the message is full.
    /// * If the parity bit on the word is wrong.
    ///
    /// # Arguments
    ///
    /// * `word` - A word to add
    ///
    fn add_status(&mut self, word: StatusWord) -> Result<()> {
        if !self.is_empty() {
            Err(Error::StatusWordNotFirst)
        } else if !word.check_parity() {
            Err(Error::InvalidWord)
        } else if self.words.len() <= self.count {
            Err(Error::MessageIsFull)
        } else {
            self.words[self.count] = word.into();
            self.count += 1;
            Ok(())
        }
    }

    /// Add a command word
    ///
    /// Performs basic checks for message validity before
    /// Adding the command word. This method will return an error
    /// if the command word-
    ///
    /// * Is not the first word in the message.
    /// * If the message is full.
    /// * If the parity bit on the word is wrong.
    ///
    /// # Arguments
    ///
    /// * `word` - A word to add
    ///
    fn add_command(&mut self, word: CommandWord) -> Result<()> {
        if !self.is_empty() {
            Err(Error::CommandWordNotFirst)
        } else if !word.check_parity() {
            Err(Error::InvalidWord)
        } else if self.words.len() <= self.count {
            Err(Error::MessageIsFull)
        } else {
            self.words[self.count] = word.into();
            self.count += 1;
            Ok(())
        }
    }

    /// Check if message starts with a command word
    #[must_use = "Returned value is not used"]
    pub fn is_command(&self) -> bool {
        self.command().is_some()
    }

    /// Check if message starts with a status word
    #[must_use = "Returned value is not used"]
    pub fn is_status(&self) -> bool {
        self.status().is_some()
    }

    /// Check if the message is full
    ///
    /// This method will return false for status messages
    /// until the maximum number of data words has been reached.
    #[must_use = "Returned value is not used"]
    pub fn is_full(&self) -> bool {
        if let Some(w) = self.command() {
            self.count() == w.word_count().into()
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
        self.words = [WordType::None; WORDS];
    }

    /// Get the number of data words
    pub fn count(&self) -> usize {
        self.words.iter().filter(|w| w.is_data()).count()
    }

    /// Get the total number of words
    pub fn length(&self) -> usize {
        self.words.iter().filter(|w| w.is_some()).count()
    }

    /// Read bytes as a message
    pub fn read<T: Word + Header>(data: &[u8]) -> Result<Self> {
        // estimate word count from given data
        let estimate = ((data.len() * 8) / 20) - 1;

        // parse the specified header word
        let word = Packet::read(data, 0)?.as_word::<T>()?;

        // get the number of expected words or an
        // estimate if the header is a status word.
        let count = word.count().unwrap_or(estimate);

        // create a new message with the header word
        let mut message: Self = Self::new().with_word(word)?;

        // return if no data words
        if count == 0 {
            return Ok(message);
        }

        // the expected number of bytes to parse
        let expected = ((count * 20) + 7) / 8;

        // return error if data is too small
        if data.len() < expected {
            return Err(Error::InvalidMessage);
        }

        let start = 1; // skip the service word
        let end = count + 1; // adjust for service word

        for index in start..end {
            let b = index * 20; // offset in bits
            let i = b / 8; // byte offset (whole)
            let o = b % 8; // byte offset (fraction)
            let bytes = &data[i..];

            // use a packet to parse the bytes and convert to a word
            message.add_data(Packet::read(bytes, o)?.try_into()?)?;
        }

        Ok(message)
    }

    /// Get the message as a byte array
    pub fn write(&self, bytes: &mut [u8]) -> Result<()> {
        let count = ((self.length() * 20) + 7) / 8;

        if bytes.len() < count {
            return Err(Error::OutOfBounds);
        }

        // TODO: rewrite this to bring it in line with parse naming

        for (i, word) in self.words.iter().enumerate() {
            let index = (i * 20) / 8;
            let offset = (i * 20) % 8;

            let packet = Packet::try_from(word)?;
            packet.write(&mut bytes[index..], offset)?;
        }

        Ok(())
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
    fn test_message_write_bytes() {
        let data = [
            0b10000011, 0b00001100, 0b01110010, 0b11010000, 0b11010010, 0b00101111, 0b00101101,
            0b11100010, 0b11001110, 0b11011110,
        ];

        let message: Message<4> = Message::read_command(&data).unwrap();

        let mut buffer: [u8; 10] = [0; 10];
        let result = message.write(&mut buffer);

        assert!(result.is_ok());
        assert_eq!(buffer, data);
    }

    #[test]
    fn test_parse_words_wrong_word_size() {
        let result: Result<Message<3>> = Message::read_command(&[
            0b10000011, 0b00001100, 0b01110010, 0b11010000, 0b11010010, 0b00101111, 0b00101101,
            0b11100010, 0b11001110, 0b11011110,
        ]);

        assert_eq!(result, Err(Error::MessageIsFull));
    }

    #[test]
    fn test_parse_words_right_word_size() {
        let result: Result<Message<4>> = Message::read_command(&[
            0b10000011, 0b00001100, 0b01110010, 0b11010000, 0b11010010, 0b00101111, 0b00101101,
            0b11100010, 0b11001110, 0b11011110,
        ]);

        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_words_wrong_byte_size() {
        let result: Result<Message<2>> = Message::read_command(&[
            0b10000011, 0b00001100, 0b01110010, 0b11010000, 0b11010010, 0b00101111, 0b00101101,
            0b11100010, 0b11001110,
        ]);

        assert_eq!(result, Err(Error::MessageIsFull));
    }

    #[test]
    fn test_read_command_three_data_words() {
        let message: Message<4> = Message::read_command(&[
            0b10000011, 0b00001100, 0b01110010, 0b11010000, 0b11010010, 0b00101111, 0b00101101,
            0b11100010, 0b11001110, 0b11011110,
        ])
        .unwrap();

        assert!(message.is_full());
        assert!(message.is_command());
        assert_eq!(message.length(), 4);
        assert_eq!(message.count(), 3);

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
    fn test_read_command_two_data_words() {
        let message: Message<3> = Message::read_command(&[
            0b10000011, 0b00001100, 0b01000010, 0b11010000, 0b11010010, 0b00101111, 0b00101101,
            0b11100000,
        ])
        .unwrap();

        assert!(message.is_full());
        assert!(message.is_command());
        assert_eq!(message.length(), 3);
        assert_eq!(message.count(), 2);
    }

    #[test]
    fn test_read_command_one_data_word() {
        let message: Message =
            Message::read_command(&[0b10000011, 0b00001100, 0b00100010, 0b11010000, 0b11010010])
                .unwrap();

        assert!(message.is_full());
        assert!(message.is_command());
        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
    }

    #[test]
    fn test_read_status_three_data_words() {
        let message: Message = Message::read_status(&[
            0b10000011, 0b00001100, 0b01110010, 0b11010000, 0b11010010, 0b00101111, 0b00101101,
            0b11100010, 0b11001110, 0b11011110,
        ])
        .unwrap();

        assert!(!message.is_full());
        assert!(message.is_status());
        assert_eq!(message.length(), 4);
        assert_eq!(message.count(), 3);

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
    fn test_read_status_two_data_words() {
        let message: Message = Message::read_status(&[
            0b10000011, 0b00001100, 0b01000010, 0b11010000, 0b11010010, 0b00101111, 0b00101101,
            0b11100000,
        ])
        .unwrap();

        assert!(!message.is_full());
        assert!(message.is_status());
        assert_eq!(message.length(), 3);
        assert_eq!(message.count(), 2);
    }

    #[test]
    fn test_read_status_one_data_word() {
        let message: Message =
            Message::read_status(&[0b10000011, 0b00001100, 0b00100010, 0b11010000, 0b11010010])
                .unwrap();

        assert!(!message.is_full());
        assert!(message.is_status());
        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
    }

    #[test]
    fn test_create_message() {
        let message: Message = Message::new();

        assert_eq!(message.is_full(), false);
        assert_eq!(message.is_empty(), true);
        assert_eq!(message.command(), None);
        assert_eq!(message.status(), None);
        assert_eq!(message.get(0), None);

        assert_eq!(message.length(), 0);
        assert_eq!(message.count(), 0);
    }

    #[test]
    fn test_message_command_data() {
        let mut message: Message = Message::new();

        message
            .add(CommandWord::from_value(0b0001100001100010))
            .unwrap();

        let expected = message.command().map(CommandWord::word_count).unwrap_or(0);

        assert_eq!(message.length(), 1);
        assert_eq!(message.count(), 0);
        assert_eq!(expected, 2);
    }

    #[test]
    fn test_message_command_add_data() {
        let mut message: Message = Message::new();

        message
            .add(CommandWord::from_value(0b0001100001100010))
            .unwrap();

        message.add(DataWord::from(0b0110100001101001)).unwrap();

        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
    }

    #[test]
    fn test_message_status_no_data() {
        let mut message: Message = Message::new();

        message
            .add(StatusWord::from_value(0b0001100000000010))
            .unwrap();

        assert_eq!(message.length(), 1);
        assert_eq!(message.count(), 0);
    }

    #[test]
    fn test_message_status_add_data() {
        let mut message: Message = Message::new();

        message
            .add(StatusWord::from_value(0b0001100000000000))
            .unwrap();

        message.add(DataWord::from(0b0110100001101001)).unwrap();

        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
    }
}
