use num_enum::{IntoPrimitive, FromPrimitive};
use super::{CommandWord, StatusWord, DataWord};

/// The parity bit following a word
/// 
/// This flag is derived from the 1-bit parity value
/// following each transmitted word.
#[derive(Debug,Clone,Copy,PartialEq,Eq,IntoPrimitive,FromPrimitive)]
#[repr(u8)]
pub enum Parity {
    
    /// The number of ones in the data word is odd
    #[default]
    Zero = 0b0,
    
    /// The number of ones in the data word is even
    One  = 0b1,
}

/// The sync waveform preceding a word
/// 
/// This flag is derived from the 3-bit sync waveform
/// preceding each transmitted word.
#[derive(Debug,Clone,Copy,PartialEq,Eq,IntoPrimitive,FromPrimitive)]
#[repr(u8)]
pub enum Sync {

    /// The sync waveform indicates data
    #[default]
    Data    = 0b001,

    /// The sync waveform indicates command or status
    NotData = 0b100,

}

/// Container enum for the different kinds of words
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Type {
    None,
    Command(CommandWord),
    Status(StatusWord),
    Data(DataWord),
}

impl Parity {

    /// Create the appropriate parity value from data
    /// 
    /// The parity flag, when added to the number of ones
    /// in the data, should make the total count odd. This 
    /// method returns `Parity::One` if the current count
    /// is even, otherwise `Parity::Zero`.
    #[must_use="Enum is created but never used"]
    pub fn from(value: &u16) -> Self {
        match value.count_ones() % 2 {
            0 => Self::One,
            _ => Self::Zero,
        }
    }

    /// Get the value of the enum as a u8
    #[must_use="Value is created but never used"]
    pub fn as_u8(&self) -> u8 {
        self.clone().into()
    }

    /// Get the value of the enum as a u32
    #[must_use="Value is created but never used"]
    pub fn as_u32(&self) -> u32 {
        self.as_u8() as u32
    }

    /// Check if the enum is the Zero variant
    #[must_use="Result of check is never used"]
    pub fn is_zero(&self) -> bool {
        match self {
            Self::Zero => true,
            _ => false
        }
    }

    /// Check if the enum is the One variant
    #[must_use="Result of check is never used"]
    pub fn is_one(&self) -> bool {
        match self {
            Self::One => true,
            _ => false
        }
    }

}

impl Sync {

    /// Get the value of the enum as a u8
    #[must_use="Value is created but never used"]
    pub fn value(&self) -> u8 {
        self.clone().into()
    }

    /// Check if the enum is the Data variant
    #[must_use="Result of check is never used"]
    pub const fn is_data(&self) -> bool {
        match self {
            Self::Data => true,
            _ => false
        }
    }

}

impl Type {

    /// Check if contained word is command
    #[must_use="Result of check is never used"]
    pub fn is_command(&self) -> bool {
        match self {
            Self::Command(_) => true,
            _ => false,
        }
    }

    /// Check if contained word is status
    #[must_use="Result of check is never used"]
    pub fn is_status(&self) -> bool {
        match self {
            Self::Status(_) => true,
            _ => false,
        }
    }

    /// Check if contained word is data
    #[must_use="Result of check is never used"]
    pub fn is_data(&self) -> bool {
        match self {
            Self::Data(_) => true,
            _ => false,
        }
    }

}