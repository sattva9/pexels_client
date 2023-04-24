use serde::{Deserialize, Serialize};

/// Color Filter
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Turquoise,
    Blue,
    Violet,
    Pink,
    Brown,
    Black,
    Gray,
    White,
    Hex(String),
}
