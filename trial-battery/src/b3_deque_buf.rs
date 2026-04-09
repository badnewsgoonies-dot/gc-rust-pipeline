#[derive(Debug, Clone)]
pub struct CircularBuf<T: Clone> {
    data: Vec<Option<T>>,
    head: usize,
    len: usize,
    capacity: usize,
}

impl<T: Clone> CircularBuf<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be greater than 0");
        Self {
            data: vec![None; capacity],
            head: 0,
            len: 0,
            capacity,
        }
    }

    pub fn push(&mut self, value: T) {
        let idx = (self.head + self.len) % self.capacity;
        self.data[idx] = Some(value);
        if self.len < self.capacity {
            self.len += 1;
        } else {
            self.head = (self.head + 1) % self.capacity;
        }
    }

    pub fn iter(&self) -> Vec<T> {
        let mut values = Vec::with_capacity(self.len);
        for offset in 0..self.len {
            let idx = (self.head + offset) % self.capacity;
            if let Some(value) = &self.data[idx] {
                values.push(value.clone());
            }
        }
        values
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::CircularBuf;

    #[test]
    fn fill_no_overflow() {
        let mut cb = CircularBuf::<i64>::new(3);
        cb.push(1);
        cb.push(2);
        cb.push(3);
        assert_eq!(cb.iter(), vec![1, 2, 3]);
        assert_eq!(cb.len(), 3);
    }

    #[test]
    fn overwrite_oldest() {
        let mut cb = CircularBuf::<i64>::new(3);
        cb.push(1);
        cb.push(2);
        cb.push(3);
        cb.push(4);
        cb.push(5);
        assert_eq!(cb.iter(), vec![3, 4, 5]);
        assert_eq!(cb.len(), 3);
    }
}
