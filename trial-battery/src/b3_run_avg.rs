#[derive(Debug, Clone)]
pub struct RunningAverage {
    sum: f64,
    count: u64,
}

impl RunningAverage {
    pub fn new() -> Self {
        Self { sum: 0.0, count: 0 }
    }

    pub fn add(&mut self, sample: f64) {
        self.sum += sample;
        self.count += 1;
    }

    pub fn average(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.sum / self.count as f64)
        }
    }

    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn reset(&mut self) {
        self.sum = 0.0;
        self.count = 0;
    }
}

impl Default for RunningAverage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::RunningAverage;

    #[test]
    fn empty_average_is_none() {
        let r = RunningAverage::new();
        assert_eq!(r.average(), None);
        assert_eq!(r.count(), 0);
    }

    #[test]
    fn single_sample() {
        let mut r = RunningAverage::new();
        r.add(5.0);
        assert_eq!(r.average(), Some(5.0));
    }

    #[test]
    fn multi_sample() {
        let mut r = RunningAverage::new();
        for v in [1.0, 2.0, 3.0, 4.0, 5.0] {
            r.add(v);
        }
        assert_eq!(r.average(), Some(3.0));
        assert_eq!(r.count(), 5);
    }

    #[test]
    fn reset_clears() {
        let mut r = RunningAverage::new();
        r.add(10.0);
        r.reset();
        assert_eq!(r.count(), 0);
        assert_eq!(r.average(), None);
    }
}
