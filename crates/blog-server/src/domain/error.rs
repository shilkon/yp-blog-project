use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use uuid::Uuid;
use tonic::Status;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum DomainError {
    #[error("validation failed: {0}")]
    Validation(String),
    #[error("user not found: {0}")]
    UserNotFound(Uuid),
    #[error("user already exists: {0}")]
    UserAlreadyExists(Uuid),
    // #[error("invalid credentials")]
    // InvalidCredentials(),
    #[error("post not found: {0}")]
    PostNotFound(Uuid),
    // #[error("forbidden")]
    // Forbidden(),
    #[error("internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Error)]
pub enum BlogError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden: {0}")]
    Forbidden(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("internal server error: {0}")]
    Internal(String),
}

#[derive(Serialize)]
struct ErrorBody<'a> {
    error: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
}

impl ResponseError for BlogError {
    fn status_code(&self) -> StatusCode {
        match self {
            BlogError::Validation(_) => StatusCode::BAD_REQUEST,
            BlogError::Unauthorized => StatusCode::UNAUTHORIZED,
            BlogError::Forbidden(_) => StatusCode::FORBIDDEN,
            BlogError::NotFound(_) => StatusCode::NOT_FOUND,
            BlogError::Conflict(_) => StatusCode::CONFLICT,
            BlogError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let message = self.to_string();
        let details = match self {
            BlogError::Validation(msg) => Some(json!({ "message": msg })),
            BlogError::Unauthorized => None,
            BlogError::Forbidden(resource) => Some(json!({ "resource": resource })),
            BlogError::NotFound(resource) => Some(json!({ "resource": resource })),
            BlogError::Conflict(resource) => Some(json!({ "resource": resource })),
            BlogError::Internal(_) => None,
        };
        let body = ErrorBody {
            error: &message,
            details,
        };
        HttpResponse::build(self.status_code()).json(body)
    }
}

impl From<DomainError> for BlogError {
    fn from(value: DomainError) -> Self {
        match value {
            DomainError::Validation(msg) => BlogError::Validation(msg),
            DomainError::UserNotFound(id) => BlogError::NotFound(format!("user {}", id)),
            DomainError::UserAlreadyExists(id) => BlogError::Conflict(format!("user {}", id)),
            DomainError::PostNotFound(id) => BlogError::NotFound(format!("post {}", id)),
            DomainError::Internal(msg) => BlogError::Internal(msg),
        }
    }
}

impl From<BlogError> for Status {
    fn from(value: BlogError) -> Self {
        match value {
            BlogError::Validation(msg) => Status::invalid_argument(msg),
            BlogError::Unauthorized => Status::unauthenticated(""),
            BlogError::Forbidden(resource) => Status::permission_denied(resource),
            BlogError::NotFound(resource) => Status::not_found(resource),
            BlogError::Conflict(resource) => Status::failed_precondition(resource),
            BlogError::Internal(msg) => Status::internal(msg),
        }
    }
}

