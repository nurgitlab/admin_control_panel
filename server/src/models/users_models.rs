use derive_more::Display;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    // pub email: String,
}

#[derive(Debug, Deserialize, Validate, Display)]
#[display("CreateUser: username={username}, password={password}")]
pub struct CreateUser {
    #[validate(length(
        min = 3,
        max = 25,
        message = "Username must be between 3 and 25 chars"
    ))]
    pub username: String,

    #[validate(length(min = 8))]
    pub password: String,
    // #[validate(email(message = "Email must be a valid email address"))]
    // pub email: String,
}

#[derive(Debug, Deserialize, Validate, Display)]
#[display("UpdateUser: username={username}, password={password}")]
pub struct UpdateUser {
    #[validate(length(
        min = 3,
        max = 25,
        message = "Username must be between 3 and 25 chars"
    ))]
    pub username: String,

    #[validate(length(min = 8))]
    pub password: String,
    // #[validate(email(message = "Email must be a valid email address"))]
    // pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UserPath {
    #[validate(range(min = 1, message = "User ID must be positive"))]
    pub user_id: i32,
}
