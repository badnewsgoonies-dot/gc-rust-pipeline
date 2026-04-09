use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

pub fn hash_bytes(data: &[u8]) -> String {
    let mut h = DefaultHasher::new();
    h.write(data);
    format!("{:016x}", h.finish())
}

pub struct ContentStore {
    data: HashMap<String, Vec<u8>>,
}

impl ContentStore {
    pub fn new() -> Self {
        ContentStore {
            data: HashMap::new(),
        }
    }

    pub fn put(&mut self, bytes: &[u8]) -> String {
        let hash = hash_bytes(bytes);
        if !self.data.contains_key(&hash) {
            self.data.insert(hash.clone(), bytes.to_vec());
        }
        hash
    }

    pub fn get(&self, hash: &str) -> Option<&Vec<u8>> {
        self.data.get(hash)
    }

    pub fn exists(&self, hash: &str) -> bool {
        self.data.contains_key(hash)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl Default for ContentStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_get() {
        let mut store = ContentStore::new();
        let hash = store.put(b"hello");
        assert_eq!(store.get(&hash), Some(&b"hello".to_vec()));
    }

    #[test]
    fn dedupe() {
        let mut store = ContentStore::new();
        store.put(b"x");
        store.put(b"x");
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn missing() {
        let store = ContentStore::new();
        assert_eq!(store.get("nope"), None);
        assert!(!store.exists("nope"));
    }
}
