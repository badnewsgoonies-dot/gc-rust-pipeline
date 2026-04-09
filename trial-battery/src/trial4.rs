use std::fmt;
use std::io;

pub enum BriefcaseError {
    Io(io::Error),
    ManifestMalformed(String),
    SchemaVersionUnsupported(u32),
    HashMismatch { expected: String, actual: String },
    SignatureInvalid(String),
    ChainBroken { generation: u64, reason: String },
    StoreCorrupt(String),
    IntentNotFound(String),
}

impl fmt::Debug for BriefcaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BriefcaseError::Io(e) => f.debug_tuple("Io").field(e).finish(),
            BriefcaseError::ManifestMalformed(s) => f.debug_tuple("ManifestMalformed").field(s).finish(),
            BriefcaseError::SchemaVersionUnsupported(v) => f.debug_tuple("SchemaVersionUnsupported").field(v).finish(),
            BriefcaseError::HashMismatch { expected, actual } => {
                f.debug_struct("HashMismatch")
                    .field("expected", expected)
                    .field("actual", actual)
                    .finish()
            }
            BriefcaseError::SignatureInvalid(s) => f.debug_tuple("SignatureInvalid").field(s).finish(),
            BriefcaseError::ChainBroken { generation, reason } => {
                f.debug_struct("ChainBroken")
                    .field("generation", generation)
                    .field("reason", reason)
                    .finish()
            }
            BriefcaseError::StoreCorrupt(s) => f.debug_tuple("StoreCorrupt").field(s).finish(),
            BriefcaseError::IntentNotFound(s) => f.debug_tuple("IntentNotFound").field(s).finish(),
        }
    }
}

impl fmt::Display for BriefcaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BriefcaseError::Io(e) => write!(f, "io error: {}", e),
            BriefcaseError::ManifestMalformed(s) => write!(f, "manifest malformed: {}", s),
            BriefcaseError::SchemaVersionUnsupported(v) => write!(f, "schema version unsupported: {}", v),
            BriefcaseError::HashMismatch { expected, actual } => {
                write!(f, "hash mismatch: expected {}, got {}", expected, actual)
            }
            BriefcaseError::SignatureInvalid(s) => write!(f, "signature invalid: {}", s),
            BriefcaseError::ChainBroken { generation, reason } => {
                write!(f, "chain broken at gen {}: {}", generation, reason)
            }
            BriefcaseError::StoreCorrupt(s) => write!(f, "store corrupt: {}", s),
            BriefcaseError::IntentNotFound(s) => write!(f, "intent not found: {}", s),
        }
    }
}

impl std::error::Error for BriefcaseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BriefcaseError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for BriefcaseError {
    fn from(e: io::Error) -> Self {
        BriefcaseError::Io(e)
    }
}

pub type BriefcaseResult<T> = Result<T, BriefcaseError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_io() {
        let err = BriefcaseError::Io(io::Error::new(io::ErrorKind::NotFound, "x"));
        assert!(format!("{}", err).starts_with("io error:"));
    }

    #[test]
    fn display_hash() {
        let err = BriefcaseError::HashMismatch {
            expected: "a".into(),
            actual: "b".into(),
        };
        assert_eq!(format!("{}", err), "hash mismatch: expected a, got b");
    }

    #[test]
    fn from_io() {
        let e: BriefcaseError = io::Error::new(io::ErrorKind::Other, "boom").into();
        assert!(matches!(e, BriefcaseError::Io(_)));
    }
}
