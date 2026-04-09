#[derive(Debug, Clone)]
pub struct LcgRng {
    state: u64,
}

impl LcgRng {
    const A: u64 = 6364136223846793005u64;
    const C: u64 = 1442695040888963407u64;

    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(Self::A)
            .wrapping_add(Self::C);
        self.state
    }

    pub fn next_range(&mut self, lo: u64, hi: u64) -> u64 {
        assert!(lo < hi, "next_range: lo must be < hi");
        lo + (self.next_u64() % (hi - lo))
    }

    pub fn next_bool(&mut self) -> bool {
        (self.next_u64() & 1) == 1
    }
}

#[cfg(test)]
mod tests {
    use super::LcgRng;

    #[test]
    fn deterministic_seed() {
        let mut a = LcgRng::new(42);
        let mut b = LcgRng::new(42);
        for _ in 0..10 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn range_within_bounds() {
        let mut r = LcgRng::new(7);
        for _ in 0..1000 {
            let v = r.next_range(10, 20);
            assert!(v >= 10 && v < 20);
        }
    }

    #[test]
    fn different_seeds_diverge() {
        let mut a = LcgRng::new(1);
        let mut b = LcgRng::new(2);
        assert_ne!(a.next_u64(), b.next_u64());
    }
}
