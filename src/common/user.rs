use serde::{Deserialize, Serialize};

/// The `User` resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub url: String,
}
