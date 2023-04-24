//! # pexels_client
//!
//! The `pexels_client` crate is a convenient wrapper around
//! [Pexels API](https://www.pexels.com/api/documentation).
//!
//! # Basic Usage
//!
//! ## Create a client
//! Creates pexels client taking authorization key as argument. You can get your authorization key [here](https://www.pexels.com/api/new/). You can make multiple requests using same client.
//! ```rust
//! use pexels_client::PexelsClient;
//!
//! fn main() {
//!     let client = PexelsClient::new("auth_key".to_string()).unwrap();
//! }
//! ```
//!
//! ## Search for photos
//!Returns photos of requested query.
//! ```rust
//! use pexels_client::{PexelsClient, photos::PhotoSearchQuery};
//!
//! async fn test() {
//!     let client = PexelsClient::new("auth_key".to_string()).unwrap();
//!     let response = client
//!                     .photo_search(PhotoSearchQuery::new("tigers".to_string()))
//!                     .await
//!                     .unwrap();
//! }
//! ```
//!
//! ## Search for photos with filters
//!Returns photos of requested query with filters.
//! ```rust
//! use pexels_client::{*, photos::PhotoSearchQuery};
//!
//! async fn test() {
//!     let client = PexelsClient::new("auth_key".to_string()).unwrap();
//!     let query = PhotoSearchQuery::new("tigers".to_string())
//!                     .orientation(Orientation::Portrait)
//!                     .size(Size::Large)
//!                     .color(Color::Blue)
//!                     .per_page(5);
//!     let response = client.photo_search(query).await.unwrap();
//! }
//! ```

pub mod collection;
mod common;
pub mod photos;
mod plexels_client;
pub mod videos;

pub use self::common::*;
pub use self::plexels_client::PexelsClient;
