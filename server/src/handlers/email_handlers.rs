use actix_web::{
    HttpResponse,
    web::{self, Data, Json},
};
use validator::Validate;

use crate::{
    errors::email_errors::EmailError,
    models::email_models::{SendEmailRequest, SendEmailResponse},
    services::email_services::EmailService,
};

pub async fn send_email(
    email_service: Data<dyn EmailService>,
    request: Json<SendEmailRequest>,
) -> Result<HttpResponse, EmailError> {
    request.validate().map_err(EmailError::Validation)?;

    log::info!("Attempting to send email to: {}", request.to);

    let result = if request.is_html.unwrap_or(false) {
        email_service
            .send_html_email(&request.to, &request.subject, &request.body)
            .await
    } else {
        email_service
            .send_text_email(&request.to, &request.subject, &request.body)
            .await
    };

    match result {
        Ok(_) => {
            log::info!("Email successfully sent to {}", request.to);
            Ok(HttpResponse::Ok().json(SendEmailResponse {
                success: true,
                message: "Email sent successfully".to_string(),
            }))
        }
        Err(e) => {
            log::error!("Failed to send email: {}", e);
            Err(e)
        }
    }
}

pub fn email_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/email").route("/send", web::post().to(send_email)),
    );
}
