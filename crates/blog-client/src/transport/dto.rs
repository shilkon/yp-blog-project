use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::blog_proto;

#[derive(Debug, Serialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct ModifyPostRequest {
    pub title: String,
    pub content: String,
}

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

// impl From<blog_proto::AuthResponse> for AuthResponse {
//     fn from(resp: blog_proto::AuthResponse) -> Self {
//         Self {
//             token: resp.token,
//             user: resp.user.into()
//         }
//     }
// }

// impl Ffrom<blog_proto::User> for User {
//     fn from(user: blog_proto::User) -> Self {
//         Self {
//             id: user.id,

//         }
//     }
// }
