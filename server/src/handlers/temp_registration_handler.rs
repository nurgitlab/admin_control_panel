use crate::{
    errors::temp_registration_errors::TempRegistrationError,
    models::temp_registration::{ConfirmRegistration, CreateTempRegistration},
    services::{
        registration_completion_service::RegistrationCompletionService,
        temp_registration_service::TempRegistrationService,
    },
};
use actix_web::{
    HttpResponse, Result, post,
    web::{Data, Json, ServiceConfig},
};
use sqlx::PgPool;
use validator::Validate;

#[post("/register/start")]
pub async fn start_registration(
    registration_data: Json<CreateTempRegistration>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, TempRegistrationError> {
    registration_data
        .validate()
        .map_err(|e| TempRegistrationError::Validation(e.to_string()))?;

    let secret_key = TempRegistrationService::start_registration(
        &pool,
        registration_data.into_inner(),
    )
    .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Registration started successfully",
        "secret_key": secret_key
    })))
}

#[post("/register/complete")]
pub async fn complete_registration(
    pool: Data<PgPool>,
    confirmation_data: Json<ConfirmRegistration>,
) -> Result<HttpResponse, TempRegistrationError> {
    let confirmation_data = confirmation_data.into_inner();

    let username = RegistrationCompletionService::complete_registration(
        &pool,
        confirmation_data.email,
        confirmation_data.secret_key,
    )
    .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Registration completed successfully",
        "username": username,
        "success": true
    })))
}

pub fn temp_registration_routes(cfg: &mut ServiceConfig) {
    cfg.service(start_registration).service(complete_registration);
}
