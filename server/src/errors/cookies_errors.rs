use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum CookieError {
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),
}

impl ResponseError for CookieError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CookieError::Validation(errors) => {
                let details: Vec<String> = errors
                    .field_errors()
                    .iter()
                    .flat_map(|(field, errors)| {
                        errors.iter().map(move |e| {
                            log::error!("Validatotion error, cookie: {e}");
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
        }
    }
}
