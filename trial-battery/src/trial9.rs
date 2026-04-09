use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

#[derive(Clone)]
pub struct ChainNode {
    pub index: u64,
    pub payload: String,
    pub parent_hash: String,
    pub self_hash: String,
}

fn compute_hash(index: u64, payload: &str, parent_hash: &str) -> String {
    let mut h = DefaultHasher::new();
    h.write(&index.to_le_bytes());
    h.write(payload.as_bytes());
    h.write(parent_hash.as_bytes());
    format!("{:016x}", h.finish())
}

pub struct Chain {
    nodes: Vec<ChainNode>,
}

impl Chain {
    pub fn new() -> Self {
        Chain {
            nodes: Vec::new(),
        }
    }

    pub fn append(&mut self, payload: String) -> String {
        let index = self.nodes.len() as u64;
        let parent_hash = self
            .nodes
            .last()
            .map(|n| n.self_hash.clone())
            .unwrap_or_default();
        let self_hash = compute_hash(index, &payload, &parent_hash);
        self.nodes.push(ChainNode {
            index,
            payload,
            parent_hash,
            self_hash: self_hash.clone(),
        });
        self_hash
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn verify(&self) -> Result<(), String> {
        for i in 0..self.nodes.len() {
            let node = &self.nodes[i];
            if i == 0 {
                if !node.parent_hash.is_empty() {
                    return Err(format!("Node 0 should have empty parent_hash, got '{}'", node.parent_hash));
                }
            } else {
                let prev_hash = &self.nodes[i - 1].self_hash;
                if node.parent_hash != *prev_hash {
                    return Err(format!("Node {} parent_hash mismatch: expected '{}', got '{}'", i, prev_hash, node.parent_hash));
                }
            }
            let computed = compute_hash(node.index, &node.payload, &node.parent_hash);
            if node.self_hash != computed {
                return Err(format!("Node {} self_hash mismatch: expected '{}', got '{}'", i, computed, node.self_hash));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chain_grows_and_verifies() {
        let mut chain = Chain::new();
        chain.append("a".to_string());
        chain.append("b".to_string());
        chain.append("c".to_string());
        assert_eq!(chain.len(), 3);
        assert!(chain.verify().is_ok());
    }

    #[test]
    fn empty_chain_verifies() {
        let chain = Chain::new();
        assert!(chain.verify().is_ok());
    }

    #[test]
    fn tampered_chain_caught() {
        let mut chain = Chain::new();
        chain.append("a".to_string());
        chain.append("b".to_string());
        chain.append("c".to_string());
        chain.nodes[1].payload = "tampered".to_string();
        assert!(chain.verify().is_err());
    }
}
