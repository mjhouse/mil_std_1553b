
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
    None,
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

    pub fn is_empty(&self) -> bool {
        self.count == 0
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

    pub fn last(&self) -> &Word {
        &self.words[self.count.saturating_sub(1)]
    }

    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut message = Message::new();

        for (i,chunk) in data.chunks(2).enumerate() {
            // convert two bytes to a single u16
            let value = ((chunk[0] as u16) << 8) | (chunk[1] as u16);

            if i == 0 {
                let word = CommandWord::new(value);

            }

            // // provide context about the message to the word parse
            // let word = Word::command(value);

            // // add the word to the message
            // message.add(word);
        }

        Ok(message)
    }

    pub fn is_valid(&self) -> bool {
        false
    }

}

/// Given data with unspecified length, removes the first
/// 20 bits and converts them to a (sync,data,parity) triplet.
fn process(align: bool, mut data: &[u8]) -> (u8,u16,u8) {
    let mut sync = 0;
    let mut word = 0;
    let mut parity = 0;

    // aligned:
    //      | 11111111 | 11111111 | 11110000 |
    // unaligned: 
    //      | 00001111 | 11111111 | 11111111 |

    if align {
        parity |= (data[2] & 0b00010000) >> 4;
        sync |= (data[0] & 0b11100000) >> 5;
        word |= (data[0] as u16 & 0b0000000000011111) << 11;
        word |= (data[1] as u16 & 0b0000000011111111) << 3;
        word |= (data[2] as u16 & 0b0000000011100000) >> 5;
    }
    else {
        parity |= data[2] & 0b00000001;
        sync |= (data[0] & 0b00001110) >> 1;
        word |= (data[0] as u16 & 0b0000000000000001) << 15;
        word |= (data[1] as u16 & 0b0000000011111111) << 7;
        word |= (data[2] as u16 & 0b0000000011111110) >> 1;
    }

    // take off the first 2 bytes
    data = data[..2];
    
    (sync,word,parity)
}

#[cfg(test)]
mod tests {
    use super::*;

    // only useable with no_std commented out
    macro_rules! debug_bytes {
        ( $w: expr ) => {
            println!("bits: {:#018b}", $w);
        }
    }


    #[test]
    fn test_process_aligned_word_pattern() {
        let (_,word,_) = process(true, &[
            0b00010101,
            0b01010101,
            0b01000000,
        ]);
        assert_eq!(word,0b1010101010101010);
    }

    #[test]
    fn test_process_unaligned_word_pattern() {
        let (_,word,_) = process(false, &[
            0b00000001,
            0b01010101,
            0b01010100,
        ]);
        assert_eq!(word,0b1010101010101010);
    }

    #[test]
    fn test_process_aligned_word_ones() {
        let (_,word,_) = process(true, &[
            0b00011111,
            0b11111111,
            0b11100000,
        ]);
        assert_eq!(word,0b1111111111111111);
    }

    #[test]
    fn test_process_unaligned_word_ones() {
        let (_,word,_) = process(false, &[
            0b00000001,
            0b11111111,
            0b11111110,
        ]);
        assert_eq!(word,0b1111111111111111);
    }

    #[test]
    fn test_process_aligned_word_zeros() {
        let (_,word,_) = process(true, &[
            0b11100000,
            0b00000000,
            0b00011111,
        ]);
        assert_eq!(word,0);
    }

    #[test]
    fn test_process_unaligned_word_zeros() {
        let (_,word,_) = process(false, &[
            0b11111110,
            0b00000000,
            0b00000001,
        ]);
        assert_eq!(word,0);
    }

    #[test]
    fn test_process_aligned_sync_zeros() {
        let (sync,_,_) = process(true, &[
            0b00011111,
            0b11111111,
            0b11111111,
        ]);
        assert_eq!(sync,0);
    }

    #[test]
    fn test_process_unaligned_sync_zeros() {
        let (sync,_,_) = process(false, &[
            0b11110001,
            0b11111111,
            0b11111111,
        ]);
        assert_eq!(sync,0);
    }

    #[test]
    fn test_process_aligned_sync_ones() {
        let (sync,_,_) = process(true, &[
            0b11100000,
            0b00000000,
            0b00000000,
        ]);
        assert_eq!(sync,7);
    }

    #[test]
    fn test_process_unaligned_sync_ones() {
        let (sync,_,_) = process(false, &[
            0b00001110,
            0b00000000,
            0b00000000,
        ]);
        assert_eq!(sync,7);
    }

    #[test]
    fn test_process_aligned_parity_bit_one() {
        let (_,_,parity) = process(true, &[
            0b00000000,
            0b00000000,
            0b00010000,
        ]);
        assert_eq!(parity,1);
    }

    #[test]
    fn test_process_unaligned_parity_bit_one() {
        let (_,_,parity) = process(false, &[
            0b00000000,
            0b00000000,
            0b00000001,
        ]);
        assert_eq!(parity,1);
    }

    #[test]
    fn test_process_aligned_parity_bit_zero() {
        let (_,_,parity) = process(true, &[
            0b11111111,
            0b11111111,
            0b11101111,
        ]);
        assert_eq!(parity,0);
    }

    #[test]
    fn test_process_unaligned_parity_bit_zero() {
        let (_,_,parity) = process(false, &[
            0b11111111,
            0b11111111,
            0b11111110,
        ]);
        assert_eq!(parity,0);
    }

}