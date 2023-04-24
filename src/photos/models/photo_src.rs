use serde::{Deserialize, Serialize};

/// The `PhotoSrc` resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoSrc {
    pub original: String,
    pub large: String,
    pub large2x: String,
    pub medium: String,
    pub small: String,
    pub portrait: String,
    pub landscape: String,
    pub tiny: String,
}
