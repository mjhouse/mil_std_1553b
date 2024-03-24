extern crate mil_std_1553b;

use mil_std_1553b::{Address, CommandWord, DataWord, Message, Result, SubAddress, Word};

fn main() -> Result<()> {
    let message = Message::<4>::new()
        .with_command(
            CommandWord::new()
                .with_address(Address::Value(0b10101))
                .with_subaddress(SubAddress::Value(0b00101))
                .with_word_count(3)
                .build()?,
        )
        .with_data(DataWord::try_from("TE")?)
        .with_data(DataWord::try_from("ST")?)
        .with_data(DataWord::try_from("Y ")?)
        .build()?;

    let word0 = message.at(0).unwrap();
    let word1 = message.at(1).unwrap();
    let word2 = message.at(2).unwrap();

    println!("{}", word0.as_string().unwrap());
    println!("{}", word1.as_string().unwrap());
    println!("{}", word2.as_string().unwrap());

    Ok(())
}
