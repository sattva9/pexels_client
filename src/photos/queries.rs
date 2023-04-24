use crate::get_request_stats;
use crate::photos::{
    Photo, PhotoCuratedQuery, PhotoQueryResponse, PhotoResponse, PhotoSearchQuery, PhotosData,
};
use crate::{PexelsClient, PexelsResult};

impl PexelsClient {
    /// Returns photos of requested query.
    /// # Example
    /// ```rust
    /// use pexels_client::{*, photos::PhotoSearchQuery};
    ///
    /// # async fn test() {
    /// # let client = PexelsClient::new("auth_key".to_string()).unwrap();
    /// let query = PhotoSearchQuery::new("tigers".to_string()).size(Size::Large);
    /// let response = client.photo_search(query).await.unwrap();
    /// # }
    /// ```
    pub async fn photo_search(&self, query: PhotoSearchQuery) -> PexelsResult<PhotoQueryResponse> {
        let url = "https://api.pexels.com/v1/search";
        let result = self.pexels_api_request(url, Some(query)).await?;
        Ok(PhotoQueryResponse {
            stats: get_request_stats(&result)?,
            data: Self::pexels_api_request_data::<PhotosData>(result).await?,
        })
    }

    /// Returns real-time photos curated by the Pexels team.
    /// # Example
    /// ```rust
    /// use pexels_client::{*, photos::PhotoCuratedQuery};
    ///
    /// # async fn test() {
    /// # let client = PexelsClient::new("auth_key".to_string()).unwrap();
    /// let response = client.curated_photo(PhotoCuratedQuery::new()).await.unwrap();
    /// # }
    /// ```
    pub async fn curated_photo(
        &self,
        query: PhotoCuratedQuery,
    ) -> PexelsResult<PhotoQueryResponse> {
        let url = "https://api.pexels.com/v1/curated";
        let result = self.pexels_api_request(url, Some(query)).await?;
        Ok(PhotoQueryResponse {
            stats: get_request_stats(&result)?,
            data: Self::pexels_api_request_data::<PhotosData>(result).await?,
        })
    }

    /// Returns specific Photo from its id.
    /// # Example
    /// ```rust
    /// use pexels_client::{*, photos::PhotoCuratedQuery};
    ///
    /// # async fn test() {
    /// # let client = PexelsClient::new("auth_key".to_string()).unwrap();
    /// let response = client.photo(2499).await.unwrap();
    /// # }
    /// ```
    pub async fn photo(&self, id: usize) -> PexelsResult<PhotoResponse> {
        let url = format!("https://api.pexels.com/v1/photos/{}", id);
        let result = self
            .pexels_api_request(&url, None::<PhotoSearchQuery>)
            .await?;
        Ok(PhotoResponse {
            stats: get_request_stats(&result)?,
            data: Self::pexels_api_request_data::<Photo>(result).await?,
        })
    }
}
