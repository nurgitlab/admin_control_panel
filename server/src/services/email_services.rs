use async_trait::async_trait;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
    message::{MessageBuilder, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
};

use crate::{configs, errors::email_errors::EmailError};

#[async_trait]
pub trait EmailService: Send + Sync {
    async fn send_email(
        &self,
        to: &str,
        subject: &str,
        text_body: &str,
        html_body: Option<&str>,
    ) -> Result<(), EmailError>;

    async fn send_text_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), EmailError> {
        self.send_email(to, subject, body, None).await
    }

    async fn send_html_email(
        &self,
        to: &str,
        subject: &str,
        html_body: &str,
    ) -> Result<(), EmailError> {
        self.send_email(to, subject, "", Some(html_body)).await
    }
}

pub struct LettreEmailService {
    transporter: AsyncSmtpTransport<Tokio1Executor>,
}

impl LettreEmailService {
    pub fn new() -> Result<Self, EmailError> {
        let email_user = configs::Config::global().email_user.clone();
        let email_password = configs::Config::global().email_password.clone();
        let email_host = configs::Config::global().email_host.clone();
        let email_port = configs::Config::global().email_port;

        let creds = Credentials::new(email_user, email_password);

        // Build the SMTP transport
        let transporter =
            AsyncSmtpTransport::<Tokio1Executor>::relay(&email_host)
                .map_err(|e| {
                    EmailError::Configuration(format!(
                        "Failed to create SMTP relay: {}",
                        e
                    ))
                })?
                .port(email_port)
                .credentials(creds)
                .build();

        Ok(Self { transporter })
    }

    // Validate emails
    fn validate_email_address(address: &str) -> Result<(), EmailError> {
        if address.is_empty() {
            return Err(EmailError::EmailValidation(
                "Email address cannot be empty".to_string(),
            ));
        }

        if !address.contains('@') {
            return Err(EmailError::EmailValidation(format!(
                "Invalid email format: {}",
                address
            )));
        }

        Ok(())
    }
}

#[async_trait]
impl EmailService for LettreEmailService {
    async fn send_email(
        &self,
        to: &str,
        subject: &str,
        text_body: &str,
        html_body: Option<&str>,
    ) -> Result<(), EmailError> {
        let email_from = configs::Config::global().email_from.clone();
        // Validate incoming parameters
        Self::validate_email_address(to)?;
        Self::validate_email_address(&email_from)?;

        if subject.trim().is_empty() {
            return Err(EmailError::EmptySubject);
        }

        if text_body.trim().is_empty()
            && html_body.map(|h| h.trim().is_empty()).unwrap_or(true)
        {
            return Err(EmailError::EmptyBody);
        }

        // Build mail message
        let email = {
            let mut builder = MessageBuilder::new();

            // Обработка поля 'from' с детальной ошибкой
            let from_address =
                email_from.parse().map_err(EmailError::AddressParse)?;
            builder = builder.from(from_address);

            // Обработка поля 'to'
            let to_address = to.parse().map_err(EmailError::AddressParse)?;
            builder = builder.to(to_address);

            // Обработка поля 'subject'
            builder = builder.subject(subject);

            // Построение тела письма
            if let Some(html) = html_body {
                builder.multipart(
                    MultiPart::alternative()
                        .singlepart(
                            SinglePart::builder()
                                .header(lettre::message::header::ContentType::TEXT_PLAIN)
                                .body(text_body.to_string()),
                        )
                        .singlepart(
                            SinglePart::builder()
                                .header(lettre::message::header::ContentType::TEXT_HTML)
                                .body(html.to_string()),
                        ),
                )
                .map_err(EmailError::MessageBuild)?
            } else {
                builder
                    .body(text_body.to_string())
                    .map_err(EmailError::MessageBuild)?
            }
        };

        // Send email
        self.transporter.send(email).await.map_err(EmailError::Smtp)?;

        log::info!("Email successfully sent to {}", to);
        Ok(())
    }
}
