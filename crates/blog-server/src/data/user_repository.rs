use sqlx::{PgPool, Row};
use tracing::{error, info};
use uuid::Uuid;

use crate::domain::{user::User, error::DomainError};

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

impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: User) -> Result<User, DomainError> {
        sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(user.id)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .execute(&self.pool)
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
        })?;

        info!(user_id = %user.id, email = %user.email, "user created");
        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, username, email, password_hash, created_at
            FROM users
            WHERE username = $1
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("failed to find user by username {}: {}", username, e);
            DomainError::Internal(format!("database error: {}", e))
        })?;

        Ok(row.map(|row| User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            created_at: row.get("created_at"),
        }))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, username, email, password_hash, created_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("failed to find user by email {}: {}", email, e);
            DomainError::Internal(format!("database error: {}", e))
        })?;

        Ok(row.map(|row| User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            created_at: row.get("created_at"),
        }))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, username, email, password_hash, created_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("failed to find user by id {}: {}", id, e);
            DomainError::Internal(format!("database error: {}", e))
        })?;

        Ok(row.map(|row| User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            created_at: row.get("created_at"),
        }))
    }
}

