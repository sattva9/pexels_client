use crate::collection::models::collection_media::CollectionMedia;
use serde::{Deserialize, Serialize};

/// Response data for `Collection Media` query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionMediaData {
    pub id: String,
    pub media: Vec<CollectionMedia>,
    pub page: i64,
    pub per_page: i64,
    pub total_results: i64,
    pub prev_page: Option<String>,
    pub next_page: Option<String>,
}
