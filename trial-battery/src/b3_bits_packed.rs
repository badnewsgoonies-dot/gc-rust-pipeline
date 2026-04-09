#[derive(Debug, Clone)]
pub struct PackedBits {
    storage: Vec<u64>,
    bits_per_value: u32,
    len: usize,
}

impl PackedBits {
    pub fn new(len: usize, bits_per_value: u32) -> Self {
        assert!(
            bits_per_value >= 1 && bits_per_value <= 32,
            "bits_per_value out of range"
        );
        let total_bits = len * bits_per_value as usize;
        let words = (total_bits + 63) / 64;
        let storage = vec![0u64; words];
        Self {
            storage,
            bits_per_value,
            len,
        }
    }

    pub fn set(&mut self, idx: usize, value: u32) {
        assert!(idx < self.len, "set out of range");
        let bit_offset = idx * self.bits_per_value as usize;
        let word_idx = bit_offset / 64;
        let bit_in_word = bit_offset % 64;
        let mask = ((1u64 << self.bits_per_value) - 1) << bit_in_word;
        self.storage[word_idx] =
            (self.storage[word_idx] & !mask) | (((value as u64) << bit_in_word) & mask);
    }

    pub fn get(&self, idx: usize) -> u32 {
        assert!(idx < self.len, "get out of range");
        let bit_offset = idx * self.bits_per_value as usize;
        let word_idx = bit_offset / 64;
        let bit_in_word = bit_offset % 64;
        let mask = (1u64 << self.bits_per_value) - 1;
        ((self.storage[word_idx] >> bit_in_word) & mask) as u32
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn bits_per_value(&self) -> u32 {
        self.bits_per_value
    }
}

#[cfg(test)]
mod tests {
    use super::PackedBits;

    #[test]
    fn set_get_round_trip() {
        let mut p = PackedBits::new(16, 4);
        for i in 0..16u32 {
            p.set(i as usize, i);
        }
        for i in 0..16u32 {
            assert_eq!(p.get(i as usize), i);
        }
    }

    #[test]
    fn overwrite_value() {
        let mut p = PackedBits::new(4, 4);
        p.set(0, 7);
        p.set(0, 3);
        assert_eq!(p.get(0), 3);
        assert_eq!(p.get(1), 0);
    }
}
