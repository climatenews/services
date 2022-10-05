use std::collections::{HashMap, HashSet};

use db::sql::news_user_referenced_tweet_query::get_all_news_user_referenced_tweet_query;
use sqlx::PgPool;
use twitter_v2::{authorization::BearerToken, TwitterApi, User};

use crate::{twitter::api::get_users_by_author_id, util::convert::numeric_id_to_i64};

pub async fn get_referenced_twitter_users(db_pool: &PgPool, twitter_api: &TwitterApi<BearerToken>) {
    let news_user_referenced_tweets = get_all_news_user_referenced_tweet_query(&db_pool)
        .await
        .unwrap();
    println!(
        "news_user_referenced_tweets: {:?} ",
        news_user_referenced_tweets.len()
    );

    // author_id -> [tweet_id]
    let mut author_map: HashMap<i64, HashSet<i64>> = HashMap::new();

    for news_user_referenced_tweet in news_user_referenced_tweets {
        let author_id = news_user_referenced_tweet.referenced_author_id;
        let tweet_id = news_user_referenced_tweet.tweet_id;
        if author_map.contains_key(&author_id) {
            let tweet_hashset = author_map.get(&author_id).unwrap();
            let mut new_tweet_hashset = tweet_hashset.clone();
            new_tweet_hashset.insert(tweet_id);
            author_map.insert(author_id, new_tweet_hashset);
        } else {
            author_map.insert(author_id, HashSet::from([tweet_id]));
        }
    }

    let mut popular_referenced_author_ids: Vec<i64> = vec![];
    for author_id in author_map.keys() {
        let tweet_hashset_len = author_map.get(&author_id).unwrap().len();
        if tweet_hashset_len > 50 && tweet_hashset_len <= 100 {
            popular_referenced_author_ids.push(*author_id);
        }
    }

    let twitter_users: Vec<User> =
        get_users_by_author_id(twitter_api, popular_referenced_author_ids).await;
    for twitter_user in twitter_users {
        let tweet_hashset_len = author_map
            .get(&numeric_id_to_i64(twitter_user.id))
            .unwrap()
            .len();

        println!(
            "username: {} referenced_tweets: {}",
            twitter_user.username, tweet_hashset_len
        )
    }
}
