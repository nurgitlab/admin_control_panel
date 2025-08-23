use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use sqlx::Error as SqlxError;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum PostError {
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),

    #[error("Database error: {0}")]
    Database(#[from] SqlxError),

    #[error("Post not found")]
    NotFound,

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

impl ResponseError for PostError {
    fn error_response(&self) -> HttpResponse {
        match self {
            PostError::Validation(errors) => {
                let details: Vec<String> = errors
                    .field_errors()
                    .iter()
                    .flat_map(|(field, errors)| {
                        errors.iter().map(move |e| {
                            log::error!("Validatotion error, post: {e}");
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

            PostError::Database(e) => {
                log::error!("Database error: {e}");
                HttpResponse::InternalServerError().json(json!({
                    "error": "database_error",
                    "message": "Database operation failed"
                }))
            }

            PostError::NotFound => HttpResponse::NotFound().json(json!({
                "error": "not_found",
                "message": "Post not found"
            })),

            PostError::Unauthorized(message) => {
                log::warn!("Unauthorized: {}", message);
                HttpResponse::Unauthorized().json(json!({
                    "error": "unauthorized",
                    "message": message
                }))
            }
        }
    }
}
