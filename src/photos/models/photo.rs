use crate::photos::PhotoSrc;
use crate::MediaType;
use serde::{Deserialize, Serialize};

/// The `Photo` resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Photo {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<MediaType>,
    pub id: i64,
    pub width: i64,
    pub height: i64,
    pub url: String,
    pub photographer: String,
    pub photographer_url: String,
    pub photographer_id: i64,
    pub avg_color: String,
    pub src: PhotoSrc,
    pub alt: String,
    pub liked: Option<bool>,
}
