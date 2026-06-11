use tonic::Request;

use super::error::TransportError;
use super::BlogClientTransport;
use crate::blog_proto::RegisterRequest;
use crate::blog_proto::blog_service_client::BlogServiceClient;
use super::dto;

pub struct GrpcClient {
    client: BlogServiceClient<tonic::transport::Channel>,
}

impl GrpcClient {
    pub async fn connect(addr: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = BlogServiceClient::connect(addr).await?;
        Ok(Self { client })
    }
}

impl BlogClientTransport for GrpcClient {
    async fn register(&mut self, username: &str, email: &str, password: &str) -> Result<dto::AuthResponse, TransportError> {
        let request = Request::new(RegisterRequest {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string()
        });

        let response = self.client.register(request).await?;

        todo!()
    }

    async fn login(&mut self, username: &str, password: &str) -> Result<dto::AuthResponse, TransportError> {
        todo!()
    }

    async fn get_post(&mut self, id: uuid::Uuid) -> Result<dto::Post, TransportError> {
        todo!()
    }

    async fn list_posts(&mut self, limit: i64, offset: i64) -> Result<dto::PostsResponse, TransportError> {
        todo!()
    }

    async fn create_post(&mut self, token: String, title: &str, content: &str) -> Result<dto::Post, TransportError> {
        todo!()
    }

    async fn update_post(&mut self, token: String, id: uuid::Uuid, title: &str, content: &str) -> Result<dto::Post, TransportError> {
        todo!()
    }

    async fn delete_post(&mut self, token: String, id: uuid::Uuid) -> Result<(), TransportError> {
        todo!()
    }
}
