use actix_web::{
    HttpResponse, Result, post,
    web::{Data, Json, ServiceConfig},
};
use sqlx::PgPool;
use validator::Validate;

use crate::{
    errors::auth_errors::AuthError,
    models::auth_models::{LoginRequest, RefreshRequest},
    services::auth_services::AuthService,
};

#[post("/login")]
pub async fn login(
    credentials: Json<LoginRequest>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AuthError> {
    credentials.validate().map_err(AuthError::Validation)?;

    let token_pair =
        AuthService::login(&pool, credentials.into_inner()).await?;
    Ok(HttpResponse::Ok().json(token_pair))
}

#[post("/refresh")]
pub async fn refresh(
    token_data: Json<RefreshRequest>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AuthError> {
    token_data.validate().map_err(AuthError::Validation)?;

    let token_pair =
        AuthService::refresh(&pool, token_data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(token_pair))
}

#[post("/logout")]
pub async fn logout(
    token_data: Json<RefreshRequest>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AuthError> {
    token_data.validate().map_err(AuthError::Validation)?;

    AuthService::logout(&pool, token_data.into_inner()).await?;
    Ok(HttpResponse::Ok().json("Logged out successfully"))
}

pub fn auth_routes(cfg: &mut ServiceConfig) {
    cfg.service(login).service(refresh).service(logout);
}
