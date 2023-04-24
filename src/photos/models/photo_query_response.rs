use crate::photos::PhotosData;
use crate::PexelsRequestStatistics;
use serde::{Deserialize, Serialize};

/// Response for `Photo` of `Curated Photo` search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoQueryResponse {
    pub(crate) stats: PexelsRequestStatistics,
    pub(crate) data: PhotosData,
}

impl PhotoQueryResponse {
    pub fn stats(&self) -> &PexelsRequestStatistics {
        &self.stats
    }

    pub fn data(&self) -> &PhotosData {
        &self.data
    }
}
