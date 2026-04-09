use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct CounterMap<K: Eq + Hash + Clone> {
    counts: HashMap<K, u64>,
}

impl<K: Eq + Hash + Clone> CounterMap<K> {
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: K) {
        *self.counts.entry(key).or_insert(0) += 1;
    }

    pub fn add_n(&mut self, key: K, n: u64) {
        *self.counts.entry(key).or_insert(0) += n;
    }

    pub fn get(&self, key: &K) -> u64 {
        *self.counts.get(key).unwrap_or(&0)
    }

    pub fn total(&self) -> u64 {
        self.counts.values().sum()
    }

    pub fn unique_keys(&self) -> usize {
        self.counts.len()
    }
}

impl<K: Eq + Hash + Clone> Default for CounterMap<K> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::CounterMap;

    #[test]
    fn add_increments() {
        let mut c = CounterMap::<String>::new();
        c.add("a".to_string());
        c.add("a".to_string());
        c.add("b".to_string());

        assert_eq!(c.get(&"a".to_string()), 2);
        assert_eq!(c.get(&"b".to_string()), 1);
        assert_eq!(c.get(&"missing".to_string()), 0);
    }

    #[test]
    fn add_n_bulk() {
        let mut c = CounterMap::<String>::new();
        c.add_n("x".to_string(), 5);
        c.add_n("y".to_string(), 3);

        assert_eq!(c.total(), 8);
        assert_eq!(c.unique_keys(), 2);
    }

    #[test]
    fn empty_total() {
        let c = CounterMap::<String>::new();

        assert_eq!(c.total(), 0);
        assert_eq!(c.unique_keys(), 0);
    }
}
