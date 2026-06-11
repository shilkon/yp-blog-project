use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransportError {
    #[error("request failed: {0}")]
    Failed(String),
    #[error("reqwest error: {0}")]
    Reqwest(String),
    #[error("tonic error: {0}")]
    Tonic(String),
}

impl From<reqwest::Error> for TransportError {
    fn from(error: reqwest::Error) -> Self {
        TransportError::Reqwest(error.to_string())
    }
}

impl From<tonic::Status> for TransportError {
    fn from(status: tonic::Status) -> Self {
        TransportError::Tonic(status.to_string())
    }
}
