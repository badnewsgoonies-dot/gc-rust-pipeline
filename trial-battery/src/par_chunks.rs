#[derive(Debug, Clone)]
pub struct Chunks<T: Clone> {
    source: Vec<T>,
    chunk_size: usize,
    pos: usize,
}

impl<T: Clone> Chunks<T> {
    pub fn new(source: Vec<T>, chunk_size: usize) -> Self {
        if chunk_size == 0 {
            panic!("chunk_size must be positive");
        }

        Self {
            source,
            chunk_size,
            pos: 0,
        }
    }
}

impl<T: Clone> Iterator for Chunks<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.pos >= self.source.len() {
            return None;
        }

        let end = std::cmp::min(self.pos + self.chunk_size, self.source.len());
        let slice = self.source[self.pos..end].to_vec();
        self.pos = end;
        Some(slice)
    }
}

#[cfg(test)]
mod tests {
    use super::Chunks;

    #[test]
    fn chunks_even_division() {
        let c = Chunks::new(vec![1, 2, 3, 4, 5, 6], 2);
        let collected: Vec<Vec<i64>> = c.collect();
        assert_eq!(collected, vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
    }

    #[test]
    fn chunks_with_remainder() {
        let c = Chunks::new(vec![1, 2, 3, 4, 5], 2);
        let collected: Vec<Vec<i64>> = c.collect();
        assert_eq!(collected, vec![vec![1, 2], vec![3, 4], vec![5]]);
    }

    #[test]
    fn chunks_empty_source() {
        let c = Chunks::new(Vec::<i64>::new(), 3);
        let collected: Vec<Vec<i64>> = c.collect();
        assert!(collected.is_empty());
    }

    #[test]
    fn chunks_larger_than_source() {
        let c = Chunks::new(vec![1, 2, 3], 10);
        let collected: Vec<Vec<i64>> = c.collect();
        assert_eq!(collected, vec![vec![1, 2, 3]]);
    }
}
