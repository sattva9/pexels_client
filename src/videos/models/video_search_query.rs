use serde::{Deserialize, Serialize};

use crate::Locale;
use crate::Orientation;
use crate::Size;

/// Query `Video` search
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct VideoSearchQuery {
    query: String,
    orientation: Option<Orientation>,
    size: Option<Size>,
    locale: Option<Locale>,
    page: Option<i64>,
    per_page: Option<i64>,
}

impl VideoSearchQuery {
    /// Sets search query. `Ocean`, `Tigers`, `Pears`, etc.
    pub fn new(query: String) -> Self {
        Self {
            query,
            ..Default::default()
        }
    }

    /// Sets desired photo orientation.
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    /// Sets minimum photo size.
    pub fn size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets locale of the search you are performing.
    pub fn locale(mut self, locale: Locale) -> Self {
        self.locale = Some(locale);
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
