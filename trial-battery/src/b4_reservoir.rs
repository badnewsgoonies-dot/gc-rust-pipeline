#[derive(Debug, Clone)]
pub struct ReservoirSampler<T: Clone> {
    reservoir: Vec<T>,
    k: usize,
    seen: u64,
    rng_state: u64,
}

impl<T: Clone> ReservoirSampler<T> {
    pub fn new(k: usize, seed: u64) -> Self {
        Self {
            reservoir: Vec::new(),
            k,
            seen: 0,
            rng_state: seed,
        }
    }

    fn next_random(&mut self) -> u64 {
        self.rng_state = self
            .rng_state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.rng_state
    }

    pub fn add(&mut self, item: T) {
        self.seen += 1;
        if self.reservoir.len() < self.k {
            self.reservoir.push(item);
        } else {
            let r = (self.next_random() % self.seen) as usize;
            if r < self.k {
                self.reservoir[r] = item;
            }
        }
    }

    pub fn sample(&self) -> &[T] {
        &self.reservoir
    }

    pub fn seen(&self) -> u64 {
        self.seen
    }
}

#[cfg(test)]
mod tests {
    use super::ReservoirSampler;

    #[test]
    fn fewer_than_k_kept() {
        let mut rs = ReservoirSampler::<i64>::new(5, 42);
        for v in [1, 2, 3] {
            rs.add(v);
        }
        assert_eq!(rs.sample().len(), 3);
        assert_eq!(rs.seen(), 3);
    }

    #[test]
    fn at_least_k_size_k() {
        let mut rs = ReservoirSampler::<i64>::new(3, 42);
        for v in 1..=100 {
            rs.add(v);
        }
        assert_eq!(rs.sample().len(), 3);
        assert_eq!(rs.seen(), 100);
    }

    #[test]
    fn deterministic_with_seed() {
        let mut a = ReservoirSampler::<i64>::new(5, 7);
        let mut b = ReservoirSampler::<i64>::new(5, 7);
        for v in 1..=50 {
            a.add(v);
            b.add(v);
        }
        assert_eq!(a.sample(), b.sample());
    }
}
