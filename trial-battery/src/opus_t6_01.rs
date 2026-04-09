pub struct GenEntry {
    pub generation: u64,
    pub snapshot_hash: String,
    pub parent_hash: Option<String>,
}

pub struct GenerationLog {
    entries: Vec<GenEntry>,
}

impl GenerationLog {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn head(&self) -> Option<&GenEntry> {
        self.entries.last()
    }

    pub fn next_generation(&self) -> u64 {
        self.head().map(|e| e.generation + 1).unwrap_or(0)
    }

    pub fn append(&mut self, snapshot_hash: String, parent_hash: Option<String>) -> Result<&GenEntry, String> {
        if parent_hash.as_deref() != self.head().map(|h| h.snapshot_hash.as_str()) {
            return Err("parent mismatch".to_string());
        }
        self.entries.push(GenEntry {
            generation: self.next_generation(),
            snapshot_hash,
            parent_hash,
        });
        Ok(self.entries.last().unwrap())
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn get(&self, generation: u64) -> Option<&GenEntry> {
        self.entries.iter().find(|e| e.generation == generation)
    }
}

impl Default for GenerationLog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_log() {
        let log = GenerationLog::new();
        assert!(log.head().is_none());
        assert_eq!(log.next_generation(), 0);
        assert_eq!(log.len(), 0);
    }

    #[test]
    fn append_chain() {
        let mut log = GenerationLog::new();
        log.append("h0".into(), None).unwrap();
        log.append("h1".into(), Some("h0".into())).unwrap();
        log.append("h2".into(), Some("h1".into())).unwrap();
        assert_eq!(log.len(), 3);
        assert_eq!(log.head().unwrap().generation, 2);
    }

    #[test]
    fn parent_mismatch_rejected() {
        let mut log = GenerationLog::new();
        log.append("h0".into(), None).unwrap();
        let err = log.append("h1".into(), Some("WRONG".into())).unwrap_err();
        assert!(err.contains("parent mismatch"));
    }
}
