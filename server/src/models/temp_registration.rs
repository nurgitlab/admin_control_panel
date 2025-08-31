use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TempRegistration {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub secret_key: String,
    pub created_at: OffsetDateTime,
    pub expires_at: OffsetDateTime,
    pub confirmed: bool,
}

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct CreateTempRegistration {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct ConfirmRegistration {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(
        min = 6,
        max = 6,
        message = "Secret key must be 6 digits"
    ))]
    pub secret_key: String,
}
