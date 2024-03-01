/// a message can only contain 32 words
pub const MAX_WORDS: usize = 33;

/// The type of the message
///
/// Directed means that the message is between one
/// terminal and another, or between a terminal and
/// the bus controller. These messages are immediately
/// followed by a status word sent from the receiving
/// terminal to the bus controller to validate receipt.
///
/// Broadcast indicates that the message is transmitted
/// to multiple remote terminals at the same time. To
/// validate receipt, the bus controller must poll the
/// terminals for their status words.
///
/// Directed and broadcast message formats are described 
/// in chapter 4 of the MIL-STD-1553 Tutorial[^1].
///
/// [^1]: [MIL-STD-1553 Tutorial](http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf)
#[derive(Clone)]
pub enum MessageType {
    /// The message is intended for a single receiver
    Directed(MessageDirection),

    /// The message is intended for multiple terminals
    Broadcast(MessageDirection),
}

/// The direction and target of a message
///
/// Defines whether the message is directed from one remote
/// terminal (RT) to another, to the bus controller, to
/// all terminals on the bus, etc. These various message 
/// forms are described in chapter 4 of the MIL-STD-1553 
/// Tutorial[^1].
///
/// [^1]: [MIL-STD-1553 Tutorial](http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf)
#[derive(Clone)]
pub enum MessageDirection {
    /// Message from the bus controller to a remote terminal
    BcToRt(MessageSide),
    /// Message from the remote terminal to the bus controller
    RtToBc(MessageSide),
    /// Message from one remote terminal to another
    RtToRt(MessageSide),
    /// Mode code message without data
    ModeWithoutData(MessageSide),
    /// Mode code message expected data in response
    ModeWithDataT(MessageSide),
    /// Mode code message including data
    ModeWithDataR(MessageSide),
}

/// The role of the terminal parsing the message
///
/// The side on which the message is being parsed can
/// determine what the message words should be parsed
/// as.
#[derive(Clone)]
pub enum MessageSide {

    /// Message is parsed or constructed by the sender
    Sending,

    /// Message is parsed or constructed by the receiver
    Receiving,
}
