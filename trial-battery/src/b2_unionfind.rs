#[derive(Debug, Clone)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u32>,
    num_sets: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect::<Vec<usize>>(),
            rank: vec![0u32; n],
            num_sets: n,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);

        if ra == rb {
            return false;
        }

        if self.rank[ra] < self.rank[rb] {
            self.parent[ra] = rb;
        } else if self.rank[ra] > self.rank[rb] {
            self.parent[rb] = ra;
        } else {
            self.parent[rb] = ra;
            self.rank[ra] += 1;
        }

        self.num_sets -= 1;
        true
    }

    pub fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    pub fn num_sets(&self) -> usize {
        self.num_sets
    }

    pub fn size(&self) -> usize {
        self.parent.len()
    }
}

#[cfg(test)]
mod tests {
    use super::UnionFind;

    #[test]
    fn init_all_disjoint() {
        let mut uf = UnionFind::new(5);
        assert_eq!(uf.num_sets(), 5);
        for i in 0..5 {
            assert_eq!(uf.find(i), i);
        }
    }

    #[test]
    fn union_merges() {
        let mut uf = UnionFind::new(5);
        assert!(uf.union(0, 1));
        assert!(uf.connected(0, 1));
        assert_eq!(uf.num_sets(), 4);
        assert!(!uf.union(0, 1));
    }

    #[test]
    fn transitive() {
        let mut uf = UnionFind::new(5);
        uf.union(0, 1);
        uf.union(1, 2);
        assert!(uf.connected(0, 2));
        assert_eq!(uf.num_sets(), 3);
    }
}
