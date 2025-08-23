use crate::{
    errors::cookies_errors::CookieError,
    models::cookies_models::{CookiePath, CookieResponse},
};
use actix_web::{
    HttpResponse, Result,
    cookie::{Cookie, time::Duration},
    get,
    web::{Data, Path, ServiceConfig},
};
use sqlx::PgPool;
use validator::Validate;

#[get("/cookie/{cookie_id}")]
pub async fn get_cookie(
    path: Path<CookiePath>,
    _: Data<PgPool>,
) -> Result<HttpResponse, CookieError> {
    path.validate().map_err(CookieError::Validation)?;

    let cookie = CookieResponse {
        id: path.cookie_id.clone(),
        message: "This is your cookie".to_string(),
    };

    let http_cookie = Cookie::build("cookie_name", path.cookie_id.clone())
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(Duration::days(1))
        .finish();

    Ok(HttpResponse::Ok().cookie(http_cookie).json(cookie))
}

pub fn cookie_routes(cfg: &mut ServiceConfig) {
    cfg.service(get_cookie);
}
