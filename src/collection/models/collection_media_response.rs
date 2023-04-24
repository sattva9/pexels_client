use crate::collection::CollectionMediaData;
use crate::PexelsRequestStatistics;
use serde::{Deserialize, Serialize};

/// Response for `Collection Media` query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionMediaResponse {
    pub(crate) stats: PexelsRequestStatistics,
    pub(crate) data: CollectionMediaData,
}

impl CollectionMediaResponse {
    pub fn stats(&self) -> &PexelsRequestStatistics {
        &self.stats
    }

    pub fn data(&self) -> &CollectionMediaData {
        &self.data
    }
}
