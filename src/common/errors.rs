/// Result type for this crate
pub type PexelsResult<T> = Result<T, PexelsError>;

/// Error type for this crate
#[derive(Debug)]
pub enum PexelsError {
    ClientBuildError(String),
    HttpError(String),
}
