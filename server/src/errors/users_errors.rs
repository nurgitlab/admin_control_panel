use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use sqlx::Error as SqlxError;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),

    #[error("Database error: {0}")]
    Database(#[from] SqlxError),

    #[error("User not found")]
    NotFound,
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UserError::Validation(errors) => {
                let details: Vec<String> = errors
                    .field_errors()
                    .iter()
                    .flat_map(|(field, errors)| {
                        errors.iter().map(move |e| {
                            log::error!("Validatotion error, user: {e}");
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
                    "message": "Validation failed",
                    "details": details
                }))
            }

            UserError::Database(e) => {
                log::error!("Database error: {e}");
                HttpResponse::InternalServerError().json(json!({
                    "error": "database_error",
                    "message": "Database operation failed"
                }))
            }

            UserError::NotFound => HttpResponse::NotFound().json(json!({
                "error": "not_found",
                "message": "User not found"
            })),
        }
    }
}
