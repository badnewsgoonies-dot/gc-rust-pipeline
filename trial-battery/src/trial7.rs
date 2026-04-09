pub struct Envelope {
    pub attendant_id: String,
    pub session_id: String,
    pub generation_id: u64,
    pub counter: u64,
    pub evidence_kind: String,
    pub payload_hash: String,
    pub observed_at: f64,
    pub signature: String,
}

impl Envelope {
    pub fn new(
        attendant_id: &str,
        session_id: &str,
        evidence_kind: &str,
        generation_id: u64,
        counter: u64,
    ) -> Envelope {
        Envelope {
            attendant_id: attendant_id.to_string(),
            session_id: session_id.to_string(),
            generation_id,
            counter,
            evidence_kind: evidence_kind.to_string(),
            payload_hash: String::new(),
            observed_at: 0.0,
            signature: String::new(),
        }
    }

    pub fn canonical_string(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}:{}",
            self.attendant_id,
            self.session_id,
            self.generation_id,
            self.counter,
            self.evidence_kind,
            self.payload_hash
        )
    }

    pub fn is_signed(&self) -> bool {
        !self.signature.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_envelope_unsigned() {
        let env = Envelope::new("test", "session", "evidence", 1, 1);
        assert!(!env.is_signed());
    }

    #[test]
    fn canonical_includes_session() {
        let env = Envelope::new("test", "abc", "evidence", 1, 1);
        assert!(env.canonical_string().contains("abc"));
    }
}
