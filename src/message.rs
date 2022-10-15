
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

#[derive(Clone,Copy)]
pub struct Packet {
    pub sync: u8,
    pub content: [u8;2],
    pub parity: u8,
}

pub struct Message {
    side: MessageType,
    limit: u8,
    count: u8,
    words: [Word;MAX_WORDS as usize],
}

macro_rules! make_count {
    ( $c:expr ) => {{
        $c[0].count_ones() + 
        $c[1].count_ones()
    }}
}

macro_rules! make_parity {
    ( $c:expr ) => {{
        match make_count!($c) % 2 {
            0 => 1,
            _ => 0,
        }
    }}
}

impl Packet {

    pub fn new(sync: u8, content: [u8;2], parity: u8) -> Self {
        Self { sync, content, parity }
    }

    pub fn data(data: [u8;2]) -> Self {
        Self::new(DATA_SYNC,data,make_parity!(data))
    }

    pub fn service(data: [u8;2]) -> Self {
        Self::new(SERV_SYNC,data,make_parity!(data))
    }

    pub fn is_data(&self) -> bool {
        self.sync == DATA_SYNC
    }

    pub fn is_service(&self) -> bool {
        self.sync == SERV_SYNC
    }

    pub fn is_valid(&self) -> bool {
        make_parity!(self.content) == self.parity
    }

}

impl Message {
    
    pub fn new(side: MessageType) -> Self {
        Self {
            side: side,
            limit: MAX_WORDS,
            count: 0,
            words: [Word::None;MAX_WORDS as usize],
        }
    }

    pub fn is_full(&self) -> bool {
        self.count >= self.limit
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

    fn has_data(&self) -> bool {
        self.data_count() > 0
    }

    fn has_space(&self) -> bool {
        if let Some(Word::Command(c)) = self.first() {
            self.data_count() < c.word_count()
        }
        else {
            false
        }
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
            (MessageSide::Receiving,Some(Word::Command(c))) if self.has_space() =>
                self.add(Word::data(packet.content)),
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
            (MessageSide::Sending,Some(Word::Status(_))) if !self.has_data() => 
                self.add(Word::data(packet.content)),
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
            (MessageSide::Receiving,Some(Word::Command(_))) if !self.has_data() => 
                self.add(Word::data(packet.content)),
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
            (MessageSide::Receiving,Some(Word::Command(c))) if self.has_space() =>
                self.add(Word::data(packet.content)),
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
            (MessageSide::Receiving,Some(Word::Command(_))) if !self.has_data() =>
                self.add(Word::data(packet.content)),
            (MessageSide::Receiving,_) => Err(MESSAGE_BAD), /* TODO: error because word should only be command or data */
            (MessageSide::Sending,_) => Err(MESSAGE_BAD), /* TODO: error because sending side should never parse */
        }
    }

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
    fn test_new_data_packet() {
        let packet = Packet::data([0b00000000,0b00000000]);
        assert!(packet.is_data());
    }

    #[test]
    fn test_new_service_packet() {
        let packet = Packet::service([0b00000000,0b00000000]);
        assert!(packet.is_service());
    }

    #[test]
    fn test_packet_parity_even_both() {
        let packet = Packet::data([0b01000000,0b00100000]);
        assert_eq!(packet.parity,1);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_even_first() {
        let packet = Packet::data([0b01100000,0b00000000]);
        assert_eq!(packet.parity,1);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_even_second() {
        let packet = Packet::data([0b00000000,0b00110000]);
        assert_eq!(packet.parity,1);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_odd_both() {
        let packet = Packet::data([0b00110000,0b00001000]);
        assert_eq!(packet.parity,0);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_odd_first() {
        let packet = Packet::data([0b00111000,0b00000000]);
        assert_eq!(packet.parity,0);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_odd_second() {
        let packet = Packet::data([0b00000000,0b00111000]);
        assert_eq!(packet.parity,0);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_bad_parity_odd() {
        let mut packet = Packet::data([0b00000000,0b00111000]);
        packet.parity = 1;
        assert_eq!(packet.parity,1);
        assert!(!packet.is_valid());
    }

    #[test]
    fn test_packet_bad_parity_even() {
        let mut packet = Packet::data([0b00000000,0b00110000]);
        packet.parity = 0;
        assert_eq!(packet.parity,0);
        assert!(!packet.is_valid());
    }

    #[test]
    fn test_parse_bc_to_rt_directed_receiving() {
        let mut message = Message::new(
            MessageType::Directed(
                DirectedMessage::BcToRt(
                    MessageSide::Receiving)));

        // receive command with word count of 2 
        let packets = vec![
            Packet::service([0b00000000, 0b00000010]),
            Packet::data([0b00000000,0b00000000]),
            Packet::data([0b00000000,0b00000000]),
        ];

        let mut result;

        // parse the command
        result = message.parse(packets[0]);
        assert_eq!(result,Ok(1));

        // parse first data word
        result = message.parse(packets[1]);
        assert_eq!(result,Ok(2));

        // parse second data word
        result = message.parse(packets[2]);
        assert_eq!(result,Ok(3));

        // parse too many data words
        result = message.parse(packets[2]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_bc_to_rt_directed_sending() {
        let mut message = Message::new(
            MessageType::Directed(
                DirectedMessage::BcToRt(
                    MessageSide::Sending)));

        // receive command with word count of 2 
        let packets = vec![
            Packet::service([0b00000000, 0b00000000]),
            Packet::data([0b00000000,0b00000000])
        ];

        let mut result;

        // parse the command
        result = message.parse(packets[0]);
        assert_eq!(result,Ok(1));

        // parse unexpected data word
        result = message.parse(packets[1]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_rt_to_bc_directed_receiving() {
        let mut message = Message::new(
            MessageType::Directed(
                DirectedMessage::RtToBc(
                    MessageSide::Receiving)));

        // receive command with word count of 2 
        let packets = vec![
            Packet::service([0b00000100, 0b00000010]),
            Packet::data([0b00000000,0b00000000])
        ];

        let mut result;

        // parse the command
        result = message.parse(packets[0]);
        assert_eq!(result,Ok(1));

        // parse unexpected data word
        result = message.parse(packets[1]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_rt_to_rt_directed_sending() {
        let mut message = Message::new(
            MessageType::Directed(
                DirectedMessage::RtToRt(
                    MessageSide::Sending)));

        // receive command with word count of 2 
        let packets = vec![
            Packet::service([0b00000000, 0b00000000]),
            Packet::data([0b00000000,0b00000000]),
            Packet::data([0b00000000,0b00000000]),
        ];

        let mut result;

        // parse the command
        result = message.parse(packets[0]);
        assert_eq!(result,Ok(1));

        // parse first data word
        result = message.parse(packets[1]);
        assert_eq!(result,Ok(2));

        // parse second data word
        result = message.parse(packets[2]);
        assert_eq!(result,Ok(3));
    }

    #[test]
    fn test_parse_rt_to_rt_directed_receiving() {
        let mut message = Message::new(
            MessageType::Directed(
                DirectedMessage::RtToRt(
                    MessageSide::Receiving)));

        // receive command with word count of 2 
        let packets = vec![
            Packet::service([0b00000000, 0b00000010]),
            Packet::service([0b00000100, 0b00000010]),
        ];

        let mut result;

        // parse the receive command
        result = message.parse(packets[0]);
        assert_eq!(result,Ok(1));

        // parse the transmit command
        result = message.parse(packets[1]);
        assert_eq!(result,Ok(2));
    }

    #[test]
    fn test_parse_mode_without_data_directed_receiving() {
        let mut message = Message::new(
            MessageType::Directed(
                DirectedMessage::ModeWithoutData(
                    MessageSide::Receiving)));

        let packets = vec![
            Packet::service([0b00000000, 0b00011111]), // mode code command
            Packet::data([0b00000000, 0b00000000]), // data word
        ];

        let mut result;

        // parse the receive command
        result = message.parse(packets[0]);
        assert_eq!(result,Ok(1));

        // parse too many words
        result = message.parse(packets[1]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_mode_without_data_directed_sending() {
        let mut message = Message::new(
            MessageType::Directed(
                DirectedMessage::ModeWithoutData(
                    MessageSide::Sending)));

        let packets = vec![
            Packet::service([0b00000000, 0b00000000]), // status word
            Packet::data([0b00000000, 0b00000000]), // data word
        ];

        let mut result;

        // parse the status word
        result = message.parse(packets[0]);
        assert_eq!(result,Ok(1));

        // parse too many words
        result = message.parse(packets[1]);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_parse_mode_with_data_t_directed_receiving() {
        let mut message = Message::new(
            MessageType::Directed(
                DirectedMessage::ModeWithDataT(
                    MessageSide::Receiving)));

        let packets = vec![
            Packet::service([0b00000000, 0b00000000]), // mode code word
            Packet::data([0b00000000, 0b00000000]), // data word
        ];

        let mut result;

        // parse the command word
        result = message.parse(packets[0]);
        assert_eq!(result,Ok(1));

        // parse too many words
        result = message.parse(packets[1]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_mode_with_data_t_directed_sending() {
        let mut message = Message::new(
            MessageType::Directed(
                DirectedMessage::ModeWithDataT(
                    MessageSide::Sending)));

        let packets = vec![
            Packet::service([0b00000000, 0b00000000]), // status word
            Packet::data([0b00000000, 0b00000000]), // data word
            Packet::data([0b00000000, 0b00000000]), // data word
        ];

        let mut result;

        // parse the command word
        result = message.parse(packets[0]);
        assert_eq!(result,Ok(1));

        // parse one data word
        result = message.parse(packets[1]);
        assert_eq!(result,Ok(2));

        // parse too many data words
        result = message.parse(packets[2]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_mode_with_data_r_directed_receiving() {
        let mut message = Message::new(
            MessageType::Directed(
                DirectedMessage::ModeWithDataR(
                    MessageSide::Receiving)));

        let packets = vec![
            Packet::service([0b00000000, 0b00000000]), // service word
            Packet::data([0b00000000, 0b00000000]), // data word
            Packet::data([0b00000000, 0b00000000]), // data word
        ];

        let mut result;

        // parse the command word
        result = message.parse(packets[0]);
        assert_eq!(result,Ok(1));

        // parse one data word
        result = message.parse(packets[1]);
        assert_eq!(result,Ok(2));

        // parse too many data words
        result = message.parse(packets[2]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_mode_with_data_r_directed_sending() {
        let mut message = Message::new(
            MessageType::Directed(
                DirectedMessage::ModeWithDataR(
                    MessageSide::Sending)));

        let packets = vec![
            Packet::service([0b00000000, 0b00000000]), // service word
            Packet::data([0b00000000, 0b00000000]), // data word
        ];

        let mut result;

        // parse the command word
        result = message.parse(packets[0]);
        assert_eq!(result,Ok(1));

        // parse too many data words
        result = message.parse(packets[1]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_bc_to_rt_broadcast_receiving() {
        let mut message = Message::new(
            MessageType::Broadcast(
                BroadcastMessage::BcToRt(
                    MessageSide::Receiving)));

        let packets = vec![
            Packet::service([0b00000000, 0b00000000]), // service word
            Packet::data([0b00000000, 0b00000000]),    // data word
            Packet::data([0b00000000, 0b00000000]),    // data word
        ];

        let mut result;

        // parse the command word
        result = message.parse(packets[0]);
        assert_eq!(result,Ok(1));

        // parse first data word
        result = message.parse(packets[1]);
        assert_eq!(result,Ok(2));

        // parse second data word
        result = message.parse(packets[2]);
        assert_eq!(result,Ok(3));
    }

    #[test]
    fn test_parse_bc_to_rt_broadcast_sending() {
        let mut message = Message::new(
            MessageType::Broadcast(
                BroadcastMessage::BcToRt(
                    MessageSide::Sending)));

        let packets = vec![
            Packet::service([0b00000000, 0b00000000]), // service word
        ];

        let mut result;

        // parse the command word
        result = message.parse(packets[0]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_rt_to_rt_broadcast_receiving() {
        let mut message = Message::new(
            MessageType::Broadcast(
                BroadcastMessage::RtToRt(
                    MessageSide::Receiving)));

        let packets = vec![
            Packet::service([0b00000000, 0b00000000]), // receive command
            Packet::service([0b00000100, 0b00000000]), // transmit command
            Packet::data([0b00000000, 0b00000000]),    // data word
        ];

        let mut result;

        // parse first command word
        result = message.parse(packets[0]);
        assert_eq!(result,Ok(1));

        // parse second command word
        result = message.parse(packets[1]);
        assert_eq!(result,Ok(2));

        // parse data word
        result = message.parse(packets[2]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_rt_to_rt_broadcast_sending() {
        let mut message = Message::new(
            MessageType::Broadcast(
                BroadcastMessage::RtToRt(
                    MessageSide::Sending)));

        let packets = vec![
            Packet::service([0b00000000, 0b00000000]), // status word
            Packet::data([0b00000000, 0b00000000]),    // data word
            Packet::data([0b00000000, 0b00000000]),    // data word
        ];

        let mut result;

        // parse status word
        result = message.parse(packets[0]);
        assert_eq!(result,Ok(1));

        // parse data word
        result = message.parse(packets[1]);
        assert_eq!(result,Ok(2));

        // parse data word
        result = message.parse(packets[2]);
        assert_eq!(result,Ok(3));
    }

    #[test]
    fn test_parse_mode_without_data_broadcast_receiving() {
        let mut message = Message::new(
            MessageType::Broadcast(
                BroadcastMessage::ModeWithoutData(
                    MessageSide::Receiving)));

        let packets = vec![
            Packet::service([0b00000000, 0b00000000]), // mode code command
            Packet::data([0b00000000, 0b00000000]), // data word
        ];

        let mut result;

        // parse command word
        result = message.parse(packets[0]);
        assert_eq!(result,Ok(1));

        // parse data word
        result = message.parse(packets[1]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_mode_without_data_broadcast_sending() {
        let mut message = Message::new(
            MessageType::Broadcast(
                BroadcastMessage::ModeWithoutData(
                    MessageSide::Sending)));

        let packets = vec![
            Packet::service([0b00000000, 0b00000000]), // status word
            Packet::data([0b00000000, 0b00000000]), // data word
        ];

        let mut result;

        // parse status word
        result = message.parse(packets[0]);
        assert!(result.is_err());

        // parse data word
        result = message.parse(packets[1]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_mode_with_data_r_broadcast_receiving() {
        let mut message = Message::new(
            MessageType::Broadcast(
                BroadcastMessage::ModeWithDataR(
                    MessageSide::Receiving)));

        let packets = vec![
            Packet::service([0b00000000, 0b00000000]), // mode code command
            Packet::data([0b00000000, 0b00000000]), // data word
        ];

        let mut result;

        // parse command word
        result = message.parse(packets[0]);
        assert_eq!(result,Ok(1));

        // parse data word
        result = message.parse(packets[1]);
        assert_eq!(result,Ok(2));
    }

    #[test]
    fn test_parse_mode_with_data_r_broadcast_sending() {
        let mut message = Message::new(
            MessageType::Broadcast(
                BroadcastMessage::ModeWithDataR(
                    MessageSide::Sending)));

        let packets = vec![
            Packet::service([0b00000000, 0b00000000]), // mode code command
            Packet::data([0b00000000, 0b00000000]), // data word
        ];

        let mut result;

        // parse command word
        result = message.parse(packets[0]);
        assert!(result.is_err());

        // parse data word
        result = message.parse(packets[1]);
        assert!(result.is_err());
    }

}