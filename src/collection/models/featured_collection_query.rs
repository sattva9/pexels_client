use serde::{Deserialize, Serialize};

/// Query for `Featured Collection`
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct FeaturedCollectionQuery {
    page: Option<i64>,
    per_page: Option<i64>,
}

impl FeaturedCollectionQuery {
    pub fn new() -> Self {
        Self::default()
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
