use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SimpleGraph {
    adj: HashMap<i64, Vec<i64>>,
}

impl SimpleGraph {
    pub fn new() -> Self {
        Self {
            adj: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: i64) {
        self.adj.entry(node).or_insert_with(Vec::new);
    }

    pub fn add_edge(&mut self, from: i64, to: i64) {
        self.add_node(from);
        self.add_node(to);
        self.adj.get_mut(&from).unwrap().push(to);
    }

    pub fn neighbors(&self, node: i64) -> Vec<i64> {
        self.adj.get(&node).cloned().unwrap_or_else(Vec::new)
    }

    pub fn has_node(&self, node: i64) -> bool {
        self.adj.contains_key(&node)
    }

    pub fn node_count(&self) -> usize {
        self.adj.len()
    }

    pub fn edge_count(&self) -> usize {
        self.adj.values().map(|v| v.len()).sum()
    }
}

impl Default for SimpleGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleGraph;

    #[test]
    fn add_nodes_and_edges() {
        let mut g = SimpleGraph::new();
        g.add_edge(1, 2);
        g.add_edge(1, 3);
        g.add_edge(2, 3);

        assert_eq!(g.node_count(), 3);
        assert_eq!(g.edge_count(), 3);
        assert!(g.has_node(1));
        assert!(g.has_node(3));
    }

    #[test]
    fn neighbors_returns_targets() {
        let mut g = SimpleGraph::new();
        g.add_edge(1, 2);
        g.add_edge(1, 3);

        let mut n = g.neighbors(1);
        n.sort();
        assert_eq!(n, vec![2, 3]);
    }

    #[test]
    fn unknown_node_has_no_neighbors() {
        let g = SimpleGraph::new();
        assert!(g.neighbors(99).is_empty());
        assert!(!g.has_node(99));
    }
}
