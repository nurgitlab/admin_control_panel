use crate::{
    configs,
    errors::auth_errors::AuthError,
    models::auth_models::{
        Claims, LoginRequest, RefreshRequest, RefreshToken, TokenPair,
    },
    repositories::{
        auth_repisitory::AuthRepository, users_repository::UserRepository,
    },
};
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation, decode, encode,
};
use sqlx::PgPool;

pub struct AuthService;

impl AuthService {
    pub async fn login(
        pool: &PgPool,
        credentials: LoginRequest,
    ) -> Result<TokenPair, AuthError> {
        let user_id = Self::authenticate_user(
            pool,
            &credentials.username,
            &credentials.password,
        )
        .await?;

        // Generate tokens
        let token_pair = Self::generate_token_pair(user_id)?;
        let refresh_token = RefreshToken::new(user_id);

        // Save refresh token in DB
        AuthRepository::save_refresh_token(pool, &refresh_token).await?;

        Ok(TokenPair {
            access_token: token_pair.access_token,
            refresh_token: refresh_token.token,
        })
    }

    pub async fn refresh(
        pool: &PgPool,
        token_data: RefreshRequest,
    ) -> Result<TokenPair, AuthError> {
        // Is refresh token valid
        let user_id = AuthRepository::validate_refresh_token(
            pool,
            &token_data.refresh_token,
        )
        .await?;

        // Delete user refresh token
        AuthRepository::delete_refresh_token(pool, &token_data.refresh_token)
            .await?;

        // Generate new pairs of tokens
        let token_pair = Self::generate_token_pair(user_id)?;
        let new_refresh_token = RefreshToken::new(user_id);

        // Save new refresh token
        AuthRepository::save_refresh_token(pool, &new_refresh_token).await?;

        Ok(TokenPair {
            access_token: token_pair.access_token,
            refresh_token: new_refresh_token.token,
        })
    }

    pub async fn logout(
        pool: &PgPool,
        token_data: RefreshRequest,
    ) -> Result<(), AuthError> {
        AuthRepository::delete_refresh_token(pool, &token_data.refresh_token)
            .await
    }

    pub async fn authenticate_user(
        pool: &PgPool,
        username: &str,
        password: &str,
    ) -> Result<i32, AuthError> {
        if username.is_empty() || password.is_empty() {
            return Err(AuthError::Authentication(
                "Username and password are required".to_string(),
            ));
        }

        let user = UserRepository::find_by_username(pool, username)
            .await
            .map_err(|e| {
                AuthError::Authentication(format!(
                    "Authentication failed: {}",
                    e
                ))
            })?;

        if username == user.username && password == user.password {
            Ok(user.id)
        } else {
            Err(AuthError::Authentication("Invalid credentials".to_string()))
        }
    }

    pub fn generate_token_pair(user_id: i32) -> Result<TokenPair, AuthError> {
        let claims = Claims::new(user_id);
        let refresh_token = RefreshToken::new(user_id);

        let jwt_access_secret =
            configs::Config::global().jwt_access_secret.as_bytes();
        log::debug!("jwt_access_secret: {:?}", jwt_access_secret);

        let access_token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_access_secret),
        )
        .map_err(AuthError::InvalidToken)?;

        Ok(TokenPair { access_token, refresh_token: refresh_token.token })
    }

    pub fn validate_access_token(token: &str) -> Result<Claims, AuthError> {
        let jwt_access_secret =
            configs::Config::global().jwt_access_secret.as_bytes();

        decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_access_secret),
            &Validation::default(),
        )
        .map(|token_data| {
            log::debug!(
                "Access token validated for user {}",
                token_data.claims.sub
            );
            token_data.claims
        })
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                log::warn!("Access token expired");
                AuthError::TokenExpired
            }
            _ => {
                log::warn!("Invalid access token: {}", e);
                AuthError::InvalidToken(e)
            }
        })
    }
}
