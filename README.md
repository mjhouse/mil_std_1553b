# MIL STD 1553B

[![tests passing](https://github.com/mjhouse/mil_std_1553b/actions/workflows/testing.yaml/badge.svg)](https://github.com/mjhouse/mil_std_1553b/actions)
[![docs passing](https://github.com/mjhouse/mil_std_1553b/actions/workflows/documentation.yaml/badge.svg)](https://mjhouse.github.io/mil_std_1553b/)
[![crates.io](https://img.shields.io/crates/v/mil_std_1553b.svg)](https://crates.io/crates/mil_std_1553b)

This library implements a complete set of Rust structs for parsing or constructing messages that comply with
the MIL STD 1553B communication protocal.

## Features

The following features make this library useable in constrained embedded systems for government, commercial, 
or military projects that can't have virally licensed dependencies.

* Does not use the standard library (`no_std`).
* Does not allocate dynamic memory.
* Has no dependencies.
* MIT licensed.

## Basic usage

### Creating a message

A message can be built using the constructor methods of Message, CommandWord, 
StatusWord, and DataWord. 

```rust
    use mil_std_1553b::*;

    let message: Message = Message::new()
        .with_command(CommandWord::new()
            .with_address(Address::Value(12))
            .with_subaddress(SubAddress::Value(5))
            .with_word_count(2)
            .build().unwrap()
        ).unwrap()
        .with_data(DataWord::new()).unwrap()
        .with_data(DataWord::new()).unwrap();

    assert_eq!(message.length(),3);
```

### Parsing a message

#### Command messages

Messages can be parsed as command messages, and the leading command word will determine
how many data words will be parsed from the buffer.

```rust
    use mil_std_1553b::*;

    let message: Message = Message::read_command(&[
        0b10000011, 
        0b00001100, 
        0b00100010, 
        0b11010000, 
        0b11010010
    ])
    .unwrap();

    assert_eq!(message.length(),2);
```

#### Status messages

```rust
    use mil_std_1553b::*;

    let message: Message = Message::read_status(&[
        0b10000011, 
        0b00001100, 
        0b01000010, 
        0b11010000, 
        0b11010010
    ])
    .unwrap();

    assert_eq!(message.length(), 2);
```

### Parsing a word

Words can be parsed from two-byte byte arrays or u16s. Data words can also be created 
from strings.

```rust
    use mil_std_1553b::*;

    let word = DataWord::new()
        .with_bytes([0b01001000, 0b01001001])
        .with_calculated_parity()
        .build()
        .unwrap();

    assert_eq!(word.as_string(),Ok("HI"));
```

## Roadmap

### 1.0.0

- [x] Words implemented
    - [x] Command, Status, and Data words created
    - [x] Words can be parsed from binary
    - [x] Words can be converted into binary
    - [x] Words have parsing tests
    - [x] Words have conversion tests
    - [x] Documentation exists for words
- [ ] Messages implemented
    - [x] Message struct is created
    - [x] Messages can be constructed from words
    - [x] Messages can be parsed from binary
    - [x] Messages have parsing tests
    - [ ] Messages have conversion tests
    - [x] Documentation exists for messages
- [ ] Integration tests implemented
    - [ ] Round-trip tests (binary -> struct -> binary) exist for messages
    - [ ] Round-trip tests (binary -> struct -> binary) exist for words
    - [ ] Configuration tests (JSON) exist for words
    - [ ] Configuration tests (JSON) exist for messages


### 2.0.0

- [ ] Message pattern constructors designed
- [ ] Directed pattern constructors implemented
    - [ ] BC - RT pattern implemented
    - [ ] BC - RT pattern tests implemented
    - [ ] RT - BC pattern implemented
    - [ ] RT - BC pattern tests implemented
    - [ ] RT - RT pattern implemented
    - [ ] RT - RT pattern tests implemented
    - [ ] Mode W/O Data (T) pattern implemented
    - [ ] Mode W/O Data (T) pattern tests implemented
    - [ ] Mode With Data (T) pattern implemented
    - [ ] Mode With Data (T) pattern tests implemented
    - [ ] Mode With Data (R) pattern implemented
    - [ ] Mode With Data (R) pattern tests implemented
- [ ] Broadcast pattern constructors implemented
    - [ ] BC - RT pattern implemented
    - [ ] BC - RT pattern tests implemented
    - [ ] RT - RT pattern implemented
    - [ ] RT - RT pattern tests implemented
    - [ ] Mode W/O Data pattern implemented
    - [ ] Mode W/O Data pattern tests implemented
    - [ ] Mode With Data pattern implemented
    - [ ] Mode With Data pattern tests implemented

## Notes

### Words

A "word" in the 1553B standard is made up of twenty bits, total. Three sync bits, 16 bits of data (in one of 
three different formats), and a trailing parity bit [^1]. This means that there are two ways of referencing a particular 
bit- either with a bit index offset from the beginning of the *word data* or as a "bit time" offset from the beginning 
of the word, including the sync bits.

| Index  | Sync1 | Sync2 | Sync3 |  0 |  1 |  2 |  3 |  4 |  5 |  6 |  7 |  8 |  9 | 10 | 11 | 12 | 13 | 14 | 15 | Parity |
|--------|---    |---    |---    |----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|---     |
| Time   | -     | -     | -     |  4 |  5 |  6 |  7 |  8 |  9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | -      |
| Offset | -     | -     | -     |  0 |  1 |  2 |  3 |  4 |  5 |  6 |  7 |  8 |  9 | 10 | 11 | 12 | 13 | 14 | 15 | -      |

The bit-time reference is used in the standard, but because we're only dealing with the 16-bit data from each word in this 
project we'll be using a zero-indexed reference in the actual code.

[^1]: <https://www.milstd1553.com/wp-content/uploads/2012/12/MIL-STD-1553B.pdf>