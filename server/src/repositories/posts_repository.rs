use crate::{
    errors::posts_errors::PostError,
    models::posts_models::{CreatePost, Post, UpdatePost},
};
use anyhow::Result;
use sqlx::PgPool;

pub struct PostsRepository;

impl PostsRepository {
    pub async fn create(
        pool: &PgPool,
        new_post: CreatePost,
        user_id: i32,
    ) -> Result<Post, PostError> {
        //TODO Need to create validation before INSERT in DB (because PSQL creating index in both cases)

        let result = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (message, user_id)
            VALUES ($1, $2)
            RETURNING 
            id, 
            message, 
            user_id, 
            created_at,
            updated_at
            "#,
            new_post.message,
            user_id
        )
        .fetch_optional(pool)
        .await;

        match result {
            Ok(Some(post)) => {
                log::info!(
                    "Post {} successfully created '{}'",
                    post.id,
                    post.user_id
                );
                Ok(post)
            }
            Ok(None) => {
                log::error!(
                    "Post {} disappeared during creating",
                    new_post.message
                );
                Err(PostError::NotFound)
            }
            Err(e) => {
                log::error!(
                    "Database error when creating post {}: {}",
                    new_post.message,
                    e
                );
                Err(PostError::Database(e))
            }
        }
    }

    pub async fn get_all(
        pool: &PgPool,
        user_id: i32,
    ) -> Result<Vec<Post>, PostError> {
        let result = sqlx::query_as!(
            Post,
            "SELECT 
                id, 
                message, 
                user_id, 
                created_at,
                updated_at
            FROM posts
            WHERE user_id = $1
            ORDER BY created_at DESC",
            user_id
        )
        .fetch_all(pool)
        .await;

        match result {
            Ok(posts) => {
                log::info!("Posts successfully finded",);
                Ok(posts)
            }
            Err(e) => {
                log::error!("Database error when finding posts: {e}");
                Err(PostError::Database(e))
            }
        }
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Post, PostError> {
        let result = sqlx::query_as!(
            Post,
            "SELECT 
                id, 
                message, 
                user_id, 
                created_at,
                updated_at
            FROM posts
            WHERE id = $1",
            id
        )
        .fetch_optional(pool)
        .await;

        match result {
            Ok(Some(post)) => {
                log::info!(
                    "Post {} successfully finded '{}'",
                    id,
                    post.user_id
                );
                Ok(post)
            }
            Ok(None) => {
                log::error!("Post {id} disappeared during finding");
                Err(PostError::NotFound)
            }
            Err(e) => {
                log::error!("Database error when finding post {id}: {e}");
                Err(PostError::Database(e))
            }
        }
    }

    pub async fn update(
        pool: &PgPool,
        id: i32,
        post_data: UpdatePost,
    ) -> Result<Post, PostError> {
        let result = sqlx::query_as!(
            Post,
            "UPDATE posts
                SET message = $1, updated_at = NOW()
                WHERE id = $2
                RETURNING 
                    id, 
                    message, 
                    user_id, 
                    created_at,
                    updated_at",
            post_data.message,
            id,
        )
        .fetch_optional(pool)
        .await;

        match result {
            Ok(Some(post)) => {
                log::info!(
                    "Post {} successfully updated with message '{}'",
                    id,
                    post.message
                );
                Ok(post)
            }
            Ok(None) => {
                log::error!("Post {id} disappeared during update");
                Err(PostError::NotFound)
            }
            Err(e) => {
                log::error!("Database error when updating post {id}: {e}");
                Err(PostError::Database(e))
            }
        }
    }

    pub async fn delete(pool: &PgPool, post_id: i32) -> Result<(), PostError> {
        let result = sqlx::query!("DELETE FROM posts WHERE id = $1", post_id)
            .execute(pool)
            .await;

        match result {
            Err(e) => {
                log::error!("Database error when deleting user {post_id}: {e}");
                Err(PostError::Database(e))
            }
            _ => {
                log::info!("Post {post_id} deleted");
                Ok(())
            }
        }
    }
}
