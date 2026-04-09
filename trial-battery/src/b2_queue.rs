use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct VQueue<T: Clone> {
    data: VecDeque<T>,
}

impl<T: Clone> VQueue<T> {
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.data.push_back(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    pub fn front(&self) -> Option<&T> {
        self.data.front()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<T: Clone> Default for VQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::VQueue;

    #[test]
    fn enqueue_dequeue_fifo() {
        let mut q = VQueue::new();
        q.enqueue(10);
        q.enqueue(20);
        q.enqueue(30);

        assert_eq!(q.dequeue(), Some(10));
        assert_eq!(q.dequeue(), Some(20));
        assert_eq!(q.len(), 1);
    }

    #[test]
    fn front_does_not_consume() {
        let mut q = VQueue::new();
        q.enqueue(5);

        assert_eq!(q.front(), Some(&5));
        assert_eq!(q.front(), Some(&5));
        assert_eq!(q.len(), 1);
    }

    #[test]
    fn empty_dequeue() {
        let mut q = VQueue::<i64>::new();

        assert_eq!(q.dequeue(), None);
        assert!(q.is_empty());
    }
}
