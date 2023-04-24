use serde::{Deserialize, Serialize};

/// Media type Filter
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CollectionMediaType {
    Photos,
    Videos,
}

/// Media Data type
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum MediaType {
    Photo,
    Video,
}
