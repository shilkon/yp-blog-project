use reqwest::Client;
use tracing::error;

use super::error::TransportError;
use super::BlogClientTransport;
use super::dto::{RegisterRequest, LoginRequest, ModifyPostRequest, AuthResponse, Post, PostsResponse};

struct HttpClient {
    client: Client,
    base_url: String
}

impl HttpClient {
    pub fn new(client: Client, base_url: String) -> Self {
        Self { client, base_url }
    }
}

impl BlogClientTransport for HttpClient {
    async fn register(&mut self, username: &str, email: &str, password: &str) -> Result<AuthResponse, TransportError> {
        let body = RegisterRequest {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string()
        };

        let resp = self.client
            .post(format!("{}/auth/register", self.base_url))
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            error!("failed to register: {}", resp.status());
            return Err(TransportError::Failed(resp.status().to_string()));
        }

        Ok(resp.json().await?)
    }

    async fn login(&mut self, username: &str, password: &str) -> Result<AuthResponse, TransportError> {
        let body = LoginRequest {
            username: username.to_string(),
            password: password.to_string()
        };

        let resp = self.client
            .post(format!("{}/auth/login", self.base_url))
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
            .get(format!("{}/posts/{}", self.base_url, id))
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
            .get(format!("{}/posts?limit={}&offset{}", self.base_url, limit, offset))
            .send()
            .await?;

        if !resp.status().is_success() {
            error!("failed to get posts: {}", resp.status());
            return Err(TransportError::Failed(resp.status().to_string()));
        }

        Ok(resp.json().await?)
    }

    async fn create_post(&mut self, token: String, title: &str, content: &str) -> Result<Post, TransportError> {
        let body = ModifyPostRequest {
            title: title.to_string(),
            content: content.to_string()
        };

        let resp = self.client
            .post(format!("{}/posts", self.base_url))
            .bearer_auth(token)
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            error!("failed to create post: {}", resp.status());
            return Err(TransportError::Failed(resp.status().to_string()));
        }

        Ok(resp.json().await?)
    }

    async fn update_post(&mut self, token: String, id: uuid::Uuid, title: &str, content: &str) -> Result<Post, TransportError> {
        let body = ModifyPostRequest {
            title: title.to_string(),
            content: content.to_string()
        };

        let resp = self.client
            .put(format!("{}/posts/{}", self.base_url, id))
            .bearer_auth(token)
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            error!("failed to update post: {}", resp.status());
            return Err(TransportError::Failed(resp.status().to_string()));
        }

        Ok(resp.json().await?)
    }

    async fn delete_post(&mut self, token: String, id: uuid::Uuid) -> Result<(), TransportError> {
        let resp = self.client
            .delete(format!("{}/posts/{}", self.base_url, id))
            .bearer_auth(token)
            .send()
            .await?;

        if !resp.status().is_success() {
            error!("failed to delete post: {}", resp.status());
            return Err(TransportError::Failed(resp.status().to_string()));
        }

        Ok(())
    }
}
