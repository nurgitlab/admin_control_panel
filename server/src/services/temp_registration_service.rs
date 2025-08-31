use crate::{
    errors::temp_registration_errors::TempRegistrationError,
    models::temp_registration::CreateTempRegistration,
    repositories::{
        temp_registration_repository::TempRegistrationRepository,
        users_repository::UserRepository,
    },
    services::email_services::{EmailService, LettreEmailService},
    utils::secret_generator::SecretGenerator,
};
use sqlx::PgPool;

pub struct TempRegistrationService;

impl TempRegistrationService {
    pub async fn start_registration(
        pool: &PgPool,
        registration_data: CreateTempRegistration,
    ) -> Result<String, TempRegistrationError> {
        let email = registration_data.email.clone();

        if UserRepository::is_email_taken(pool, &registration_data.email)
            .await
            .map_err(|e| {
                log::error!("User repository error: {}", e);
                TempRegistrationError::Internal
            })?
        {
            return Err(TempRegistrationError::EmailAlreadyTaken);
        }

        let can_update =
            TempRegistrationRepository::can_update_registration(pool, &email)
                .await?;

        if !can_update {
            return Err(TempRegistrationError::AlreadyInProgress);
        }

        let secret_key = SecretGenerator::generate_numeric_code();

        TempRegistrationRepository::create(
            pool,
            registration_data,
            secret_key.clone(),
        )
        .await?;

        Self::send_confirmation_email(&email, &secret_key).await.map_err(
            |e| {
                log::error!("Failed to send confirmation email: {}", e);
                TempRegistrationError::Internal
            },
        )?;

        Ok(secret_key)
    }

    async fn send_confirmation_email(
        email: &str,
        secret_key: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let email_service = LettreEmailService::new()
            .map_err(|e| format!("Failed to create email service: {}", e))?;

        EmailService::send_email(
            &email_service,
            email,
            "Confirm registration",
            &format!("Your secret code: {}", secret_key),
            Some(&format!("<p>Your secret code: <b>{}</b></p>", secret_key)),
        )
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        log::info!("Confirmation email sent to: {}", email);
        Ok(())
    }
}
