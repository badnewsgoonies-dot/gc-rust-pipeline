#[derive(Debug, Clone)]
pub struct TopKSmallest<T: Ord + Clone> {
    items: Vec<T>,
    k: usize,
}

impl<T: Ord + Clone> TopKSmallest<T> {
    pub fn new(k: usize) -> Self {
        Self {
            items: Vec::new(),
            k,
        }
    }

    pub fn add(&mut self, item: T) {
        if self.items.len() < self.k {
            self.items.push(item);
            self.items.sort();
        } else if let Some(last) = self.items.last() {
            if &item < last {
                self.items.pop();
                self.items.push(item);
                self.items.sort();
            }
        }
    }

    pub fn into_sorted(self) -> Vec<T> {
        self.items
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::TopKSmallest;

    #[test]
    fn keeps_k_smallest() {
        let mut tk = TopKSmallest::<i64>::new(3);
        for v in [5, 1, 8, 2, 9, 3, 7] {
            tk.add(v);
        }
        assert_eq!(tk.into_sorted(), vec![1, 2, 3]);
    }

    #[test]
    fn fewer_than_k() {
        let mut tk = TopKSmallest::<i64>::new(5);
        tk.add(2);
        tk.add(1);
        tk.add(3);
        assert_eq!(tk.into_sorted(), vec![1, 2, 3]);
    }

    #[test]
    fn k_one() {
        let mut tk = TopKSmallest::<i64>::new(1);
        for v in [10, 5, 20, 3, 15] {
            tk.add(v);
        }
        assert_eq!(tk.into_sorted(), vec![3]);
    }
}
