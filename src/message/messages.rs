use crate::word::WordType;
use crate::word::{CommandWord, DataWord, StatusWord};
use crate::{errors::*, Header, Packet, Word};

/// Default value for word array
const ARRAY_NONE: WordType = WordType::None;

/// A message sent between two terminals on the bus
///
/// The Message struct does very minimal message validation
/// for the message structure:
///
/// * Command or status words are always the first word.
/// * Data words are limited based on the command word count.
/// * For status words, data words are parsed to the end of the buffer
///
/// Messages do not validate larger messaging patterns that
/// require context about previous messages or terminal type.
///
/// ## Example
///
/// ```rust
/// # use mil_std_1553b::*;
/// # fn main() -> Result<()> {
///     let message= Message::<3>::new()
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
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Message<const WORDS: usize = 33> {
    count: usize,
    words: [WordType; WORDS],
}

impl<const WORDS: usize> Message<WORDS> {
    /// Create a new message struct
    pub fn new() -> Self {
        Self {
            count: 0,
            words: [ARRAY_NONE; WORDS],
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

    /// Method to finalize construction
    pub fn build(self) -> Result<Self> {
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
    /// being parsed is aligned to the beginning of the slice
    /// (the leftmost three bits of the first byte are the sync
    /// field of the command word).
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of bytes to parse
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use mil_std_1553b::*;
    /// # fn main() -> Result<()> {
    ///     let message = Message::<2>::read_command(&[
    ///         0b10000011, 
    ///         0b00001100, 
    ///         0b00100010, 
    ///         0b11010000, 
    ///         0b11010010
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
    /// It is assumed that the message being parsed is aligned
    /// to the beginning of the slice (the leftmost three bits
    /// of the first byte are the sync field of the status word).
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
    /// # fn main() -> Result<()> {
    ///     let message = Message::<2>::read_status(&[
    ///         0b10000011, 
    ///         0b00001100, 
    ///         0b00100010, 
    ///         0b11010000, 
    ///         0b11010010
    ///     ])?;
    /// 
    ///     assert!(message.is_full());
    ///     assert!(message.is_status());
    ///     assert_eq!(message.length(),2);
    ///     assert_eq!(message.count(),1);
    ///     
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
        if let Some(WordType::Command(w)) = &self.words.first() {
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
        if let Some(WordType::Status(w)) = &self.words.first() {
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
            _ => Err(Error::InvalidWord),
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
        if self.is_full() {
            Err(Error::MessageFull)
        } else if self.is_empty() {
            Err(Error::DataFirst)
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
            Err(Error::HeaderNotFirst)
        } else if self.is_full() {
            Err(Error::MessageFull)
        } else if !word.check_parity() {
            Err(Error::InvalidWord)
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
            Err(Error::HeaderNotFirst)
        } else if self.is_full() {
            Err(Error::MessageFull)
        } else if !word.check_parity() {
            Err(Error::InvalidWord)
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
            self.count() == w.count()
        } else {
            self.length() == self.size()
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
        self.words = [ARRAY_NONE; WORDS];
    }

    /// Get the number of data words
    pub fn count(&self) -> usize {
        self.words.iter().filter(|w| w.is_data()).count()
    }

    /// Get the total number of words
    pub fn length(&self) -> usize {
        self.words.iter().filter(|w| w.is_some()).count()
    }

    /// Get the maximum number of words
    pub fn size(&self) -> usize {
        self.words.len()
    }

    /// Read bytes as a message
    /// 
    /// # Arguments
    ///
    /// * `data` - A slice of bytes to read
    /// 
    pub fn read<T: Word + Header>(data: &[u8]) -> Result<Self> {
        let min = 3;

        // need at least three bytes to parse
        if data.len() < min {
            return Err(Error::InvalidMessage);
        }

        // estimate word count from given data
        let estimate = ((data.len() * 8) / 20).saturating_sub(1);

        // parse the specified header word
        let word = Packet::read(data, 0)?.as_word::<T>()?;

        // get the number of expected words or an
        // estimate if the header is a status word.
        let count = word.count().unwrap_or(estimate);

        // the expected number of bytes to parse
        let expected = (((count + 1) * 20) + 7) / 8;

        // return error if data is too small
        if data.len() < expected {
            return Err(Error::InvalidMessage);
        }

        // create a new message with the header word
        let mut message: Self = Self::new().with_word(word)?;

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

    /// Write the message to a byte array
    /// 
    /// # Arguments
    ///
    /// * `data` - A slice of bytes to write
    /// 
    pub fn write(&self, data: &mut [u8]) -> Result<()> {
        let count = ((self.length() * 20) + 7) / 8;

        if data.len() < count {
            return Err(Error::OutOfBounds);
        }

        for (index, word) in self.words.iter().take_while(|w| w.is_some()).enumerate() {
            let b = index * 20;
            let i = b / 8;
            let o = b % 8;

            let packet = Packet::try_from(word)?;
            packet.write(&mut data[i..], o)?;
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
    use rstest::rstest;

    #[test]
    fn test_message_default() {
        let message: Message = Message::default();
        assert_eq!(message.length(), 0);
        assert_eq!(message.count(), 0);
        assert_eq!(message.size(), 33);
    }

    #[test]
    fn test_message_clone() {
        let message1 = Message::<2>::new()
            .with_command(0b0000000000000001)
            .unwrap()
            .with_data(0b0000000000000001)
            .unwrap()
            .build()
            .unwrap();

        let message2 = message1.clone();
        assert_eq!(message1, message2);
    }

    #[test]
    fn test_message_new() {
        let message: Message = Message::new();
        assert_eq!(message.length(), 0);
        assert_eq!(message.count(), 0);
        assert_eq!(message.size(), 33);
    }

    #[test]
    fn test_message_with_command() {
        let message = Message::<2>::new()
            .with_command(0b0000000000000001)
            .unwrap()
            .with_data(0b0000000000000001)
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
        assert_eq!(message.size(), 2);
        assert!(message.is_command());
        assert!(!message.is_status());
    }

    #[test]
    fn test_message_command_add_data() {
        let mut message = Message::<2>::new().with_command(0b0001100001100010).unwrap();

        let result = message.add_data(0b0110100001101001.into());

        assert!(result.is_ok());
        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
    }

    #[test]
    fn test_message_with_command_fail_duplicate_header() {
        // adding two header words to the message
        let message = Message::<2>::new()
            .with_command(0b0000000000000001)
            .unwrap()
            .with_command(0b0000000000000001);
        assert!(message.is_err());
    }

    #[test]
    fn test_message_with_command_fail_header_not_first() {
        // manually adding a data word to the message
        let mut message = Message::<2>::new();
        message.words[0] = DataWord::new().into();
        message.count = 1;

        let result = message.with_command(0b0000000000000001);
        assert!(result.is_err());
    }

    #[test]
    fn test_message_with_command_fail_message_full() {
        let result = Message::<0>::new().with_command(0b0000000000000001);
        assert!(result.is_err());
    }

    #[test]
    fn test_message_add_command_fail_parity() {
        let word = CommandWord::new()
            .with_value(0b0000000000000000)
            .with_parity(0);

        let mut message = Message::<1>::new();
        let result = message.add_command(word);

        assert_eq!(result, Err(Error::InvalidWord));
    }

    #[test]
    fn test_message_with_status() {
        let message = Message::<2>::new()
            .with_status(0b0000000000000001)
            .unwrap()
            .with_data(0b0000000000000001)
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
        assert_eq!(message.size(), 2);
        assert!(message.is_status());
        assert!(!message.is_command());
    }

    #[test]
    fn test_message_status_add_data() {
        let mut message = Message::<2>::new().with_status(0b0001100001100010).unwrap();

        let result = message.add_data(0b0110100001101001.into());

        assert!(result.is_ok());
        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
    }

    #[test]
    fn test_message_with_status_fail_duplicate_header() {
        // adding two header words to the message
        let message = Message::<2>::new()
            .with_status(0b0000000000000001)
            .unwrap()
            .with_status(0b0000000000000001);
        assert!(message.is_err());
    }

    #[test]
    fn test_message_with_status_fail_header_not_first() {
        // manually adding a data word to the message
        let mut message = Message::<2>::new();
        message.words[0] = DataWord::new().into();
        message.count = 1;

        let result = message.with_status(0b0000000000000001);
        assert!(result.is_err());
    }

    #[test]
    fn test_message_with_status_fail_message_full() {
        let result = Message::<0>::new().with_status(0b0000000000000001);
        assert!(result.is_err());
    }

    #[test]
    fn test_message_add_status_fail_parity() {
        let word = StatusWord::new()
            .with_value(0b0000000000000000)
            .with_parity(0);

        let mut message = Message::<1>::new();
        let result = message.add_status(word);

        assert_eq!(result, Err(Error::InvalidWord));
    }

    #[test]
    fn test_message_with_data() {
        let message = Message::<2>::new()
            .with_status(0b0000000000000001)
            .unwrap()
            .with_data(0b0000000000000001)
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
        assert_eq!(message.size(), 2);
    }

    #[test]
    fn test_message_with_data_fail() {
        let result = Message::<2>::new().with_data(0b0000000000000001);
        assert!(result.is_err());
    }

    #[test]
    fn test_message_with_data_fail_message_full() {
        let result = Message::<2>::new()
            .with_status(0b0000000000000001)
            .unwrap()
            .with_data(0b0000000000000001)
            .unwrap()
            .with_data(0b0000000000000001);
        assert!(result.is_err());
    }

    #[test]
    fn test_message_with_word_command() {
        let message = Message::<2>::new()
            .with_word(CommandWord::from(0b0000000000000001))
            .unwrap()
            .with_word(DataWord::from(0b0000000000000001))
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
        assert_eq!(message.size(), 2);
    }

    #[test]
    fn test_message_with_word_status() {
        let message = Message::<2>::new()
            .with_word(StatusWord::from(0b0000000000000001))
            .unwrap()
            .with_word(DataWord::from(0b0000000000000001))
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
        assert_eq!(message.size(), 2);
    }

    #[rstest]
    #[case(&[0b10000011, 0b00001100, 0b00100010, 0b11010000, 0b11010010], 2)]
    #[case(&[0b10000011, 0b00001100, 0b01110010, 0b11010000, 0b11010010, 0b00101111, 0b00101101, 0b11100010, 0b11001110, 0b11011110], 4)]
    fn test_message_read_write_command(#[case] input: &[u8], #[case] length: usize) -> Result<()> {
        let l = input.len();
        let mut buffer = [0; 10];

        let message= Message::<4>::read_command(&input)?;
        message.write(&mut buffer)?;

        assert_eq!(&buffer[..l], input);
        assert_eq!(message.length(), length);
        Ok(())
    }

    #[rstest]
    #[case(&[0b10000011, 0b00001100, 0b00100010, 0b11010000, 0b11010010], 2)]
    #[case(&[0b10000011, 0b00001100, 0b01110010, 0b11010000, 0b11010010, 0b00101111, 0b00101101, 0b11100010, 0b11001110, 0b11011110], 4)]
    fn test_message_read_write_status(#[case] input: &[u8], #[case] length: usize) -> Result<()> {
        let l = input.len();
        let mut buffer = [0; 10];

        let message= Message::<4>::read_status(&input)?;
        message.write(&mut buffer)?;

        assert_eq!(&buffer[..l], input);
        assert_eq!(message.length(), length);
        Ok(())
    }

    #[test]
    fn test_message_read_status_no_data() {
        let input = [0b10000011, 0b00001100, 0b00100010];
        let message = Message::<1>::read_status(&input).unwrap();
        assert_eq!(message.length(), 1);
    }

    #[test]
    fn test_message_read_status_fail_buffer_too_small() {
        let input = [0b10000011, 0b00001100];
        let result = Message::<1>::read_status(&input);
        assert_eq!(result, Err(Error::InvalidMessage));
    }

    #[test]
    fn test_message_read_command_fail_buffer_too_small() {
        let input = [0b10000011, 0b00001100, 0b00100010];
        let result = Message::<2>::read_command(&input);
        assert_eq!(result, Err(Error::InvalidMessage));
    }

    #[test]
    fn test_message_write_command_fail_buffer_too_small() {
        let mut buffer = [0, 0, 0];

        let message = Message::<2>::new()
            .with_command(0b0001100001100001)
            .unwrap()
            .with_data(0b0000000000000000)
            .unwrap()
            .build()
            .unwrap();

        let result = message.write(&mut buffer);
        assert_eq!(result, Err(Error::OutOfBounds));
    }

    #[test]
    fn test_message_write_status_fail_buffer_too_small() {
        let mut buffer = [0, 0, 0];

        let message = Message::<2>::new()
            .with_status(0b0001100001100001)
            .unwrap()
            .with_data(0b0000000000000000)
            .unwrap()
            .build()
            .unwrap();

        let result = message.write(&mut buffer);
        assert_eq!(result, Err(Error::OutOfBounds));
    }

    #[test]
    fn test_message_status() {
        let message = Message::<2>::new().with_status(0b0000000000000001).unwrap();
        assert!(message.status().is_some());
    }

    #[test]
    fn test_message_command() {
        let message = Message::<2>::new().with_command(0b0000000000000001).unwrap();
        assert!(message.command().is_some());
    }

    #[test]
    fn test_message_get() {
        let data1: DataWord = 0b0000000000000101.into();
        let data2: DataWord = 0b0000000000010101.into();
        let data3: DataWord = 0b0000000001010101.into();

        let message = Message::<4>::new()
            .with_command(0b0000000000000011)
            .unwrap()
            .with_data(data1)
            .unwrap()
            .with_data(data2)
            .unwrap()
            .with_data(data3)
            .unwrap()
            .build()
            .unwrap();

        let word1 = message.get(0);
        let word2 = message.get(1);
        let word3 = message.get(2);
        let word4 = message.get(3);

        assert_eq!(word1, Some(&data1));
        assert_eq!(word2, Some(&data2));
        assert_eq!(word3, Some(&data3));
        assert_eq!(word4, None);
    }

    #[test]
    fn test_message_clear() {
        let mut message = Message::<2>::new()
            .with_command(0b0000000000000001)
            .unwrap()
            .with_data(0b0000000000000001)
            .unwrap()
            .build()
            .unwrap();

        message.clear();

        assert_eq!(message.length(), 0);
        assert_eq!(message.count(), 0);
        assert_eq!(message.size(), 2);
        assert!(message.is_empty());
        assert!(!message.is_full());
    }
}
