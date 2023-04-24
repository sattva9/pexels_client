use serde::{Deserialize, Serialize};

/// Query for `Popular Video` search
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PopularVideoQuery {
    min_width: Option<i64>,
    min_height: Option<i64>,
    min_duration: Option<i64>,
    max_duration: Option<i64>,
    page: Option<i64>,
    per_page: Option<i64>,
}

impl PopularVideoQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn min_width(mut self, min_width: i64) -> Self {
        self.min_width = Some(min_width);
        self
    }

    pub fn min_height(mut self, min_height: i64) -> Self {
        self.min_height = Some(min_height);
        self
    }

    pub fn min_duration(mut self, min_duration: i64) -> Self {
        self.min_duration = Some(min_duration);
        self
    }

    pub fn max_duration(mut self, max_duration: i64) -> Self {
        self.max_duration = Some(max_duration);
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
