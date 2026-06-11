use actix_web::dev::ServiceRequest;
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::{HttpMessage, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::application::auth_service::AuthService;
use crate::data::user_repository::PostgresUserRepository;

pub async fn jwt_validator(
    req: ServiceRequest,
    credentials: BearerAuth
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
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
