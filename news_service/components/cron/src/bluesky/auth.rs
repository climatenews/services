use crate::bluesky::BlueskySession;
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct CreateSessionRequest {
    identifier: String,
    password: String,
}

#[derive(Deserialize)]
struct CreateSessionResponse {
    #[serde(rename = "accessJwt", alias = "access_jwt")]
    access_jwt: String,
    #[serde(rename = "refreshJwt", alias = "refresh_jwt")]
    refresh_jwt: String,
    did: String,
    handle: String,
}

pub async fn create_session(
    client: &Client,
    service: &str,
    handle: &str,
    app_password: &str,
) -> Result<BlueskySession> {
    let url = format!("{}/xrpc/com.atproto.server.createSession", service);
    let body = CreateSessionRequest {
        identifier: handle.to_string(),
        password: app_password.to_string(),
    };

    let resp: CreateSessionResponse = client
        .post(&url)
        .json(&body)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(BlueskySession {
        access_jwt: resp.access_jwt,
        refresh_jwt: resp.refresh_jwt,
        did: resp.did,
        handle: resp.handle,
    })
}
