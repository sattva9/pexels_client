use crate::collection::Collection;
use serde::{Deserialize, Serialize};

/// Response data for collection query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionData {
    pub collections: Vec<Collection>,
    pub page: i64,
    pub per_page: i64,
    pub total_results: i64,
    pub prev_page: Option<String>,
    pub next_page: Option<String>,
}
