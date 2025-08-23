use serde::Deserialize;

#[derive(Deserialize)]
pub struct PingPongUser {
    pub user_id: u32,
    pub friend: String,
}
