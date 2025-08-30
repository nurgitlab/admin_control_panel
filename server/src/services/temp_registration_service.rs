use crate::{
    errors::temp_registration_errors::TempRegistrationError,
    models::temp_registration::CreateTempRegistration,
    repositories::temp_registration_repository::TempRegistrationRepository,
    utils::secret_generator::SecretGenerator,
};
use sqlx::PgPool;

pub struct TempRegistrationService;

impl TempRegistrationService {
    pub async fn start_registration(
        pool: &PgPool,
        registration_data: CreateTempRegistration,
    ) -> Result<String, TempRegistrationError> {
        let is_already_registering =
            TempRegistrationRepository::is_email_in_registration(
                pool,
                &registration_data.email,
            )
            .await?;

        if is_already_registering {
            return Err(TempRegistrationError::AlreadyInProgress);
        }

        let secret_key = SecretGenerator::generate_numeric_code();

        TempRegistrationRepository::create(
            pool,
            registration_data.clone(),
            secret_key.clone(),
        )
        .await?;

        // Отправляем email с секретным ключом
        Self::send_confirmation_email(&registration_data.email, &secret_key)
            .await
            .map_err(|e| {
                log::error!("Failed to send confirmation email: {}", e);
                TempRegistrationError::Internal
            })?;

        Ok(secret_key)
    }

    async fn send_confirmation_email(
        email: &str,
        secret_key: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Здесь вызывается ваш существующий сервис отправки email
        // Например: email_service::send_confirmation(email, secret_key).await?;

        // Заглушка для демонстрации
        println!("Sending confirmation email to: {}", email);
        println!("Secret key: {}", secret_key);

        Ok(())
    }
}
