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
use anyhow::Result;
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

pub async fn get_all_user_tweets(
    db_pool: &PgPool,
    twitter_api: &TwitterApi<BearerToken>,
) -> Result<()> {
    info!("get_all_user_tweets - {:?}", Local::now());
    fetch_user_tweets(db_pool, twitter_api).await?;
    update_news_twitter_users_scores(db_pool).await?;
    info!("get_all_user_tweets complete - {:?}", Local::now());
    Ok(())
}

async fn fetch_user_tweets(db_pool: &PgPool, twitter_api: &TwitterApi<BearerToken>) -> Result<()> {
    let english_language_detector = EnglishLanguageDetector::new();
    let mut users: Vec<User> = get_users_by_username(twitter_api, TWITTER_USERNAMES.to_vec())
        .await
        .unwrap();
    for list_id in TWITTER_LISTS {
        let news_twitter_list = parse_twitter_list(db_pool, list_id).await?;

        let last_checked_hours_diff = datetime_hours_diff(news_twitter_list.last_checked_at);
        // Check if last_checked is over 7 days
        if last_checked_hours_diff > (24 * 7) {
            let list_users: Vec<User> =
                get_list_members(twitter_api, i64_to_numeric_id(list_id)).await;
            for list_user in list_users {
                users.push(list_user);
            }
            // TODO ensure users are saved before updating list_last_checked_at
            update_news_twitter_list_last_checked_at(db_pool, list_id, now_utc_timestamp())
                .await
                .unwrap();
        }
    }
    // TODO add unit test
    // Remove duplicate users
    users.dedup_by(|a, b| a.id.as_u64() == b.id.as_u64());
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
            let tweets: Vec<Tweet> = get_user_tweets(twitter_api, user.id, last_tweet_id).await?;
            fetch_user_tweet_references(db_pool, twitter_api, &tweets, &english_language_detector)
                .await?;
            update_user_last_updated_at(db_pool, &news_twitter_user, &tweets).await?;
        }
        update_user_last_checked_at(db_pool, &news_twitter_user).await?;
    }
    Ok(())
}

async fn update_news_twitter_users_scores(db_pool: &PgPool) -> Result<()> {
    let news_twitter_users = find_all_news_twitter_users(db_pool).await?;

    for news_twitter_user in news_twitter_users {
        let news_user_referenced_tweets_result =
            get_news_user_referenced_tweet_query(db_pool, news_twitter_user.user_id).await;

        let mut user_referenced_tweets_count = 0;
        match news_user_referenced_tweets_result {
            Ok(news_user_referenced_tweets) => {
                user_referenced_tweets_count = news_user_referenced_tweets.len() as i32;
            }
            Err(_) => {}
        }
        let user_score = calc_user_score(&news_twitter_user, user_referenced_tweets_count);

        // save user score & referenced_tweets_count
        update_news_twitter_user_stats(
            db_pool,
            news_twitter_user.user_id,
            user_referenced_tweets_count,
            user_score,
        )
        .await?;
    }
    Ok(())
}

async fn update_user_last_checked_at(
    db_pool: &PgPool,
    news_twitter_user: &NewsTwitterUser,
) -> Result<()> {
    let last_checked_at = now_utc_timestamp();
    update_news_twitter_user_last_checked_at(db_pool, news_twitter_user.user_id, last_checked_at)
        .await?;
    Ok(())
}

async fn update_user_last_updated_at(
    db_pool: &PgPool,
    news_twitter_user: &NewsTwitterUser,
    tweets: &Vec<Tweet>,
) -> Result<()> {
    if let Some(last_tweet) = tweets.clone().first() {
        let last_tweet_id = numeric_id_to_i64(last_tweet.id);
        let last_updated_at = now_utc_timestamp();
        update_news_twitter_user_last_updated_at(
            db_pool,
            news_twitter_user.user_id,
            last_tweet_id,
            last_updated_at,
        )
        .await?;
    }
    Ok(())
}

async fn fetch_user_tweet_references(
    db_pool: &PgPool,
    twitter_api: &TwitterApi<BearerToken>,
    tweets: &Vec<Tweet>,
    english_language_detector: &EnglishLanguageDetector,
) -> Result<()> {
    let mut all_news_referenced_tweets: Vec<NewsReferencedTweet> = vec![];
    // TODO keep track of referenced author_ids and create author_id to username hashmap
    for tweet in tweets.clone() {
        parse_and_insert_tweet(db_pool, &tweet, english_language_detector).await?;
        let news_referenced_tweets = parse_news_referenced_tweets(&tweet);
        all_news_referenced_tweets = [all_news_referenced_tweets, news_referenced_tweets].concat();
    }
    if !all_news_referenced_tweets.is_empty() {
        // pass hashmap of author_id to username
        parse_and_insert_all_news_referenced_tweets(
            db_pool,
            twitter_api,
            all_news_referenced_tweets,
            english_language_detector,
        )
        .await?;
    }
    Ok(())
}
