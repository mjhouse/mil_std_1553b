// #![no_std]

mod errors;

use errors::{Error,Result};

pub struct Word(u16);

impl Word {

    pub fn full() -> Self {
        Self(0b1111111111111111)
    }

    pub fn empty() -> Self {
        Self(0b0000000000000000)
    }

    pub fn new(value: u16) -> Self {
        Self(value)
    }

    /// Gets the value of a particular bit in the word
    ///
    /// # Arguments
    ///
    /// * `index` - Left-to-left index position
    ///
    /// # Examples
    ///
    /// ```
    /// use mil_std_1553b::Word;
    /// 
    /// let word = Word::full();
    /// let value = word.get(0).unwrap();
    /// 
    /// assert!(value);
    /// ```
    pub fn get(&self, index: usize) -> Result<bool> {
        if index < 16 { 
            Ok((self.0 & ((1 << 15) >> index)) != 0)
        }
        else {
            Err(Error::OutOfBounds)
        }
    }

    /// Sets the value of a particular bit in the word
    ///
    /// # Arguments
    ///
    /// * `index` - Left-to-left index position
    /// * `state` - Boolean state to set (true = 1, false = 0)
    ///
    /// # Examples
    ///
    /// ```
    /// use mil_std_1553b::Word;
    /// 
    /// let mut word = Word::full();
    /// word.set(0,false).unwrap();
    /// 
    /// let value = word.get(0).unwrap();
    /// 
    /// assert!(!value);
    /// ```
    pub fn set(&mut self, index: usize, state: bool) -> Result<bool> {
        if index < 16 {
            if state {
                self.0 |= (1 << 15) >> index;
            }
            else {
                self.0 &= !((1 << 15) >> index);
            }
            Ok(state)
        }
        else {
            Err(Error::OutOfBounds)
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // fn test_set_bit_index() {
    //     let mut word = Word::full();
    //     println!("bits: {:#018b}", word.0);
        
    //     word.set(0,false).unwrap();
    //     println!("bits: {:#018b}", word.0);

    //     word.set(1,false).unwrap();
    //     println!("bits: {:#018b}", word.0);

    //     word.set(2,false).unwrap();
    //     println!("bits: {:#018b}", word.0);

    //     word.set(3,false).unwrap();
    //     println!("bits: {:#018b}", word.0);

    //     word.set(13,false).unwrap();
    //     println!("bits: {:#018b}", word.0);

    //     word.set(14,false).unwrap();
    //     println!("bits: {:#018b}", word.0);

    //     word.set(15,false).unwrap();
    //     println!("bits: {:#018b}", word.0);
    // }

    #[test]
    fn test_set_bit_index_positive() {
        let mut word = Word::empty();
        word.set(0,true).unwrap();
        word.set(9,true).unwrap();
        word.set(15,true).unwrap();
        let err = word.set(21,true).is_err();

        let b0 = word.get(0).unwrap();
        let b9 = word.get(9).unwrap();
        let b15 = word.get(15).unwrap();
        let b21 = word.get(21).is_err();

        assert!(b0);
        assert!(b9);
        assert!(b15);
        assert!(b21);
        assert!(err);
    }

    #[test]
    fn test_set_bit_index_negative() {
        let mut word = Word::full();
        word.set(0,false).unwrap();
        word.set(9,false).unwrap();
        word.set(15,false).unwrap();
        let err = word.set(21,false).is_err();

        let b0 = word.get(0).unwrap();
        let b9 = word.get(9).unwrap();
        let b15 = word.get(15).unwrap();
        let b21 = word.get(21).is_err();

        assert!(!b0);
        assert!(!b9);
        assert!(!b15);
        assert!(b21);
        assert!(err);
    }
}
