pub struct GenEntry {
    pub generation: u64,
    pub snapshot_hash: String,
    pub parent_hash: Option<String>,
}

pub struct GenerationLog {
    entries: Vec<GenEntry>,
}

impl GenerationLog {
    pub fn new() -> GenerationLog {
        GenerationLog {
            entries: Vec::new(),
        }
    }

    pub fn head(&self) -> Option<&GenEntry> {
        self.entries.last()
    }

    pub fn next_generation(&self) -> u64 {
        match self.head() {
            Some(entry) => entry.generation + 1,
            None => 0,
        }
    }

    pub fn append(&mut self, snapshot_hash: String, parent_hash: Option<String>) -> Result<(), String> {
        match self.head() {
            Some(head) => {
                if parent_hash != Some(head.snapshot_hash.clone()) {
                    return Err("parent mismatch".to_string());
                }
            }
            None => {
                if parent_hash.is_some() {
                    return Err("parent mismatch".to_string());
                }
            }
        }

        let generation = self.next_generation();
        self.entries.push(GenEntry {
            generation,
            snapshot_hash,
            parent_hash,
        });

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.entries.len()
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
        assert_eq!(log.len(), 0);
    }

    #[test]
    fn append_chain() {
        let mut log = GenerationLog::new();
        log.append("hash1".to_string(), None).unwrap();
        log.append("hash2".to_string(), Some("hash1".to_string())).unwrap();
        log.append("hash3".to_string(), Some("hash2".to_string())).unwrap();
        assert_eq!(log.len(), 3);
        assert_eq!(log.head().unwrap().generation, 2);
    }

    #[test]
    fn parent_mismatch_rejected() {
        let mut log = GenerationLog::new();
        log.append("hash1".to_string(), None).unwrap();
        let result = log.append("hash2".to_string(), Some("wrong_hash".to_string()));
        assert!(result.is_err());
    }
}
