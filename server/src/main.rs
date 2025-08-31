use std::sync::Arc;

use crate::{
    handlers::ping_pong_handler::get_ping_pong,
    services::email_services::{EmailService, LettreEmailService},
};
use actix_web::{
    App, HttpServer,
    middleware::Logger,
    web::{Data, scope},
};
use configs::config;
use sqlx::postgres::PgPoolOptions;

mod errors;
mod handlers;
mod middlewares;
mod models;
mod repositories;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    unsafe {
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("BACKTRACE", "1");
    }
    env_logger::init();

    // Initialize config
    config::Config::init().expect("Failed to initialize config");

    // Create DB pool
    let database_url = configs::Config::global().database_url.clone();
    println!("database_url: {database_url}");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // apply_migrations(&pool).await.expect("Failed to apply migrations");

    // Create email service
    let email_service = LettreEmailService::new().map_err(|e| {
        log::error!("Failed to create email service: {}", e);
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Service error: {}", e),
        )
    })?;
    let email_service1: Arc<dyn EmailService> = Arc::new(email_service);

    // Start HTTP server
    let server_host: String = configs::Config::global().server_host.clone();
    let server_port = configs::Config::global().server_port.clone();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(Data::new(pool.clone()))
            .app_data(Data::from(Arc::clone(&email_service1)))
            .service(
                scope("/api")
                    .service(get_ping_pong)
                    .configure(handlers::users_handler::users_routes)
                    .configure(handlers::cookies_handler::cookie_routes)
                    .configure(handlers::posts_handler::posts_routes)
                    .configure(handlers::auth_handler::auth_routes)
                    .configure(handlers::email_handlers::email_routes)
                    .configure(handlers::temp_registration_handler::temp_registration_routes),
            )
    })
    .bind((server_host, server_port))?
    .run()
    .await
}
