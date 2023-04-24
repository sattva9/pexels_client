//! Structs for collection

mod collection;
mod collection_data;
mod collection_media;
mod collection_media_query;
mod collection_media_response;
mod collection_query_response;
mod featured_collection_query;
mod media_data;
mod user_collection_query;

pub use self::collection::Collection;
pub use self::collection_data::CollectionData;
pub use self::collection_media::CollectionMedia;
pub use self::collection_media_query::CollectionMediaQuery;
pub use self::collection_media_response::CollectionMediaResponse;
pub use self::collection_query_response::CollectionQueryResponse;
pub use self::featured_collection_query::FeaturedCollectionQuery;
pub use self::media_data::CollectionMediaData;
pub use self::user_collection_query::UserCollectionQuery;
