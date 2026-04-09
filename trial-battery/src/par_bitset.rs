#[derive(Debug, Clone, PartialEq)]
pub struct BitSet {
    words: Vec<u64>,
    num_bits: usize,
}

impl BitSet {
    pub fn new(num_bits: usize) -> Self {
        let num_words = (num_bits + 63) / 64;
        Self {
            words: vec![0; num_words],
            num_bits,
        }
    }

    pub fn set(&mut self, idx: usize) {
        if idx >= self.num_bits {
            panic!("bitset index out of range");
        }
        self.words[idx / 64] |= 1u64 << (idx % 64);
    }

    pub fn clear(&mut self, idx: usize) {
        if idx >= self.num_bits {
            panic!("bitset index out of range");
        }
        self.words[idx / 64] &= !(1u64 << (idx % 64));
    }

    pub fn get(&self, idx: usize) -> bool {
        if idx >= self.num_bits {
            return false;
        }
        ((self.words[idx / 64] >> (idx % 64)) & 1) == 1
    }

    pub fn toggle(&mut self, idx: usize) {
        if idx >= self.num_bits {
            panic!("bitset index out of range");
        }
        self.words[idx / 64] ^= 1u64 << (idx % 64);
    }

    pub fn count_ones(&self) -> usize {
        self.words.iter().map(|word| word.count_ones() as usize).sum()
    }

    pub fn num_bits(&self) -> usize {
        self.num_bits
    }
}

#[cfg(test)]
mod tests {
    use super::BitSet;

    #[test]
    fn set_get_clear() {
        let mut b = BitSet::new(128);
        assert!(!b.get(42));
        b.set(42);
        assert!(b.get(42));
        b.clear(42);
        assert!(!b.get(42));
    }

    #[test]
    fn count_ones_tracks_sets() {
        let mut b = BitSet::new(100);
        assert_eq!(b.count_ones(), 0);
        b.set(0);
        b.set(33);
        b.set(99);
        assert_eq!(b.count_ones(), 3);
    }

    #[test]
    fn toggle_flips() {
        let mut b = BitSet::new(64);
        b.toggle(7);
        assert!(b.get(7));
        b.toggle(7);
        assert!(!b.get(7));
    }

    #[test]
    fn out_of_range_get_returns_false() {
        let b = BitSet::new(10);
        assert!(!b.get(100));
    }
}
