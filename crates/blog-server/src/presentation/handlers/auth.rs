use actix_web::{HttpResponse, Responder, Scope, post, web};
use tracing::info;

use crate::application::auth_service::AuthService;
use crate::data::user_repository::PostgresUserRepository;
use crate::domain::error::BlogError;
use crate::presentation::dto::{LoginRequest, RegisterRequest, AuthResponse};

pub fn scope() -> Scope {
    web::scope("/auth")
        .service(register)
        .service(login)
}

#[post("/register")]
async fn register(
    service: web::Data<AuthService<PostgresUserRepository>>,
    payload: web::Json<RegisterRequest>,
) -> Result<impl Responder, BlogError> {
    let user = service
        .register(&payload.username, &payload.email, &payload.password)
        .await?;

    info!(user_id = %user.id, username = %user.username, email = %user.email, "user registered");

    let (token, user) = service.login(&payload.username, &payload.password).await?;
    
    info!(username = %user.username, "user logged in");

    Ok(HttpResponse::Created().json(AuthResponse {
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
    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user
    }))
}
