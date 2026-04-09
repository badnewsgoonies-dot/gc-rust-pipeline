pub struct BoundedQueue {
    items: Vec<i64>,
    capacity: usize,
}

impl BoundedQueue {
    pub fn new(capacity: usize) -> Self {
        BoundedQueue {
            items: Vec::new(),
            capacity,
        }
    }

    pub fn push(&mut self, value: i64) -> Result<(), String> {
        if self.items.len() == self.capacity {
            return Err("queue full".to_string());
        }
        self.items.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<i64> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.items.len() == self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop_fifo() {
        let mut q = BoundedQueue::new(3);
        q.push(1).unwrap();
        q.push(2).unwrap();
        q.push(3).unwrap();
        assert_eq!(q.pop(), Some(1));
        assert_eq!(q.pop(), Some(2));
        assert_eq!(q.len(), 1);
    }

    #[test]
    fn push_full_returns_err() {
        let mut q = BoundedQueue::new(2);
        assert!(q.push(10).is_ok());
        assert!(q.push(20).is_ok());
        let result = q.push(30);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("queue full"));
    }

    #[test]
    fn empty_pop_is_none() {
        let mut q = BoundedQueue::new(5);
        assert_eq!(q.pop(), None);
        assert!(q.is_empty());
        assert!(!q.is_full());
    }
}
