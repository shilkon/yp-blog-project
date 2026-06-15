use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransportError {
    #[error("request failed: {0}")]
    Failed(String),

    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[cfg(feature = "grpc")]
    #[error("tonic error: {0}")]
    Tonic(#[from] tonic::Status),

    #[error("invalid response: {0}")]
    Response(String),

    #[error("invalid token")]
    Token
}
