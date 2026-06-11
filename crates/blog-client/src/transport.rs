mod error;
mod dto;
mod http_client;
mod grpc_client;

use error::TransportError;

use dto::{AuthResponse, Post, PostsResponse};
use uuid::Uuid;



pub trait BlogClientTransport {
    async fn register(&mut self, username: &str, email: &str, password: &str) -> Result<AuthResponse, TransportError>;
    async fn login(&mut self, username: &str, password: &str) -> Result<AuthResponse, TransportError>;

    async fn get_post(&mut self, id: Uuid) -> Result<Post, TransportError>;
    async fn list_posts(&mut self, limit: i64, offset: i64) -> Result<PostsResponse, TransportError>;

    async fn create_post(&mut self, token: String, title: &str, content: &str) -> Result<Post, TransportError>;
    async fn update_post(&mut self, token: String, id: Uuid, title: &str, content: &str) -> Result<Post, TransportError>;
    async fn delete_post(&mut self, token: String, id: Uuid) -> Result<(), TransportError>;
}
