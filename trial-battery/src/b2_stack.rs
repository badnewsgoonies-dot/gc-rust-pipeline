#[derive(Debug, Clone, PartialEq)]
pub struct Stack<T: Clone> {
    data: Vec<T>,
}

impl<T: Clone> Stack<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<T: Clone> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn push_pop_lifo() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn peek_does_not_consume() {
        let mut stack = Stack::new();
        stack.push(42);

        assert_eq!(stack.peek(), Some(&42));
        assert_eq!(stack.peek(), Some(&42));
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn empty_ops() {
        let mut stack = Stack::<i64>::new();

        assert!(stack.is_empty());
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.peek(), None);
    }
}
