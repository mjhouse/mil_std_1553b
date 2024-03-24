extern crate mil_std_1553b;

use mil_std_1553b::{
    derive::{field, Word},
    Field, Message,
};

#[allow(dead_code)]
#[derive(Default, Word)]
struct CustomWord {
    #[data]
    mydata: [u8; 2],

    #[parity]
    myparity: u8,

    // you can have other fields on your word
    another: u8,
}

impl CustomWord {
    #[field(STATUS_LIGHT, 0b1000000000000000)]
    pub fn status_light_on(&self) -> bool {
        Self::STATUS_LIGHT.get(self) == 1
    }
}

fn main() {
    // Each word is 20 bits wide, containing three sync bits, 16 body bits, and one
    // parity bit. Because this doesn't map easily to bytes, the first word runs to
    // the middle of the third byte, the second to the end of the fifth, the third to
    // the middle of the eighth, and so on.
    //
    // WORD: 12300000000000000004
    //
    // In the below example, the first "status light on" flag is the last bit of the
    // third byte ('1'), and the second "status light on" flag is the fourth bit of
    // of the sixth byte ('0').
    let input = [
        0b10000011, 0b00001100, 0b01110011, 0b11010000, 0b11010011, 0b00101111, 0b00101101,
        0b11100010, 0b11001110, 0b11011110,
    ];

    // Parse the buffer into a Message struct that has room for
    // four words.
    let message = Message::<4>::read_command(&input).unwrap();

    // Get words by getting the appropriate data word and mapping
    // them to our custom word.
    let word1 = message.get::<CustomWord>(0);
    let word2 = message.get::<CustomWord>(1);

    // Access the 'status light on' flag of the custom word
    let status_light_on_1 = word1.unwrap().status_light_on();
    let status_light_on_2 = word2.unwrap().status_light_on();

    // Display the 'status light on' flag
    println!("status_light_on_1: {}", status_light_on_1);
    println!("status_light_on_2: {}", status_light_on_2);
}
