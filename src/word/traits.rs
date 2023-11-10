use super::Parity;

/// Generic methods for all words
pub trait Word {

    /// Get the body of the word
    fn data(&self) -> u16;

    /// Get the parity bit
    fn parity(&self) -> Parity;

    /// Check the parity of the word
    /// 
    /// Words on a 1553 bus use odd parity for
    /// simple error checking.
    fn is_valid(&self) -> bool {
        let check = self.parity().as_u32();
        let data = self.data().count_ones();

        // true if v is odd
        ((data + check) % 2) != 0
    }

}

macro_rules! impl_word {
    ( $t:ident ) => {
        impl Word for $t {
            fn data(&self) -> u16 {
                self.data
            }
        
            fn parity(&self) -> Parity {
                self.parity
            }
        }
    };
}