use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct SendEmailRequest {
    #[validate(email(message = "Invalid email address"))]
    pub to: String,

    #[validate(length(min = 1, message = "Subject cannot be empty"))]
    pub subject: String,

    #[validate(length(min = 1, message = "Body cannot be empty"))]
    pub body: String,

    pub is_html: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct SendEmailResponse {
    pub success: bool,
    pub message: String,
}
