//! jsonrpc id field

use serde::{Deserialize, Serialize};

/// Request Id
#[derive(Debug, PartialEq, Clone, Hash, Ord, PartialOrd, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Id {
    /// No id (notification)
    Null,
    /// Numeric id
    Num(u64),
    /// String id
    Str(String),
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Id::Null => {
                write!(f, "null")
            }
            Id::Num(n) => {
                write!(f, "{}", n)
            }
            Id::Str(s) => {
                write!(f, "{}", s)
            }
        }
    }
}

impl From<u64> for Id {
    fn from(n: u64) -> Self {
        Self::Num(n)
    }
}
impl From<String> for Id {
    fn from(s: String) -> Self {
        Self::Str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn id_deserialization() {
        let s = r#""2""#;
        let deserialized: Id = serde_json::from_str(s).unwrap();
        assert_eq!(deserialized, Id::Str("2".into()));

        let s = r#"2"#;
        let deserialized: Id = serde_json::from_str(s).unwrap();
        assert_eq!(deserialized, Id::Num(2));

        let s = r#""2x""#;
        let deserialized: Id = serde_json::from_str(s).unwrap();
        assert_eq!(deserialized, Id::Str("2x".to_owned()));

        let s = r#"[null, 0, 2, "3"]"#;
        let deserialized: Vec<Id> = serde_json::from_str(s).unwrap();
        assert_eq!(
            deserialized,
            vec![Id::Null, Id::Num(0), Id::Num(2), Id::Str("3".into())]
        );
    }

    #[test]
    fn id_serialization() {
        let d = vec![
            Id::Null,
            Id::Num(0),
            Id::Num(2),
            Id::Num(3),
            Id::Str("3".to_owned()),
            Id::Str("test".to_owned()),
        ];
        let serialized = serde_json::to_string(&d).unwrap();
        assert_eq!(serialized, r#"[null,0,2,3,"3","test"]"#);
    }
}
