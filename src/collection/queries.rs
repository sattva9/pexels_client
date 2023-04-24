use crate::collection::{
    CollectionData, CollectionMediaData, CollectionMediaQuery, CollectionMediaResponse,
    CollectionQueryResponse, FeaturedCollectionQuery, UserCollectionQuery,
};
use crate::common::get_request_stats;
use crate::{PexelsClient, PexelsResult};

impl PexelsClient {
    /// Returns all featured collections on Pexels.
    /// # Example
    /// ```rust
    /// use pexels_client::{*, collection::FeaturedCollectionQuery};
    ///
    /// # async fn test() {
    /// # let client = PexelsClient::new("auth_key".to_string()).unwrap();
    /// let response = client.featured_collection(FeaturedCollectionQuery::new()).await.unwrap();
    /// # }
    /// ```
    pub async fn featured_collection(
        &self,
        query: FeaturedCollectionQuery,
    ) -> PexelsResult<CollectionQueryResponse> {
        let url = "https://api.pexels.com/v1/collections/featured";
        let result = self.pexels_api_request(url, Some(query)).await?;
        Ok(CollectionQueryResponse {
            stats: get_request_stats(&result)?,
            data: Self::pexels_api_request_data::<CollectionData>(result).await?,
        })
    }

    /// Return returns all of user's collections.
    /// # Example
    /// ```rust
    /// use pexels_client::{*, collection::UserCollectionQuery};
    ///
    /// # async fn test() {
    /// # let client = PexelsClient::new("auth_key".to_string()).unwrap();
    /// let response = client.user_collection(UserCollectionQuery::new()).await.unwrap();
    /// # }
    /// ```
    pub async fn user_collection(
        &self,
        query: UserCollectionQuery,
    ) -> PexelsResult<CollectionQueryResponse> {
        let url = "https://api.pexels.com/v1/collections";
        let result = self.pexels_api_request(url, Some(query)).await?;
        Ok(CollectionQueryResponse {
            stats: get_request_stats(&result)?,
            data: Self::pexels_api_request_data::<CollectionData>(result).await?,
        })
    }

    /// Returns all the media (photos and videos) within a single collection.
    /// # Example
    /// ```rust
    /// use pexels_client::{*, collection::UserCollectionQuery};
    ///
    /// # async fn test() {
    /// # use pexels_client::collection::CollectionMediaQuery;
    /// let client = PexelsClient::new("auth_key".to_string()).unwrap();
    /// let response = client.collection_media(CollectionMediaQuery::new("bep4gxl".to_string())).await.unwrap();
    /// # }
    /// ```
    pub async fn collection_media(
        &self,
        query: CollectionMediaQuery,
    ) -> PexelsResult<CollectionMediaResponse> {
        let url = format!("https://api.pexels.com/v1/collections/{}", query.id);
        let result = self.pexels_api_request(&url, Some(query)).await?;

        Ok(CollectionMediaResponse {
            stats: get_request_stats(&result)?,
            data: Self::pexels_api_request_data::<CollectionMediaData>(result).await?,
        })
    }
}
