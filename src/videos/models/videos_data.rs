use crate::videos::Video;
use serde::{Deserialize, Serialize};

/// Response data for `Video` or `Popular Video` search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideosData {
    pub videos: Vec<Video>,
    pub url: Option<String>,
    pub page: i64,
    pub per_page: i64,
    pub total_results: i64,
    pub prev_page: Option<String>,
    pub next_page: Option<String>,
}
