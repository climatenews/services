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
use url::Url;

use twitter_v2::authorization::BearerToken;
use twitter_v2::id::NumericId;
use twitter_v2::{Tweet, TwitterApi, User};

use sqlx::PgPool;

use crate::util::convert::{
    i64_to_numeric_id, numeric_id_to_i64, opt_numeric_id_to_opt_i64,
    referenced_tweet_kind_to_string,
};

use super::api::{get_tweets, split_requests_into_max_amount};

pub async fn init_db(reset_db: bool) -> PgPool {
    let db_pool = init_db_pool().await.unwrap();
    if reset_db {
        truncate_db_tables(&db_pool).await;
    }
    db_pool
}

pub async fn truncate_db_tables(db_pool: &PgPool) {
    truncate_news_twitter_user(&db_pool).await.unwrap();
    truncate_news_tweet(&db_pool).await.unwrap();
    truncate_news_referenced_tweet(&db_pool).await.unwrap();
    truncate_news_tweet_url(&db_pool).await.unwrap();
    truncate_news_referenced_tweet_url(&db_pool).await.unwrap();
    truncate_news_feed_url(&db_pool).await.unwrap();
}

pub async fn parse_twitter_user(db_pool: &PgPool, user: &User) -> Option<NewsTwitterUser> {
    let user_id = numeric_id_to_i64(user.id);
    let followers_count = user
        .public_metrics
        .clone()
        .map_or_else(|| 0i32, |pm| pm.followers_count as i32);

    let listed_count = user
        .public_metrics
        .clone()
        .map_or_else(|| 0i32, |pm| pm.listed_count as i32);

    let news_twitter_user_db = find_news_twitter_user_by_user_id(&db_pool, user_id).await;
    if news_twitter_user_db.is_none() {
        let news_twitter_user = NewsTwitterUser {
            user_id: user_id,
            username: user.username.clone(),
            followers_count: followers_count,
            listed_count: listed_count,
            user_referenced_tweets_count: None,
            user_score: None,
            last_tweet_id: None,
            last_updated_at: now_utc_timestamp(),
        };
        insert_news_twitter_user(&db_pool, news_twitter_user).await
    } else {
        news_twitter_user_db
    }
}

pub async fn parse_and_insert_tweet(db_pool: &PgPool, tweet: &Tweet) {
    let tweet_id = numeric_id_to_i64(tweet.id);
    let news_tweet_db = find_news_tweet_by_tweet_id(&db_pool, tweet_id).await;
    if news_tweet_db.is_none() {
        if let (Some(author_id), Some(created_at)) = (tweet.author_id, tweet.created_at) {
            let news_tweet = NewsTweet {
                tweet_id: tweet_id,
                author_id: numeric_id_to_i64(author_id),
                conversation_id: opt_numeric_id_to_opt_i64(tweet.conversation_id),
                in_reply_to_user_id: opt_numeric_id_to_opt_i64(tweet.in_reply_to_user_id),
                text: tweet.text.clone(),
                created_at: created_at.unix_timestamp(),
                created_at_str: datetime_to_str(created_at),
            };
            insert_news_tweet(&db_pool, news_tweet).await.unwrap();
        }
    }
    parse_tweet_urls(&db_pool, tweet).await;
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
                    &db_pool,
                    expanded_url_parsed.clone(),
                )
                .await;
                match news_tweet_url_db_result {
                    None => {
                        if let Some(created_at) = tweet.created_at {
                            let is_twitter_url =
                                url.expanded_url.starts_with("https://twitter.com")
                                    || url.expanded_url.starts_with("https://mobile.twitter.com");
                            let news_tweet_url = NewsTweetUrl {
                                url: url.url,
                                expanded_url: url.expanded_url,
                                expanded_url_parsed: expanded_url_parsed,
                                expanded_url_host: expanded_url_host,
                                display_url: url.display_url,
                                is_twitter_url: is_twitter_url,
                                title: url.title,
                                description: url.description,
                                created_at: created_at.unix_timestamp(),
                                created_at_str: datetime_to_str(created_at),
                            };

                            let news_tweet_url_db = insert_news_tweet_url(&db_pool, news_tweet_url)
                                .await
                                .unwrap();
                            parse_news_referenced_tweet_url(
                                db_pool,
                                tweet_id,
                                news_tweet_url_db.id,
                            )
                            .await;
                        }
                    }
                    Some(news_tweet_url_db) => {
                        parse_news_referenced_tweet_url(db_pool, tweet_id, news_tweet_url_db.id)
                            .await;
                    }
                }
            }
        }
    }
}

pub async fn parse_news_referenced_tweet_url(db_pool: &PgPool, tweet_id: i64, url_id: i32) {
    let news_referenced_tweet_url = NewsReferencedTweetUrl {
        tweet_id: tweet_id,
        url_id: url_id,
    };
    let news_referenced_tweet_url_vec =
        find_news_referenced_tweet_url_by_tweet_id_and_url_id(&db_pool, tweet_id, url_id).await;
    if news_referenced_tweet_url_vec.is_none() {
        insert_news_referenced_tweet_url(&db_pool, news_referenced_tweet_url)
            .await
            .unwrap();
    }
}

// Remove all the query parameters from non-whitelisted urls
pub fn get_expanded_url_parsed(mut expanded_url_parsed: Url) -> String {
    // Whitelisted Urls
    if expanded_url_parsed.host_str().unwrap() == "youtube.com" {
        //TODO parse video param only
        return expanded_url_parsed.to_string();
    }
    // Remove all the query parameters
    expanded_url_parsed.set_query(None);
    expanded_url_parsed.to_string()
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
                println!("referenced_tweet {:?}", referenced_tweet);
                parse_and_insert_tweet(&db_pool, &referenced_tweet).await;
            }
        }
    }

    for news_referenced_tweet in all_news_referenced_tweets {
        insert_news_referenced_tweet(db_pool, news_referenced_tweet)
            .await
            .unwrap();
    }
}
