use crate::constants::*;

macro_rules! make_count {
    ( $c:expr ) => {{
        $c[0].count_ones() + 
        $c[1].count_ones()
    }}
}

macro_rules! make_parity {
    ( $c:expr ) => {{
        match make_count!($c) % 2 {
            0 => 1,
            _ => 0,
        }
    }}
}

#[derive(Clone,Copy)]
pub struct Packet {
    pub sync: u8,
    pub content: [u8;2],
    pub parity: u8,
}

impl Packet {

    pub fn new(sync: u8, content: [u8;2], parity: u8) -> Self {
        Self { sync, content, parity }
    }

    pub fn data(data: [u8;2]) -> Self {
        Self::new(DATA_SYNC,data,make_parity!(data))
    }

    pub fn service(data: [u8;2]) -> Self {
        Self::new(SERV_SYNC,data,make_parity!(data))
    }

    pub fn is_data(&self) -> bool {
        self.sync == DATA_SYNC
    }

    pub fn is_service(&self) -> bool {
        self.sync == SERV_SYNC
    }

    pub fn is_valid(&self) -> bool {
        make_parity!(self.content) == self.parity
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    // only useable with no_std commented out
    macro_rules! debug_bytes {
        ( $w: expr ) => {
            println!("bits: {:#018b}", $w);
        }
    }

    #[test]
    fn test_new_data_packet() {
        let packet = Packet::data([0b00000000,0b00000000]);
        assert!(packet.is_data());
    }

    #[test]
    fn test_new_service_packet() {
        let packet = Packet::service([0b00000000,0b00000000]);
        assert!(packet.is_service());
    }

    #[test]
    fn test_packet_parity_even_both() {
        let packet = Packet::data([0b01000000,0b00100000]);
        assert_eq!(packet.parity,1);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_even_first() {
        let packet = Packet::data([0b01100000,0b00000000]);
        assert_eq!(packet.parity,1);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_even_second() {
        let packet = Packet::data([0b00000000,0b00110000]);
        assert_eq!(packet.parity,1);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_odd_both() {
        let packet = Packet::data([0b00110000,0b00001000]);
        assert_eq!(packet.parity,0);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_odd_first() {
        let packet = Packet::data([0b00111000,0b00000000]);
        assert_eq!(packet.parity,0);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_parity_odd_second() {
        let packet = Packet::data([0b00000000,0b00111000]);
        assert_eq!(packet.parity,0);
        assert!(packet.is_valid());
    }

    #[test]
    fn test_packet_bad_parity_odd() {
        let mut packet = Packet::data([0b00000000,0b00111000]);
        packet.parity = 1;
        assert_eq!(packet.parity,1);
        assert!(!packet.is_valid());
    }

    #[test]
    fn test_packet_bad_parity_even() {
        let mut packet = Packet::data([0b00000000,0b00110000]);
        packet.parity = 0;
        assert_eq!(packet.parity,0);
        assert!(!packet.is_valid());
    }
}