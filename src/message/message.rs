
use crate::fields::*;
use crate::flags::*;
use crate::errors::*;
use crate::word::*;
use super::enums::*;

use crate::word::Type as Word;

pub struct Message {
    side: MessageType,
    limit: u8,
    count: u8,
    words: [Word;MAX_WORDS as usize],
}

impl Message {
    
    pub fn new(side: MessageType) -> Self {
        Self {
            side,
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
            return Err(Error::MessageFull);
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
            match c.word_count() {
                Some(v) => self.data_count() < v,
                None => false,
            }
        }
        else {
            false
        }
    }

    // /// parses a single word and adds it to the message,
    // /// returning either an error or the new length of the parsed
    // /// message.
    // pub fn parse(&mut self, packet: Packet) -> Result<u8> {

    //     if !packet.is_valid() {
    //         return Err(Error::ReservedUsed);
    //     }

    //     use MessageType::*;
    //     use DirectedMessage as D;
    //     use BroadcastMessage as B;

    //     match self.side.clone() {
    //         Directed(D::BcToRt(side)) => 
    //             self.parse_bc_to_rt_directed(side,packet),
    //         Directed(D::RtToBc(side)) => 
    //             self.parse_rt_to_bc_directed(side,packet),
    //         Directed(D::RtToRt(side)) => 
    //             self.parse_rt_to_rt_directed(side,packet),
    //         Directed(D::ModeWithoutData(side)) => 
    //             self.parse_mode_without_data_directed(side,packet),
    //         Directed(D::ModeWithDataT(side)) => 
    //             self.parse_mode_with_data_t_directed(side,packet),
    //         Directed(D::ModeWithDataR(side)) => 
    //             self.parse_mode_with_data_r_directed(side,packet),
    //         Broadcast(B::BcToRt(side)) => 
    //             self.parse_bc_to_rt_broadcast(side,packet),
    //         Broadcast(B::RtToRt(side)) => 
    //             self.parse_rt_to_rt_broadcast(side,packet),
    //         Broadcast(B::ModeWithoutData(side)) => 
    //             self.parse_mode_without_data_broadcast(side,packet),
    //         Broadcast(B::ModeWithDataR(side)) => 
    //             self.parse_mode_with_data_r_broadcast(side,packet),
    //         _ => Err(Error::UnknownMessage),
    //     }

    // }

    // /// Parse BC -> RT Directed
    // ///
    // /// If this message is being parsed on the "receiving" side, then
    // /// we are parsing the SEND message (and then responding with the
    // /// RESP message). If it's being parsed on the "sending" side, then 
    // /// we're going to be parsing the RESP only.
    // ///
    // /// Pattern:
    // ///     SEND: RECV COMM | DATA | DATA | ...
    // ///     RESP: STAT
    // ///
    // fn parse_bc_to_rt_directed(&mut self, side: MessageSide, packet: Packet) -> Result<u8> {
    //     match (side,self.first()) {
    //         (MessageSide::Receiving,Some(Word::Command(c))) if self.has_space() =>
    //             self.add(Word::data(packet.content)),
    //         (MessageSide::Receiving,None) => 
    //             self.add(Word::receive_command(packet.content)?),
    //         (MessageSide::Sending,None) => 
    //             self.add(Word::status(packet.content)),
    //         (MessageSide::Receiving,_) => Err(Error::MessageBad), /* TODO: error because first word should be command */
    //         (MessageSide::Sending,_) => Err(Error::MessageBad), /* TODO: error because message should be empty */
    //     }
    // }

    // /// Parse RT -> BC Directed
    // ///
    // /// If this message is being parsed on the "receiving" side, then
    // /// we are parsing the SEND message (and then responding with the
    // /// RESP message). If it's being parsed on the "sending" side, then 
    // /// we're going to be parsing the RESP only.
    // ///
    // /// Pattern:
    // ///     SEND: TRAN COMM 
    // ///     RESP: STAT | DATA | DATA | ...
    // ///
    // fn parse_rt_to_bc_directed(&mut self, side: MessageSide, packet: Packet) -> Result<u8> {
    //     match (side,self.first()) {
    //         (MessageSide::Receiving,None) =>
    //             self.add(Word::transmit_command(packet.content)?),
    //         (MessageSide::Sending,Some(Word::Status(_))) => 
    //             self.add(Word::data(packet.content)),
    //         (MessageSide::Sending,None) => 
    //             self.add(Word::status(packet.content)),
    //         (MessageSide::Receiving,_) => Err(Error::MessageBad), /* TODO: error because there should be only one word */
    //         (MessageSide::Sending,_) => Err(Error::MessageBad), /* TODO: error because word should be status and isn't */
    //     }
    // }

    // /// Parse RT -> RT Directed
    // ///
    // /// If this message is being parsed on the "receiving" side, then
    // /// we are parsing the SEND message (and then responding with the
    // /// RESP message). If it's being parsed on the "sending" side, then 
    // /// we're going to be parsing the RESP only.
    // ///
    // /// Pattern:
    // ///     SEND: RECV COMM | TRAN COMM |
    // ///     RESP: STAT | DATA | DATA | ...
    // ///
    // fn parse_rt_to_rt_directed(&mut self, side: MessageSide, packet: Packet) -> Result<u8> {
    //     match (side,self.first()) {
    //         (MessageSide::Receiving,None) =>
    //             self.add(Word::receive_command(packet.content)?),
    //         (MessageSide::Receiving,Some(Word::Command(_))) =>
    //             self.add(Word::transmit_command(packet.content)?),
    //         (MessageSide::Sending,None) => 
    //             self.add(Word::status(packet.content)),
    //         (MessageSide::Sending,Some(Word::Status(_))) => 
    //             self.add(Word::data(packet.content)),
    //         (MessageSide::Receiving,_) => Err(Error::MessageBad), /* TODO: error because words should only be commands */
    //         (MessageSide::Sending,_) => Err(Error::MessageBad), /* TODO: error because word should be status and isn't */
    //     }
    // }

    // /// Parse Without Data Directed
    // ///
    // /// If this message is being parsed on the "receiving" side, then
    // /// we are parsing the SEND message (and then responding with the
    // /// RESP message). If it's being parsed on the "sending" side, then 
    // /// we're going to be parsing the RESP only.
    // ///
    // /// Pattern:
    // ///     SEND: MOD COMM
    // ///     RESP: STAT
    // ///
    // fn parse_mode_without_data_directed(&mut self, side: MessageSide, packet: Packet) -> Result<u8> {
    //     match (side,self.first()) {
    //         (MessageSide::Receiving,None) =>
    //             self.add(Word::mode_code_command(packet.content)?),
    //         (MessageSide::Sending,None) => 
    //             self.add(Word::status(packet.content)),
    //         (MessageSide::Receiving,_) => Err(Error::MessageBad), /* TODO: error because word should only be command */
    //         (MessageSide::Sending,_) => Err(Error::MessageBad), /* TODO: error because word should only be status */
    //     }
    // }

    // /// Parse With Data Directed
    // ///
    // /// If this message is being parsed on the "receiving" side, then
    // /// we are parsing the SEND message (and then responding with the
    // /// RESP message). If it's being parsed on the "sending" side, then 
    // /// we're going to be parsing the RESP only.
    // ///
    // /// Pattern:
    // ///     SEND: MOD COMM
    // ///     RESP: STAT | DATA |
    // ///
    // fn parse_mode_with_data_t_directed(&mut self, side: MessageSide, packet: Packet) -> Result<u8> {
    //     match (side,self.first()) {
    //         (MessageSide::Receiving,None) =>
    //             self.add(Word::mode_code_command(packet.content)?),
    //         (MessageSide::Sending,None) => 
    //             self.add(Word::status(packet.content)),
    //         (MessageSide::Sending,Some(Word::Status(_))) if !self.has_data() => 
    //             self.add(Word::data(packet.content)),
    //         (MessageSide::Receiving,_) => Err(Error::MessageBad), /* TODO: error because word should only be command */
    //         (MessageSide::Sending,_) => Err(Error::MessageBad), /* TODO: error because word should only be status/data */
    //     }
    // }

    // /// Parse With Data R Directed
    // ///
    // /// If this message is being parsed on the "receiving" side, then
    // /// we are parsing the SEND message (and then responding with the
    // /// RESP message). If it's being parsed on the "sending" side, then 
    // /// we're going to be parsing the RESP only.
    // ///
    // /// Pattern:
    // ///     SEND: MOD COMM | DATA |
    // ///     RESP: STAT
    // ///
    // fn parse_mode_with_data_r_directed(&mut self, side: MessageSide, packet: Packet) -> Result<u8> {
    //     match (side,self.first()) {
    //         (MessageSide::Receiving,None) =>
    //             self.add(Word::mode_code_command(packet.content)?),
    //         (MessageSide::Receiving,Some(Word::Command(_))) if !self.has_data() => 
    //             self.add(Word::data(packet.content)),
    //         (MessageSide::Sending,None) => 
    //             self.add(Word::status(packet.content)),
    //         (MessageSide::Receiving,_) => Err(Error::MessageBad), /* TODO: error because word should only be command/data */
    //         (MessageSide::Sending,_) => Err(Error::MessageBad), /* TODO: error because word should only be status */
    //     }
    // }

    // /// Parse BC to RT Broadcast
    // ///
    // /// If this message is being parsed on the "receiving" side, then
    // /// we are parsing the SEND message (and then responding with the
    // /// RESP message). If it's being parsed on the "sending" side, then 
    // /// we're going to be parsing the RESP only.
    // ///
    // /// Pattern:
    // ///     SEND: RECV COMM | DATA | DATA | ...
    // ///
    // fn parse_bc_to_rt_broadcast(&mut self, side: MessageSide, packet: Packet) -> Result<u8> {
    //     match (side,self.first()) {
    //         (MessageSide::Receiving,None) => 
    //             self.add(Word::receive_command(packet.content)?),
    //         (MessageSide::Receiving,Some(Word::Command(c))) if self.has_space() =>
    //             self.add(Word::data(packet.content)),
    //         (MessageSide::Receiving,_) => Err(Error::MessageBad), /* TODO: error because word should only be command/data */
    //         (MessageSide::Sending,_) => Err(Error::MessageBad), /* TODO: error because sending side should never parse */
    //     }
    // }

    // /// Parse RT to RT Broadcast
    // ///
    // /// If this message is being parsed on the "receiving" side, then
    // /// we are parsing the SEND message (and then responding with the
    // /// RESP message). If it's being parsed on the "sending" side, then 
    // /// we're going to be parsing the RESP only.
    // ///
    // /// Pattern:
    // ///     SEND: RECV COMM | TRAN COMM |
    // ///     RESP: STAT | DATA | DATA | ...
    // ///
    // fn parse_rt_to_rt_broadcast(&mut self, side: MessageSide, packet: Packet) -> Result<u8> {
    //     match (side,self.first()) {
    //         (MessageSide::Receiving,None) =>
    //             self.add(Word::receive_command(packet.content)?),
    //         (MessageSide::Receiving,Some(Word::Command(_))) => 
    //             self.add(Word::transmit_command(packet.content)?),
    //         (MessageSide::Sending,None) => 
    //             self.add(Word::status(packet.content)),
    //         (MessageSide::Sending,Some(Word::Status(_))) => 
    //             self.add(Word::data(packet.content)),
    //         (MessageSide::Receiving,_) => Err(Error::MessageBad), /* TODO: error because word should only be command/data */
    //         (MessageSide::Sending,_) => Err(Error::MessageBad), /* TODO: error because word should only be status/data */
    //     }
    // }

    // /// Parse Mode Without Data Broadcast
    // ///
    // /// If this message is being parsed on the "receiving" side, then
    // /// we are parsing the SEND message (and then responding with the
    // /// RESP message). If it's being parsed on the "sending" side, then 
    // /// we're going to be parsing the RESP only.
    // ///
    // /// Pattern:
    // ///     SEND: MOD COMM
    // ///
    // fn parse_mode_without_data_broadcast(&mut self, side: MessageSide, packet: Packet) -> Result<u8> {
    //     match (side,self.first()) {
    //         (MessageSide::Receiving,None) => 
    //             self.add(Word::mode_code_command(packet.content)?),
    //         (MessageSide::Receiving,_) => Err(Error::MessageBad), /* TODO: error because word should only be command */
    //         (MessageSide::Sending,_) => Err(Error::MessageBad), /* TODO: error because sending side should never parse */
    //     }
    // }

    // /// Parse Mode With Data Broadcast
    // ///
    // /// If this message is being parsed on the "receiving" side, then
    // /// we are parsing the SEND message (and then responding with the
    // /// RESP message). If it's being parsed on the "sending" side, then 
    // /// we're going to be parsing the RESP only.
    // ///
    // /// Pattern:
    // ///     SEND: MOD COMM | DATA |
    // ///
    // fn parse_mode_with_data_r_broadcast(&mut self, side: MessageSide, packet: Packet) -> Result<u8> {
    //     match (side,self.first()) {
    //         (MessageSide::Receiving,None) => 
    //             self.add(Word::mode_code_command(packet.content)?),
    //         (MessageSide::Receiving,Some(Word::Command(_))) if !self.has_data() =>
    //             self.add(Word::data(packet.content)),
    //         (MessageSide::Receiving,_) => Err(Error::MessageBad), /* TODO: error because word should only be command or data */
    //         (MessageSide::Sending,_) => Err(Error::MessageBad), /* TODO: error because sending side should never parse */
    //     }
    // }

}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_parse_bc_to_rt_directed_receiving() {
//         let mut message = Message::new(
//             MessageType::Directed(
//                 DirectedMessage::BcToRt(
//                     MessageSide::Receiving)));

//         // receive command with word count of 2 
//         let packets = &[
//             Packet::service([0b00000000, 0b00000010]),
//             Packet::data([0b00000000,0b00000000]),
//             Packet::data([0b00000000,0b00000000]),
//         ];

//         let mut result;

//         // parse the command
//         result = message.parse(packets[0]);
//         assert_eq!(result,Ok(1));

//         // parse first data word
//         result = message.parse(packets[1]);
//         assert_eq!(result,Ok(2));

//         // parse second data word
//         result = message.parse(packets[2]);
//         assert_eq!(result,Ok(3));

//         // parse too many data words
//         result = message.parse(packets[2]);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_parse_bc_to_rt_directed_sending() {
//         let mut message = Message::new(
//             MessageType::Directed(
//                 DirectedMessage::BcToRt(
//                     MessageSide::Sending)));

//         // receive command with word count of 2 
//         let packets = &[
//             Packet::service([0b00000000, 0b00000000]),
//             Packet::data([0b00000000,0b00000000])
//         ];

//         let mut result;

//         // parse the command
//         result = message.parse(packets[0]);
//         assert_eq!(result,Ok(1));

//         // parse unexpected data word
//         result = message.parse(packets[1]);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_parse_rt_to_bc_directed_receiving() {
//         let mut message = Message::new(
//             MessageType::Directed(
//                 DirectedMessage::RtToBc(
//                     MessageSide::Receiving)));

//         // receive command with word count of 2 
//         let packets = &[
//             Packet::service([0b00000100, 0b00000010]),
//             Packet::data([0b00000000,0b00000000])
//         ];

//         let mut result;

//         // parse the command
//         result = message.parse(packets[0]);
//         assert_eq!(result,Ok(1));

//         // parse unexpected data word
//         result = message.parse(packets[1]);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_parse_rt_to_rt_directed_sending() {
//         let mut message = Message::new(
//             MessageType::Directed(
//                 DirectedMessage::RtToRt(
//                     MessageSide::Sending)));

//         // receive command with word count of 2 
//         let packets = &[
//             Packet::service([0b00000000, 0b00000000]),
//             Packet::data([0b00000000,0b00000000]),
//             Packet::data([0b00000000,0b00000000]),
//         ];

//         let mut result;

//         // parse the command
//         result = message.parse(packets[0]);
//         assert_eq!(result,Ok(1));

//         // parse first data word
//         result = message.parse(packets[1]);
//         assert_eq!(result,Ok(2));

//         // parse second data word
//         result = message.parse(packets[2]);
//         assert_eq!(result,Ok(3));
//     }

//     #[test]
//     fn test_parse_rt_to_rt_directed_receiving() {
//         let mut message = Message::new(
//             MessageType::Directed(
//                 DirectedMessage::RtToRt(
//                     MessageSide::Receiving)));

//         // receive command with word count of 2 
//         let packets = &[
//             Packet::service([0b00000000, 0b00000010]),
//             Packet::service([0b00000100, 0b00000010]),
//         ];

//         let mut result;

//         // parse the receive command
//         result = message.parse(packets[0]);
//         assert_eq!(result,Ok(1));

//         // parse the transmit command
//         result = message.parse(packets[1]);
//         assert_eq!(result,Ok(2));
//     }

//     #[test]
//     fn test_parse_mode_without_data_directed_receiving() {
//         let mut message = Message::new(
//             MessageType::Directed(
//                 DirectedMessage::ModeWithoutData(
//                     MessageSide::Receiving)));

//         let packets = &[
//             Packet::service([0b00000000, 0b00011111]), // mode code command
//             Packet::data([0b00000000, 0b00000000]), // data word
//         ];

//         let mut result;

//         // parse the receive command
//         result = message.parse(packets[0]);
//         assert_eq!(result,Ok(1));

//         // parse too many words
//         result = message.parse(packets[1]);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_parse_mode_without_data_directed_sending() {
//         let mut message = Message::new(
//             MessageType::Directed(
//                 DirectedMessage::ModeWithoutData(
//                     MessageSide::Sending)));

//         let packets = &[
//             Packet::service([0b00000000, 0b00000000]), // status word
//             Packet::data([0b00000000, 0b00000000]), // data word
//         ];

//         let mut result;

//         // parse the status word
//         result = message.parse(packets[0]);
//         assert_eq!(result,Ok(1));

//         // parse too many words
//         result = message.parse(packets[1]);
//         assert!(result.is_err());
//     }
    
//     #[test]
//     fn test_parse_mode_with_data_t_directed_receiving() {
//         let mut message = Message::new(
//             MessageType::Directed(
//                 DirectedMessage::ModeWithDataT(
//                     MessageSide::Receiving)));

//         let packets = &[
//             Packet::service([0b00000000, 0b00000000]), // mode code word
//             Packet::data([0b00000000, 0b00000000]), // data word
//         ];

//         let mut result;

//         // parse the command word
//         result = message.parse(packets[0]);
//         assert_eq!(result,Ok(1));

//         // parse too many words
//         result = message.parse(packets[1]);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_parse_mode_with_data_t_directed_sending() {
//         let mut message = Message::new(
//             MessageType::Directed(
//                 DirectedMessage::ModeWithDataT(
//                     MessageSide::Sending)));

//         let packets = &[
//             Packet::service([0b00000000, 0b00000000]), // status word
//             Packet::data([0b00000000, 0b00000000]), // data word
//             Packet::data([0b00000000, 0b00000000]), // data word
//         ];

//         let mut result;

//         // parse the command word
//         result = message.parse(packets[0]);
//         assert_eq!(result,Ok(1));

//         // parse one data word
//         result = message.parse(packets[1]);
//         assert_eq!(result,Ok(2));

//         // parse too many data words
//         result = message.parse(packets[2]);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_parse_mode_with_data_r_directed_receiving() {
//         let mut message = Message::new(
//             MessageType::Directed(
//                 DirectedMessage::ModeWithDataR(
//                     MessageSide::Receiving)));

//         let packets = &[
//             Packet::service([0b00000000, 0b00000000]), // service word
//             Packet::data([0b00000000, 0b00000000]), // data word
//             Packet::data([0b00000000, 0b00000000]), // data word
//         ];

//         let mut result;

//         // parse the command word
//         result = message.parse(packets[0]);
//         assert_eq!(result,Ok(1));

//         // parse one data word
//         result = message.parse(packets[1]);
//         assert_eq!(result,Ok(2));

//         // parse too many data words
//         result = message.parse(packets[2]);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_parse_mode_with_data_r_directed_sending() {
//         let mut message = Message::new(
//             MessageType::Directed(
//                 DirectedMessage::ModeWithDataR(
//                     MessageSide::Sending)));

//         let packets = &[
//             Packet::service([0b00000000, 0b00000000]), // service word
//             Packet::data([0b00000000, 0b00000000]), // data word
//         ];

//         let mut result;

//         // parse the command word
//         result = message.parse(packets[0]);
//         assert_eq!(result,Ok(1));

//         // parse too many data words
//         result = message.parse(packets[1]);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_parse_bc_to_rt_broadcast_receiving() {
//         let mut message = Message::new(
//             MessageType::Broadcast(
//                 BroadcastMessage::BcToRt(
//                     MessageSide::Receiving)));

//         let packets = &[
//             Packet::service([0b00000000, 0b00000000]), // service word
//             Packet::data([0b00000000, 0b00000000]),    // data word
//             Packet::data([0b00000000, 0b00000000]),    // data word
//         ];

//         let mut result;

//         // parse the command word
//         result = message.parse(packets[0]);
//         assert_eq!(result,Ok(1));

//         // parse first data word
//         result = message.parse(packets[1]);
//         assert_eq!(result,Ok(2));

//         // parse second data word
//         result = message.parse(packets[2]);
//         assert_eq!(result,Ok(3));
//     }

//     #[test]
//     fn test_parse_bc_to_rt_broadcast_sending() {
//         let mut message = Message::new(
//             MessageType::Broadcast(
//                 BroadcastMessage::BcToRt(
//                     MessageSide::Sending)));

//         let packets = &[
//             Packet::service([0b00000000, 0b00000000]), // service word
//         ];

//         // parse the command word
//         let result = message.parse(packets[0]);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_parse_rt_to_rt_broadcast_receiving() {
//         let mut message = Message::new(
//             MessageType::Broadcast(
//                 BroadcastMessage::RtToRt(
//                     MessageSide::Receiving)));

//         let packets = &[
//             Packet::service([0b00000000, 0b00000000]), // receive command
//             Packet::service([0b00000100, 0b00000000]), // transmit command
//             Packet::data([0b00000000, 0b00000000]),    // data word
//         ];

//         let mut result;

//         // parse first command word
//         result = message.parse(packets[0]);
//         assert_eq!(result,Ok(1));

//         // parse second command word
//         result = message.parse(packets[1]);
//         assert_eq!(result,Ok(2));

//         // parse data word
//         result = message.parse(packets[2]);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_parse_rt_to_rt_broadcast_sending() {
//         let mut message = Message::new(
//             MessageType::Broadcast(
//                 BroadcastMessage::RtToRt(
//                     MessageSide::Sending)));

//         let packets = &[
//             Packet::service([0b00000000, 0b00000000]), // status word
//             Packet::data([0b00000000, 0b00000000]),    // data word
//             Packet::data([0b00000000, 0b00000000]),    // data word
//         ];

//         let mut result;

//         // parse status word
//         result = message.parse(packets[0]);
//         assert_eq!(result,Ok(1));

//         // parse data word
//         result = message.parse(packets[1]);
//         assert_eq!(result,Ok(2));

//         // parse data word
//         result = message.parse(packets[2]);
//         assert_eq!(result,Ok(3));
//     }

//     #[test]
//     fn test_parse_mode_without_data_broadcast_receiving() {
//         let mut message = Message::new(
//             MessageType::Broadcast(
//                 BroadcastMessage::ModeWithoutData(
//                     MessageSide::Receiving)));

//         let packets = &[
//             Packet::service([0b00000000, 0b00000000]), // mode code command
//             Packet::data([0b00000000, 0b00000000]), // data word
//         ];

//         let mut result;

//         // parse command word
//         result = message.parse(packets[0]);
//         assert_eq!(result,Ok(1));

//         // parse data word
//         result = message.parse(packets[1]);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_parse_mode_without_data_broadcast_sending() {
//         let mut message = Message::new(
//             MessageType::Broadcast(
//                 BroadcastMessage::ModeWithoutData(
//                     MessageSide::Sending)));

//         let packets = &[
//             Packet::service([0b00000000, 0b00000000]), // status word
//             Packet::data([0b00000000, 0b00000000]), // data word
//         ];

//         let mut result;

//         // parse status word
//         result = message.parse(packets[0]);
//         assert!(result.is_err());

//         // parse data word
//         result = message.parse(packets[1]);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_parse_mode_with_data_r_broadcast_receiving() {
//         let mut message = Message::new(
//             MessageType::Broadcast(
//                 BroadcastMessage::ModeWithDataR(
//                     MessageSide::Receiving)));

//         let packets = &[
//             Packet::service([0b00000000, 0b00000000]), // mode code command
//             Packet::data([0b00000000, 0b00000000]), // data word
//         ];

//         let mut result;

//         // parse command word
//         result = message.parse(packets[0]);
//         assert_eq!(result,Ok(1));

//         // parse data word
//         result = message.parse(packets[1]);
//         assert_eq!(result,Ok(2));
//     }

//     #[test]
//     fn test_parse_mode_with_data_r_broadcast_sending() {
//         let mut message = Message::new(
//             MessageType::Broadcast(
//                 BroadcastMessage::ModeWithDataR(
//                     MessageSide::Sending)));

//         let packets = &[
//             Packet::service([0b00000000, 0b00000000]), // mode code command
//             Packet::data([0b00000000, 0b00000000]), // data word
//         ];

//         let mut result;

//         // parse command word
//         result = message.parse(packets[0]);
//         assert!(result.is_err());

//         // parse data word
//         result = message.parse(packets[1]);
//         assert!(result.is_err());
//     }

// }