mod domain;
mod application;
mod data;
mod presentation;
mod infrastructure;

pub mod blog_proto {
    tonic::include_proto!("blog"); 
}

use std::net::SocketAddr;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::{web, App, HttpServer};
use anyhow::Context;
use application::auth_service::AuthService;
use application::blog_service::BlogService;
use data::user_repository::PostgresUserRepository;
use data::post_repository::PostgresPostRepository;
use infrastructure::config::AppConfig;
use infrastructure::database::{create_pool, run_migrations};
use infrastructure::logging::init_logging;
use infrastructure::security::JwtKeys;
use presentation::handlers;
use tonic::transport::Server;

use crate::blog_proto::blog_service_server::BlogServiceServer;
use crate::data::post_repository::PostRepository;
use crate::data::user_repository::UserRepository;
use crate::presentation::grpc::blog_service::BlogGrpcService;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    init_logging();

    let config = AppConfig::from_env().expect("invalid configuration");
    let pool = create_pool(&config.database_url)
        .await
        .expect("failed to connect to database");
    run_migrations(&pool)
        .await
        .expect("failed to run migrations");

    let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));
    let post_repo = Arc::new(PostgresPostRepository::new(pool.clone()));

    let auth_service = AuthService::new(Arc::clone(&user_repo), JwtKeys::new(config.jwt_secret.clone()));
    let blog_service = BlogService::new(Arc::clone(&post_repo));

    let grpc_service = BlogGrpcService::new(auth_service.clone(), blog_service.clone());

    tokio::select!{
        res = run_grpc_server(config.clone(), grpc_service) => res,
        res = run_http_server(config.clone(), auth_service, blog_service) => res
    }.context("Error shutting down server")?;

    Ok(())
}

async fn run_grpc_server<U, P>(config: AppConfig, service: BlogGrpcService<U, P>) -> anyhow::Result<()>
where
    U: UserRepository + Clone + 'static,
    P: PostRepository + Clone + 'static
{
    Server::builder()
        .add_service(BlogServiceServer::new(service))
        .serve(SocketAddr::new(config.host, config.grpc_port))
        .await?;

    Ok(())
}

async fn run_http_server<U, P>(config: AppConfig, auth_service: AuthService<U>, blog_service: BlogService<P>) -> anyhow::Result<()>
where
    U: UserRepository + Clone + 'static,
    P: PostRepository + Clone + 'static
{
    let config_data = config.clone();
    HttpServer::new(move || {
        let cors = build_cors(&config_data);
        App::new()
            .wrap(Logger::default())
            .wrap(DefaultHeaders::new()
                .add(("X-Content-Type-Options", "nosniff"))
                .add(("Referrer-Policy", "no-referrer"))
                .add(("Permissions-Policy", "geolocation=()"))
                .add(("Cross-Origin-Opener-Policy", "same-origin")))
            .wrap(cors)
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(blog_service.clone()))
            // .app_data(web::Data::new(exchange_service.clone()))
            .service(
                web::scope("/api")
                    .service(handlers::auth::scope())
                    .service(handlers::post::scope())
            )
    })
    .bind(SocketAddr::new(config.host, config.http_port))?
    .run()
    .await?;

    Ok(())
}

fn build_cors(config: &AppConfig) -> Cors {
    let mut cors = Cors::default()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allow_any_origin()
        .allow_any_header()
        .supports_credentials()
        .max_age(3600);

    for origin in &config.cors_origins {
        cors = cors.allowed_origin(origin);
    }

    cors
}
