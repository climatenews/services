use crate::news_feed::constants::TWITTER_USERNAMES;
use crate::news_feed::user_score::calc_user_score;
use crate::twitter::api::get_user_tweets;
use crate::twitter::api::get_users_by_username;
use crate::twitter::db::{
    parse_all_news_referenced_tweets, parse_news_referenced_tweets, parse_tweet, parse_twitter_user,
};
use crate::util::convert::{numeric_id_to_i64, opt_i64_to_opt_numeric_id};
use chrono::Local;
use db::models::news_referenced_tweet::NewsReferencedTweet;
use db::models::news_twitter_user::NewsTwitterUser;
use db::sql::news_twitter_user::{
    find_all_news_twitter_users, update_news_twitter_user_last_tweet_id,
    update_news_twitter_user_stats,
};
use db::sql::news_user_referenced_tweet_query::get_news_user_referenced_tweet_query;
use db::util::convert::now_utc_timestamp;
use db::util::convert::seconds_in_hour;
use sqlx::PgPool;
use twitter_v2::authorization::BearerToken;
use twitter_v2::{Tweet, TwitterApi, User};

pub async fn get_all_user_tweets(db_pool: &PgPool, twitter_api: &TwitterApi<BearerToken>) {
    println!("get_all_user_tweets - {:?}", Local::now());
    fetch_user_tweets(db_pool, twitter_api).await;
    update_news_twitter_users_scores(db_pool).await;
    println!("get_all_user_tweets complete - {:?}", Local::now());
}

async fn fetch_user_tweets(db_pool: &PgPool, twitter_api: &TwitterApi<BearerToken>) {
    // TODO avoid making API request if use is in the db?
    let users: Option<Vec<User>> =
        get_users_by_username(twitter_api, TWITTER_USERNAMES.to_vec()).await;
    if let Some(users) = users {
        for user in users {
            println!("username: {}", user.username);
            let news_twitter_user = parse_twitter_user(db_pool, &user).await.unwrap();
            let last_updated_diff = now_utc_timestamp() - news_twitter_user.last_updated_at;
            // Check if user has not been updated in over an hour or has no recent tweets
            if last_updated_diff > seconds_in_hour() || news_twitter_user.last_tweet_id.is_none() {
                let last_tweet_id = opt_i64_to_opt_numeric_id(news_twitter_user.last_tweet_id);
                let tweets: Vec<Tweet> = get_user_tweets(twitter_api, user.id, last_tweet_id).await;
                fetch_user_tweet_references(db_pool, twitter_api, &tweets).await;
                update_user_last_tweet_id(db_pool, &news_twitter_user, &tweets).await;
            }
        }
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

async fn update_user_last_tweet_id(
    db_pool: &PgPool,
    news_twitter_user: &NewsTwitterUser,
    tweets: &Vec<Tweet>,
) {
    if let Some(last_tweet) = tweets.clone().first() {
        let last_tweet_id = numeric_id_to_i64(last_tweet.id);
        let last_updated_at = now_utc_timestamp();
        update_news_twitter_user_last_tweet_id(
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
) {
    let mut all_news_referenced_tweets: Vec<NewsReferencedTweet> = vec![];
    for tweet in tweets.clone() {
        parse_tweet(db_pool, &tweet).await;
        let news_referenced_tweets = parse_news_referenced_tweets(&tweet);
        all_news_referenced_tweets = [all_news_referenced_tweets, news_referenced_tweets].concat();
    }
    if all_news_referenced_tweets.len() > 0 {
        parse_all_news_referenced_tweets(db_pool, twitter_api, all_news_referenced_tweets).await;
    }
}
