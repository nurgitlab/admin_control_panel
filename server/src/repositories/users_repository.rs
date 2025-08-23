use crate::{
    errors::users_errors::UserError,
    models::users_models::{CreateUser, UpdateUser, User},
};
use anyhow::Result;
use sqlx::PgPool;

pub struct UserRepository;

impl UserRepository {
    pub async fn create(
        pool: &PgPool,
        user_data: CreateUser,
    ) -> Result<User, UserError> {
        //TODO Need to create validation before INSERT in DB (because PSQL creating index in both cases)

        let result = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, password)
            VALUES ($1, $2)
            RETURNING id, username, password
            "#,
            user_data.username,
            user_data.password,
        )
        .fetch_optional(pool)
        .await;

        match result {
            Ok(Some(user)) => {
                log::info!(
                    "User {} successfully created '{}'",
                    user.id,
                    user.username
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
        let result =
            sqlx::query_as!(User, "SELECT id, username, password FROM users")
                .fetch_all(pool)
                .await;

        match result {
            Ok(users) => {
                log::info!("Users successfully finded",);
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
            "SELECT id, username, password FROM users WHERE id = $1",
            user_id
        )
        .fetch_optional(pool)
        .await;

        match result {
            Ok(Some(user)) => {
                log::info!(
                    "User {} successfully finded '{}'",
                    user_id,
                    user.username
                );
                Ok(user)
            }
            Ok(None) => {
                log::error!("User {user_id} disappeared during finding");
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
            "SELECT id, username, password FROM users WHERE username = $1",
            username
        )
        .fetch_optional(pool)
        .await;

        match result {
            Ok(Some(user)) => {
                log::info!(
                    "User {} successfully finded '{}'",
                    username,
                    user.username
                );
                Ok(user)
            }
            Ok(None) => {
                log::error!("User {username} disappeared during finding");
                Err(UserError::NotFound)
            }
            Err(e) => {
                log::error!("Database error when finding user {username}: {e}");
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
            "UPDATE users SET username = $1, password = $3 WHERE id = $2 RETURNING id, username, password",
            user_data.username,
            user_id,
            user_data.password,
        )
        .fetch_optional(pool)
        .await;

        match result {
            Ok(Some(user)) => {
                log::info!(
                    "User {} successfully updated with username '{}'",
                    user_id,
                    user.username
                );
                Ok(user)
            }
            Ok(None) => {
                log::error!("User {user_id} disappeared during update");
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
            Err(e) => {
                log::error!("Database error when deleting user {user_id}: {e}");
                Err(UserError::Database(e))
            }
            _ => {
                log::info!("User {user_id} updated");
                Ok(())
            }
        }
    }
}
