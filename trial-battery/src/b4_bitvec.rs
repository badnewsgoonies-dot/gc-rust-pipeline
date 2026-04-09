#[derive(Debug, Clone, PartialEq)]
pub struct SimpleBitVec {
    data: Vec<u64>,
    len: usize,
}

impl SimpleBitVec {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            len: 0,
        }
    }

    pub fn push(&mut self, bit: bool) {
        let word_idx = self.len / 64;
        while self.data.len() <= word_idx {
            self.data.push(0);
        }
        if bit {
            self.data[word_idx] |= 1u64 << (self.len % 64);
        }
        self.len += 1;
    }

    pub fn get(&self, idx: usize) -> bool {
        assert!(idx < self.len, "get out of range");
        ((self.data[idx / 64] >> (idx % 64)) & 1) == 1
    }

    pub fn set(&mut self, idx: usize, bit: bool) {
        assert!(idx < self.len, "set out of range");
        let word_idx = idx / 64;
        let bit_idx = idx % 64;
        if bit {
            self.data[word_idx] |= 1u64 << bit_idx;
        } else {
            self.data[word_idx] &= !(1u64 << bit_idx);
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl Default for SimpleBitVec {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleBitVec;

    #[test]
    fn push_then_get() {
        let mut bv = SimpleBitVec::new();
        bv.push(true);
        bv.push(false);
        bv.push(true);
        assert!(bv.get(0));
        assert!(!bv.get(1));
        assert!(bv.get(2));
        assert_eq!(bv.len(), 3);
    }

    #[test]
    fn set_changes_value() {
        let mut bv = SimpleBitVec::new();
        for _ in 0..10 {
            bv.push(false);
        }
        bv.set(5, true);
        assert!(bv.get(5));
        assert!(!bv.get(4));
        assert!(!bv.get(6));
    }

    #[test]
    fn cross_word_boundary() {
        let mut bv = SimpleBitVec::new();
        for i in 0..130 {
            bv.push(i % 7 == 0);
        }
        assert!(bv.get(0));
        assert!(bv.get(7));
        assert!(bv.get(63));
        assert!(bv.get(70));
        assert!(bv.get(126));
        assert!(!bv.get(1));
    }
}
