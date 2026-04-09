#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval {
    pub start: i64,
    pub end: i64,
}

impl Interval {
    pub fn new(start: i64, end: i64) -> Self {
        if start > end {
            panic!("interval start must be <= end");
        }
        Interval { start, end }
    }

    pub fn len(&self) -> i64 {
        self.end - self.start
    }

    pub fn contains(&self, point: i64) -> bool {
        point >= self.start && point < self.end
    }

    pub fn overlaps(&self, other: &Interval) -> bool {
        self.start < other.end && other.start < self.end
    }

    pub fn intersection(&self, other: &Interval) -> Option<Interval> {
        if self.overlaps(other) {
            Some(Interval {
                start: self.start.max(other.start),
                end: self.end.min(other.end),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Interval;

    #[test]
    fn contains_endpoints() {
        let i = Interval::new(0, 10);
        assert!(i.contains(0));
        assert!(i.contains(9));
        assert!(!i.contains(10));
        assert!(!i.contains(-1));
    }

    #[test]
    fn overlaps_cases() {
        let a = Interval::new(0, 5);
        let b = Interval::new(3, 8);
        let c = Interval::new(10, 20);
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
        assert!(!a.overlaps(&c));
    }

    #[test]
    fn intersection_nonempty() {
        let a = Interval::new(0, 10);
        let b = Interval::new(5, 15);
        assert_eq!(a.intersection(&b), Some(Interval::new(5, 10)));
    }

    #[test]
    fn intersection_empty() {
        let a = Interval::new(0, 5);
        let b = Interval::new(10, 20);
        assert_eq!(a.intersection(&b), None);
    }
}
