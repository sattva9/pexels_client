use crate::videos::{VideoFiles, VideoPictures};
use crate::{MediaType, User};
use serde::{Deserialize, Serialize};

/// The `Video` resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<MediaType>,
    pub id: i64,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub url: Option<String>,
    pub image: Option<String>,
    pub duration: Option<i64>,
    pub user: Option<User>,
    pub video_files: Vec<VideoFiles>,
    pub video_pictures: Option<Vec<VideoPictures>>,
}
