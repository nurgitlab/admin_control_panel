use dotenv::dotenv;
use serde::Deserialize;
use std::sync::OnceLock;
use std::{env, sync::Arc};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub jwt_access_secret: String,
    pub jwt_access_expires: i64,
    pub jwt_refresh_expires: i64,

    pub email_host: String,
    pub email_user: String,
    pub email_password: String,
    pub email_port: u16,
    pub email_from: String,
}

static CONFIG: OnceLock<Arc<Config>> = OnceLock::new();

impl Config {
    pub fn init() -> Result<(), Box<dyn std::error::Error>> {
        dotenv().expect("Failed to load .env file");

        let config = Config {
            database_url: env::var("DATABASE_URL")?,
            server_port: env::var("SERVER_PORT")
                .unwrap_or("8080".to_string())
                .parse()?,
            server_host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            jwt_access_secret: env::var("JWT_ACCESS_SECRET")?,
            jwt_access_expires: env::var("JWT_ACCESS_EXPIRES")
                .unwrap_or("60".to_string())
                .parse()?,
            jwt_refresh_expires: env::var("JWT_REFRESH_EXPIRES")
                .unwrap_or("3600".to_string())
                .parse()?,
            email_host: env::var("EMAIL_HOST")?,
            email_user: env::var("EMAIL_USER")?,
            email_password: env::var("EMAIL_PASSWORD")?,
            email_port: env::var("EMAIL_PORT")
                .unwrap_or("465".to_string())
                .parse()?,
            email_from: env::var("EMAIL_FROM")?,
        };

        CONFIG
            .set(Arc::new(config))
            .map_err(|_| "Config already initialized".into())
    }

    pub fn global() -> &'static Arc<Config> {
        CONFIG.get().expect("Config not initialized. Call Config::init() first")
    }
}
