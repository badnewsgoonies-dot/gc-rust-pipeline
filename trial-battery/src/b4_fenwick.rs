#[derive(Debug, Clone)]
pub struct FenwickTree {
    tree: Vec<i64>,
    n: usize,
}

impl FenwickTree {
    pub fn new(n: usize) -> Self {
        Self {
            tree: vec![0i64; n + 1],
            n,
        }
    }

    pub fn update(&mut self, mut idx: usize, delta: i64) {
        assert!(idx < self.n);
        idx += 1;
        while idx <= self.n {
            self.tree[idx] += delta;
            idx += idx & idx.wrapping_neg();
        }
    }

    pub fn prefix_sum(&self, mut idx: usize) -> i64 {
        if idx == 0 {
            return 0;
        }
        let mut sum = 0i64;
        while idx > 0 {
            sum += self.tree[idx];
            idx -= idx & idx.wrapping_neg();
        }
        sum
    }

    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        self.prefix_sum(r) - self.prefix_sum(l)
    }

    pub fn len(&self) -> usize {
        self.n
    }
}

#[cfg(test)]
mod tests {
    use super::FenwickTree;

    #[test]
    fn updates_and_prefix() {
        let mut bit = FenwickTree::new(5);
        for (i, v) in [1, 2, 3, 4, 5].iter().enumerate() {
            bit.update(i, *v);
        }
        assert_eq!(bit.prefix_sum(5), 15);
        assert_eq!(bit.prefix_sum(3), 6);
        assert_eq!(bit.prefix_sum(0), 0);
    }

    #[test]
    fn range_query() {
        let mut bit = FenwickTree::new(5);
        for (i, v) in [10, 20, 30, 40, 50].iter().enumerate() {
            bit.update(i, *v);
        }
        assert_eq!(bit.range_sum(1, 4), 90);
        assert_eq!(bit.range_sum(0, 5), 150);
    }

    #[test]
    fn len_matches() {
        let bit = FenwickTree::new(7);
        assert_eq!(bit.len(), 7);
    }
}
