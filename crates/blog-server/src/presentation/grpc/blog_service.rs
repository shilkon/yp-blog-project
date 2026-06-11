use tonic::{Request, Response, Status};
use tracing::info;
use uuid::Uuid;

use crate::application::{auth_service, blog_service};
use crate::data::{
    post_repository::PostRepository,
    user_repository::UserRepository
};
use crate::blog_proto::{
    blog_service_server,
    CreatePostRequest,
    LoginRequest,
    Post,
    PostIdRequest,
    RegisterRequest,
    UpdatePostRequest,
    AuthResponse,
    DeletePostResponse,
    ListPostsRequest,
    ListPostsResponse
};
use crate::domain::user::AuthenticatedUser;
use crate::presentation::grpc::RequestAuthExtractor;

#[derive(Clone)]
pub struct BlogGrpcService<U, P>
where
    U: UserRepository + 'static,
    P: PostRepository + 'static
{
    pub auth_service: auth_service::AuthService<U>,
    pub blog_service: blog_service::BlogService<P>
}

impl<U, P> BlogGrpcService<U, P>
where
    U: UserRepository + 'static,
    P: PostRepository + 'static
{
    pub fn new(auth_service: auth_service::AuthService<U>, blog_service: blog_service::BlogService<P>) -> Self {
        Self {
            auth_service,
            blog_service
        }
    }
}

impl<T, U, P> super::RequestAuthExtractor<T> for BlogGrpcService<U, P>
where
    T: Sync,
    U: UserRepository + 'static,
    P: PostRepository + 'static,
{
    async fn extract_user(&self, request: &Request<T>) -> Result<AuthenticatedUser, Status> {
        let auth_header = request
            .metadata()
            .get("authorization")
            .ok_or_else(|| Status::unauthenticated("Missing authorization metadata"))?
            .to_str()
            .map_err(|_| Status::unauthenticated("Malformed authorization string"))?;

        let Some(token) = auth_header.strip_prefix("Bearer ") else {
            return Err(Status::unauthenticated("Authorization must use Bearer schema"));
        };

        let claims = self.auth_service
            .keys()
            .verify_token(token)
            .map_err(|_| Status::unauthenticated("Invalid or expired token"))?;

        let user_id = Uuid::parse_str(&claims.user_id)
            .map_err(|_| Status::unauthenticated("Invalid user identity format"))?;

        match self.auth_service.get_user(user_id).await {
            Ok(Some(user)) => Ok(AuthenticatedUser {
                user_id: user.id,
                username: user.username,
            }),
            Ok(None) => Err(Status::unauthenticated("User account no longer exists")),
            Err(_) => Err(Status::internal("Database query failure")),
        }
    }
}

#[tonic::async_trait]
impl<U, P> blog_service_server::BlogService for BlogGrpcService<U, P>
where
    U: UserRepository + 'static,
    P: PostRepository + 'static
{
    async fn register(
        &self,
        request: Request<RegisterRequest>
    ) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();
        let user = self.auth_service
            .register(&req.username, &req.email, &req.password)
            .await?;

        info!(user_id = %user.id, username = %user.username, email = %user.email, "user registered");

        let (token, user) = self.auth_service.login(&req.username, &req.password).await?;
        
        info!(username = %user.username, "user logged in");

        Ok(Response::new(AuthResponse {
            token,
            user: Some(user.into())
        }))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>
    ) ->  Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();

        let (token, user) = self.auth_service.login(&req.username, &req.password).await?;

        info!(username = %req.username, "user logged in");

        Ok(Response::new(AuthResponse {
            token,
            user: Some(user.into())
        }))
    }

    async fn create_post(
        &self,
        request: Request<CreatePostRequest>
    ) ->  Result<Response<Post>, Status> {
        let user = self.extract_user(&request).await?;

        let req = request.into_inner();
        let post = self.blog_service
            .create_post(user.user_id, &req.title, &req.content)
            .await?;

        info!(
            user_id = %user.user_id.to_string(),
            post_id = %post.id.to_string(),
            "post created"
        );

        Ok(Response::new(post.into()))
    }

    async fn get_post(
        &self,
        request: Request<PostIdRequest>
    ) ->  Result<Response<Post>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|e| Status::invalid_argument(format!("id: {e}")))?;

        let post = self.blog_service.get_post(id).await?;

        info!(
            post_id = %post.id.to_string(),
            "post fetched"
        );

        Ok(Response::new(post.into()))
    }

    async fn update_post(
        &self,
        request: Request<UpdatePostRequest>
    ) ->  Result<Response<Post>, Status> {
        let user = self.extract_user(&request).await?;

        let req = request.into_inner();
        let id = Uuid::parse_str(&req.id)
            .map_err(|e| Status::invalid_argument(format!("id: {e}")))?;

        let post = self.blog_service.update_post(user.user_id, id, &req.title, &req.content).await?;

        info!(
            user_id = %user.user_id.to_string(),
            post_id = %post.id.to_string(),
            "post updated"
        );

        Ok(Response::new(post.into()))
    }

    async fn delete_post(
        &self,
        request: Request<PostIdRequest>
    ) ->  Result<Response<DeletePostResponse>, Status> {
        let user = self.extract_user(&request).await?;

        let req = request.into_inner();
        let id = Uuid::parse_str(&req.id)
            .map_err(|e| Status::invalid_argument(format!("id: {e}")))?;
        self.blog_service.delete_post(user.user_id, id).await?;

        info!(
            user_id = %user.user_id.to_string(),
            post_id = %id.to_string(),
            "post deleted"
        );

        Ok(Response::new(DeletePostResponse{}))
    }

    async fn list_posts(
        &self,
        request: Request<ListPostsRequest>
    ) ->  Result<Response<ListPostsResponse>, Status> {
        let req = request.into_inner();
        let posts = self.blog_service.list_posts(req.limit, req.offset).await?;

        info!(
            limit = %req.limit,
            offset = %req.offset,
            "posts fetched"
        );

        let total = posts.len() as u64;
        Ok(Response::new(ListPostsResponse {
            posts: posts.into_iter().map(|p| p.into()).collect(),
            total,
            limit: req.limit,
            offset: req.offset
        }))
    }
}
