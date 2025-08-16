pub fn internet_checksum(data: &[u8]) -> u16 {
    let mut sum: u32 = 0;

    let chunks = data.chunks_exact(2);
    let remainder = chunks.remainder();

    for chunk in chunks {
        let word = u16::from_be_bytes([chunk[0], chunk[1]]) as u32;
        sum += word;
    }

    if !remainder.is_empty() {
        sum += (remainder[0] as u32) << 8;
    }

    sum = (sum & 0xFFFF) + (sum >> 16);
    sum = (sum & 0xFFFF) + (sum >> 16);

    !(sum as u16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internet_checksum() {
        let data: [u8; 20] = [
            0x45, 0x00, 0x00, 0x73, 0x00, 0x00, 0x40, 0x00, 0x40, 0x11, 0x00, 0x00, 0xc0, 0xa8,
            0x00, 0x01, 0xc0, 0xa8, 0x00, 0xc7,
        ];
        let checksum = internet_checksum(&data);
        assert_eq!(checksum, 0xb861);
    }

    #[test]
    fn test_empty_data() {
        let data: [u8; 0] = [];
        let checksum = internet_checksum(&data);
        assert_eq!(checksum, 0xFFFF);
    }

    #[test]
    fn test_single_byte() {
        let data = [0x42];
        let checksum = internet_checksum(&data);
        assert_eq!(checksum, !0x4200);
    }

    #[test]
    fn test_odd_length() {
        let data = [0x12, 0x34, 0x56];
        let checksum = internet_checksum(&data);
        assert_eq!(checksum, !(0x1234 + 0x5600));
    }

    #[test]
    fn test_carry_propagation() {
        let data = [0xFF, 0xFF, 0xFF, 0xFF];
        let checksum = internet_checksum(&data);
        assert_eq!(checksum, 0);
    }
}
