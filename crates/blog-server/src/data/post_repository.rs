use sqlx::{PgPool, Row, postgres::PgRow};
use tracing::{error, info};
use uuid::Uuid;

use crate::domain::{post::Post, error::DomainError};

pub trait PostRepository: Send + Sync {
    async fn create(&self, post: Post) -> Result<Post, DomainError>;
    async fn get(&self, id: Uuid) -> Result<Option<Post>, DomainError>;
    async fn update(&self, id: Uuid, title: &str, content: &str) -> Result<Option<Post>, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Post>, DomainError>;
}

impl From<sqlx::Error> for DomainError {
    fn from(e: sqlx::Error) -> Self {
        DomainError::Internal(format!("row decode error: {}", e))
    }
}

impl TryFrom<PgRow> for Post {
    type Error = DomainError;
    
    fn try_from(row: PgRow) -> Result<Self, Self::Error> {
        Ok(Post{
            id: row.try_get("id")?,
            author_id: row.try_get("author_id")?,
            title: row.try_get("title")?,
            content: row.try_get("content")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
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

impl PostRepository for PostgresPostRepository {
    async fn create(&self, mut post: Post) -> Result<Post, DomainError> {
        let row = sqlx::query(
            r#"
            INSERT INTO posts (id, author_id, title, content, created_at, updated_at)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(post.id)
        .bind(post.author_id)
        .bind(&post.title)
        .bind(&post.content)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("failed to create post: {}", e);
            DomainError::Internal(format!("database error: {}", e))
        })?;

        post = row.try_into()?;
        info!(post_id = %post.id, author_id = %post.author_id, "post created");
        Ok(post)
    }

    async fn get(&self, id: Uuid) -> Result<Option<Post>, DomainError> {
        let found_row = sqlx::query(
            r#"
            SELECT * FROM posts
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("failed to fetch post {}: {}", id, e);
            DomainError::Internal(format!("database error: {}", e))
        })?;

        match found_row {
            Some(row) => Ok(Some(row.try_into()?)),
            None => Ok(None)
        }
    }

    async fn update(&self, id: Uuid, title: &str, content: &str) -> Result<Option<Post>, DomainError> {
        let updated_row = sqlx::query(
            r#"
            UPDATE posts
            SET
                title = $2,
                content = $3,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(title)
        .bind(content)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("failed to update post: {}", e);
            DomainError::Internal(format!("database error: {}", e))
        })?;

        match updated_row {
            Some(row) => {
                let post: Post = row.try_into()?;
                info!(post_id = %post.id, author_id = %post.author_id, "post updated");
                Ok(Some(post))
            },
            None => Ok(None)
        }
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            DELETE FROM posts
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            error!("failed to delete post: {}", e);
            DomainError::Internal(format!("database error: {}", e))
        })?;

        Ok(())
    }

    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Post>, DomainError> {
        let post_rows = sqlx::query(
            r#"
            SELECT * FROM posts
            LIMIT $1
            OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!("failed to fetch posts : {}", e);
            DomainError::Internal(format!("database error: {}", e))
        })?;

        let posts: Vec<Post> = post_rows.into_iter()
            .map(TryInto::<Post>::try_into)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(posts)
    }
}
