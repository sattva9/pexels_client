use crate::common::get_request_stats;
use crate::videos::{
    PopularVideoQuery, Video, VideoQueryResponse, VideoResponse, VideoSearchQuery, VideosData,
};
use crate::{PexelsClient, PexelsResult};

impl PexelsClient {
    /// Returns videos of requested query.
    /// # Example
    /// ```rust
    /// use pexels_client::{*, videos::VideoSearchQuery};
    ///
    /// # async fn test() {
    /// let query = VideoSearchQuery::new("tigers".to_string()).orientation(Orientation::Portrait);
    /// # let client = PexelsClient::new("auth_key".to_string()).unwrap();
    /// let response = client.video_search(query).await.unwrap();
    /// # }
    /// ```
    pub async fn video_search(&self, query: VideoSearchQuery) -> PexelsResult<VideoQueryResponse> {
        let url = "https://api.pexels.com/videos/search";
        let result = self.pexels_api_request(url, Some(query)).await?;
        Ok(VideoQueryResponse {
            stats: get_request_stats(&result)?,
            data: Self::pexels_api_request_data::<VideosData>(result).await?,
        })
    }

    /// Returns the current popular Pexels videos.
    /// # Example
    /// ```rust
    /// use pexels_client::{*, videos::PopularVideoQuery};
    ///
    /// # async fn test() {
    /// # let client = PexelsClient::new("auth_key".to_string()).unwrap();
    /// let response = client.popular_videos(PopularVideoQuery::new()).await.unwrap();
    /// # }
    /// ```
    pub async fn popular_videos(
        &self,
        query: PopularVideoQuery,
    ) -> PexelsResult<VideoQueryResponse> {
        let url = "https://api.pexels.com/videos/popular";
        let result = self.pexels_api_request(url, Some(query)).await?;
        Ok(VideoQueryResponse {
            stats: get_request_stats(&result)?,
            data: Self::pexels_api_request_data::<VideosData>(result).await?,
        })
    }

    /// Returns a specific Video from its id.
    /// # Example
    /// ```rust
    /// use pexels_client::*;
    ///
    /// # async fn test() {
    /// # let client = PexelsClient::new("auth_key".to_string()).unwrap();
    /// let response = client.video(3421).await.unwrap();
    /// # }
    /// ```
    pub async fn video(&self, id: usize) -> PexelsResult<VideoResponse> {
        let url = format!("https://api.pexels.com/videos/videos/{}", id);
        let result = self
            .pexels_api_request(&url, None::<VideoSearchQuery>)
            .await?;
        Ok(VideoResponse {
            stats: get_request_stats(&result)?,
            data: Self::pexels_api_request_data::<Video>(result).await?,
        })
    }
}
