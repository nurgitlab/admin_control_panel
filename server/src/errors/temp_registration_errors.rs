use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TempRegistrationError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Database error: {0}")]
    Database(#[from] SqlxError),

    #[error("Temporary registration not found")]
    NotFound,

    #[error("Email is already taken")]
    EmailAlreadyTaken,

    #[error("Registration already in progress")]
    AlreadyInProgress,

    // #[error("Invalid secret key")]
    // InvalidSecretKey,
    #[error("Registration expired")]
    Expired,

    #[error("Internal server error")]
    Internal,
}

impl ResponseError for TempRegistrationError {
    fn error_response(&self) -> HttpResponse {
        match self {
            TempRegistrationError::Validation(message) => {
                log::error!("Validation error: {}", message);
                HttpResponse::BadRequest().json(json!({
                    "error": "validation_failed",
                    "message": message
                }))
            }

            TempRegistrationError::Database(e) => {
                log::error!("Database error: {e}");
                HttpResponse::InternalServerError().json(json!({
                    "error": "database_error",
                    "message": "Database operation failed"
                }))
            }

            TempRegistrationError::EmailAlreadyTaken => {
                HttpResponse::NotFound().json(json!({
                    "error": "email_already_taken",
                    "message": "Email is already taken"
                }))
            }

            TempRegistrationError::NotFound => {
                HttpResponse::NotFound().json(json!({
                    "error": "not_found",
                    "message": "Temporary registration not found"
                }))
            }

            TempRegistrationError::AlreadyInProgress => HttpResponse::Conflict(
            )
            .json(json!({
                "error": "already_in_progress",
                "message": "Registration for this email is already in progress"
            })),

            // TempRegistrationError::InvalidSecretKey => {
            //     HttpResponse::BadRequest().json(json!({
            //         "error": "invalid_secret_key",
            //         "message": "Invalid confirmation key"
            //     }))
            // }
            TempRegistrationError::Expired => {
                HttpResponse::BadRequest().json(json!({
                    "error": "expired",
                    "message": "Registration link has expired"
                }))
            }

            TempRegistrationError::Internal => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "internal_error",
                    "message": "Internal server error"
                }))
            }
        }
    }
}
