use crate::common::errors::PexelsResult;
use serde::{Deserialize, Serialize};

/// Struct for Pexels API stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PexelsRequestStatistics {
    pub x_ratelimit_limit: Option<i64>,
    pub x_ratelimit_remaining: Option<i64>,
    pub x_ratelimit_reset: Option<i64>,
    pub x_runtime: Option<f32>,
    pub x_request_id: Option<String>,
}

pub(crate) fn get_request_stats(
    result: &reqwest::Response,
) -> PexelsResult<PexelsRequestStatistics> {
    let headers = result.headers();
    Ok(PexelsRequestStatistics {
        x_ratelimit_limit: get_header_value::<i64>(headers, "X-Ratelimit-Limit"),
        x_ratelimit_remaining: get_header_value::<i64>(headers, "X-Ratelimit-Remaining"),
        x_ratelimit_reset: get_header_value::<i64>(headers, "X-Ratelimit-Reset"),
        x_runtime: get_header_value::<f32>(headers, "X-Runtime"),
        x_request_id: get_header_value::<String>(headers, "X-Request-Id"),
    })
}

fn get_header_value<T: std::str::FromStr>(
    headers: &reqwest::header::HeaderMap,
    key: &str,
) -> Option<T> {
    headers
        .get(key)
        .map(|value| value.to_str())
        .transpose()
        .unwrap_or_default()
        .map(|value| value.parse::<T>())
        .transpose()
        .unwrap_or_default()
}
