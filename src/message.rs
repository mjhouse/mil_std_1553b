
use crate::constants::*;
use crate::flags::*;
use crate::errors::*;
use crate::words::*;

/// a message can only contain 32 words
const MAX_WORDS: usize = 32;

/// The information transfer formats (DirectedMessage) are based on the command/response
/// philosophy in that all error free transmissions received by a remote
/// terminal are followed by the transmission of a status word from the
/// terminal to the bus controller. This handshaking principle validates the
/// receipt of the message by the remote terminal.
///
/// See: http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf (p. 29-30)
pub enum DirectedMessage {
    BcToRt,
    RtToBc,
    RtToRt,
    ModeWithoutData,
    ModeWithDataT,
    ModeWithDataR,
}

/// Broadcast messages are transmitted to multiple remote terminals at the
/// same time. The terminals suppress the transmission of their status words
/// (not doing so would have multiple boxes trying to talk at the same time and
/// thereby “jam” the bus). In order for the bus controller to determine if a
/// terminal received the message, a polling sequence to each terminal must be
/// initiated to collect the status words.
///
/// See: http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf (p. 29-30)
pub enum BroadcastMessage {
    BcToRt,
    RtToRt,
    ModeWithoutData,
    ModeWithData,
}

pub enum MessageType {
    Directed(DirectedMessage),
    Broadcast(BroadcastMessage),
}

pub struct Message {
    count: usize,
    words: [Word;32]
}

impl Message {
    
    pub fn new() -> Self {
        Self {
            count: 0,
            words: [Word::None;32],
        }
    }

    pub fn is_full(&self) -> bool {
        self.count >= MAX_WORDS
    }

    pub fn add(&mut self, word: Word) {
        if !self.is_full() {
            self.words[self.count] = word;
            self.count += 1;  
        }
    }

    pub fn clear(&mut self) {
        self.words = [Word::None;32];
        self.count = 0;
    }

    /// Function assumes that the given data contains sync and parity
    /// bits and is aligned to the first sync bits (first 3 bits of the 
    /// first byte are sync).
    pub fn parse(data: &[u8]) -> Self {
        let mut message = Message::new();
        let mut index = 0;

        while index < data.len() && !message.is_full() {
            let first = index == 0;
            let chunk = &data[index..index + 3];
            let word  = Word::parse(first, chunk);

            message.add(word);
            index += 3;
        }

        message
    }

    pub fn is_valid(&self) -> bool {
        false
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn testy_mctest_face() {

    // }

}