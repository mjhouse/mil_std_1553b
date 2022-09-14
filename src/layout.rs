//! Layouts for data word patterns. I'll provide some example layouts and a macro
//! that consumers of this library can use to make their own for different data 
//! word patterns.

macro_rules! define {
    ($name:ident => (size: $size:expr, sign: $sign:expr, msb: $msb:expr, lsb: $lsb:expr)) => {
        pub struct $name {
            data: [u16;$size],
            size: u8,
            sign: u8,
            msb: u8,
            lsb: u8,
        }

        
    }
}

pub enum Coding {
    Integer,
    Fractional,
}

/// defines a collection of data words in some particular format
pub struct Layout {
    pub name: &'static str,  // name- used for debugging
    pub units: &'static str,  // units- used for debugging
    pub coding: Coding,      // formats value as type
    pub msb: u8,             // almost always 0-1
    pub lsb: u8,             // variable length
}

define!(DoublePrecisionFloat => (size: 2, sign: 0, msb: 0, lsb: 26));