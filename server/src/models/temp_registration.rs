use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

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

#[derive(Debug, Deserialize, Clone)]
pub struct CreateTempRegistration {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfirmRegistration {
    pub email: String,
    pub secret_key: String,
}
