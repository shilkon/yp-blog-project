use serde::{Deserialize, Serialize};

use crate::domain::{post::Post, user::User};

#[derive(Debug, Serialize)]
pub struct AuthResponse {
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
pub struct ModifyPostRequest {
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
pub struct PostsResponse {
    pub posts: Vec<Post>,
    pub total: u64,
    pub limit: i64,
    pub offset: i64,
}
