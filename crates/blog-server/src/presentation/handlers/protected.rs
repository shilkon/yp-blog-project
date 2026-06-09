use actix_web::{HttpMessage, HttpRequest, HttpResponse, Scope, delete, get, post, put, web};
use tracing::info;
use uuid::Uuid;

use crate::domain::error::BlogError;
use crate::application::blog_service::BlogService;
// use crate::application::exchange_service::ExchangeService;
use crate::data::post_repository::PostgresPostRepository;
use crate::presentation::middleware::AuthenticatedUser;
use crate::presentation::dto::{
    CreatePostRequest, PostResponse, UpdatePostRequest
};

pub fn scope() -> Scope {
    web::scope("/posts")
        .service(create_post)
        .service(update_post)
        .service(delete_post)
}

// fn ensure_owner(account: &AccountResponse, user: &AuthenticatedUser) -> Result<(), BankError> {
//     if account.owner_id != user.id {
//         Err(BankError::Unauthorized)
//     } else {
//         Ok(())
//     }
// }

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

    Ok(HttpResponse::Created().json(PostResponse::from(post)))
}

#[put("")]
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

    Ok(HttpResponse::Ok().json(PostResponse::from(post)))
}

#[delete("")]
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
