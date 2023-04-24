use serde::{Deserialize, Serialize};

/// The `Video Pictures` resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoPictures {
    pub id: Option<i64>,
    pub picture: Option<String>,
    pub nr: Option<i64>,
}
