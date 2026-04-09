#[derive(Debug, Clone)]
pub struct OrderedSet<T: Ord + Clone> {
    inner: std::collections::BTreeSet<T>,
}

impl<T: Ord + Clone> OrderedSet<T> {
    pub fn new() -> Self {
        Self {
            inner: std::collections::BTreeSet::new(),
        }
    }

    pub fn insert(&mut self, value: T) -> bool {
        self.inner.insert(value)
    }

    pub fn contains(&self, value: &T) -> bool {
        self.inner.contains(value)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn min(&self) -> Option<&T> {
        self.inner.iter().next()
    }

    pub fn max(&self) -> Option<&T> {
        self.inner.iter().next_back()
    }
}

impl<T: Ord + Clone> Default for OrderedSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::OrderedSet;

    #[test]
    fn insert_dedup() {
        let mut s = OrderedSet::<i64>::new();
        assert!(s.insert(5));
        assert!(!s.insert(5));
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn contains_check() {
        let mut s = OrderedSet::<i64>::new();
        s.insert(10);
        assert!(s.contains(&10));
        assert!(!s.contains(&20));
    }

    #[test]
    fn min_max() {
        let mut s = OrderedSet::<i64>::new();
        for v in [3, 1, 4, 1, 5, 9, 2, 6] {
            s.insert(v);
        }
        assert_eq!(s.min(), Some(&1));
        assert_eq!(s.max(), Some(&9));
    }
}
