use reqwest::Client;
use serde::Serialize;
use tracing::error;

use super::error::TransportError;
use super::BlogClientTransport;
use super::{AuthResponse, Post, PostsResponse};

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

pub struct HttpClient {
    client: Client,
    url: String,
    token: Option<String>
}

impl HttpClient {
    pub fn new(url: String) -> reqwest::Result<Self> {
        Ok(Self {
            client: Client::builder().build()?,
            url,
            token: None
        })
    }
}

impl BlogClientTransport for HttpClient {
    fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    fn get_token(&self) -> Option<String> {
        self.token.clone()
    }

    async fn register(&mut self, username: String, email: String, password: String) -> Result<AuthResponse, TransportError> {
        let body = RegisterRequest { username, email, password };

        let resp = self.client
            .post(format!("{}/auth/register", self.url))
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            error!("failed to register: {}", resp.status());
            return Err(TransportError::Failed(resp.status().to_string()));
        }

        Ok(resp.json().await?)
    }

    async fn login(&mut self, username: String, password: String) -> Result<AuthResponse, TransportError> {
        let body = LoginRequest { username, password };

        let resp = self.client
            .post(format!("{}/auth/login", self.url))
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            error!("failed to login: {}", resp.status());
            return Err(TransportError::Failed(resp.status().to_string()));
        }

        Ok(resp.json().await?)
    }

    async fn get_post(&mut self, id: uuid::Uuid) -> Result<Post, TransportError> {
        let resp = self.client
            .get(format!("{}/posts/{}", self.url, id))
            .send()
            .await?;

        if !resp.status().is_success() {
            error!("failed to get post: {}", resp.status());
            return Err(TransportError::Failed(resp.status().to_string()));
        }

        Ok(resp.json().await?)
    }

    async fn list_posts(&mut self, limit: i64, offset: i64) -> Result<PostsResponse, TransportError> {
        let resp = self.client
            .get(format!("{}/posts?limit={}&offset{}", self.url, limit, offset))
            .send()
            .await?;

        if !resp.status().is_success() {
            error!("failed to get posts: {}", resp.status());
            return Err(TransportError::Failed(resp.status().to_string()));
        }

        Ok(resp.json().await?)
    }

    async fn create_post(&mut self, title: String, content: String) -> Result<Post, TransportError> {
        let body = ModifyPostRequest { title, content };

        let resp = self.client
            .post(format!("{}/posts", self.url))
            .bearer_auth(self.token.as_ref().ok_or_else(|| TransportError::Token)?)
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            error!("failed to create post: {}", resp.status());
            return Err(TransportError::Failed(resp.status().to_string()));
        }

        Ok(resp.json().await?)
    }

    async fn update_post(&mut self, id: uuid::Uuid, title: String, content: String) -> Result<Post, TransportError> {
        let body = ModifyPostRequest { title, content };

        let resp = self.client
            .put(format!("{}/posts/{}", self.url, id))
            .bearer_auth(self.token.as_ref().ok_or_else(|| TransportError::Token)?)
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            error!("failed to update post: {}", resp.status());
            return Err(TransportError::Failed(resp.status().to_string()));
        }

        Ok(resp.json().await?)
    }

    async fn delete_post(&mut self, id: uuid::Uuid) -> Result<(), TransportError> {
        let resp = self.client
            .delete(format!("{}/posts/{}", self.url, id))
            .bearer_auth(self.token.as_ref().ok_or_else(|| TransportError::Token)?)
            .send()
            .await?;

        if !resp.status().is_success() {
            error!("failed to delete post: {}", resp.status());
            return Err(TransportError::Failed(resp.status().to_string()));
        }

        Ok(())
    }
}
