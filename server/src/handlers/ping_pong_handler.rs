use crate::models::ping_pong_models::PingPongUser;
use actix_web::{Result, get, web::Path};

#[get("/pingpong/{user_id}/{friend}")]
pub async fn get_ping_pong(info: Path<PingPongUser>) -> Result<String> {
    Ok(format!("Welcome to Auth, {}, user_id {}!", info.friend, info.user_id))
}
