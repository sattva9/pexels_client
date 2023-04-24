use crate::videos::Video;
use crate::PexelsRequestStatistics;
use serde::{Deserialize, Serialize};

/// Response for `Video`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoResponse {
    pub(crate) stats: PexelsRequestStatistics,
    pub(crate) data: Video,
}

impl VideoResponse {
    pub fn stats(&self) -> &PexelsRequestStatistics {
        &self.stats
    }

    pub fn data(&self) -> &Video {
        &self.data
    }
}
