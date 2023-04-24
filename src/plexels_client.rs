use crate::{PexelsError, PexelsResult};
use reqwest::{header, Client as HttpClient, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Wrapper for Pexels API
pub struct PexelsClient {
    http_client: HttpClient,
}

impl PexelsClient {
    /// Creates pexels client taking authorization key as argument. You can get your authorization key here. You can make multiple requests using same client.
    /// # Example
    /// ```rust
    /// use pexels_client::PexelsClient;
    ///
    /// # fn main()  {
    /// let client = PexelsClient::new("auth_key".to_string()).unwrap();
    /// # }
    /// ```
    pub fn new(authorization_key: String) -> PexelsResult<PexelsClient> {
        let mut headers = header::HeaderMap::new();
        let header_value = header::HeaderValue::from_str(&authorization_key)
            .map_err(|e| PexelsError::ClientBuildError(format!("{e}")))?;
        headers.insert(header::AUTHORIZATION, header_value);

        let http_client = HttpClient::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| PexelsError::ClientBuildError(format!("{e}")))?;

        Ok(Self { http_client })
    }

    pub(crate) async fn pexels_api_request<T: Serialize>(
        &self,
        url: &str,
        query: Option<T>,
    ) -> PexelsResult<Response> {
        let mut request_builder = self.http_client.get(url);
        if let Some(query) = query {
            request_builder = request_builder.query(&query);
        }

        request_builder
            .send()
            .await
            .map_err(|e| PexelsError::HttpError(format!("{e}")))?
            .error_for_status()
            .map_err(|e| PexelsError::HttpError(format!("{e}")))
    }

    pub(crate) async fn pexels_api_request_data<T: DeserializeOwned>(
        result: Response,
    ) -> PexelsResult<T> {
        result.json::<T>().await.map_err(|e| {
            PexelsError::HttpError(format!("Error while deserializing http response: {e}"))
        })
    }
}
