# MIL STD 1553B

Rust implementation of 1553 message parsing 

## Words

A "word" in the 1553B standard is made up of twenty bits, total. Three sync bits,
16 bits of data (in one of three different formats), and a trailing parity
bit. This means that there are two ways of referencing a particular bit- either with
a bit index offset from the beginning of the *word data* or as a "bit time" offset
from the begining of the word, including the sync bits.

| Index  | Sync1 | Sync2 | Sync3 |  0 |  1 |  2 |  3 |  4 |  5 |  6 |  7 |  8 |  9 | 10 | 11 | 12 | 13 | 14 | 15 | Parity |
|--------|---    |---    |---    |----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|---     |
| Time   | -     | -     | -     |  4 |  5 |  6 |  7 |  8 |  9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | -      |
| Offset | -     | -     | -     | 15 | 14 | 13 | 12 | 11 | 10 |  9 |  8 |  7 |  6 |  5 |  4 |  3 |  2 |  1 |  0 | -      |

The bit-time reference is used in the standard, but because we're only dealing with
the 16-bit data from each word in this project we'll be using a zero-indexed reference
in the actual code.

## References

* [MIL-STD-1553 Tutorial](http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf)
* [MIL-HDBK-1553A_1779](http://everyspec.com/MIL-HDBK/MIL-HDBK-1500-1799/MIL-HDBK-1553A_1779/)
* [UEI Reference](https://www.ueidaq.com/mil-std-1553-tutorial-reference-guide)
* [1553B Simulator](https://github.com/yabozj/1553B-Simulator)
* [Designing Command and Telemetry Systems](https://digitalcommons.usu.edu/cgi/viewcontent.cgi?article=2107&context=smallsat)
* [MIL-STD-1553 Verilog](https://github.com/fpga-soc/mil-std-1553b-soc)
