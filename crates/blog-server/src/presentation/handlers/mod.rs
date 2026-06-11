pub mod auth;
pub mod post;

use std::pin::Pin;

use actix_web::{HttpMessage, HttpRequest, dev::Payload, error::{ErrorInternalServerError, ErrorUnauthorized}, web};
use uuid::Uuid;

use crate::{
    application::auth_service::AuthService,
    data::user_repository::PostgresUserRepository,
    domain::user::{AuthenticatedUser, Claims}
};

impl actix_web::FromRequest for AuthenticatedUser {
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
    
    fn extract(req: &HttpRequest) -> Self::Future {
        Self::from_request(req, &mut actix_web::dev::Payload::None)
    }
}

