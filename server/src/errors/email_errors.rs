use actix_web::{HttpResponse, ResponseError};
use lettre::{
    address::AddressError, error::Error as LettreError,
    transport::smtp::Error as SmtpError,
};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum EmailError {
    #[error("Failed to parse email address: {0}")]
    AddressParse(#[from] AddressError),

    #[error("SMTP error: {0}")]
    Smtp(#[from] SmtpError),

    #[error("Email message construction failed: {0}")]
    MessageBuild(#[from] LettreError),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),

    #[error("Email validation failed: {0}")]
    EmailValidation(String),

    #[error("Email body cannot be empty")]
    EmptyBody,

    #[error("Email subject cannot be empty")]
    EmptySubject,

    #[error("Failed to send email: {0}")]
    SendFailed(String),

    #[error("SMTP server unavailable: {0}")]
    ServiceUnavailable(String),
}

impl ResponseError for EmailError {
    fn error_response(&self) -> HttpResponse {
        match self {
            EmailError::Validation(errors) => {
                let details: Vec<String> = errors
                    .field_errors()
                    .iter()
                    .flat_map(|(field, errors)| {
                        errors.iter().map(move |e| {
                            log::error!(
                                "Validation error in email request: {e}"
                            );
                            format!(
                                "{}: {}",
                                field,
                                e.message.as_deref().unwrap_or("invalid")
                            )
                        })
                    })
                    .collect();

                HttpResponse::BadRequest().json(json!({
                    "error": "validation_failed",
                    "message": "Email validation failed",
                    "details": details
                }))
            }

            EmailError::EmailValidation(message) => {
                log::warn!("Email validation failed: {}", message);
                HttpResponse::BadRequest().json(json!({
                    "error": "email_validation_failed",
                    "message": message
                }))
            }

            EmailError::AddressParse(e) => {
                log::warn!("Invalid email address: {}", e);
                HttpResponse::BadRequest().json(json!({
                    "error": "invalid_email_address",
                    "message": "Invalid email address format"
                }))
            }

            EmailError::EmptyBody => {
                log::warn!("Email body cannot be empty");
                HttpResponse::BadRequest().json(json!({
                    "error": "empty_email_body",
                    "message": "Email body cannot be empty"
                }))
            }

            EmailError::EmptySubject => {
                log::warn!("Email subject cannot be empty");
                HttpResponse::BadRequest().json(json!({
                    "error": "empty_email_subject",
                    "message": "Email subject cannot be empty"
                }))
            }

            EmailError::Smtp(e) => {
                log::error!("SMTP error: {}", e);
                HttpResponse::ServiceUnavailable().json(json!({
                    "error": "smtp_error",
                    "message": "Email service temporarily unavailable"
                }))
            }

            EmailError::ServiceUnavailable(message) => {
                log::error!("SMTP service unavailable: {}", message);
                HttpResponse::ServiceUnavailable().json(json!({
                    "error": "email_service_unavailable",
                    "message": message
                }))
            }

            EmailError::MessageBuild(e) => {
                log::error!("Failed to build email message: {}", e);
                HttpResponse::InternalServerError().json(json!({
                    "error": "email_build_failed",
                    "message": "Failed to create email message"
                }))
            }

            EmailError::Configuration(message) => {
                log::error!("Email configuration error: {}", message);
                HttpResponse::InternalServerError().json(json!({
                    "error": "email_configuration_error",
                    "message": "Email service configuration error"
                }))
            }

            EmailError::SendFailed(message) => {
                log::error!("Email sending failed: {}", message);
                HttpResponse::InternalServerError().json(json!({
                    "error": "email_send_failed",
                    "message": message
                }))
            }
        }
    }
}
