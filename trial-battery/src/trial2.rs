pub struct Manifest {
    workstation_format_version: u32,
    project_id: String,
    project_name: String,
    latest_generation_id: i64,
    attendants: Vec<String>,
}

impl Manifest {
    fn fresh(project_id: &str, project_name: &str) -> Self {
        Manifest {
            workstation_format_version: 1,
            project_id: project_id.to_string(),
            project_name: project_name.to_string(),
            latest_generation_id: -1,
            attendants: vec!["intents".to_string(), "code".to_string(), "verification".to_string(), "ideas".to_string(), "environment".to_string()],
        }
    }

    fn to_toml_text(&self) -> String {
        format!(
            "workstation_format_version = {}\nproject_id = \"{}\"\nproject_name = \"{}\"\nlatest_generation_id = {}\nattendants = [\"intents\", \"code\", \"verification\", \"ideas\", \"environment\"]\n",
            self.workstation_format_version,
            self.project_id,
            self.project_name,
            self.latest_generation_id
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes() {
        let manifest = Manifest::fresh("p1", "Project");
        let expected = "workstation_format_version = 1\nproject_id = \"p1\"\nproject_name = \"Project\"\nlatest_generation_id = -1\nattendants = [\"intents\", \"code\", \"verification\", \"ideas\", \"environment\"]\n";
        assert_eq!(manifest.to_toml_text(), expected);
    }
}
