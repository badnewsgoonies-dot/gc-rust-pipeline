use std::fmt;
use std::error::Error;

pub enum ManifestError {
    Empty,
    BadVersion(u32),
    BadName(String),
}

impl fmt::Display for ManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ManifestError::Empty => write!(f, "manifest is empty"),
            ManifestError::BadVersion(n) => write!(f, "bad version: {}", n),
            ManifestError::BadName(s) => write!(f, "bad name: {}", s),
        }
    }
}

impl fmt::Debug for ManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for ManifestError {}

pub struct Manifest {
    pub workstation_format_version: u32,
    pub project_name: String,
}

impl Manifest {
    pub fn validate(&self) -> Result<(), ManifestError> {
        if self.workstation_format_version != 1 {
            return Err(ManifestError::BadVersion(self.workstation_format_version));
        }
        if self.project_name.is_empty() {
            return Err(ManifestError::Empty);
        }
        if self.project_name == "BAD" {
            return Err(ManifestError::BadName(self.project_name.clone()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_passes() {
        let manifest = Manifest {
            workstation_format_version: 1,
            project_name: "good".to_string(),
        };
        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn bad_version_caught() {
        let manifest = Manifest {
            workstation_format_version: 2,
            project_name: "good".to_string(),
        };
        assert!(matches!(manifest.validate(), Err(ManifestError::BadVersion(2))));
    }

    #[test]
    fn empty_name_caught() {
        let manifest = Manifest {
            workstation_format_version: 1,
            project_name: "".to_string(),
        };
        assert!(matches!(manifest.validate(), Err(ManifestError::Empty)));
    }

    #[test]
    fn bad_name_caught() {
        let manifest = Manifest {
            workstation_format_version: 1,
            project_name: "BAD".to_string(),
        };
        assert!(matches!(manifest.validate(), Err(ManifestError::BadName(ref s)) if s == "BAD"));
    }
}
