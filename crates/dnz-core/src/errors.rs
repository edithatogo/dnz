use std::time::Duration;
use thiserror::Error;

/// Stable, non-secret failures returned by DigitalNZ requests.
#[derive(Debug, Error)]
pub enum DnzError {
    #[error("DigitalNZ returned HTTP status {status} after retries")]
    HttpStatus {
        status: u16,
        retry_after: Option<Duration>,
    },
    #[error("DigitalNZ response could not be decoded")]
    Decode,
    #[error("DigitalNZ request failed during transport")]
    Transport,
    #[error("DigitalNZ response format is not supported: {format}")]
    UnsupportedFormat { format: String },
}

impl DnzError {
    pub fn status(&self) -> Option<u16> {
        match self {
            Self::HttpStatus { status, .. } => Some(*status),
            _ => None,
        }
    }

    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            Self::HttpStatus { retry_after, .. } => *retry_after,
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structured_http_error_preserves_status_and_retry_after() {
        let error = DnzError::HttpStatus {
            status: 429,
            retry_after: Some(Duration::from_secs(60)),
        };
        assert_eq!(error.status(), Some(429));
        assert_eq!(error.retry_after(), Some(Duration::from_secs(60)));
    }
}
