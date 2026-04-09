pub fn bit_reverse_u32(input: u32) -> u32 {
    input.reverse_bits()
}

#[cfg(test)]
mod tests {
    use super::bit_reverse_u32;

    #[test]
    fn zero_unchanged() {
        assert_eq!(bit_reverse_u32(0), 0);
    }

    #[test]
    fn all_ones_unchanged() {
        assert_eq!(bit_reverse_u32(u32::MAX), u32::MAX);
    }

    #[test]
    fn one_bit_low_to_high() {
        assert_eq!(bit_reverse_u32(1), 1u32 << 31);
    }

    #[test]
    fn double_reverse_identity() {
        for v in [0u32, 1, 42, 1000, 0xCAFEBABE, u32::MAX] {
            assert_eq!(bit_reverse_u32(bit_reverse_u32(v)), v);
        }
    }
}
