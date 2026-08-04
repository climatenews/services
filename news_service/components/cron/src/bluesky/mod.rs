use reqwest::Client;
use serde::{Deserialize, Serialize};

pub mod api;
pub mod auth;
pub mod db;

pub const BLUESKY_SERVICE: &str = "https://bsky.social";

#[derive(Debug, Clone)]
pub struct BlueskyAgent {
    pub client: Client,
    pub session: BlueskySession,
    pub service: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueskySession {
    #[serde(rename = "accessJwt", alias = "access_jwt")]
    pub access_jwt: String,
    #[serde(rename = "refreshJwt", alias = "refresh_jwt")]
    pub refresh_jwt: String,
    pub did: String,
    pub handle: String,
}

pub async fn init_bluesky_agent() -> Result<BlueskyAgent, anyhow::Error> {
    let handle = std::env::var("BLUESKY_HANDLE").expect("BLUESKY_HANDLE is not set");
    let app_password = std::env::var("BLUESKY_APP_PASSWORD").expect("BLUESKY_APP_PASSWORD is not set");
    let service = std::env::var("BLUESKY_SERVICE").unwrap_or_else(|_| BLUESKY_SERVICE.to_string());

    let client = Client::new();
    let session = auth::create_session(&client, &service, &handle, &app_password).await?;

    Ok(BlueskyAgent {
        client,
        session,
        service,
    })
}
