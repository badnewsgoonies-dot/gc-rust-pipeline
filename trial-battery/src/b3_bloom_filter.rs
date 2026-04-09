#[derive(Debug, Clone)]
pub struct BloomFilter {
    bits: Vec<u64>,
    num_bits: usize,
}

impl BloomFilter {
    pub fn new(num_bits: usize) -> Self {
        assert!(num_bits != 0, "num_bits must be greater than 0");
        Self {
            bits: vec![0u64; (num_bits + 63) / 64],
            num_bits,
        }
    }

    fn hash1(&self, key: u64) -> usize {
        ((key
            .wrapping_mul(0x9E3779B97F4A7C15)
            .wrapping_add(0x12345)) as usize)
            % self.num_bits
    }

    fn hash2(&self, key: u64) -> usize {
        ((key
            .wrapping_mul(0xC2B2AE3D27D4EB4F)
            .wrapping_add(0x9E370001)) as usize)
            % self.num_bits
    }

    fn set_bit(&mut self, idx: usize) {
        self.bits[idx / 64] |= 1u64 << (idx % 64);
    }

    fn get_bit(&self, idx: usize) -> bool {
        (self.bits[idx / 64] >> (idx % 64)) & 1 == 1
    }

    pub fn insert(&mut self, key: u64) {
        let h1 = self.hash1(key);
        let h2 = self.hash2(key);
        self.set_bit(h1);
        self.set_bit(h2);
    }

    pub fn contains(&self, key: u64) -> bool {
        let h1 = self.hash1(key);
        let h2 = self.hash2(key);
        self.get_bit(h1) && self.get_bit(h2)
    }
}

#[cfg(test)]
mod tests {
    use super::BloomFilter;

    #[test]
    fn inserted_keys_present() {
        let mut bf = BloomFilter::new(1024);
        bf.insert(42);
        bf.insert(100);
        bf.insert(7);
        assert!(bf.contains(42));
        assert!(bf.contains(100));
        assert!(bf.contains(7));
    }

    #[test]
    fn empty_does_not_contain() {
        let bf = BloomFilter::new(1024);
        assert!(!bf.contains(42));
    }
}
