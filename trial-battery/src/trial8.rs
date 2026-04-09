pub trait Attendant {
    fn id(&self) -> &'static str;
    fn record_count(&self) -> usize;
    fn put(&mut self, key: String, payload: String);
}

pub struct IntentsAttendant {
    records: std::collections::HashMap<String, String>,
}

impl IntentsAttendant {
    pub fn new() -> Self {
        IntentsAttendant {
            records: std::collections::HashMap::new(),
        }
    }
}

impl Attendant for IntentsAttendant {
    fn id(&self) -> &'static str {
        "intents"
    }

    fn record_count(&self) -> usize {
        self.records.len()
    }

    fn put(&mut self, key: String, payload: String) {
        self.records.insert(key, payload);
    }
}

pub struct CodeAttendant {
    records: std::collections::HashMap<String, String>,
}

impl CodeAttendant {
    pub fn new() -> Self {
        CodeAttendant {
            records: std::collections::HashMap::new(),
        }
    }
}

impl Attendant for CodeAttendant {
    fn id(&self) -> &'static str {
        "code"
    }

    fn record_count(&self) -> usize {
        self.records.len()
    }

    fn put(&mut self, key: String, payload: String) {
        self.records.insert(key, payload);
    }
}

pub fn dispatch_to(att: &mut dyn Attendant, key: String, payload: String) {
    att.put(key, payload);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intents_id_correct() {
        let att = IntentsAttendant::new();
        assert_eq!(att.id(), "intents");
        assert_eq!(att.record_count(), 0);
    }

    #[test]
    fn code_grows() {
        let mut att = CodeAttendant::new();
        att.put("k1".to_string(), "v1".to_string());
        att.put("k2".to_string(), "v2".to_string());
        assert_eq!(att.record_count(), 2);
    }

    #[test]
    fn dyn_dispatch() {
        let mut att: Box<dyn Attendant> = Box::new(IntentsAttendant::new());
        dispatch_to(&mut *att, "k".to_string(), "v".to_string());
        assert_eq!(att.record_count(), 1);
    }
}
