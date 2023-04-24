use crate::videos::VideosData;
use crate::PexelsRequestStatistics;
use serde::{Deserialize, Serialize};

/// Response for `Video` or `Popular Video` search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoQueryResponse {
    pub(crate) stats: PexelsRequestStatistics,
    pub(crate) data: VideosData,
}

impl VideoQueryResponse {
    pub fn stats(&self) -> &PexelsRequestStatistics {
        &self.stats
    }

    pub fn data(&self) -> &VideosData {
        &self.data
    }
}
