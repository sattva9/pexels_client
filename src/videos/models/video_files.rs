use serde::{Deserialize, Serialize};

/// The `Video Files` resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoFiles {
    pub id: i64,
    pub quality: Option<String>,
    pub file_type: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub fps: Option<f32>,
    pub link: Option<String>,
}
