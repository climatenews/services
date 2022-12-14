use std::env::var;
use std::ops::Deref;
use tokio::sync::Mutex;
use twitter_v2::authorization::{Oauth2Client, Oauth2Token};
use twitter_v2::TwitterApi;

lazy_static::lazy_static! {
    static ref OAUTH2_TOKEN: Mutex<Oauth2Token> = Mutex::new(serde_json::from_reader(
        std::fs::File::open(
            var("TWITTER_OAUTH_TOKEN_FILE").expect("could not find TWITTER_OAUTH_TOKEN_FILE")
        ).expect("TWITTER_OAUTH_TOKEN_FILE file not found"),
    )
    .expect("oauth2_token.json not valid json"));
}
async fn get_token() -> Oauth2Token {
    let oauth2_client = Oauth2Client::new(
        var("TWITTER_CLIENT_ID").expect("could not find TWITTER_CLIENT_ID"),
        var("TWITTER_CLIENT_SECRET").expect("could not find TWITTER_CLIENT_SECRET"),
        "http://127.0.0.1:3000/callback".parse().unwrap(),
    );
    let mut token = OAUTH2_TOKEN.lock().await;
    if oauth2_client
        .refresh_token_if_expired(&mut token)
        .await
        .unwrap()
    {
        let file_path =
            var("TWITTER_OAUTH_TOKEN_FILE").expect("could not find TWITTER_OAUTH_TOKEN_FILE");
        serde_json::to_writer(
            std::fs::File::create(file_path).expect("oauth2_token.json not found"),
            token.deref(),
        )
        .expect("couldn't save token");
    }
    token.clone()
}
#[allow(dead_code)]
pub async fn get_api_user_ctx() -> TwitterApi<Oauth2Token> {
    TwitterApi::new(get_token().await)
}
