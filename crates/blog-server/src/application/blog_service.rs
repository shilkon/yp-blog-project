use std::sync::Arc;

use uuid::Uuid;

use crate::{data::post_repository::PostRepository, domain::{error::BlogError, post::Post}};

#[derive(Clone)]
pub struct BlogService<R: PostRepository + 'static> {
    repo: Arc<R>,
}

impl<R> BlogService<R>
where
    R: PostRepository + 'static
{
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    pub async fn create_post(
        &self,
        author_id: Uuid,
        title: &str,
        content: &str
    ) -> Result<Post, BlogError> {
        let post = Post::new(author_id, title, content);
        self.repo.create(post).await.map_err(BlogError::from)
    }

    pub async fn get_post(&self, id: Uuid) -> Result<Post, BlogError> {
        self.repo.get(id).await.map_err(BlogError::from)
            .and_then(|p| p.ok_or_else(|| BlogError::NotFound(format!("post {}", id))))
    }

    pub async fn get_user_post(&self, user_id: Uuid, id: Uuid) -> Result<Post, BlogError> {
        let post = self.get_post(id).await?;

        if post.author_id != user_id {
            return Err(BlogError::Forbidden(format!("post {}", id)));
        }

        Ok(post)
    }

    pub async fn update_post(
        &self,
        user_id: Uuid,
        id: Uuid,
        title: &str,
        content: &str
    ) -> Result<Post, BlogError> {
        self.get_user_post(user_id, id).await?;

        self.repo.update(id, title, content).await.map_err(BlogError::from)
            .and_then(|p| p.ok_or_else(|| BlogError::NotFound(format!("post {}", id))))
    }

    pub async fn delete_post(
        &self,
        user_id: Uuid,
        id: Uuid
    ) -> Result<(), BlogError> {
        self.get_user_post(user_id, id).await?;

        self.repo.delete(id).await.map_err(BlogError::from)
    }

    pub async fn list_posts(&self, limit: i64, offset: i64) -> Result<Vec<Post>, BlogError> {
        if limit < 0 {
            return Err(BlogError::Validation("Invalid limit".to_string()));
        }
        if offset < 0 {
            return Err(BlogError::Validation("Invalid offset".to_string()));
        }

        self.repo.list(limit, offset).await.map_err(BlogError::from)
    }
}
