
/// Define a custom word
#[macro_export]
macro_rules! impl_basic_word {
    ( $name: ident ) => {

        pub struct $name {
            /// Data of the word
            data: [u8; 2],
        
            /// Parity of the word
            parity: u8,
        }
        
        impl mil_std_1553b::Word for $name {
            fn new() -> Self {
                Self {
                    data: [0, 0],
                    parity: 1,
                }
            }
        
            fn with_value(mut self, data: u16) -> Self {
                self.set_value(data);
                self
            }
        
            fn with_bytes(mut self, data: [u8; 2]) -> Self {
                self.set_bytes(data);
                self
            }
        
            fn with_parity(mut self, parity: u8) -> Self {
                self.set_parity(parity);
                self
            }
        
            fn with_calculated_parity(mut self) -> Self {
                self.parity = self.calculate_parity();
                self
            }
        
            fn build(self) -> mil_std_1553b::Result<Self> {
                if self.check_parity() {
                    Ok(self)
                } else {
                    Err(mil_std_1553b::Error::InvalidWord)
                }
            }
        
            fn from_value(data: u16) -> Self {
                Self::new().with_value(data).with_calculated_parity()
            }
        
            fn from_bytes(data: [u8; 2]) -> Self {
                Self::new().with_bytes(data)
            }
        
            fn as_bytes(&self) -> [u8; 2] {
                self.data
            }
        
            fn as_value(&self) -> u16 {
                self.into()
            }
        
            fn set_value(&mut self, data: u16) {
                self.data = data.to_be_bytes();
                self.parity = self.calculate_parity();
            }
        
            fn set_bytes(&mut self, data: [u8; 2]) {
                self.data = data;
                self.parity = self.calculate_parity();
            }
        
            fn parity(&self) -> u8 {
                self.parity
            }
        
            fn set_parity(&mut self, parity: u8) {
                self.parity = parity;
            }
        
            fn calculate_parity(&self) -> u8 {
                match self.as_value().count_ones() % 2 {
                    0 => 1,
                    _ => 0,
                }
            }
        
            fn check_parity(&self) -> bool {
                self.parity() == self.calculate_parity()
            }
        }

    };
}

/// Define a custom word
#[macro_export]
macro_rules! word {
    ( $name: ident ) => {
        mil_std_1553b::impl_basic_word!($name);

        impl From<&$name> for mil_std_1553b::DataWord {
            fn from(word: &$name) -> Self {
                use mil_std_1553b::Word;
                Self::new()
                    .with_bytes(word.as_bytes())
                    .with_parity(word.parity())
            }
        }

        impl From<$name> for mil_std_1553b::DataWord {
            fn from(word: $name) -> Self {
                Self::from(&word)
            }
        }

        impl From<&mil_std_1553b::DataWord> for $name {
            fn from(word: &mil_std_1553b::DataWord) -> Self {
                use mil_std_1553b::Word;
                Self::new()
                    .with_bytes(word.as_bytes())
                    .with_parity(word.parity())
            }
        }

        impl From<mil_std_1553b::DataWord> for $name {
            fn from(word: mil_std_1553b::DataWord) -> Self {
                Self::from(&word)
            }
        }

        impl From<&$name> for u16 {
            fn from(value: &$name) -> Self {
                u16::from_be_bytes(value.data)
            }
        }
        
        impl From<$name> for u16 {
            fn from(value: $name) -> Self {
                u16::from_be_bytes(value.data)
            }
        }
    };
}