#[derive(Debug, Clone, PartialEq)]
pub struct SortedVec<T: Ord + Clone> {
    data: Vec<T>,
}

impl<T: Ord + Clone> SortedVec<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn insert(&mut self, value: T) {
        match self.data.binary_search(&value) {
            Ok(idx) => self.data.insert(idx, value),
            Err(idx) => self.data.insert(idx, value),
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.data.binary_search(value).is_ok()
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T: Ord + Clone> Default for SortedVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::SortedVec;

    #[test]
    fn insert_maintains_order() {
        let mut sv = SortedVec::<i64>::new();
        for v in [5, 1, 4, 2, 3] {
            sv.insert(v);
        }
        assert_eq!(sv.as_slice(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn contains_check() {
        let mut sv = SortedVec::<i64>::new();
        sv.insert(10);
        sv.insert(20);
        sv.insert(30);
        assert!(sv.contains(&20));
        assert!(!sv.contains(&15));
    }

    #[test]
    fn allows_duplicates() {
        let mut sv = SortedVec::<i64>::new();
        sv.insert(5);
        sv.insert(5);
        sv.insert(5);
        assert_eq!(sv.len(), 3);
        assert_eq!(sv.as_slice(), &[5, 5, 5]);
    }
}
