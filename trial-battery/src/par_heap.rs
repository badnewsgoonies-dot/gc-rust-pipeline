#[derive(Debug, Clone)]
pub struct BinaryMinHeap<T: Ord + Clone> {
    data: Vec<T>,
}

impl<T: Ord + Clone> BinaryMinHeap<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
        let last_idx = self.data.len() - 1;
        self.sift_up(last_idx);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            None
        } else {
            let removed = self.data.swap_remove(0);
            if !self.data.is_empty() {
                self.sift_down(0);
            }
            Some(removed)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if self.data[idx] < self.data[parent] {
                self.data.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, mut idx: usize) {
        let n = self.data.len();
        loop {
            let left = 2 * idx + 1;
            if left >= n {
                break;
            }
            let right = left + 1;
            let smallest = if right < n && self.data[right] < self.data[left] {
                right
            } else {
                left
            };
            if self.data[smallest] < self.data[idx] {
                self.data.swap(idx, smallest);
                idx = smallest;
            } else {
                break;
            }
        }
    }
}

impl<T: Ord + Clone> Default for BinaryMinHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::BinaryMinHeap;

    #[test]
    fn push_pop_sorted() {
        let mut heap = BinaryMinHeap::new();
        heap.push(5);
        heap.push(1);
        heap.push(3);
        heap.push(2);
        heap.push(4);

        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn peek_does_not_consume() {
        let mut heap = BinaryMinHeap::new();
        heap.push(10);
        heap.push(5);

        assert_eq!(heap.peek(), Some(&5));
        assert_eq!(heap.peek(), Some(&5));
        assert_eq!(heap.len(), 2);
    }

    #[test]
    fn empty_ops() {
        let mut heap = BinaryMinHeap::<i64>::new();
        assert!(heap.is_empty());
        assert_eq!(heap.pop(), None);
        assert_eq!(heap.peek(), None);
    }
}
