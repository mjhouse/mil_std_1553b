# MIL STD 1553B

![tests passing](https://github.com/mjhouse/mil_std_1553b/actions/workflows/testing.yaml/badge.svg) [![docs passing](https://github.com/mjhouse/mil_std_1553b/actions/workflows/documentation.yaml/badge.svg)](https://mjhouse.github.io/mil_std_1553b/)

This library implements a complete set of Rust structs for parsing or constructing messages that comply with
the MIL STD 1553B communication protocal.

## Features

The following features make this library useable in constrained embedded systems for government, commercial, 
or military projects that can't have virally licensed dependencies.

* Does not use the standard library (`no_std`).
* Does not allocate dynamic memory.
* Has no dependencies.
* MIT licensed.

## Usage

### Creating a message

```rust
    use mil_std_1553b::*;

    let mut message = Message::new();

    message.add_command(CommandWord::new()
        .with_subaddress(12)
        .with_subaddress(5)
        .with_word_count(2)
    ).unwrap();

    message.add_data(DataWord::new()).unwrap();
    message.add_data(DataWord::new()).unwrap();

    assert!(message.is_full());
    assert_eq!(message.word_count(),3);
    assert_eq!(message.data_count(),2);
    assert_eq!(message.data_expected(),2);
```

## Words

A "word" in the 1553B standard is made up of twenty bits, total. Three sync bits, 16 bits of data (in one of 
three different formats), and a trailing parity bit. This means that there are two ways of referencing a particular 
bit- either with a bit index offset from the beginning of the *word data* or as a "bit time" offset from the beginning 
of the word, including the sync bits.

| Index  | Sync1 | Sync2 | Sync3 |  0 |  1 |  2 |  3 |  4 |  5 |  6 |  7 |  8 |  9 | 10 | 11 | 12 | 13 | 14 | 15 | Parity |
|--------|---    |---    |---    |----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|---     |
| Time   | -     | -     | -     |  4 |  5 |  6 |  7 |  8 |  9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | -      |
| Offset | -     | -     | -     |  0 |  1 |  2 |  3 |  4 |  5 |  6 |  7 |  8 |  9 | 10 | 11 | 12 | 13 | 14 | 15 | -      |

The bit-time reference is used in the standard, but because we're only dealing with the 16-bit data from each word in this 
project we'll be using a zero-indexed reference in the actual code.

## References

* [MIL-STD-1553 Tutorial](http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf)
* [MIL-HDBK-1553A_1779](http://everyspec.com/MIL-HDBK/MIL-HDBK-1500-1799/MIL-HDBK-1553A_1779/)
* [UEI Reference](https://www.ueidaq.com/mil-std-1553-tutorial-reference-guide)
* [1553B Simulator](https://github.com/yabozj/1553B-Simulator)
* [Designing Command and Telemetry Systems](https://digitalcommons.usu.edu/cgi/viewcontent.cgi?article=2107&context=smallsat)
* [MIL-STD-1553 Verilog](https://github.com/fpga-soc/mil-std-1553b-soc)
