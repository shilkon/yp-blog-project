use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{post::Post, user::User};

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub title: String,
    pub content: String,
}

const DEFAULT_LIMIT: i64 = 10;
const DEFAULT_OFFSET: i64 = 0;

#[derive(Debug, Deserialize)]
pub struct GetPostsRequest {
    #[serde(default = "default_limit")]
    pub limit: i64,

    #[serde(default = "default_offset")]
    pub offset: i64
}

fn default_limit() -> i64 { DEFAULT_LIMIT }
fn default_offset() -> i64 { DEFAULT_OFFSET }

#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Post> for PostResponse {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            author_id: post.author_id,
            title: post.title,
            content: post.content,
            created_at: post.created_at,
            updated_at: post.updated_at
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PostsResponse {
    pub posts: Vec<Post>,
    pub total: u64,
    pub limit: i64,
    pub offset: i64,
}
