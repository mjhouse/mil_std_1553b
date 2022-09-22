# mil_std_1553b
Rust implementation of 1553 message parsing

* http://www.horntech.cn/techDocuments/MIL-STD-1553Tutorial.pdf (MAIN)
* http://everyspec.com/MIL-HDBK/MIL-HDBK-1500-1799/MIL-HDBK-1553A_1779/ (page 430)
* https://www.ueidaq.com/mil-std-1553-tutorial-reference-guide
* https://github.com/yabozj/1553B-Simulator
* https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.406.9303&rep=rep1&type=pdf
* https://digitalcommons.usu.edu/cgi/viewcontent.cgi?article=2107&context=smallsat
* https://github.com/fpga-soc/mil-std-1553b-soc

## Questions

* How common are embedded systems with actual software in avionics vs LRU's with FPGAs?
* Are the sync bits and parity bits included in the binary data that must be processed? If so, how is a 20-bit chunk aligned inside a buffer measured in bytes?

## Notes

NA
