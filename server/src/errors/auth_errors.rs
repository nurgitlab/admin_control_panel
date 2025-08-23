use actix_web::{HttpResponse, ResponseError};
use jsonwebtoken::errors::Error as JwtError;
use serde_json::json;
use sqlx::Error as SqlxError;
use thiserror::Error;
use time::error::ComponentRange;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid token: {0}")]
    InvalidToken(#[from] JwtError),

    #[error("Invalid timestamp: {0}")]
    InvalidTime(#[from] ComponentRange),

    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Token expired")]
    TokenExpired,

    #[error("Refresh token not found")]
    RefreshTokenNotFound,

    // #[error("Unauthorized: {0}")]
    // Unauthorized(String),

    // #[error("Internal server error")]
    // InternalServerError,
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),

    #[error("Database error: {0}")]
    Database(#[from] SqlxError),
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::Validation(errors) => {
                let details: Vec<String> = errors
                    .field_errors()
                    .iter()
                    .flat_map(|(field, errors)| {
                        errors.iter().map(move |e| {
                            log::error!("Validatotion error, AuthError: {e}");
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

            AuthError::Authentication(message) => {
                log::warn!("Authentication failed: {}", message);
                HttpResponse::Unauthorized().json(json!({
                    "error": "authentication_failed",
                    "message": message
                }))
            }

            AuthError::InvalidToken(e) => {
                log::warn!("Invalid token: {}", e);
                HttpResponse::Unauthorized().json(json!({
                    "error": "invalid_token",
                    "message": "Invalid or malformed token"
                }))
            }

            AuthError::TokenExpired => {
                log::warn!("Token expired");
                HttpResponse::Unauthorized().json(json!({
                    "error": "token_expired",
                    "message": "Token has expired"
                }))
            }

            AuthError::InvalidTime(e) => {
                log::error!("Invalid timestamp: {}", e);
                HttpResponse::InternalServerError().json(json!({
                    "error": "invalid_time",
                    "message": "Invalid timestamp operation"
                }))
            }

            AuthError::RefreshTokenNotFound => {
                log::warn!("Refresh token not found");
                HttpResponse::Unauthorized().json(json!({
                    "error": "refresh_token_not_found",
                    "message": "Refresh token not found or already expired"
                }))
            }

            // AuthError::Unauthorized(message) => {
            //     log::warn!("Unauthorized: {}", message);
            //     HttpResponse::Unauthorized().json(json!({
            //         "error": "unauthorized",
            //         "message": message
            //     }))
            // }

            // AuthError::InternalServerError => {
            //     log::error!("Internal server error");
            //     HttpResponse::InternalServerError().json(json!({
            //         "error": "internal_server_error",
            //         "message": "Something went wrong"
            //     }))
            // }
            AuthError::Database(e) => {
                log::error!("Database error: {e}");
                HttpResponse::InternalServerError().json(json!({
                    "error": "database_error",
                    "message": "Database operation failed"
                }))
            }
        }
    }
}
