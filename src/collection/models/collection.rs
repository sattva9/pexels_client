use serde::{Deserialize, Serialize};

/// The `Collection` resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: String,
    pub title: String,
    pub description: String,
    pub private: bool,
    pub media_count: i64,
    pub photos_count: i64,
    pub videos_count: i64,
}
