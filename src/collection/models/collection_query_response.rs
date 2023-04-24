use crate::collection::CollectionData;
use crate::PexelsRequestStatistics;
use serde::{Deserialize, Serialize};

/// Response for `UserFeatured Collection` or `User Collection` query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionQueryResponse {
    pub(crate) stats: PexelsRequestStatistics,
    pub(crate) data: CollectionData,
}

impl CollectionQueryResponse {
    pub fn stats(&self) -> &PexelsRequestStatistics {
        &self.stats
    }

    pub fn data(&self) -> &CollectionData {
        &self.data
    }
}
