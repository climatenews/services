use super::constants::TWITTER_LISTS;
use crate::language::english_language_detector::EnglishLanguageDetector;
use crate::news_feed::constants::TWITTER_USERNAMES;
use crate::news_feed::user_score::calc_user_score;
use crate::twitter::api::get_list_members;
use crate::twitter::api::get_user_tweets;
use crate::twitter::api::get_users_by_username;
use crate::twitter::db::parse_twitter_list;
use crate::twitter::db::{
    parse_and_insert_all_news_referenced_tweets, parse_and_insert_tweet,
    parse_news_referenced_tweets, parse_twitter_user,
};
use crate::util::convert::i64_to_numeric_id;
use crate::util::convert::{numeric_id_to_i64, opt_i64_to_opt_numeric_id};
use chrono::Local;
use db::models::news_referenced_tweet::NewsReferencedTweet;
use db::models::news_twitter_user::NewsTwitterUser;
use db::sql::news_twitter_list::update_news_twitter_list_last_checked_at;
use db::sql::news_twitter_user::update_news_twitter_user_last_checked_at;
use db::sql::news_twitter_user::{
    find_all_news_twitter_users, update_news_twitter_user_last_updated_at,
    update_news_twitter_user_stats,
};
use db::sql::news_user_referenced_tweet_query::get_news_user_referenced_tweet_query;
use db::util::convert::now_utc_timestamp;
use db::util::time::datetime_hours_diff;
use db::util::time::datetime_minutes_diff;
use log::info;
use sqlx::PgPool;
use twitter_v2::authorization::BearerToken;
use twitter_v2::{Tweet, TwitterApi, User};

pub async fn get_all_user_tweets(db_pool: &PgPool, twitter_api: &TwitterApi<BearerToken>) {
    info!("get_all_user_tweets - {:?}", Local::now());
    fetch_user_tweets(db_pool, twitter_api).await;
    update_news_twitter_users_scores(db_pool).await;
    info!("get_all_user_tweets complete - {:?}", Local::now());
}

async fn fetch_user_tweets(db_pool: &PgPool, twitter_api: &TwitterApi<BearerToken>) {
    let english_language_detector = EnglishLanguageDetector::new();
    let mut users: Vec<User> = get_users_by_username(twitter_api, TWITTER_USERNAMES.to_vec())
        .await
        .unwrap();
    for list_id in TWITTER_LISTS {
        let news_twitter_list = parse_twitter_list(db_pool, list_id).await.unwrap();
        let last_checked_hours_diff = datetime_hours_diff(news_twitter_list.last_checked_at);
        // Check if last_checked is over 7 days
        if last_checked_hours_diff > (24 * 7) {
            let list_users: Vec<User> =
                get_list_members(twitter_api, i64_to_numeric_id(list_id)).await;
            for list_user in list_users {
                let followers_count = list_user
                    .public_metrics
                    .clone()
                    .map_or_else(|| 0i32, |pm| pm.followers_count as i32);
                if followers_count > 1000 {
                    users.push(list_user);
                }
            }
            // TODO ensure users are saved before updating list_last_checked_at
            update_news_twitter_list_last_checked_at(db_pool, list_id, now_utc_timestamp())
            .await
            .unwrap();
        }
    }
    // Remove duplicate users
    users.dedup_by(|a, b| a.username == b.username);
    info!("num users: {} ", users.len());

    for user in users {
        let news_twitter_user = parse_twitter_user(db_pool, &user).await.unwrap();
        let last_checked_minutes_diff = datetime_minutes_diff(news_twitter_user.last_checked_at);
        let last_updated_minutes_diff = datetime_minutes_diff(news_twitter_user.last_updated_at);
        if news_twitter_user.last_tweet_id.is_some() {
            info!(
                "username: {} last_checked {} mins ago, last_updated: {} mins ago",
                user.username, last_checked_minutes_diff, last_updated_minutes_diff
            );
        } else {
            info!("Adding username: {}", user.username);
        }
        // Check if last_checked is over 30 mins or has no recent tweets
        if last_checked_minutes_diff > 30 || news_twitter_user.last_tweet_id.is_none() {
            let last_tweet_id = opt_i64_to_opt_numeric_id(news_twitter_user.last_tweet_id);
            let tweets: Vec<Tweet> = get_user_tweets(twitter_api, user.id, last_tweet_id).await;
            fetch_user_tweet_references(db_pool, twitter_api, &tweets, &english_language_detector)
                .await;
            update_user_last_updated_at(db_pool, &news_twitter_user, &tweets).await;
        }
        update_user_last_checked_at(db_pool, &news_twitter_user).await;
    }
}

async fn update_news_twitter_users_scores(db_pool: &PgPool) {
    let news_twitter_users = find_all_news_twitter_users(db_pool).await.unwrap();

    for news_twitter_user in news_twitter_users {
        let news_user_referenced_tweets =
            get_news_user_referenced_tweet_query(db_pool, news_twitter_user.user_id)
                .await
                .unwrap();
        let user_referenced_tweets_count = news_user_referenced_tweets.len() as i32;
        let user_score = calc_user_score(&news_twitter_user, user_referenced_tweets_count);

        // save user score
        update_news_twitter_user_stats(
            db_pool,
            news_twitter_user.user_id,
            user_referenced_tweets_count,
            user_score,
        )
        .await
        .unwrap();
    }
}

async fn update_user_last_checked_at(db_pool: &PgPool, news_twitter_user: &NewsTwitterUser) {
    let last_checked_at = now_utc_timestamp();
    update_news_twitter_user_last_checked_at(db_pool, news_twitter_user.user_id, last_checked_at)
        .await
        .unwrap();
}

async fn update_user_last_updated_at(
    db_pool: &PgPool,
    news_twitter_user: &NewsTwitterUser,
    tweets: &Vec<Tweet>,
) {
    if let Some(last_tweet) = tweets.clone().first() {
        let last_tweet_id = numeric_id_to_i64(last_tweet.id);
        let last_updated_at = now_utc_timestamp();
        update_news_twitter_user_last_updated_at(
            db_pool,
            news_twitter_user.user_id,
            last_tweet_id,
            last_updated_at,
        )
        .await
        .unwrap();
    }
}

async fn fetch_user_tweet_references(
    db_pool: &PgPool,
    twitter_api: &TwitterApi<BearerToken>,
    tweets: &Vec<Tweet>,
    english_language_detector: &EnglishLanguageDetector,
) {
    let mut all_news_referenced_tweets: Vec<NewsReferencedTweet> = vec![];
    for tweet in tweets.clone() {
        parse_and_insert_tweet(db_pool, &tweet, english_language_detector).await;
        let news_referenced_tweets = parse_news_referenced_tweets(&tweet);
        all_news_referenced_tweets = [all_news_referenced_tweets, news_referenced_tweets].concat();
    }
    if !all_news_referenced_tweets.is_empty() {
        parse_and_insert_all_news_referenced_tweets(
            db_pool,
            twitter_api,
            all_news_referenced_tweets,
            english_language_detector,
        )
        .await;
    }
}
