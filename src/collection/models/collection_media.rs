use crate::photos::Photo;
use crate::videos::Video;
use serde::{Deserialize, Serialize};

/// `Photo` or `Video` type in Collection
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CollectionMedia {
    Photo(Photo),
    Video(Video),
}
