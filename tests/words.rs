use mil_std_1553b::*;
#[test]
fn test_word1() {
    let item = CommandWord::from(0);
    assert_eq!( item.count(), 32 );
    assert_eq!( item.subaddress(), SubAddress::ModeCode(0) );
}
