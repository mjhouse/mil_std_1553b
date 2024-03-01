/// Given data with unspecified length, removes the first
/// 20 bits and converts them to a (sync,data,parity) triplet.
pub fn process(align: bool, data: &[u8]) -> (u8, u16, u8) {
    let mut sync = 0;
    let mut word = 0;
    let mut parity = 0;

    // aligned:
    //      | 11111111 | 11111111 | 11110000 |
    // unaligned:
    //      | 00001111 | 11111111 | 11111111 |

    if align {
        parity |= (data[2] & 0b00010000) >> 4;
        sync |= (data[0] & 0b11100000) >> 5;
        word |= (data[0] as u16 & 0b0000000000011111) << 11;
        word |= (data[1] as u16 & 0b0000000011111111) << 3;
        word |= (data[2] as u16 & 0b0000000011100000) >> 5;
    } else {
        parity |= data[2] & 0b00000001;
        sync |= (data[0] & 0b00001110) >> 1;
        word |= (data[0] as u16 & 0b0000000000000001) << 15;
        word |= (data[1] as u16 & 0b0000000011111111) << 7;
        word |= (data[2] as u16 & 0b0000000011111110) >> 1;
    }

    (sync, word, parity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_aligned_word_pattern() {
        let (_, word, _) = process(true, &[0b00010101, 0b01010101, 0b01000000]);
        assert_eq!(word, 0b1010101010101010);
    }

    #[test]
    fn test_process_unaligned_word_pattern() {
        let (_, word, _) = process(false, &[0b00000001, 0b01010101, 0b01010100]);
        assert_eq!(word, 0b1010101010101010);
    }

    #[test]
    fn test_process_aligned_word_ones() {
        let (_, word, _) = process(true, &[0b00011111, 0b11111111, 0b11100000]);
        assert_eq!(word, 0b1111111111111111);
    }

    #[test]
    fn test_process_unaligned_word_ones() {
        let (_, word, _) = process(false, &[0b00000001, 0b11111111, 0b11111110]);
        assert_eq!(word, 0b1111111111111111);
    }

    #[test]
    fn test_process_aligned_word_zeros() {
        let (_, word, _) = process(true, &[0b11100000, 0b00000000, 0b00011111]);
        assert_eq!(word, 0);
    }

    #[test]
    fn test_process_unaligned_word_zeros() {
        let (_, word, _) = process(false, &[0b11111110, 0b00000000, 0b00000001]);
        assert_eq!(word, 0);
    }

    #[test]
    fn test_process_aligned_sync_zeros() {
        let (sync, _, _) = process(true, &[0b00011111, 0b11111111, 0b11111111]);
        assert_eq!(sync, 0);
    }

    #[test]
    fn test_process_unaligned_sync_zeros() {
        let (sync, _, _) = process(false, &[0b11110001, 0b11111111, 0b11111111]);
        assert_eq!(sync, 0);
    }

    #[test]
    fn test_process_aligned_sync_ones() {
        let (sync, _, _) = process(true, &[0b11100000, 0b00000000, 0b00000000]);
        assert_eq!(sync, 7);
    }

    #[test]
    fn test_process_unaligned_sync_ones() {
        let (sync, _, _) = process(false, &[0b00001110, 0b00000000, 0b00000000]);
        assert_eq!(sync, 7);
    }

    #[test]
    fn test_process_aligned_parity_bit_one() {
        let (_, _, parity) = process(true, &[0b00000000, 0b00000000, 0b00010000]);
        assert_eq!(parity, 1);
    }

    #[test]
    fn test_process_unaligned_parity_bit_one() {
        let (_, _, parity) = process(false, &[0b00000000, 0b00000000, 0b00000001]);
        assert_eq!(parity, 1);
    }

    #[test]
    fn test_process_aligned_parity_bit_zero() {
        let (_, _, parity) = process(true, &[0b11111111, 0b11111111, 0b11101111]);
        assert_eq!(parity, 0);
    }

    #[test]
    fn test_process_unaligned_parity_bit_zero() {
        let (_, _, parity) = process(false, &[0b11111111, 0b11111111, 0b11111110]);
        assert_eq!(parity, 0);
    }
}
