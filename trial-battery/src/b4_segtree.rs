#[derive(Debug, Clone)]
pub struct SegmentTreeSum {
    tree: Vec<i64>,
    n: usize,
}

impl SegmentTreeSum {
    pub fn new(values: &[i64]) -> Self {
        let n = values.len();
        let tree = vec![0i64; 4 * n.max(1)];
        let mut st = Self { tree, n };
        if n > 0 {
            st.build(1, 0, n, values);
        }
        st
    }

    fn build(&mut self, node: usize, lo: usize, hi: usize, values: &[i64]) {
        if lo + 1 == hi {
            self.tree[node] = values[lo];
            return;
        }
        let mid = (lo + hi) / 2;
        self.build(2 * node, lo, mid, values);
        self.build(2 * node + 1, mid, hi, values);
        self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
    }

    pub fn update(&mut self, idx: usize, value: i64) {
        self.update_inner(1, 0, self.n, idx, value);
    }

    fn update_inner(&mut self, node: usize, lo: usize, hi: usize, idx: usize, value: i64) {
        if lo + 1 == hi {
            self.tree[node] = value;
            return;
        }
        let mid = (lo + hi) / 2;
        if idx < mid {
            self.update_inner(2 * node, lo, mid, idx, value);
        } else {
            self.update_inner(2 * node + 1, mid, hi, idx, value);
        }
        self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
    }

    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        self.query_inner(1, 0, self.n, l, r)
    }

    fn query_inner(&self, node: usize, lo: usize, hi: usize, l: usize, r: usize) -> i64 {
        if r <= lo || hi <= l {
            return 0;
        }
        if l <= lo && hi <= r {
            return self.tree[node];
        }
        let mid = (lo + hi) / 2;
        self.query_inner(2 * node, lo, mid, l, r)
            + self.query_inner(2 * node + 1, mid, hi, l, r)
    }
}

#[cfg(test)]
mod tests {
    use super::SegmentTreeSum;

    #[test]
    fn build_and_query() {
        let st = SegmentTreeSum::new(&[1, 2, 3, 4, 5]);
        assert_eq!(st.range_sum(0, 5), 15);
        assert_eq!(st.range_sum(1, 4), 9);
        assert_eq!(st.range_sum(2, 3), 3);
    }

    #[test]
    fn update_and_query() {
        let mut st = SegmentTreeSum::new(&[1, 2, 3, 4, 5]);
        st.update(2, 10);
        assert_eq!(st.range_sum(0, 5), 22);
        assert_eq!(st.range_sum(2, 3), 10);
    }

    #[test]
    fn empty_query() {
        let st = SegmentTreeSum::new(&[]);
        assert_eq!(st.range_sum(0, 0), 0);
    }
}
