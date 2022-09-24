use twitter_v2::authorization::BearerToken;
use twitter_v2::TwitterApi;

pub mod api;
pub mod db;

pub fn init_twitter_api() -> TwitterApi<BearerToken> {
    let auth = BearerToken::new(std::env::var("APP_BEARER_TOKEN").unwrap());
    TwitterApi::new(auth)
}
