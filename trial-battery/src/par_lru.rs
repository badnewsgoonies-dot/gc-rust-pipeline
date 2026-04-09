#[derive(Debug, Clone)]
pub struct LruCache<K: std::hash::Hash + Eq + Clone, V: Clone> {
    entries: Vec<(K, V)>,
    capacity: usize,
}

impl<K: std::hash::Hash + Eq + Clone, V: Clone> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be greater than zero");
        Self {
            entries: Vec::new(),
            capacity,
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == &key) {
            self.entries.remove(pos);
            self.entries.push((key, value));
            return;
        }

        if self.entries.len() == self.capacity {
            self.entries.remove(0);
        }

        self.entries.push((key, value));
    }

    pub fn get(&mut self, key: &K) -> Option<V> {
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == key) {
            let (k, v) = self.entries.remove(pos);
            let out = v.clone();
            self.entries.push((k, v));
            Some(out)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::LruCache;

    #[test]
    fn put_get_roundtrip() {
        let mut cache = LruCache::<String, i64>::new(2);
        cache.put("a".to_string(), 1);
        assert_eq!(cache.get(&"a".to_string()), Some(1));
    }

    #[test]
    fn evict_oldest() {
        let mut cache = LruCache::<String, i64>::new(2);
        cache.put("a".to_string(), 1);
        cache.put("b".to_string(), 2);
        cache.put("c".to_string(), 3);

        assert_eq!(cache.get(&"a".to_string()), None);
        assert_eq!(cache.get(&"b".to_string()), Some(2));
        assert_eq!(cache.get(&"c".to_string()), Some(3));
    }

    #[test]
    fn touch_updates_recency() {
        let mut cache = LruCache::<String, i64>::new(2);
        cache.put("a".to_string(), 1);
        cache.put("b".to_string(), 2);
        assert_eq!(cache.get(&"a".to_string()), Some(1));
        cache.put("c".to_string(), 3);

        assert_eq!(cache.get(&"b".to_string()), None);
        assert_eq!(cache.get(&"a".to_string()), Some(1));
        assert_eq!(cache.get(&"c".to_string()), Some(3));
    }
}
