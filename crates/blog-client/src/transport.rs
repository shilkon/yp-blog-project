mod error;
mod http_client;

#[cfg(feature = "grpc")]
mod grpc_client;

use chrono::{DateTime, Utc};
use enum_dispatch::enum_dispatch;
use serde::Deserialize;
use uuid::Uuid;

pub use error::TransportError;
pub use http_client::HttpClient;

#[cfg(feature = "grpc")]
pub use grpc_client::GrpcClient;

#[derive(Debug, Clone, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct PostsResponse {
    pub posts: Vec<Post>,
    pub total: u64,
    pub limit: i64,
    pub offset: i64,
}

#[enum_dispatch(BlogClientTransport)]
pub enum Transport {
    Http(HttpClient),

    #[cfg(feature = "grpc")]
    Grpc(GrpcClient)
}

#[enum_dispatch]
pub trait BlogClientTransport {
    fn set_token(&mut self, token: String);
    fn get_token(&self) -> Option<String>;

    async fn register(&mut self, username: String, email: String, password: String) -> Result<AuthResponse, TransportError>;
    async fn login(&mut self, username: String, password: String) -> Result<AuthResponse, TransportError>;

    async fn get_post(&mut self, id: Uuid) -> Result<Post, TransportError>;
    async fn list_posts(&mut self, limit: i64, offset: i64) -> Result<PostsResponse, TransportError>;

    async fn create_post(&mut self, title: String, content: String) -> Result<Post, TransportError>;
    async fn update_post(&mut self, id: Uuid, title: String, content: String) -> Result<Post, TransportError>;
    async fn delete_post(&mut self, id: Uuid) -> Result<(), TransportError>;
}
