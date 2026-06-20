use sqlx::PgPool;
use tracing::error;
use uuid::Uuid;

use crate::domain::{post::Post, error::DomainError};

#[async_trait::async_trait]
pub trait PostRepository: Send + Sync {
    async fn create(&self, post: Post) -> Result<Post, DomainError>;
    async fn get(&self, id: Uuid) -> Result<Option<Post>, DomainError>;
    async fn update(&self, id: Uuid, title: &str, content: &str) -> Result<Option<Post>, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Post>, DomainError>;
}

#[derive(Clone)]
pub struct PostgresPostRepository {
    pool: PgPool
}

impl PostgresPostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl PostRepository for PostgresPostRepository {
    async fn create(&self, post: Post) -> Result<Post, DomainError> {
        sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (id, author_id, title, content)
            VALUES ($1, $2, $3, $4)
            RETURNING id, author_id, title, content, created_at, updated_at
            "#,
            post.id,
            post.author_id,
            post.title,
            post.content
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("failed to create post: {}", e);
            DomainError::Internal(format!("database error: {}", e))
        })
    }

    async fn get(&self, id: Uuid) -> Result<Option<Post>, DomainError> {
        sqlx::query_as!(
            Post,
            r#"
            SELECT id, author_id, title, content, created_at, updated_at FROM posts
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("failed to fetch post {}: {}", id, e);
            DomainError::Internal(format!("database error: {}", e))
        })
    }

    async fn update(&self, id: Uuid, title: &str, content: &str) -> Result<Option<Post>, DomainError> {
        sqlx::query_as!(
            Post,
            r#"
            UPDATE posts
            SET
                title = $2,
                content = $3,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, author_id, title, content, created_at, updated_at
            "#,
            id,
            title,
            content
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("failed to update post: {}", e);
            DomainError::Internal(format!("database error: {}", e))
        })
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            DELETE FROM posts
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            error!("failed to delete post: {}", e);
            DomainError::Internal(format!("database error: {}", e))
        })?;

        Ok(())
    }

    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Post>, DomainError> {
        sqlx::query_as!(
            Post,
            r#"
            SELECT id, author_id, title, content, created_at, updated_at FROM posts
            LIMIT $1
            OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!("failed to fetch posts : {}", e);
            DomainError::Internal(format!("database error: {}", e))
        })
    }
}
