use anyhow::Result;
use sqlx::PgPool;
use time::OffsetDateTime;

use crate::{
    errors::auth_errors::AuthError, models::auth_models::RefreshToken,
};

pub struct AuthRepository;

impl AuthRepository {
    pub async fn validate_refresh_token(
        pool: &PgPool,
        token: &str,
    ) -> Result<i32, AuthError> {
        let result = sqlx::query!(
            r#"
            SELECT user_id, expires_at
            FROM refresh_tokens
            WHERE token = $1
            "#,
            token
        )
        .fetch_optional(pool)
        .await;

        match result {
            Ok(Some(record)) => {
                let expires_at: OffsetDateTime = record.expires_at;
                if expires_at < OffsetDateTime::now_utc() {
                    log::warn!(
                        "Refresh token expired for user {}",
                        record.user_id
                    );
                    Err(AuthError::TokenExpired)
                } else {
                    log::debug!(
                        "Refresh token validated for user {}",
                        record.user_id
                    );
                    Ok(record.user_id)
                }
            }
            Ok(None) => {
                log::warn!("Refresh token not found: {}", token);
                Err(AuthError::RefreshTokenNotFound)
            }
            Err(e) => {
                log::error!(
                    "Database error when validating refresh token: {}",
                    e
                );
                Err(AuthError::Authentication(e.to_string()))
            }
        }
    }

    pub async fn delete_refresh_token(
        pool: &PgPool,
        token: &str,
    ) -> Result<(), AuthError> {
        let result = sqlx::query!(
            r#"
                DELETE FROM refresh_tokens
                WHERE token = $1
                "#,
            token
        )
        .execute(pool)
        .await;

        match result {
            Ok(res) if res.rows_affected() > 0 => {
                log::info!("Refresh token deleted: {}", token);
                Ok(())
            }
            Ok(_) => {
                log::warn!("Refresh token not found for deletion: {}", token);
                Err(AuthError::RefreshTokenNotFound)
            }
            Err(e) => {
                log::error!(
                    "Database error when deleting refresh token: {}",
                    e
                );
                Err(AuthError::Authentication(e.to_string()))
            }
        }
    }

    pub async fn save_refresh_token(
        pool: &PgPool,
        token: &RefreshToken,
    ) -> Result<(), AuthError> {
        let result = sqlx::query!(
            r#"
            INSERT INTO refresh_tokens (token, user_id, expires_at)
            VALUES ($1, $2, $3)
            "#,
            token.token,
            token.user_id as i32,
            token.expires_at
        )
        .execute(pool)
        .await;

        match result {
            Ok(_) => {
                log::info!("Refresh token saved for user {}", token.user_id);
                Ok(())
            }
            Err(e) => {
                log::error!(
                    "Failed to save refresh token for user {}: {}",
                    token.user_id,
                    e
                );
                Err(AuthError::Authentication(e.to_string()))
            }
        }
    }
}
