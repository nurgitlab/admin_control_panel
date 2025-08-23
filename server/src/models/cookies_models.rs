use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CookiePath {
    #[validate(length(
        min = 5,
        max = 10,
        message = "Must be between 5 and 10 characters"
    ))]
    pub cookie_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CookieResponse {
    pub id: String,
    pub message: String,
}
