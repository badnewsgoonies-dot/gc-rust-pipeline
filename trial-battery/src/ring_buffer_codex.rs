#[derive(Debug, Clone, PartialEq)]
pub struct RingBuffer<T: Clone> {
    buf: Vec<Option<T>>,
    head: usize,
    len: usize,
    capacity: usize,
}

impl<T: Clone> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: vec![None; capacity],
            head: 0,
            len: 0,
            capacity,
        }
    }

    pub fn push(&mut self, value: T) -> Result<(), String> {
        if self.len == self.capacity {
            return Err("ring buffer full".to_string());
        }
        let idx = (self.head + self.len) % self.capacity;
        self.buf[idx] = Some(value);
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let value = self.buf[self.head].take();
        self.head = (self.head + 1) % self.capacity;
        self.len -= 1;
        value
    }

    pub fn peek(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            self.buf[self.head].as_ref()
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == self.capacity
    }
}

impl<T: Clone> Default for RingBuffer<T> {
    fn default() -> Self {
        Self::new(8)
    }
}

#[cfg(test)]
mod tests {
    use super::RingBuffer;

    #[test]
    fn push_pop_fifo() {
        let mut rb = RingBuffer::<i64>::new(3);
        rb.push(10).unwrap();
        rb.push(20).unwrap();
        rb.push(30).unwrap();

        assert_eq!(rb.pop(), Some(10));
        assert_eq!(rb.pop(), Some(20));
        assert_eq!(rb.len(), 1);
    }

    #[test]
    fn peek_does_not_consume() {
        let mut rb = RingBuffer::<i64>::new(3);
        rb.push(42).unwrap();

        assert_eq!(rb.peek(), Some(&42));
        assert_eq!(rb.peek(), Some(&42));
        assert_eq!(rb.len(), 1);
        assert_eq!(rb.pop(), Some(42));
        assert_eq!(rb.peek(), None);
    }

    #[test]
    fn push_full_returns_err() {
        let mut rb = RingBuffer::<i64>::new(2);
        rb.push(1).unwrap();
        rb.push(2).unwrap();

        let err = rb.push(3).unwrap_err();
        assert!(err.contains("full"));
        assert_eq!(rb.len(), 2);
    }

    #[test]
    fn wrap_around() {
        let mut rb = RingBuffer::<i64>::new(2);
        rb.push(1).unwrap();
        assert_eq!(rb.pop(), Some(1));
        rb.push(2).unwrap();
        rb.push(3).unwrap();
        assert_eq!(rb.pop(), Some(2));
        assert_eq!(rb.pop(), Some(3));
        assert_eq!(rb.pop(), None);
    }
}
