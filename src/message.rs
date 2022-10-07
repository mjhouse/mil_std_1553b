
use crate::constants::*;
use crate::flags::*;
use crate::errors::*;
use crate::words::*;

const DATA_SYNC: u8 = 0b00000111;
const SERV_SYNC: u8 = 0b00111000;

/// a message can only contain 32 words
const MAX_WORDS: u8 = 32;

/// Whether a message should be parsed as a sender or receiver
#[derive(Clone)]
pub enum MessageSide {
    Sending,
    Receiving,
}

/// The information transfer formats (DirectedMessage) are based on the command/response
/// philosophy in that all error free transmissions received by a remote
/// terminal are followed by the transmission of a status word from the
/// terminal to the bus controller. This handshaking principle validates the
/// receipt of the message by the remote terminal.
///
/// See: http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf (p. 29-30)
#[derive(Clone)]
pub enum DirectedMessage {
    BcToRt(MessageSide),
    RtToBc(MessageSide),
    RtToRt(MessageSide),
    ModeWithoutData(MessageSide),
    ModeWithDataT(MessageSide),
    ModeWithDataR(MessageSide),
}

/// Broadcast messages are transmitted to multiple remote terminals at the
/// same time. The terminals suppress the transmission of their status words
/// (not doing so would have multiple boxes trying to talk at the same time and
/// thereby “jam” the bus). In order for the bus controller to determine if a
/// terminal received the message, a polling sequence to each terminal must be
/// initiated to collect the status words.
///
/// See: http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf (p. 29-30)
#[derive(Clone)]
pub enum BroadcastMessage {
    BcToRt(MessageSide),
    RtToRt(MessageSide),
    ModeWithoutData(MessageSide),
    ModeWithDataR(MessageSide),
}

/// MessageType is used to signal the type of message that should be parsed
/// next.
#[derive(Clone)]
pub enum MessageType {
    Directed(DirectedMessage),
    Broadcast(BroadcastMessage),
}

pub struct Packet {
    sync:        u8,
    content: [u8;2],
    parity:      u8,
}

pub struct Message {
    side: MessageType,
    count: u8,
    words: [Word;32],
}

impl Packet {

    pub fn is_data(&self) -> bool {
        self.sync == DATA_SYNC
    }

    pub fn is_valid(&self) -> bool {
        let count = self.content[0].count_ones() +
                    self.content[1].count_ones() + 
                    (self.parity & 0b00000001) as u32;
        (count % 2) != 0
    }

}

impl Message {
    
    pub fn new(side: MessageType) -> Self {
        Self {
            side: side,
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

    fn add(&mut self, word: Word) -> Result<u8> {

        if self.is_full() {
            return Err(MESSAGE_FULL);
        }

        self.words[self.count as usize] = word;
        self.count += 1;  
        Ok(self.count)
    }

    fn clear(&mut self) {
        self.words = [Word::None;32];
        self.count = 0;
    }

    fn last(&self) -> Option<&Word> {
        if !self.is_empty() {
            let index = self.count.saturating_sub(1);
            Some(&self.words[index as usize])
        } else {
            None
        }
    }

    fn first(&self) -> Option<&Word> {
        if !self.is_empty() {
            Some(&self.words[0])
        } else {
            None
        }
    }

    fn data_count(&self) -> u8 {
        self.words
            .iter()
            .filter(|w| w.is_data())
            .count() as u8
    }

    /// parses a single word and adds it to the message,
    /// returning either an error or the new length of the parsed
    /// message.
    pub fn parse(&mut self, packet: Packet) -> Result<u8> {

        if !packet.is_valid() {
            return Err(RESERVED_USED);
        }

        use MessageType::*;
        use DirectedMessage as D;
        use BroadcastMessage as B;

        match self.side.clone() {
            Directed(D::BcToRt(side)) => 
                self.parse_bc_to_rt_directed(side,packet),
            Directed(D::RtToBc(side)) => 
                self.parse_rt_to_bc_directed(side,packet),
            Directed(D::RtToRt(side)) => 
                self.parse_rt_to_rt_directed(side,packet),
            Directed(D::ModeWithoutData(side)) => 
                self.parse_mode_without_data_directed(side,packet),
            Directed(D::ModeWithDataT(side)) => 
                self.parse_mode_with_data_t_directed(side,packet),
            Directed(D::ModeWithDataR(side)) => 
                self.parse_mode_with_data_r_directed(side,packet),
            Broadcast(B::BcToRt(side)) => 
                self.parse_bc_to_rt_broadcast(side,packet),
            Broadcast(B::RtToRt(side)) => 
                self.parse_rt_to_rt_broadcast(side,packet),
            Broadcast(B::ModeWithoutData(side)) => 
                self.parse_mode_without_data_broadcast(side,packet),
            Broadcast(B::ModeWithDataR(side)) => 
                self.parse_mode_with_data_r_broadcast(side,packet),
            _ => Err(UNKNOWN_MESSAGE_TYPE),
        }

    }

    /// Parse BC -> RT Directed
    ///
    /// If this message is being parsed on the "receiving" side, then
    /// we are parsing the SEND message (and then responding with the
    /// RESP message). If it's being parsed on the "sending" side, then 
    /// we're going to be parsing the RESP only.
    ///
    /// Pattern:
    ///     SEND: RECV COMM | DATA | DATA | ...
    ///     RESP: STAT
    ///
    fn parse_bc_to_rt_directed(&mut self, side: MessageSide, packet: Packet) -> Result<u8> {
        match (side,self.first()) {
            (MessageSide::Receiving,Some(Word::Command(c))) => {

                if self.data_count().saturating_add(1) > c.word_count() {
                    return Err(MESSAGE_BAD);
                }

                self.add(Word::data(packet.content))
            },
            (MessageSide::Receiving,None) => 
                self.add(Word::receive_command(packet.content)?),
            (MessageSide::Sending,None) => 
                self.add(Word::status(packet.content)),
            (MessageSide::Receiving,_) => Err(MESSAGE_BAD), /* TODO: error because first word should be command */
            (MessageSide::Sending,_) => Err(MESSAGE_BAD), /* TODO: error because message should be empty */
        }
    }

    /// Parse RT -> BC Directed
    ///
    /// If this message is being parsed on the "receiving" side, then
    /// we are parsing the SEND message (and then responding with the
    /// RESP message). If it's being parsed on the "sending" side, then 
    /// we're going to be parsing the RESP only.
    ///
    /// Pattern:
    ///     SEND: TRAN COMM 
    ///     RESP: STAT | DATA | DATA | ...
    ///
    fn parse_rt_to_bc_directed(&mut self, side: MessageSide, packet: Packet) -> Result<u8> {
        match (side,self.first()) {
            (MessageSide::Receiving,None) =>
                self.add(Word::transmit_command(packet.content)?),
            (MessageSide::Sending,Some(Word::Status(_))) => 
                self.add(Word::data(packet.content)),
            (MessageSide::Sending,None) => 
                self.add(Word::status(packet.content)),
            (MessageSide::Receiving,_) => Err(MESSAGE_BAD), /* TODO: error because there should be only one word */
            (MessageSide::Sending,_) => Err(MESSAGE_BAD), /* TODO: error because word should be status and isn't */
        }
    }

    /// Parse RT -> RT Directed
    ///
    /// If this message is being parsed on the "receiving" side, then
    /// we are parsing the SEND message (and then responding with the
    /// RESP message). If it's being parsed on the "sending" side, then 
    /// we're going to be parsing the RESP only.
    ///
    /// Pattern:
    ///     SEND: RECV COMM | TRAN COMM |
    ///     RESP: STAT | DATA | DATA | ...
    ///
    fn parse_rt_to_rt_directed(&mut self, side: MessageSide, packet: Packet) -> Result<u8> {
        match (side,self.first()) {
            (MessageSide::Receiving,None) =>
                self.add(Word::receive_command(packet.content)?),
            (MessageSide::Receiving,Some(Word::Command(_))) =>
                self.add(Word::transmit_command(packet.content)?),
            (MessageSide::Sending,None) => 
                self.add(Word::status(packet.content)),
            (MessageSide::Sending,Some(Word::Status(_))) => 
                self.add(Word::data(packet.content)),
            (MessageSide::Receiving,_) => Err(MESSAGE_BAD), /* TODO: error because words should only be commands */
            (MessageSide::Sending,_) => Err(MESSAGE_BAD), /* TODO: error because word should be status and isn't */
        }
    }

    /// Parse Without Data Directed
    ///
    /// If this message is being parsed on the "receiving" side, then
    /// we are parsing the SEND message (and then responding with the
    /// RESP message). If it's being parsed on the "sending" side, then 
    /// we're going to be parsing the RESP only.
    ///
    /// Pattern:
    ///     SEND: MOD COMM
    ///     RESP: STAT
    ///
    fn parse_mode_without_data_directed(&mut self, side: MessageSide, packet: Packet) -> Result<u8> {
        match (side,self.first()) {
            (MessageSide::Receiving,None) =>
                self.add(Word::mode_code_command(packet.content)?),
            (MessageSide::Sending,None) => 
                self.add(Word::status(packet.content)),
            (MessageSide::Receiving,_) => Err(MESSAGE_BAD), /* TODO: error because word should only be command */
            (MessageSide::Sending,_) => Err(MESSAGE_BAD), /* TODO: error because word should only be status */
        }
    }

    /// Parse With Data Directed
    ///
    /// If this message is being parsed on the "receiving" side, then
    /// we are parsing the SEND message (and then responding with the
    /// RESP message). If it's being parsed on the "sending" side, then 
    /// we're going to be parsing the RESP only.
    ///
    /// Pattern:
    ///     SEND: MOD COMM
    ///     RESP: STAT | DATA |
    ///
    fn parse_mode_with_data_t_directed(&mut self, side: MessageSide, packet: Packet) -> Result<u8> {
        match (side,self.first()) {
            (MessageSide::Receiving,None) =>
                self.add(Word::mode_code_command(packet.content)?),
            (MessageSide::Sending,None) => 
                self.add(Word::status(packet.content)),
            (MessageSide::Sending,Some(Word::Status(_))) => {
                if self.data_count() > 0 {
                    return Err(MESSAGE_BAD);
                }

                self.add(Word::data(packet.content))
            },
            (MessageSide::Receiving,_) => Err(MESSAGE_BAD), /* TODO: error because word should only be command */
            (MessageSide::Sending,_) => Err(MESSAGE_BAD), /* TODO: error because word should only be status/data */
        }
    }

    /// Parse With Data R Directed
    ///
    /// If this message is being parsed on the "receiving" side, then
    /// we are parsing the SEND message (and then responding with the
    /// RESP message). If it's being parsed on the "sending" side, then 
    /// we're going to be parsing the RESP only.
    ///
    /// Pattern:
    ///     SEND: MOD COMM | DATA |
    ///     RESP: STAT
    ///
    fn parse_mode_with_data_r_directed(&mut self, side: MessageSide, packet: Packet) -> Result<u8> {
        match (side,self.first()) {
            (MessageSide::Receiving,None) =>
                self.add(Word::mode_code_command(packet.content)?),
            (MessageSide::Receiving,Some(Word::Command(_))) => {
                if self.data_count() > 0 {
                    return Err(MESSAGE_BAD);
                }

                self.add(Word::data(packet.content))
            },
            (MessageSide::Sending,None) => 
                self.add(Word::status(packet.content)),
            (MessageSide::Receiving,_) => Err(MESSAGE_BAD), /* TODO: error because word should only be command/data */
            (MessageSide::Sending,_) => Err(MESSAGE_BAD), /* TODO: error because word should only be status */
        }
    }

    /// Parse BC to RT Broadcast
    ///
    /// If this message is being parsed on the "receiving" side, then
    /// we are parsing the SEND message (and then responding with the
    /// RESP message). If it's being parsed on the "sending" side, then 
    /// we're going to be parsing the RESP only.
    ///
    /// Pattern:
    ///     SEND: RECV COMM | DATA | DATA | ...
    ///
    fn parse_bc_to_rt_broadcast(&mut self, side: MessageSide, packet: Packet) -> Result<u8> {
        match (side,self.first()) {
            (MessageSide::Receiving,None) => 
                self.add(Word::receive_command(packet.content)?),
            (MessageSide::Receiving,Some(Word::Command(c))) => {

                if self.data_count().saturating_add(1) > c.word_count() {
                    return Err(MESSAGE_BAD);
                }

                self.add(Word::data(packet.content))

            },
            (MessageSide::Receiving,_) => Err(MESSAGE_BAD), /* TODO: error because word should only be command/data */
            (MessageSide::Sending,_) => Err(MESSAGE_BAD), /* TODO: error because sending side should never parse */
        }
    }

    /// Parse RT to RT Broadcast
    ///
    /// If this message is being parsed on the "receiving" side, then
    /// we are parsing the SEND message (and then responding with the
    /// RESP message). If it's being parsed on the "sending" side, then 
    /// we're going to be parsing the RESP only.
    ///
    /// Pattern:
    ///     SEND: RECV COMM | TRAN COMM |
    ///     RESP: STAT | DATA | DATA | ...
    ///
    fn parse_rt_to_rt_broadcast(&mut self, side: MessageSide, packet: Packet) -> Result<u8> {
        match (side,self.first()) {
            (MessageSide::Receiving,None) =>
                self.add(Word::receive_command(packet.content)?),
            (MessageSide::Receiving,Some(Word::Command(_))) => 
                self.add(Word::transmit_command(packet.content)?),
            (MessageSide::Sending,None) => 
                self.add(Word::status(packet.content)),
            (MessageSide::Sending,Some(Word::Status(_))) => 
                self.add(Word::data(packet.content)),
            (MessageSide::Receiving,_) => Err(MESSAGE_BAD), /* TODO: error because word should only be command/data */
            (MessageSide::Sending,_) => Err(MESSAGE_BAD), /* TODO: error because word should only be status/data */
        }
    }

    /// Parse Mode Without Data Broadcast
    ///
    /// If this message is being parsed on the "receiving" side, then
    /// we are parsing the SEND message (and then responding with the
    /// RESP message). If it's being parsed on the "sending" side, then 
    /// we're going to be parsing the RESP only.
    ///
    /// Pattern:
    ///     SEND: MOD COMM
    ///
    fn parse_mode_without_data_broadcast(&mut self, side: MessageSide, packet: Packet) -> Result<u8> {
        match (side,self.first()) {
            (MessageSide::Receiving,None) => 
                self.add(Word::mode_code_command(packet.content)?),
            (MessageSide::Receiving,_) => Err(MESSAGE_BAD), /* TODO: error because word should only be command */
            (MessageSide::Sending,_) => Err(MESSAGE_BAD), /* TODO: error because sending side should never parse */
        }
    }

    /// Parse Mode With Data Broadcast
    ///
    /// If this message is being parsed on the "receiving" side, then
    /// we are parsing the SEND message (and then responding with the
    /// RESP message). If it's being parsed on the "sending" side, then 
    /// we're going to be parsing the RESP only.
    ///
    /// Pattern:
    ///     SEND: MOD COMM | DATA |
    ///
    fn parse_mode_with_data_r_broadcast(&mut self, side: MessageSide, packet: Packet) -> Result<u8> {
        match (side,self.first()) {
            (MessageSide::Receiving,None) => 
                self.add(Word::mode_code_command(packet.content)?),
            (MessageSide::Receiving,Some(Word::Command(_))) => {
                if self.data_count() > 0 {
                    return Err(MESSAGE_BAD);
                }

                self.add(Word::data(packet.content))
            },
            (MessageSide::Receiving,_) => Err(MESSAGE_BAD), /* TODO: error because word should only be command or data */
            (MessageSide::Sending,_) => Err(MESSAGE_BAD), /* TODO: error because sending side should never parse */
        }
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
    data = &data[..2];
    
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