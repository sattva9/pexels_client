use serde::{Deserialize, Serialize};

/// Size Filter
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Size {
    Large,
    Medium,
    Small,
}
