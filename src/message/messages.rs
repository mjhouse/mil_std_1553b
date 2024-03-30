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
///         )
///         .with_data(DataWord::new())
///         .with_data(DataWord::new())
///         .build()?;
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
    words: [WordType; WORDS],
    error: Option<Error>,
}

impl<const WORDS: usize> Message<WORDS> {
    /// Create a new message struct
    pub fn new() -> Self {
        Self {
            words: [ARRAY_NONE; WORDS],
            error: None,
        }
    }

    /// Constructor method to set the message body from a string
    ///
    /// See [set_string][Self::set_string] for more information.
    ///
    /// # Arguments
    ///
    /// * `data` - String to include
    ///
    pub fn with_string(mut self, data: &str) -> Self {
        self.set_string(data);
        self
    }

    /// Constructor method to set the message body from bytes
    ///
    /// See [set_bytes][Self::set_bytes] for more information.
    ///
    /// # Arguments
    ///
    /// * `data` - Bytes to include
    ///
    pub fn with_bytes(mut self, data: &[u8]) -> Self {
        self.set_bytes(data);
        self
    }

    /// Constructor method to add a command word to the message
    ///
    /// # Arguments
    ///
    /// * `word` - A word to add
    ///
    pub fn with_command<T: Into<CommandWord>>(mut self, word: T) -> Self {
        self.add_command(word.into());
        self
    }

    /// Constructor method to add a status word to the message
    ///
    /// # Arguments
    ///
    /// * `word` - A word to add
    ///
    pub fn with_status<T: Into<StatusWord>>(mut self, word: T) -> Self {
        self.add_status(word.into());
        self
    }

    /// Constructor method to add a data word to the message
    ///
    /// # Arguments
    ///
    /// * `word` - A word to add
    ///
    pub fn with_data<T: Into<DataWord>>(mut self, word: T) -> Self {
        self.add_data(word.into());
        self
    }

    /// Constructor method to add a word to the message
    ///
    /// # Arguments
    ///
    /// * `word` - A word to add
    ///
    pub fn with_word<T: Word>(mut self, word: T) -> Self {
        self.add(word);
        self
    }

    /// Method to finalize construction
    ///
    /// Performs basic checks for message validity, returning
    /// an error:
    ///
    /// * If the wrong number of words were added during construction
    /// * If there are multiple header words (command or status)
    /// * If any word has a bad parity
    /// * If the first word is a data word
    ///
    pub fn build(self) -> Result<Self> {
        self.validate().map(|_| self)
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
    pub fn at(&self, index: usize) -> Option<&DataWord> {
        if let Some(WordType::Data(w)) = &self.words.get(index + 1) {
            Some(w)
        } else {
            None
        }
    }

    /// Get a custom data word from the message by index
    ///
    /// An index of 0 will return the first *data word*, not
    /// the leading command or status word.
    ///
    /// # Arguments
    ///
    /// * `index` - An index
    ///
    pub fn get<'a, T>(&'a self, index: usize) -> Option<T>
    where
        T: TryFrom<&'a DataWord>,
    {
        if let Some(WordType::Data(w)) = &self.words.get(index + 1) {
            T::try_from(w).ok()
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
    pub fn add<T: Word>(&mut self, word: T) {
        let index = self
            .words
            .iter()
            .position(WordType::is_none)
            .unwrap_or(self.words.len());

        if index < self.words.len() {
            self.words[index] = word.into();
        } else {
            self.error = Some(Error::OutOfBounds);
        }
    }

    /// Add words from a string
    ///
    /// This method breaks the string into two-byte
    /// chunks and adds them as [DataWord]s to the
    /// message. If it fails, [is_valid][Self::is_valid] will
    /// return false and [validate][Self::validate] will
    /// return an error.
    ///
    /// # Arguments
    ///
    /// * `data` - Words to add
    ///
    pub fn add_string(&mut self, data: &str) {
        for chunk in data.as_bytes().chunks(2).map(|s| match s.len() {
            2 => [s[0], s[1]],
            1 => [s[0], 0],
            _ => [0, 0],
        }) {
            self.add_data(chunk.into());
        }
    }

    /// Add words from bytes
    ///
    /// This method breaks the given bytes into two-byte
    /// chunks and adds them as [DataWord]s to the
    /// message. If it fails, [is_valid][Self::is_valid] will
    /// return false and [validate][Self::validate] will
    /// return an error.
    ///
    /// **Given data should only contain the words,
    /// without sync or parity bits.**
    ///
    /// # Arguments
    ///
    /// * `data` - Words to add
    ///
    pub fn add_bytes(&mut self, data: &[u8]) {
        for chunk in data.chunks(2).map(|s| match s.len() {
            2 => [s[0], s[1]],
            1 => [s[0], 0],
            _ => [0, 0],
        }) {
            self.add_data(chunk.into());
        }
    }

    /// Add a data word
    ///
    /// # Arguments
    ///
    /// * `word` - A word to add
    ///
    pub fn add_data(&mut self, word: DataWord) {
        self.add(word);
    }

    /// Add a status word
    ///
    /// # Arguments
    ///
    /// * `word` - A word to add
    ///
    pub fn add_status(&mut self, word: StatusWord) {
        self.add(word);
    }

    /// Add a command word
    ///
    /// # Arguments
    ///
    /// * `word` - A word to add
    ///
    pub fn add_command(&mut self, word: CommandWord) {
        self.add(word);
    }

    /// Set words from a string
    ///
    /// This method overwrites existing data in the
    /// message by breaking the given string into two-byte
    /// chunks and adding them as [DataWord]s. If it fails,
    /// [is_valid][Self::is_valid] will return false and
    /// [validate][Self::validate] will return an error.
    ///
    /// # Arguments
    ///
    /// * `data` - Words to add
    ///
    pub fn set_string(&mut self, data: &str) {
        self.words[1..].fill(WordType::None);
        self.add_string(data);
    }

    /// Set words from bytes
    ///
    /// This method overwrites existing data in the
    /// message by breaking the given string into two-byte
    /// chunks and adding them as [DataWord]s. If it fails,
    /// [is_valid][Self::is_valid] will return false and
    /// [validate][Self::validate] will return an error.
    ///
    /// **Given data should only contain the words,
    /// without sync or parity bits.**
    ///
    /// # Arguments
    ///
    /// * `data` - Words to add
    ///
    pub fn set_bytes(&mut self, data: &[u8]) {
        self.words[1..].fill(WordType::None);
        self.add_bytes(data);
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
        self.length() == 0
    }

    /// Check if the message is valid
    #[must_use = "Returned value is not used"]
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    /// Method to validate message
    ///
    /// Performs basic checks for message validity, returning
    /// an error:
    ///
    /// * If and error was generated during construction
    /// * If there are multiple header words (command or status)
    /// * If any word has a bad parity
    /// * If the first word is a data word
    ///
    pub fn validate(&self) -> Result<()> {
        // fail if an error is set
        if let Some(e) = self.error {
            return Err(e);
        }

        // fail if there are multiple header words
        if self.count() != (self.length() - 1) {
            return Err(Error::InvalidWord);
        }

        // fail if any word has a bad parity bit
        if self.words.iter().any(|w| !w.check_parity()) {
            return Err(Error::InvalidWord);
        }

        // fail if the first word is a data word
        if self.words.first().map(|w| w.is_data()).unwrap_or(false) {
            return Err(Error::HeaderNotFirst);
        }

        // fail if there are multiple header words
        if let Some(w) = self.command() {
            if w.count() != self.count() {
                return Err(Error::InvalidMessage);
            }
        }

        Ok(())
    }

    /// Clear all words from the message
    pub fn clear(&mut self) {
        self.words = [ARRAY_NONE; WORDS];
        self.error = None;
    }

    /// Get the current number of data words
    pub fn count(&self) -> usize {
        self.words.iter().filter(|w| w.is_data()).count()
    }

    /// Get the current number of words
    pub fn length(&self) -> usize {
        self.words.iter().filter(|w| w.is_some()).count()
    }

    /// Get the total possible size of the message
    pub fn size(&self) -> usize {
        self.words.len()
    }

    /// Get expected number of data words
    pub fn limit(&self) -> usize {
        self.command()
            .map(CommandWord::count)
            .unwrap_or(self.size().saturating_sub(1))
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
        let mut message = Self::new().with_word(word);

        let start = 1; // skip the service word
        let end = count + 1; // adjust for service word

        for index in start..end {
            let b = index * 20; // offset in bits
            let i = b / 8; // byte offset (whole)
            let o = b % 8; // byte offset (fraction)
            let bytes = &data[i..];

            // use a packet to parse the bytes and convert to a word
            message.add_data(Packet::read(bytes, o)?.try_into()?);
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

    #[test]
    fn test_message_with_string_0() {
        let message = Message::<3>::new()
            .with_command(0b0000000000000010)
            .with_string("TEST")
            .build()
            .unwrap();
        assert!(message.is_full());
        assert_eq!(message.length(), 3);
        assert_eq!(message.count(), 2);
        assert_eq!(message.size(), 3);
        assert!(message.is_command());
        assert!(!message.is_status());
    }

    #[test]
    fn test_message_with_string_1() {
        let message = Message::<2>::new()
            .with_command(0b0000000000000010)
            .with_string("TEST")
            .build();

        // error because the string was too long
        // for the given Message
        assert!(message.is_err());
    }

    #[test]
    fn test_message_with_string_2() {
        let message = Message::<3>::new()
            .with_command(0b0000000000000001)
            .with_string("TEST")
            .build();

        // error because the string was too long
        // for the command data word count
        assert!(message.is_err());
    }

    #[test]
    fn test_message_with_string_3() {
        let message = Message::<2>::new()
            .with_status(0b0000000000000000)
            .with_string("TEST")
            .build();

        // error because the string was too long
        // for the given Message
        assert!(message.is_err());
    }

    #[test]
    fn test_message_with_bytes_0() {
        let message = Message::<3>::new()
            .with_command(0b0000000000000010)
            .with_bytes(&[1, 2, 3, 4])
            .build()
            .unwrap();
        assert!(message.is_full());
        assert_eq!(message.length(), 3);
        assert_eq!(message.count(), 2);
        assert_eq!(message.size(), 3);
        assert!(message.is_command());
        assert!(!message.is_status());
    }

    #[test]
    fn test_message_with_bytes_1() {
        let message = Message::<2>::new()
            .with_command(0b0000000000000010)
            .with_bytes(&[1, 2, 3, 4])
            .build();

        // error because the string was too long
        // for the given Message
        assert!(message.is_err());
    }

    #[test]
    fn test_message_with_bytes_2() {
        let message = Message::<3>::new()
            .with_command(0b0000000000000001)
            .with_bytes(&[1, 2, 3, 4])
            .build();

        // error because the string was too long
        // for the command data word count
        assert!(message.is_err());
    }

    #[test]
    fn test_message_with_bytes_3() {
        let message = Message::<2>::new()
            .with_status(0b0000000000000000)
            .with_bytes(&[1, 2, 3, 4])
            .build();

        // error because the string was too long
        // for the given Message
        assert!(message.is_err());
    }

    #[test]
    fn test_message_set_string_command_0() {
        let mut message = Message::<2>::new().with_command(0b0000000000000001);

        message.add_string("TE");
        let string1 = message.get(0);
        assert_eq!(string1, Some("TE"));

        message.set_string("ST");
        let string2 = message.get(0);
        assert_eq!(string2, Some("ST"));

        assert!(message.is_valid());
        assert!(message.is_full());
        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
        assert_eq!(message.size(), 2);
        assert!(message.is_command());
        assert!(!message.is_status());
    }

    #[test]
    fn test_message_set_string_command_1() {
        let mut message = Message::<3>::new().with_command(0b0000000000000010);

        message.add_string("TEST");
        let string1 = message.get(0);
        let string2 = message.get(1);
        assert_eq!(string1, Some("TE"));
        assert_eq!(string2, Some("ST"));

        message.set_string("TSET");
        let string1 = message.get(0);
        let string2 = message.get(1);
        assert_eq!(string1, Some("TS"));
        assert_eq!(string2, Some("ET"));

        assert!(message.is_valid());
        assert_eq!(message.length(), 3);
        assert_eq!(message.count(), 2);
        assert_eq!(message.size(), 3);
        assert!(message.is_command());
        assert!(!message.is_status());
    }

    #[test]
    fn test_message_set_string_command_2() {
        let mut message = Message::<2>::new().with_command(0b0000000000000001);

        message.set_string("TEST");

        let string1 = message.get::<&str>(0);
        let string2 = message.get::<&str>(1);

        assert_eq!(string1, Some("TE"));
        assert_eq!(string2, None);

        // failed because the given string is too long
        // to fit in a message with size 2
        assert!(!message.is_valid());

        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
        assert_eq!(message.size(), 2);
        assert!(message.is_command());
        assert!(!message.is_status());
    }

    #[test]
    fn test_message_set_bytes_command_0() {
        let mut message = Message::<2>::new().with_command(0b0000000000000001);

        message.add_bytes(&[1, 2]);
        let bytes1 = message.get(0);
        assert_eq!(bytes1, Some([1, 2]));

        message.set_bytes(&[2, 1]);
        let bytes2 = message.get(0);
        assert_eq!(bytes2, Some([2, 1]));

        assert!(message.is_valid());
        assert!(message.is_full());
        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
        assert_eq!(message.size(), 2);
        assert!(message.is_command());
        assert!(!message.is_status());
    }

    #[test]
    fn test_message_set_bytes_command_1() {
        let mut message = Message::<3>::new().with_command(0b0000000000000010);

        message.set_bytes(&[1, 2, 3, 4]);
        let bytes1 = message.get(0);
        let bytes2 = message.get(1);
        assert_eq!(bytes1, Some([1, 2]));
        assert_eq!(bytes2, Some([3, 4]));

        message.set_bytes(&[4, 3, 2, 1]);
        let bytes1 = message.get(0);
        let bytes2 = message.get(1);
        assert_eq!(bytes1, Some([4, 3]));
        assert_eq!(bytes2, Some([2, 1]));

        assert!(message.is_valid());
        assert_eq!(message.length(), 3);
        assert_eq!(message.count(), 2);
        assert_eq!(message.size(), 3);
        assert!(message.is_command());
        assert!(!message.is_status());
    }

    #[test]
    fn test_message_set_bytes_command_2() {
        let mut message = Message::<2>::new().with_command(0b0000000000000001);

        message.add_bytes(&[1, 2, 3, 4]);

        let string1 = message.get::<[u8; 2]>(0);
        let string2 = message.get::<[u8; 2]>(1);

        assert_eq!(string1, Some([1, 2]));
        assert_eq!(string2, None);

        // failed because the given string is too long
        // to fit in a message with size 2
        assert!(!message.is_valid());

        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
        assert_eq!(message.size(), 2);
        assert!(message.is_command());
        assert!(!message.is_status());
    }

    #[test]
    fn test_message_add_string_command_0() {
        // build a command message with word count 1
        let mut message = Message::<2>::new().with_command(0b0000000000000001);

        message.add_string("TE");

        assert!(message.is_valid());
        assert!(message.is_full());
        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
        assert_eq!(message.size(), 2);
        assert!(message.is_command());
        assert!(!message.is_status());
    }

    #[test]
    fn test_message_add_string_command_1() {
        // build a command message with word count 1
        let mut message = Message::<2>::new().with_command(0b0000000000000001);

        message.add_string("TE");
        message.add_string("ST");

        // failed because the message only has enough space
        // for two characters (one data word), but two were given.
        assert!(!message.is_valid());

        assert!(message.is_full());
        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
        assert_eq!(message.size(), 2);
        assert!(message.is_command());
        assert!(!message.is_status());
    }

    #[test]
    fn test_message_add_string_command_2() {
        // build a command message with word count 1
        let mut message = Message::<2>::new().with_command(0b0000000000000001);

        message.add_string("TEST");

        // failed because the given string is too long
        // to fit in a message with size 2
        assert!(!message.is_valid());

        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
        assert_eq!(message.size(), 2);
        assert!(message.is_command());
        assert!(!message.is_status());
    }

    #[test]
    fn test_message_add_string_command_3() {
        // build a command message with word count 2
        let mut message = Message::<3>::new().with_command(0b0000000000000010);

        message.add_string("TEST");

        assert!(message.is_valid());
        assert_eq!(message.length(), 3);
        assert_eq!(message.count(), 2);
        assert_eq!(message.size(), 3);
        assert!(message.is_command());
        assert!(!message.is_status());
    }

    #[test]
    fn test_message_add_bytes_command_0() {
        // build a command message with word count 1
        let mut message = Message::<2>::new().with_command(0b0000000000000001);

        message.add_bytes(&[1, 2]);

        assert!(message.is_valid());
        assert!(message.is_full());
        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
        assert_eq!(message.size(), 2);
        assert!(message.is_command());
        assert!(!message.is_status());
    }

    #[test]
    fn test_message_add_bytes_command_1() {
        // build a command message with word count 1
        let mut message = Message::<2>::new().with_command(0b0000000000000001);

        message.add_bytes(&[1, 2]);
        message.add_bytes(&[3, 4]);

        // failed because the message only has enough space
        // for two characters (one data word), but two were given.
        assert!(!message.is_valid());

        assert!(message.is_full());
        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
        assert_eq!(message.size(), 2);
        assert!(message.is_command());
        assert!(!message.is_status());
    }

    #[test]
    fn test_message_add_bytes_command_2() {
        // build a command message with word count 1
        let mut message = Message::<2>::new().with_command(0b0000000000000001);

        message.add_bytes(&[1, 2, 3, 4]);

        // failed because the given string is too long
        // to fit in a message with size 2
        assert!(!message.is_valid());

        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
        assert_eq!(message.size(), 2);
        assert!(message.is_command());
        assert!(!message.is_status());
    }

    #[test]
    fn test_message_add_bytes_command_3() {
        // build a command message with word count 2
        let mut message = Message::<3>::new().with_command(0b0000000000000010);

        message.add_bytes(&[1, 2, 3, 4]);

        assert!(message.is_valid());
        assert_eq!(message.length(), 3);
        assert_eq!(message.count(), 2);
        assert_eq!(message.size(), 3);
        assert!(message.is_command());
        assert!(!message.is_status());
    }

    #[test]
    fn test_message_with_bad_count_fail() {
        // adding two header words to the message
        let message = Message::<2>::new()
            .with_command(0b0000000000000001)
            .with_data(0b0000000000000000)
            .with_data(0b0000000000000000);
        assert!(message.error.is_some());
        assert!(!message.is_valid());
    }

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
            .with_data(0b0000000000000001)
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
            .with_data(0b0000000000000001)
            .build()
            .unwrap();
        assert!(message.is_full());
        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
        assert_eq!(message.size(), 2);
        assert!(message.is_command());
        assert!(!message.is_status());
    }

    #[test]
    fn test_message_with_command_fail_duplicate_header() {
        // adding two header words to the message
        let message = Message::<2>::new()
            .with_command(0b0000000000000001)
            .with_command(0b0000000000000001)
            .build();
        assert!(message.is_err());
    }

    #[test]
    fn test_message_with_command_fail_header_not_first() {
        // add a data word to the message first
        let message = Message::<2>::new()
            .with_data(0b0000000000000001)
            .with_command(0b0000000000000001)
            .build();
        assert!(message.is_err());
    }

    #[test]
    fn test_message_with_command_fail_message_full() {
        let result = Message::<0>::new().with_command(0b0000000000000001).build();
        assert!(result.is_err());
    }

    #[test]
    fn test_message_add_command_fail_parity() {
        let word = CommandWord::new()
            .with_value(0b0000000000000000)
            .with_parity(0);

        let result = Message::<1>::new().with_command(word).build();

        assert_eq!(result, Err(Error::InvalidWord));
    }

    #[test]
    fn test_message_with_status() {
        let message = Message::<2>::new()
            .with_status(0b0000000000000001)
            .with_data(0b0000000000000001)
            .build()
            .unwrap();
        assert!(message.is_valid());
        assert!(message.is_full());
        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
        assert_eq!(message.size(), 2);
        assert!(message.is_status());
        assert!(!message.is_command());
    }

    #[test]
    fn test_message_with_status_fail_duplicate_header() {
        // adding two header words to the message
        let message = Message::<2>::new()
            .with_status(0b0000000000000001)
            .with_status(0b0000000000000001)
            .build();
        assert!(message.is_err());
    }

    #[test]
    fn test_message_with_status_fail_header_not_first() {
        // add a data word to the message first
        let message = Message::<2>::new()
            .with_data(0b0000000000000001)
            .with_status(0b0000000000000001)
            .build();
        assert!(message.is_err());
    }

    #[test]
    fn test_message_with_status_fail_message_full() {
        let result = Message::<0>::new().with_status(0b0000000000000001).build();
        assert!(result.is_err());
    }

    #[test]
    fn test_message_add_status_fail_parity() {
        let word = StatusWord::new()
            .with_value(0b0000000000000000)
            .with_parity(0);

        let result = Message::<1>::new().with_status(word).build();

        assert_eq!(result, Err(Error::InvalidWord));
    }

    #[test]
    fn test_message_with_data() {
        let message = Message::<2>::new()
            .with_status(0b0000000000000001)
            .with_data(0b0000000000000001)
            .build()
            .unwrap();
        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
        assert_eq!(message.size(), 2);
    }

    #[test]
    fn test_message_with_data_fail() {
        let result = Message::<2>::new().with_data(0b0000000000000001).build();
        assert!(result.is_err());
    }

    #[test]
    fn test_message_with_data_fail_message_full() {
        let result = Message::<2>::new()
            .with_status(0b0000000000000001)
            .with_data(0b0000000000000001)
            .with_data(0b0000000000000001)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_message_with_word_command() {
        let message = Message::<2>::new()
            .with_word(CommandWord::from(0b0000000000000001))
            .with_word(DataWord::from(0b0000000000000001))
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
            .with_word(DataWord::from(0b0000000000000001))
            .build()
            .unwrap();
        assert_eq!(message.length(), 2);
        assert_eq!(message.count(), 1);
        assert_eq!(message.size(), 2);
    }

    #[test]
    fn test_message_read_write_command_0() {
        let input = [0b10000011, 0b00001100, 0b00100010, 0b11010000, 0b11010010];
        let length = 2;

        let l = input.len();
        let mut buffer = [0; 10];

        let message = Message::<4>::read_command(&input).unwrap();
        message.write(&mut buffer).unwrap();

        assert_eq!(&buffer[..l], input);
        assert_eq!(message.length(), length);
    }

    #[test]
    fn test_message_read_write_command_1() {
        let input = [
            0b10000011, 0b00001100, 0b01110010, 0b11010000, 0b11010010, 0b00101111, 0b00101101,
            0b11100010, 0b11001110, 0b11011110,
        ];
        let length = 4;

        let l = input.len();
        let mut buffer = [0; 10];

        let message = Message::<4>::read_command(&input).unwrap();
        message.write(&mut buffer).unwrap();

        assert_eq!(&buffer[..l], input);
        assert_eq!(message.length(), length);
    }

    #[test]
    fn test_message_read_write_status_0() {
        let input = [0b10000011, 0b00001100, 0b00100010, 0b11010000, 0b11010010];
        let length = 2;

        let l = input.len();
        let mut buffer = [0; 10];

        let message = Message::<4>::read_status(&input).unwrap();
        message.write(&mut buffer).unwrap();

        assert_eq!(&buffer[..l], input);
        assert_eq!(message.length(), length);
    }

    #[test]
    fn test_message_read_write_status_1() {
        let input = [
            0b10000011, 0b00001100, 0b01110010, 0b11010000, 0b11010010, 0b00101111, 0b00101101,
            0b11100010, 0b11001110, 0b11011110,
        ];
        let length = 4;

        let l = input.len();
        let mut buffer = [0; 10];

        let message = Message::<4>::read_status(&input).unwrap();
        message.write(&mut buffer).unwrap();

        assert_eq!(&buffer[..l], input);
        assert_eq!(message.length(), length);
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
            .with_data(0b0000000000000000)
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
            .with_data(0b0000000000000000)
            .build()
            .unwrap();

        let result = message.write(&mut buffer);
        assert_eq!(result, Err(Error::OutOfBounds));
    }

    #[test]
    fn test_message_status() {
        let message = Message::<1>::new()
            .with_status(0b0000000000000001)
            .build()
            .unwrap();
        assert!(message.status().is_some());
    }

    #[test]
    fn test_message_command() {
        let message = Message::<2>::new()
            .with_command(0b0000000000000001)
            .with_data(0b0000000000000000)
            .build()
            .unwrap();
        assert!(message.command().is_some());
    }

    #[test]
    fn test_message_at() {
        let data1: DataWord = 0b0000000000000101.into();
        let data2: DataWord = 0b0000000000010101.into();
        let data3: DataWord = 0b0000000001010101.into();

        let message = Message::<4>::new()
            .with_command(0b0000000000000011)
            .with_data(data1)
            .with_data(data2)
            .with_data(data3)
            .build()
            .unwrap();

        let word1 = message.at(0);
        let word2 = message.at(1);
        let word3 = message.at(2);
        let word4 = message.at(3);

        assert_eq!(word1, Some(&data1));
        assert_eq!(word2, Some(&data2));
        assert_eq!(word3, Some(&data3));
        assert_eq!(word4, None);
    }

    #[test]
    fn test_message_get() {
        let data1: u16 = 0b0000000000000101;
        let data2: u16 = 0b0000000000010101;
        let data3: u16 = 0b0000000001010101;

        let message = Message::<4>::new()
            .with_command(0b0000000000000011)
            .with_data(data1)
            .with_data(data2)
            .with_data(data3)
            .build()
            .unwrap();

        let word1 = message.get::<u16>(0);
        let word2 = message.get::<u16>(1);
        let word3 = message.get::<u16>(2);
        let word4 = message.get::<u16>(3);

        assert_eq!(word1, Some(data1));
        assert_eq!(word2, Some(data2));
        assert_eq!(word3, Some(data3));
        assert_eq!(word4, None);
    }

    #[test]
    fn test_message_clear() {
        let mut message = Message::<2>::new()
            .with_command(0b0000000000000001)
            .with_data(0b0000000000000001)
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
