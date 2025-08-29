use crate::{
    errors::users_errors::UserError,
    models::users_models::{CreateUser, UpdateUser, User},
};
use sqlx::PgPool;

pub struct UserRepository;

impl UserRepository {
    pub async fn create(
        pool: &PgPool,
        user_data: CreateUser,
    ) -> Result<User, UserError> {
        let result = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password, created_at, updated_at)
            VALUES ($1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
            RETURNING id, username, email, password, created_at, updated_at
            "#,
            user_data.username,
            user_data.email,
            user_data.password,
        )
        .fetch_optional(pool)
        .await;

        match result {
            Ok(Some(user)) => {
                log::info!(
                    "User {} successfully created '{}' with email '{}'",
                    user.id,
                    user.username,
                    user.email
                );
                Ok(user)
            }
            Ok(None) => {
                log::error!(
                    "User {} disappeared during creating",
                    user_data.username
                );
                Err(UserError::NotFound)
            }
            Err(e) => {
                log::error!(
                    "Database error when creating user {}: {}",
                    user_data.username,
                    e
                );
                Err(UserError::Database(e))
            }
        }
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<User>, UserError> {
        let result = sqlx::query_as!(
            User, 
            "SELECT id, username, email, password, created_at, updated_at FROM users"
        )
        .fetch_all(pool)
        .await;

        match result {
            Ok(users) => {
                log::info!("Users successfully found");
                Ok(users)
            }
            Err(e) => {
                log::error!("Database error when finding users: {e}");
                Err(UserError::Database(e))
            }
        }
    }

    pub async fn find_by_id(
        pool: &PgPool,
        user_id: i32,
    ) -> Result<User, UserError> {
        let result = sqlx::query_as!(
            User,
            "SELECT id, username, email, password, created_at, updated_at FROM users WHERE id = $1",
            user_id
        )
        .fetch_optional(pool)
        .await;

        match result {
            Ok(Some(user)) => {
                log::info!(
                    "User {} successfully found '{}'",
                    user_id,
                    user.username
                );
                Ok(user)
            }
            Ok(None) => {
                log::error!("User {user_id} not found");
                Err(UserError::NotFound)
            }
            Err(e) => {
                log::error!("Database error when finding user {user_id}: {e}");
                Err(UserError::Database(e))
            }
        }
    }

    pub async fn find_by_username(
        pool: &PgPool,
        username: &str,
    ) -> Result<User, UserError> {
        let result = sqlx::query_as!(
            User,
            "SELECT id, username, email, password, created_at, updated_at FROM users WHERE username = $1",
            username
        )
        .fetch_optional(pool)
        .await;

        match result {
            Ok(Some(user)) => {
                log::info!(
                    "User {} successfully found '{}'",
                    username,
                    user.username
                );
                Ok(user)
            }
            Ok(None) => {
                log::error!("User {username} not found");
                Err(UserError::NotFound)
            }
            Err(e) => {
                log::error!("Database error when finding user {username}: {e}");
                Err(UserError::Database(e))
            }
        }
    }

    pub async fn find_by_email(
        pool: &PgPool,
        email: &str,
    ) -> Result<User, UserError> {
        let result = sqlx::query_as!(
            User,
            "SELECT id, username, email, password, created_at, updated_at FROM users WHERE email = $1",
            email
        )
        .fetch_optional(pool)
        .await;

        match result {
            Ok(Some(user)) => {
                log::info!(
                    "User with email {} successfully found '{}'",
                    email,
                    user.username
                );
                Ok(user)
            }
            Ok(None) => {
                log::error!("User with email {email} not found");
                Err(UserError::NotFound)
            }
            Err(e) => {
                log::error!("Database error when finding user with email {email}: {e}");
                Err(UserError::Database(e))
            }
        }
    }

    pub async fn is_email_taken(
        pool: &PgPool,
        email: &str,
    ) -> Result<bool, UserError> {
        let result = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)",
            email
        )
        .fetch_one(pool)
        .await;

        match result {
            Ok(Some(exists)) => Ok(exists),
            Ok(None) => Ok(false),
            Err(e) => {
                log::error!("Database error when checking email {email}: {e}");
                Err(UserError::Database(e))
            }
        }
    }

    pub async fn update(
        pool: &PgPool,
        user_id: i32,
        user_data: UpdateUser,
    ) -> Result<User, UserError> {
        let result = sqlx::query_as!(
            User,
            r#"
            UPDATE users 
            SET username = $1, email = $2, password = $3, updated_at = CURRENT_TIMESTAMP 
            WHERE id = $4 
            RETURNING id, username, email, password, created_at, updated_at
            "#,
            user_data.username,
            user_data.email,
            user_data.password,
            user_id,
        )
        .fetch_optional(pool)
        .await;

        match result {
            Ok(Some(user)) => {
                log::info!(
                    "User {} successfully updated with username '{}' and email '{}'",
                    user_id,
                    user.username,
                    user.email
                );
                Ok(user)
            }
            Ok(None) => {
                log::error!("User {user_id} not found during update");
                Err(UserError::NotFound)
            }
            Err(e) => {
                log::error!("Database error when updating user {user_id}: {e}");
                Err(UserError::Database(e))
            }
        }
    }

    pub async fn delete(pool: &PgPool, user_id: i32) -> Result<(), UserError> {
        let result = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(pool)
            .await;

        match result {
            Ok(_) => {
                log::info!("User {user_id} deleted successfully");
                Ok(())
            }
            Err(e) => {
                log::error!("Database error when deleting user {user_id}: {e}");
                Err(UserError::Database(e))
            }
        }
    }
}