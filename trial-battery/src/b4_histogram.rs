#[derive(Debug, Clone)]
pub struct Histogram {
    boundaries: Vec<f64>,
    counts: Vec<u64>,
}

impl Histogram {
    pub fn new(boundaries: Vec<f64>) -> Self {
        assert!(!boundaries.is_empty());
        assert!(boundaries.windows(2).all(|w| w[0] <= w[1]));
        let counts = vec![0u64; boundaries.len() + 1];
        Self { boundaries, counts }
    }

    pub fn add(&mut self, value: f64) {
        let mut idx = 0;
        for (i, &b) in self.boundaries.iter().enumerate() {
            if value >= b {
                idx = i + 1;
            } else {
                break;
            }
        }
        self.counts[idx] += 1;
    }

    pub fn count_in_bucket(&self, idx: usize) -> u64 {
        self.counts[idx]
    }

    pub fn total_count(&self) -> u64 {
        self.counts.iter().sum()
    }

    pub fn num_buckets(&self) -> usize {
        self.counts.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Histogram;

    #[test]
    fn buckets_and_counts() {
        let mut h = Histogram::new(vec![10.0, 20.0, 30.0]);
        for v in [5.0, 15.0, 15.0, 25.0, 35.0, 35.0, 35.0] {
            h.add(v);
        }
        assert_eq!(h.count_in_bucket(0), 1);
        assert_eq!(h.count_in_bucket(1), 2);
        assert_eq!(h.count_in_bucket(2), 1);
        assert_eq!(h.count_in_bucket(3), 3);
        assert_eq!(h.total_count(), 7);
    }

    #[test]
    fn num_buckets_correct() {
        let h = Histogram::new(vec![1.0, 2.0]);
        assert_eq!(h.num_buckets(), 3);
    }
}
