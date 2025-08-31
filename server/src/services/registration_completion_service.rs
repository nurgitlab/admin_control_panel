use crate::{
    errors::temp_registration_errors::TempRegistrationError,
    models::{temp_registration::TempRegistration, users_models::CreateUser},
    repositories::{
        temp_registration_repository::TempRegistrationRepository,
        users_repository::UserRepository,
    },
    utils::secret_generator::SecretGenerator,
};
use sqlx::PgPool;

pub struct RegistrationCompletionService;

impl RegistrationCompletionService {
    pub async fn complete_registration(
        pool: &PgPool,
        email: String,
        secret_key: String,
    ) -> Result<String, TempRegistrationError> {
        let temp_registration =
            Self::validate_registration(pool, &email, &secret_key).await?;

        let username =
            Self::create_user_from_temp(pool, temp_registration).await?;

        Self::cleanup_temp_data(pool, &email, &secret_key).await?;

        Ok(username)
    }

    async fn validate_registration(
        pool: &PgPool,
        email: &str,
        secret_key: &str,
    ) -> Result<TempRegistration, TempRegistrationError> {
        let registration =
            TempRegistrationRepository::find_valid_by_email_and_key(
                pool, email, secret_key,
            )
            .await?;

        if registration.expires_at < time::OffsetDateTime::now_utc() {
            return Err(TempRegistrationError::Expired);
        }

        if registration.confirmed {
            return Err(TempRegistrationError::Validation(
                "Registration already confirmed".to_string(),
            ));
        }

        Ok(registration)
    }

    async fn create_user_from_temp(
        pool: &PgPool,
        temp_registration: TempRegistration,
    ) -> Result<String, TempRegistrationError> {
        let username = SecretGenerator::generate_alphanumeric_code(15);
        let new_user = CreateUser {
            username: username.clone(),
            password: temp_registration.password,
            email: temp_registration.email,
        };

        UserRepository::create(pool, new_user).await.map_err(|e| {
            log::error!("Failed to create user: {}", e);
            TempRegistrationError::Internal
        })?;

        Ok(username)
    }

    async fn cleanup_temp_data(
        pool: &PgPool,
        email: &str,
        secret_key: &str,
    ) -> Result<(), TempRegistrationError> {
        TempRegistrationRepository::mark_as_confirmed(pool, email, secret_key)
            .await?;
        TempRegistrationRepository::delete_by_email(pool, email).await?;
        Ok(())
    }
}
