use mil_std_1553b::*;

#[test]
fn test_word2() {
    let word = StatusWord::from(0b0000000000000000);
    assert!( word.address() == 0u8.into(), "address != 0");
    assert!( word.instrumentation() == 0u8.into(), "instrumentation != 0");
    assert!( word.service_request() == 0u8.into(), "service_request != 0");
    assert!( word.reserved() == 0u8.into(), "reserved != 0");
    assert!( word.broadcast_received() == 0u8.into(), "broadcast_received != 0");
    assert!( word.terminal_busy() == 0u8.into(), "terminal_busy != 0");
    assert!( word.dynamic_bus_acceptance() == 0u8.into(), "dynamic_bus_acceptance != 0");
    assert!( word.message_error() == 0u8.into(), "message_error != 0");
    assert!( word.subsystem_error() == 0u8.into(), "subsystem_error != 0");
    assert!( word.terminal_error() == 0u8.into(), "terminal_error != 0");
    assert!( word.count() == None, "count != None");
    assert!( word.as_bytes() == [0, 0], "as_bytes != [0, 0]");
    assert!( word.as_value() == 0b0000000000000000, "as_value != 0b0000000000000000");
}

#[test]
fn test_word3() {
    let word = DataWord::from(0b0000000000000000);
    assert!( word.as_bytes() == [0, 0], "as_bytes != [0, 0]");
    assert!( word.as_value() == 0b0000000000000000, "as_value != 0b0000000000000000");
}

#[test]
fn test_word1() {
    let word = CommandWord::from(0b0000000000000000);
    assert!( word.subaddress() == 0u8.into(), "subaddress != 0");
    assert!( word.transmit_receive() == 0u8.into(), "transmit_receive != 0");
    assert!( word.mode_code() == 0u8.into(), "mode_code != 0");
    assert!( word.word_count() == 32u8.into(), "word_count != 32");
    assert!( word.address() == 0u8.into(), "address != 0");
    assert!( word.is_mode_code() == true, "is_mode_code != true");
    assert!( word.is_transmit() == false, "is_transmit != false");
    assert!( word.is_receive() == true, "is_receive != true");
    assert!( word.count() == Some(32), "count != Some(32)");
    assert!( word.as_bytes() == [0, 0], "as_bytes != [0, 0]");
    assert!( word.as_value() == 0b0000000000000000, "as_value != 0b0000000000000000");
}
