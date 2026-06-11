use actix_web::{HttpResponse, Scope, delete, get, post, put, web};
use actix_web_httpauth::middleware::HttpAuthentication;
use tracing::info;
use uuid::Uuid;

use crate::domain::error::BlogError;
use crate::application::blog_service::BlogService;
// use crate::application::exchange_service::ExchangeService;
use crate::data::post_repository::PostgresPostRepository;
use crate::domain::user::AuthenticatedUser;
use crate::presentation::middleware::jwt_validator;
use crate::presentation::dto::{
    CreatePostRequest, GetPostsRequest, PostsResponse, UpdatePostRequest
};

pub fn scope() -> Scope {
    web::scope("/posts")
        .service(get_post)
        .service(list_posts)
        .service(
            web::scope("")
                .wrap(HttpAuthentication::bearer(jwt_validator))
                .service(create_post)
                .service(update_post)
                .service(delete_post)
        )
}

#[get("/{id}")]
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

    Ok(HttpResponse::Ok().json(post))
}

#[get("")]
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

#[post("")]
async fn create_post(
    service: web::Data<BlogService<PostgresPostRepository>>,
    user: AuthenticatedUser,
    payload: web::Json<CreatePostRequest>
) -> Result<HttpResponse, BlogError> {
    let post = service
        .create_post(user.user_id, &payload.title, &payload.content)
        .await?;

    info!(
        user_id = %user.user_id.to_string(),
        post_id = %post.id.to_string(),
        "post created"
    );

    Ok(HttpResponse::Created().json(post))
}

#[put("/{id}")]
async fn update_post(
    service: web::Data<BlogService<PostgresPostRepository>>,
    path: web::Path<Uuid>,
    user: AuthenticatedUser,
    payload: web::Json<UpdatePostRequest>
) -> Result<HttpResponse, BlogError> {
    let id = path.into_inner();
    let req = payload.into_inner();
    let post = service.update_post(user.user_id, id, &req.title, &req.content).await?;

    info!(
        user_id = %user.user_id.to_string(),
        post_id = %post.id.to_string(),
        "post updated"
    );

    Ok(HttpResponse::Ok().json(post))
}

#[delete("{id}")]
async fn delete_post(
    service: web::Data<BlogService<PostgresPostRepository>>,
    path: web::Path<Uuid>,
    user: AuthenticatedUser
) -> Result<HttpResponse, BlogError> {
    let id = path.into_inner();
    service.delete_post(user.user_id, id).await?;

    info!(
        user_id = %user.user_id.to_string(),
        post_id = %id.to_string(),
        "post deleted"
    );

    Ok(HttpResponse::NoContent().finish())
}
