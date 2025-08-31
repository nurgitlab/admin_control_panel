use crate::{
    errors::temp_registration_errors::TempRegistrationError,
    models::temp_registration::{CreateTempRegistration, TempRegistration},
};
use sqlx::PgPool;
use time::{Duration, OffsetDateTime};

pub struct TempRegistrationRepository;

impl TempRegistrationRepository {
    pub async fn create(
        pool: &PgPool,
        registration_data: CreateTempRegistration,
        secret_key: String,
    ) -> Result<TempRegistration, TempRegistrationError> {
        let expires_at = OffsetDateTime::now_utc() + Duration::hours(24);

        let registration = sqlx::query_as!(
            TempRegistration,
            r#"
            INSERT INTO temp_registrations (email, password, secret_key, created_at, expires_at, confirmed)
            VALUES ($1, $2, $3, NOW(), $4, FALSE)
            RETURNING id, email, password, secret_key, created_at, expires_at, confirmed
            "#,
            registration_data.email,
            registration_data.password,
            secret_key,
            expires_at
        )
        .fetch_one(pool)
        .await
        .map_err(TempRegistrationError::Database)?;

        Ok(registration)
    }

    pub async fn find_by_email(
        pool: &PgPool,
        email: &str,
    ) -> Result<Option<TempRegistration>, TempRegistrationError> {
        let registration = sqlx::query_as!(
            TempRegistration,
            r#"
            SELECT 
                id, 
                email, 
                password, 
                secret_key, 
                created_at, 
                expires_at, 
                confirmed
            FROM temp_registrations WHERE email = $1
            "#,
            email
        )
        .fetch_optional(pool)
        .await
        .map_err(TempRegistrationError::Database)?;

        Ok(registration)
    }

    pub async fn find_valid_by_email_and_key(
        pool: &PgPool,
        email: &str,
        secret_key: &str,
    ) -> Result<TempRegistration, TempRegistrationError> {
        let registration = sqlx::query_as!(
            TempRegistration,
            r#"
            SELECT 
                id, 
                email, 
                password, 
                secret_key, 
                created_at, 
                expires_at, 
                confirmed
            FROM temp_registrations 
            WHERE email = $1 AND secret_key = $2 AND expires_at > NOW()
            "#,
            email,
            secret_key
        )
        .fetch_optional(pool)
        .await
        .map_err(TempRegistrationError::Database)?;

        registration.ok_or(TempRegistrationError::NotFound)
    }

    pub async fn mark_as_confirmed(
        pool: &PgPool,
        email: &str,
        secret_key: &str,
    ) -> Result<bool, TempRegistrationError> {
        let result = sqlx::query!(
            r#"
            UPDATE temp_registrations 
            SET confirmed = TRUE 
            WHERE email = $1 AND secret_key = $2 AND expires_at > NOW()
            "#,
            email,
            secret_key
        )
        .execute(pool)
        .await
        .map_err(TempRegistrationError::Database)?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn can_update_registration(
        pool: &PgPool,
        email: &str,
    ) -> Result<bool, TempRegistrationError> {
        if let Some(existing) = Self::find_by_email(pool, email).await? {
            let one_minute_ago =
                OffsetDateTime::now_utc() - Duration::minutes(1);
            Ok(existing.created_at <= one_minute_ago)
        } else {
            Ok(true) // Записи нет, можно создавать
        }
    }

    pub async fn delete_by_email(
        pool: &PgPool,
        email: &str,
    ) -> Result<(), TempRegistrationError> {
        sqlx::query!(
            r#"
            DELETE FROM temp_registrations WHERE email = $1
            "#,
            email
        )
        .execute(pool)
        .await
        .map_err(TempRegistrationError::Database)?;

        Ok(())
    }

    pub async fn cleanup_expired(
        pool: &PgPool,
    ) -> Result<u64, TempRegistrationError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM temp_registrations WHERE expires_at <= NOW()
            "#
        )
        .execute(pool)
        .await
        .map_err(TempRegistrationError::Database)?;

        Ok(result.rows_affected())
    }

    pub async fn is_email_in_registration(
        pool: &PgPool,
        email: &str,
    ) -> Result<bool, TempRegistrationError> {
        let exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM temp_registrations 
                WHERE email = $1 AND expires_at > NOW() AND confirmed = FALSE
            )
            "#,
            email
        )
        .fetch_one(pool)
        .await
        .map_err(TempRegistrationError::Database)?;

        Ok(exists.unwrap_or(false))
    }
}
