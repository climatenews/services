use std::collections::HashMap;

use super::api::{get_tweets, split_requests_into_max_amount};
use crate::util::convert::{
    i64_to_numeric_id, numeric_id_to_i64, opt_numeric_id_to_opt_i64,
    referenced_tweet_kind_to_string,
};
use db::init_db_pool;
use db::models::news_referenced_tweet::NewsReferencedTweet;
use db::models::news_referenced_tweet_url::NewsReferencedTweetUrl;
use db::models::news_tweet::NewsTweet;
use db::models::news_tweet_url::NewsTweetUrl;
use db::models::news_twitter_user::NewsTwitterUser;
use db::sql::news_feed_url::truncate_news_feed_url;
use db::sql::news_referenced_tweet::{
    insert_news_referenced_tweet, truncate_news_referenced_tweet,
};
use db::sql::news_referenced_tweet_url::{
    find_news_referenced_tweet_url_by_tweet_id_and_url_id, insert_news_referenced_tweet_url,
    truncate_news_referenced_tweet_url,
};
use db::sql::news_tweet::{find_news_tweet_by_tweet_id, insert_news_tweet, truncate_news_tweet};
use db::sql::news_tweet_url::{
    find_news_tweet_urls_by_expanded_url_parsed, insert_news_tweet_url, truncate_news_tweet_url,
};
use db::sql::news_twitter_user::{
    find_news_twitter_user_by_user_id, insert_news_twitter_user, truncate_news_twitter_user,
};
use db::util::convert::{datetime_to_str, now_utc_timestamp};
use sqlx::PgPool;
use twitter_v2::authorization::BearerToken;
use twitter_v2::data::UrlImage;
use twitter_v2::id::NumericId;
use twitter_v2::{Tweet, TwitterApi, User};
use url::Url;

pub async fn init_db(reset_db: bool) -> PgPool {
    let db_pool = init_db_pool().await.unwrap();
    if reset_db {
        truncate_db_tables(&db_pool).await;
    }
    db_pool
}

pub async fn truncate_db_tables(db_pool: &PgPool) {
    truncate_news_twitter_user(db_pool).await.unwrap();
    truncate_news_tweet(db_pool).await.unwrap();
    truncate_news_referenced_tweet(db_pool).await.unwrap();
    truncate_news_tweet_url(db_pool).await.unwrap();
    truncate_news_referenced_tweet_url(db_pool).await.unwrap();
    truncate_news_feed_url(db_pool).await.unwrap();
}

pub async fn parse_twitter_user(db_pool: &PgPool, user: &User) -> Option<NewsTwitterUser> {
    // TODO parse verified status
    let user_id = numeric_id_to_i64(user.id);
    let followers_count = user
        .public_metrics
        .clone()
        .map_or_else(|| 0i32, |pm| pm.followers_count as i32);

    let listed_count = user
        .public_metrics
        .clone()
        .map_or_else(|| 0i32, |pm| pm.listed_count as i32);

    let profile_image_url: Option<String> = user
        .profile_image_url
        .clone()
        .map_or_else(|| None, |url| Some(url.to_string()));

    let news_twitter_user_db = find_news_twitter_user_by_user_id(db_pool, user_id).await;
    if news_twitter_user_db.is_none() {
        let news_twitter_user = NewsTwitterUser {
            user_id,
            username: user.username.clone(),
            profile_image_url,
            description: user.description.clone(),
            verified: user.verified,
            followers_count,
            listed_count,
            user_referenced_tweets_count: None,
            user_score: None,
            last_tweet_id: None,
            last_updated_at: now_utc_timestamp(),
            last_checked_at: now_utc_timestamp(),
        };
        insert_news_twitter_user(db_pool, news_twitter_user).await
    } else {
        news_twitter_user_db
    }
}

pub async fn parse_and_insert_tweet(db_pool: &PgPool, tweet: &Tweet) {
    let tweet_id = numeric_id_to_i64(tweet.id);
    let news_tweet_db = find_news_tweet_by_tweet_id(db_pool, tweet_id).await;
    if news_tweet_db.is_none() {
        if let (Some(author_id), Some(created_at)) = (tweet.author_id, tweet.created_at) {
            let news_tweet = NewsTweet {
                tweet_id,
                author_id: numeric_id_to_i64(author_id),
                conversation_id: opt_numeric_id_to_opt_i64(tweet.conversation_id),
                in_reply_to_user_id: opt_numeric_id_to_opt_i64(tweet.in_reply_to_user_id),
                text: tweet.text.clone(),
                created_at: created_at.unix_timestamp(),
                created_at_str: datetime_to_str(created_at),
            };
            insert_news_tweet(db_pool, news_tweet).await.unwrap();
        }
    }
    parse_tweet_urls(db_pool, tweet).await;
}

pub async fn parse_tweet_urls(db_pool: &PgPool, tweet: &Tweet) {
    let tweet_id = numeric_id_to_i64(tweet.id);
    if let Some(entities) = tweet.entities.clone() {
        if let Some(urls) = entities.urls {
            for url in urls {
                let expanded_url = Url::parse(&url.expanded_url).unwrap();
                let expanded_url_host = expanded_url.host_str().unwrap();
                // Remove www prefix from host
                let expanded_url_host = str::replace(expanded_url_host, "www.", "");
                let expanded_url_parsed = get_expanded_url_parsed(expanded_url.clone());

                let news_tweet_url_db_result = find_news_tweet_urls_by_expanded_url_parsed(
                    db_pool,
                    expanded_url_parsed.clone(),
                )
                .await;
                match news_tweet_url_db_result {
                    None => {
                        if let Some(created_at) = tweet.created_at {
                            let is_twitter_url =
                                url.expanded_url.starts_with("https://twitter.com")
                                    || url.expanded_url.starts_with("https://mobile.twitter.com");

                            let (preview_image_thumbnail_url, preview_image_url) =
                                parse_tweet_url_images(url.images);

                            let news_tweet_url = NewsTweetUrl {
                                url: url.url,
                                expanded_url: url.expanded_url,
                                expanded_url_parsed,
                                expanded_url_host,
                                display_url: url.display_url,
                                is_twitter_url,
                                title: url.title,
                                description: url.description,
                                preview_image_thumbnail_url,
                                preview_image_url,
                                created_at: created_at.unix_timestamp(),
                                created_at_str: datetime_to_str(created_at),
                            };

                            let news_tweet_url_db = insert_news_tweet_url(db_pool, news_tweet_url)
                                .await
                                .unwrap();
                            parse_and_insert_news_referenced_tweet_url(
                                db_pool,
                                tweet_id,
                                news_tweet_url_db.id,
                            )
                            .await;
                        }
                    }
                    Some(news_tweet_url_db) => {
                        parse_and_insert_news_referenced_tweet_url(
                            db_pool,
                            tweet_id,
                            news_tweet_url_db.id,
                        )
                        .await;
                    }
                }
            }
        }
    }
}

// Parses the preview image thumbnail and preview image for a shared link
pub fn parse_tweet_url_images(
    url_images: Option<Vec<UrlImage>>,
) -> (Option<String>, Option<String>) {
    if let Some(images) = url_images {
        let mut preview_image_thumbnail_url: Option<String> = None;
        let mut preview_image_url: Option<String> = None;

        for image in images {
            if image.width == 150 && image.height == 150 {
                preview_image_thumbnail_url = Some(String::from(image.url));
            } else {
                preview_image_url = Some(String::from(image.url));
            }
        }
        if let (Some(preview_image_thumbnail_url), Some(preview_image_url)) =
            (preview_image_thumbnail_url, preview_image_url)
        {
            return (Some(preview_image_thumbnail_url), Some(preview_image_url));
        }
    }
    (None, None)
}

pub async fn parse_and_insert_news_referenced_tweet_url(
    db_pool: &PgPool,
    tweet_id: i64,
    url_id: i32,
) {
    let news_referenced_tweet_url = NewsReferencedTweetUrl { tweet_id, url_id };
    let news_referenced_tweet_url_vec =
        find_news_referenced_tweet_url_by_tweet_id_and_url_id(db_pool, tweet_id, url_id).await;
    if news_referenced_tweet_url_vec.is_none() {
        insert_news_referenced_tweet_url(db_pool, news_referenced_tweet_url)
            .await
            .unwrap();
    }
}

pub fn parse_news_referenced_tweets(tweet: &Tweet) -> Vec<NewsReferencedTweet> {
    let mut news_referenced_tweets: Vec<NewsReferencedTweet> = vec![];
    if let Some(referenced_tweets) = tweet.referenced_tweets.clone() {
        news_referenced_tweets = referenced_tweets
            .iter()
            .map(|rt| NewsReferencedTweet {
                tweet_id: numeric_id_to_i64(tweet.id),
                referenced_tweet_id: numeric_id_to_i64(rt.id),
                referenced_tweet_kind: referenced_tweet_kind_to_string(rt.kind.clone()),
            })
            .collect::<Vec<NewsReferencedTweet>>();
    }
    news_referenced_tweets
}

pub async fn parse_and_insert_all_news_referenced_tweets(
    db_pool: &PgPool,
    twitter_api: &TwitterApi<BearerToken>,
    all_news_referenced_tweets: Vec<NewsReferencedTweet>,
) {
    let tweet_ids: Vec<NumericId> = all_news_referenced_tweets
        .iter()
        .map(|rt| i64_to_numeric_id(rt.referenced_tweet_id))
        .collect();

    // TODO move to api.rs
    //split tweet_ids into max 100 elements
    let split_tweet_ids_vec = split_requests_into_max_amount(tweet_ids);

    for split_tweet_ids in split_tweet_ids_vec {
        let referenced_tweets: Option<Vec<Tweet>> = get_tweets(twitter_api, split_tweet_ids).await;
        if let Some(referenced_tweets) = referenced_tweets.clone() {
            for referenced_tweet in referenced_tweets {
                parse_and_insert_tweet(db_pool, &referenced_tweet).await;
            }
        }
    }

    for news_referenced_tweet in all_news_referenced_tweets {
        insert_news_referenced_tweet(db_pool, news_referenced_tweet)
            .await
            .unwrap();
    }
}

// Remove all the query parameters from non-whitelisted urls
pub fn get_expanded_url_parsed(expanded_url: Url) -> String {
    let expanded_url = expanded_url;
    let mut expanded_url_parsed = expanded_url.clone();
    // Remove all the query parameters
    expanded_url_parsed.set_query(None);

    // Handle Youtube URL params
    // TODO convert mobile links to desktop
    if expanded_url.host_str().unwrap().contains("youtube.com") {
        let hash_query: HashMap<String, String> =
            expanded_url.clone().query_pairs().into_owned().collect();
        if let Some(v_param) = hash_query.get("v") {
            expanded_url_parsed.set_query(Some(&format!("v={}", v_param)));
        }
        if let Some(list_param) = hash_query.get("list") {
            expanded_url_parsed.set_query(Some(&format!("list={}", list_param)));
        }
    }
    expanded_url_parsed.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_expanded_url_parsed_params_test() {
        let expanded_url =
            Url::parse("https://github.com/rust-lang/rust/issues?labels=E-easy&state=open")
                .unwrap();
        let expanded_url_parsed = get_expanded_url_parsed(expanded_url);
        assert_eq!(
            expanded_url_parsed,
            "https://github.com/rust-lang/rust/issues"
        );
    }

    #[test]
    fn get_expanded_url_parsed_no_params_test() {
        let expanded_url = Url::parse("http://youtube.com/ecopracticas").unwrap();
        let expanded_url_parsed = get_expanded_url_parsed(expanded_url);
        assert_eq!(expanded_url_parsed, "http://youtube.com/ecopracticas");
    }

    #[test]
    fn get_expanded_url_parsed_youtube_params_test() {
        let expanded_url = Url::parse("https://m.youtube.com/watch?v=3SPVIUV2_uY").unwrap();
        let expanded_url_parsed = get_expanded_url_parsed(expanded_url);
        assert_eq!(
            expanded_url_parsed,
            "https://m.youtube.com/watch?v=3SPVIUV2_uY"
        );
    }

    #[test]
    fn get_expanded_url_parsed_youtube_mobile_params_test() {
        let expanded_url =
            Url::parse("http://youtube.com/watch?v=3TTF-muHLIQ&feature=youtu.be").unwrap();
        let expanded_url_parsed = get_expanded_url_parsed(expanded_url);
        assert_eq!(
            expanded_url_parsed,
            "http://youtube.com/watch?v=3TTF-muHLIQ"
        );
    }

    #[test]
    fn get_expanded_url_parsed_youtube_playlist_params_test() {
        let expanded_url =
            Url::parse("https://www.youtube.com/playlist?list=PLhQpDGfX5e7CSp3rm5SDv7D_idfkRzje-")
                .unwrap();
        let expanded_url_parsed = get_expanded_url_parsed(expanded_url);
        assert_eq!(
            expanded_url_parsed,
            "https://www.youtube.com/playlist?list=PLhQpDGfX5e7CSp3rm5SDv7D_idfkRzje-"
        );
    }
}
