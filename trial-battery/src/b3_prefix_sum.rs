#[derive(Debug, Clone)]
pub struct PrefixSum {
    sums: Vec<i64>,
}

impl PrefixSum {
    pub fn new(values: &[i64]) -> Self {
        let mut sums = vec![0; values.len() + 1];
        for i in 0..values.len() {
            sums[i + 1] = sums[i] + values[i];
        }
        Self { sums }
    }

    pub fn range_sum(&self, lo: usize, hi: usize) -> i64 {
        if hi >= self.sums.len() || lo > hi {
            panic!("range_sum: out of bounds");
        }
        self.sums[hi] - self.sums[lo]
    }

    pub fn total(&self) -> i64 {
        *self.sums.last().unwrap_or(&0)
    }

    pub fn len(&self) -> usize {
        self.sums.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::PrefixSum;

    #[test]
    fn range_sums_match() {
        let p = PrefixSum::new(&[1, 2, 3, 4, 5]);
        assert_eq!(p.range_sum(0, 5), 15);
        assert_eq!(p.range_sum(1, 4), 9);
        assert_eq!(p.range_sum(2, 3), 3);
        assert_eq!(p.range_sum(0, 0), 0);
    }

    #[test]
    fn total_matches_full_range() {
        let p = PrefixSum::new(&[10, 20, 30]);
        assert_eq!(p.total(), 60);
        assert_eq!(p.len(), 3);
    }

    #[test]
    fn empty_input() {
        let p = PrefixSum::new(&[]);
        assert_eq!(p.total(), 0);
        assert_eq!(p.len(), 0);
    }
}
