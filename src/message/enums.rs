/// a message can only contain 32 words
pub const MAX_WORDS: usize = 33;

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
/// Broadcast messages are transmitted to multiple remote terminals at the
/// same time. The terminals suppress the transmission of their status words
/// (not doing so would have multiple boxes trying to talk at the same time and
/// thereby “jam” the bus). In order for the bus controller to determine if a
/// terminal received the message, a polling sequence to each terminal must be
/// initiated to collect the status words.
///
/// See: http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf (p. 29-30)
#[derive(Clone)]
pub enum MessageDirection {
    BcToRt(MessageSide),
    RtToBc(MessageSide),
    RtToRt(MessageSide),
    ModeWithoutData(MessageSide),
    ModeWithDataT(MessageSide),
    ModeWithDataR(MessageSide),
}

/// MessageType is used to signal the type of message that should be parsed
/// next.
#[derive(Clone)]
pub enum MessageType {
    Directed(MessageDirection),
    Broadcast(MessageDirection),
}
