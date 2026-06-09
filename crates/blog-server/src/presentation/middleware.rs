use std::pin::Pin;

use actix_web::dev::{Payload, ServiceRequest};
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::{Error, FromRequest, HttpMessage, HttpRequest, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use uuid::Uuid;
use tracing::info;

use crate::application::auth_service::AuthService;
use crate::data::user_repository::PostgresUserRepository;
use crate::infrastructure::security::Claims;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub username: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let Some(auth_service) = req
            .app_data::<web::Data<AuthService<PostgresUserRepository>>>()
            .cloned()
        else {
            return Box::pin(async { Err(ErrorInternalServerError("AuthService missing")) });
        };

        let extensions = req.extensions();
        let Some(claims) = extensions.get::<Claims>() else {
            return Box::pin(async { Err(ErrorUnauthorized("Authentication required")) });
        };

        let Ok(user_id) = Uuid::parse_str(&claims.user_id) else {
            return Box::pin(async { Err(ErrorUnauthorized("Invalid user identity format")) })
        };

        Box::pin(async move {
            match auth_service.get_user(user_id).await {
                Ok(Some(user)) => Ok(AuthenticatedUser{
                    user_id: user.id,
                    username: user.username
                }),
                Ok(None) => Err(ErrorUnauthorized("User account no longer exists")),
                Err(_) => Err(ErrorInternalServerError("Database query failure")),
            }
        })
    }
}

pub async fn jwt_validator(
    req: ServiceRequest,
    credentials: BearerAuth
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let Some(auth_service) = req
        .app_data::<web::Data<AuthService<PostgresUserRepository>>>()
        .cloned()
    else {
        return Err((ErrorInternalServerError("AuthService missing"), req));
    };

    match auth_service.keys().verify_token(credentials.token()) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        },
        Err(_) => Err((ErrorUnauthorized("Invalid token"), req))
    }
}
