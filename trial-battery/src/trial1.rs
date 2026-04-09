use std::time::{SystemTime, UNIX_EPOCH};

pub struct Manifest {
    pub workstation_format_version: u32,
    pub project_id: String,
    pub project_name: String,
    pub head_snapshot_hash: String,
    pub latest_generation_id: i64,
    pub created_at: f64,
    pub updated_at: f64,
    pub attendants: Vec<String>,
    pub default_verify_profiles: Vec<String>,
    pub import_origin: Option<String>,
}

impl Manifest {
    pub fn fresh(project_id: &str, project_name: &str) -> Manifest {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();
        Manifest {
            workstation_format_version: 1,
            project_id: project_id.to_string(),
            project_name: project_name.to_string(),
            head_snapshot_hash: String::new(),
            latest_generation_id: -1,
            created_at: now,
            updated_at: now,
            attendants: vec![
                "intents".to_string(),
                "code".to_string(),
                "verification".to_string(),
                "ideas".to_string(),
                "environment".to_string(),
            ],
            default_verify_profiles: vec!["repo_smoke".to_string()],
            import_origin: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_has_empty_head() {
        let m = Manifest::fresh("p1", "Project One");
        assert!(m.head_snapshot_hash.is_empty());
        assert_eq!(m.latest_generation_id, -1);
        assert_eq!(m.attendants.len(), 5);
        assert_eq!(m.project_id, "p1");
    }
}
