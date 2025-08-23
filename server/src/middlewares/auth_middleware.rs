use crate::services::auth_services::AuthService;
use actix_web::HttpMessage;
use actix_web::{Error, dev::ServiceRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn auth_middleware_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();

    log::debug!("Validating token: {}", token);

    match AuthService::validate_access_token(token) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(e) => {
            log::warn!("Token validation failed: {}", e);
            Err((e.into(), req)) // Теперь возвращаем кортеж (ошибка, запрос)
        }
    }
}
