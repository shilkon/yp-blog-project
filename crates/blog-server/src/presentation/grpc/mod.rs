use crate::{blog_proto, domain::{self, user::AuthenticatedUser}};

pub mod blog_service;

pub trait RequestAuthExtractor<T: Sync> {
    async fn extract_user(&self, request: &tonic::Request<T>) -> Result<AuthenticatedUser, tonic::Status>;
}

impl From<domain::user::User> for blog_proto::User {
    fn from(user: domain::user::User) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            password_hash: user.password_hash,
            created_at: user.created_at.timestamp_millis()
        }
    }
}

impl From<domain::post::Post> for blog_proto::Post {
    fn from(post: domain::post::Post) -> Self {
        Self {
            id: post.id.to_string(),
            author_id: post.author_id.to_string(),
            title: post.title,
            content: post.content,
            created_at: post.created_at.timestamp_millis(),
            updated_at: post.updated_at.timestamp_millis(),
        }
    }
}
