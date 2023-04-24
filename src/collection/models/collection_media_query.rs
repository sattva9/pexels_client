use crate::CollectionMediaType;
use serde::{Deserialize, Serialize};

/// Query for `Collection Media`
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CollectionMediaQuery {
    pub(crate) id: String,
    #[serde(rename = "type")]
    media_type: Option<CollectionMediaType>,
    page: Option<i64>,
    per_page: Option<i64>,
}

impl CollectionMediaQuery {
    pub fn new(id: String) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    /// Sets type of media you are requesting. If not given, all media will be returned.
    pub fn media_type(mut self, media_type: CollectionMediaType) -> Self {
        self.media_type = Some(media_type);
        self
    }

    /// Sets page number you are requesting. `Default: 1`
    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
        self
    }

    /// Sets number of results you are requesting per page. `Default: 15` `Max: 80`
    pub fn per_page(mut self, per_page: i64) -> Self {
        self.per_page = Some(per_page);
        self
    }
}
