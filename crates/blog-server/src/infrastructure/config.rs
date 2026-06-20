use std::net::IpAddr;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub host: IpAddr,
    pub http_port: u16,
    pub grpc_port: u16,
    pub database_url: String,
    pub jwt_secret: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let http_port = std::env::var("HTTP_PORT")
            .unwrap_or_else(|_| "8080".into())
            .parse()
            .map_err(|e| anyhow::anyhow!("invalid HTTP_PORT: {}", e))?;
        let grpc_port = std::env::var("GRPC_PORT")
            .unwrap_or_else(|_| "50051".into())
            .parse()
            .map_err(|e| anyhow::anyhow!("invalid GRPC_PORT: {}", e))?;
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| anyhow::anyhow!("DATABASE_URL must be set"))?;
        let jwt_secret = std::env::var("JWT_SECRET")
            .map_err(|_| anyhow::anyhow!("JWT_SECRET must be set"))?;

        Ok(Self {
            host: host.parse()?,
            http_port,
            grpc_port,
            database_url,
            jwt_secret,
        })
    }
}

