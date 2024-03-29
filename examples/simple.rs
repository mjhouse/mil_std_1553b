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

    let word0: &str = message.get(0).unwrap();
    let word1: &str = message.get(1).unwrap();
    let word2: &str = message.get(2).unwrap();

    println!("{}", word0);
    println!("{}", word1);
    println!("{}", word2);

    Ok(())
}
