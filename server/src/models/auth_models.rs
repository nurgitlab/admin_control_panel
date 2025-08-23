use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;
use validator::Validate;

use crate::configs;

#[derive(Debug, Serialize, Deserialize)]

// JWT Claims structure
// sub: subject (user id)
// exp: expiration time (as UTC timestamp)
// iat: issued at (as UTC timestamp)
pub struct Claims {
    pub sub: i32, // user id
    pub exp: i32,
    pub iat: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshToken {
    pub token: String,
    pub user_id: i32,
    pub expires_at: OffsetDateTime,
}

impl Claims {
    pub fn new(user_id: i32) -> Self {
        let jwt_access_expires = configs::Config::global().jwt_access_expires;

        let iat = OffsetDateTime::now_utc();
        let exp = iat + Duration::seconds(jwt_access_expires); // Access token expires in 3 minutes

        Claims {
            sub: user_id,
            exp: exp.unix_timestamp() as i32,
            iat: iat.unix_timestamp() as i32,
        }
    }
}

impl RefreshToken {
    pub fn new(user_id: i32) -> Self {
        let jwt_refresh_expires = configs::Config::global().jwt_refresh_expires;

        let token = Uuid::new_v4().to_string();
        let expires_at =
            OffsetDateTime::now_utc() + Duration::seconds(jwt_refresh_expires); // Refresh token expires in 30 days

        RefreshToken { token, user_id, expires_at }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    pub username: String,

    #[validate(length(
        min = 8,
        max = 64,
        message = "Password must be between 8 and 64 characters"
    ))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RefreshRequest {
    #[validate(length(
        min = 36,
        max = 36,
        message = "Refresh token must be 36 characters long"
    ))]
    pub refresh_token: String,
}
