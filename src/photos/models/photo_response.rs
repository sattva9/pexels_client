use crate::photos::Photo;
use crate::PexelsRequestStatistics;
use serde::{Deserialize, Serialize};

/// Response for `Photo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoResponse {
    pub(crate) stats: PexelsRequestStatistics,
    pub(crate) data: Photo,
}

impl PhotoResponse {
    pub fn stats(&self) -> &PexelsRequestStatistics {
        &self.stats
    }

    pub fn data(&self) -> &Photo {
        &self.data
    }
}
