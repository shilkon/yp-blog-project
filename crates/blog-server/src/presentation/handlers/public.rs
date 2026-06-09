use actix_web::{HttpResponse, Responder, Scope, get, post, web};
use tracing::info;
use uuid::Uuid;

use crate::application::auth_service::AuthService;
use crate::application::blog_service::BlogService;
use crate::data::post_repository::PostgresPostRepository;
use crate::data::user_repository::PostgresUserRepository;
use crate::domain::error::BlogError;
use crate::presentation::dto::{GetPostsRequest, LoginRequest, PostResponse, PostsResponse, RegisterRequest, UserResponse};

pub fn scope() -> Scope {
    web::scope("")
        .service(
            web::scope("/auth")
                .service(register)
                .service(login)
        )
        .service(get_post)
        .service(list_posts)
}

#[post("/register")]
async fn register(
    service: web::Data<AuthService<PostgresUserRepository>>,
    payload: web::Json<RegisterRequest>,
) -> Result<impl Responder, BlogError> {
    let user = service
        .register(payload.username.clone(), payload.email.clone(), payload.password.clone())
        .await?;

    info!(user_id = %user.id, username = %user.username, email = %user.email, "user registered");

    let (token, user) = service.login(&payload.email, &payload.password).await?;
    
    info!(username = %user.username, "user logged in");

    Ok(HttpResponse::Created().json(UserResponse {
        token,
        user
    }))
}

#[post("/login")]
async fn login(
    service: web::Data<AuthService<PostgresUserRepository>>,
    payload: web::Json<LoginRequest>,
) -> Result<impl Responder, BlogError> {
    let (token, user) = service.login(&payload.username, &payload.password).await?;
    info!(username = %payload.username, "user logged in");
    Ok(HttpResponse::Ok().json(UserResponse {
        token,
        user
    }))
}

#[get("/posts")]
async fn get_post(
    service: web::Data<BlogService<PostgresPostRepository>>,
    path: web::Path<Uuid>
) -> Result<HttpResponse, BlogError> {
    let id = path.into_inner();
    let post = service.get_post(id).await?;

    info!(
        post_id = %post.id.to_string(),
        "post fetched"
    );

    Ok(HttpResponse::Ok().json(PostResponse::from(post)))
}

#[get("/posts")]
async fn list_posts(
    service: web::Data<BlogService<PostgresPostRepository>>,
    query: web::Query<GetPostsRequest>
) -> Result<HttpResponse, BlogError> {
    let limit = query.limit;
    let offset = query.offset;
    let posts = service.list_posts(limit, offset).await?;

    info!(
        limit = %limit,
        offset = %offset,
        "posts fetched"
    );

    let total = posts.len() as u64;
    Ok(HttpResponse::Ok().json(PostsResponse {
        posts,
        total,
        limit,
        offset
    }))
}
