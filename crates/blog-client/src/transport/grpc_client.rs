use chrono::DateTime;
use tonic::Request;
use tonic::metadata::MetadataMap;
use uuid::Uuid;

use super::error::TransportError;
use super::BlogClientTransport;
use crate::blog_proto;
use crate::blog_proto::blog_service_client::BlogServiceClient;

pub struct GrpcClient {
    client: BlogServiceClient<tonic::transport::Channel>,
}

impl GrpcClient {
    pub async fn connect(addr: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = BlogServiceClient::connect(addr).await?;
        Ok(Self { client })
    }

    fn add_token(meta: &mut MetadataMap, token: &str) -> Result<(), TransportError> {
        meta.insert(
            "authorization",
            format!("Bearer {token}")
                .parse()
                .map_err(|_| TransportError::Token)?
        );

        Ok(())
    }
}

impl BlogClientTransport for GrpcClient {
    async fn register(&mut self, username: &str, email: &str, password: &str) -> Result<super::AuthResponse, TransportError> {
        let request = Request::new(blog_proto::RegisterRequest {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string()
        });

        let response = self.client.register(request).await?;
        response.into_inner().try_into()
    }

    async fn login(&mut self, username: &str, password: &str) -> Result<super::AuthResponse, TransportError> {
        let request = Request::new(blog_proto::LoginRequest {
            username: username.to_string(),
            password: password.to_string()
        });

        let response = self.client.login(request).await?;
        response.into_inner().try_into()
    }

    async fn get_post(&mut self, id: uuid::Uuid) -> Result<super::Post, TransportError> {
        let request = Request::new( blog_proto::PostIdRequest {
            id: id.to_string()
        });

        let response = self.client.get_post(request).await?;
        response.into_inner().try_into()
    }

    async fn list_posts(&mut self, limit: i64, offset: i64) -> Result<super::PostsResponse, TransportError> {
        let request = Request::new( blog_proto::ListPostsRequest {
            limit, offset
        });

        let response = self.client.list_posts(request).await?;
        response.into_inner().try_into()
    }

    async fn create_post(&mut self, token: &str, title: &str, content: &str) -> Result<super::Post, TransportError> {
        let mut request = Request::new(blog_proto::CreatePostRequest {
            title: title.to_string(),
            content: content.to_string()
        });
        Self::add_token(request.metadata_mut(), &token)?;

        let response = self.client.create_post(request).await?;
        response.into_inner().try_into()
    }

    async fn update_post(&mut self, token: &str, id: uuid::Uuid, title: &str, content: &str) -> Result<super::Post, TransportError> {
        let mut request = Request::new(blog_proto::UpdatePostRequest {
            id: id.to_string(),
            title: title.to_string(),
            content: content.to_string()
        });
        Self::add_token(request.metadata_mut(), &token)?;

        let response = self.client.update_post(request).await?;
        response.into_inner().try_into()
    }

    async fn delete_post(&mut self, token: &str, id: uuid::Uuid) -> Result<(), TransportError> {
        let mut request = Request::new(blog_proto::PostIdRequest {
            id: id.to_string(),
        });
        Self::add_token(request.metadata_mut(), &token)?;

        self.client.delete_post(request).await?;
        Ok(())
    }
}

impl TryFrom<blog_proto::AuthResponse> for super::AuthResponse {
    type Error = TransportError;

    fn try_from(resp: blog_proto::AuthResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            token: resp.token,
            user: resp.user
                .ok_or_else(|| TransportError::Response("Missed user".to_string()))?
                .try_into()?
        })
    }
}

impl TryFrom<blog_proto::User> for super::User {
    type Error = TransportError;

    fn try_from(user: blog_proto::User) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::parse_str(&user.id)
                .map_err(|_| TransportError::Response("Invalid id".to_string()))?,
            username: user.username,
            email: user.email,
            password_hash: user.password_hash,
            created_at: DateTime::from_timestamp_millis(user.created_at)
                .ok_or_else(|| TransportError::Response("Invalid timestamp".to_string()))?
        })
    }
}

impl TryFrom<blog_proto::Post> for super::Post {
    type Error = TransportError;

    fn try_from(post: blog_proto::Post) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::parse_str(&post.id)
                .map_err(|_| TransportError::Response("Invalid id".to_string()))?,
            author_id: Uuid::parse_str(&post.author_id)
                .map_err(|_| TransportError::Response("Invalid id".to_string()))?,
            title: post.title,
            content: post.content,
            created_at: DateTime::from_timestamp_millis(post.created_at)
                .ok_or_else(|| TransportError::Response("Invalid timestamp".to_string()))?,
            updated_at: DateTime::from_timestamp_millis(post.updated_at)
                .ok_or_else(|| TransportError::Response("Invalid timestamp".to_string()))?
        })
    }
}

impl TryFrom<blog_proto::ListPostsResponse> for super::PostsResponse {
    type Error = TransportError;

    fn try_from(resp: blog_proto::ListPostsResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            posts: resp.posts.into_iter()
                .map(TryInto::<super::Post>::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            total: resp.total,
            limit: resp.limit,
            offset: resp.offset
        })
    }
}
