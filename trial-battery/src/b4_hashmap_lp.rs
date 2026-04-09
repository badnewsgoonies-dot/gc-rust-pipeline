use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct SimpleHashMap<K: std::hash::Hash + Eq + Clone, V: Clone> {
    slots: Vec<Option<(K, V)>>,
    len: usize,
}

impl<K: std::hash::Hash + Eq + Clone, V: Clone> SimpleHashMap<K, V> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        Self {
            slots: vec![None; capacity],
            len: 0,
        }
    }

    fn hash_key(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.slots.len()
    }

    pub fn put(&mut self, key: K, value: V) {
        assert!(self.len < self.slots.len(), "hashmap full");
        let mut idx = self.hash_key(&key);
        loop {
            match &mut self.slots[idx] {
                Some((k, v)) if *k == key => {
                    *v = value;
                    return;
                }
                None => {
                    self.slots[idx] = Some((key, value));
                    self.len += 1;
                    return;
                }
                _ => idx = (idx + 1) % self.slots.len(),
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let mut idx = self.hash_key(key);
        for _ in 0..self.slots.len() {
            match &self.slots[idx] {
                Some((k, v)) if k == key => return Some(v.clone()),
                None => return None,
                _ => idx = (idx + 1) % self.slots.len(),
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleHashMap;

    #[test]
    fn put_get() {
        let mut m = SimpleHashMap::<String, i64>::new(16);
        m.put("alice".to_string(), 30);
        m.put("bob".to_string(), 25);
        assert_eq!(m.get(&"alice".to_string()), Some(30));
        assert_eq!(m.get(&"bob".to_string()), Some(25));
        assert_eq!(m.get(&"missing".to_string()), None);
    }

    #[test]
    fn overwrite() {
        let mut m = SimpleHashMap::<String, i64>::new(16);
        m.put("k".to_string(), 1);
        m.put("k".to_string(), 2);
        assert_eq!(m.get(&"k".to_string()), Some(2));
        assert_eq!(m.len(), 1);
    }

    #[test]
    fn many_keys() {
        let mut m = SimpleHashMap::<String, i64>::new(64);
        for i in 0..40 {
            m.put(format!("k{}", i), i as i64);
        }
        for i in 0..40 {
            assert_eq!(m.get(&format!("k{}", i)), Some(i as i64));
        }
    }
}
