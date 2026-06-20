use sqlx::PgPool;
use tracing::error;
use uuid::Uuid;

use crate::domain::{user::User, error::DomainError};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<User, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
}

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: PgPool
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: User) -> Result<User, DomainError> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, username, email, password_hash)
            VALUES ($1, $2, $3, $4)
            RETURNING id, username, email, password_hash, created_at
            "#,
            user.id,
            user.username,
            user.email,
            user.password_hash
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("failed to create user: {}", e);
            if let Some(err) = e.as_database_error().and_then(|db| db.constraint()) {
                DomainError::Validation(format!("{} already registered", 
                    if err.contains("users_username") {
                        "username"
                    } else if err.contains("users_email") {
                        "email"
                    } else {
                        "user"
                    }
                ))
            } else {
                DomainError::Internal(format!("database error: {}", e))
            }
        })
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, created_at
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("failed to find user by username {}: {}", username, e);
            DomainError::Internal(format!("database error: {}", e))
        })
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, created_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("failed to find user by email {}: {}", email, e);
            DomainError::Internal(format!("database error: {}", e))
        })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, created_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("failed to find user by id {}: {}", id, e);
            DomainError::Internal(format!("database error: {}", e))
        })
    }
}

