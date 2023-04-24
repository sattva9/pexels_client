use crate::photos::Photo;
use serde::{Deserialize, Serialize};

/// Response Data for `Photo` or `Curated Photo` search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotosData {
    pub photos: Vec<Photo>,
    pub page: i64,
    pub per_page: i64,
    pub total_results: i64,
    pub prev_page: Option<String>,
    pub next_page: Option<String>,
}
