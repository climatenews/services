use super::time_decay::time_decayed_url_score;
use crate::news_feed::algorithm::helper::{populate_author_score_map, populate_url_to_tweet_map};
use crate::news_feed::models::tweet_info::TweetInfo;
use chrono::Local;
use db::models::news_feed_url::NewsFeedUrl;
use db::sql::news_feed_url::{
    find_news_feed_url_by_url_id, insert_news_feed_url, reset_news_feed_url_url_scores,
    update_news_feed_url_url_score_and_num_references,
};
use db::sql::news_referenced_url_query::get_news_referenced_urls;
use db::util::convert::{
    datetime_from_unix_timestamp, datetime_to_str, now_utc_timestamp, seconds_in_hour,
};
use db::util::time::past_days;
use log::info;
use rust_decimal_macros::dec;
use sqlx::PgPool;
use std::collections::HashMap;

pub async fn populate_news_feed_v1(db_pool: &PgPool) {
    info!("populate_news_feed_v1 - {:?}", Local::now());
    let last_week_timestamp = past_days(3).unix_timestamp();

    // Direct & indirect references
    let news_referenced_urls = get_news_referenced_urls(db_pool, last_week_timestamp).await;

    let author_score_map: HashMap<i64, i32> = populate_author_score_map(&news_referenced_urls);

    let url_to_tweet_map: HashMap<i32, Vec<TweetInfo>> =
        populate_url_to_tweet_map(&news_referenced_urls);

    // Insert News feed urls
    populate_news_feed_urls_v1(db_pool, author_score_map, url_to_tweet_map).await;
    info!("populate_news_feed_v1 complete - {:?}", Local::now());
}

async fn populate_news_feed_urls_v1(
    db_pool: &PgPool,
    author_score_map: HashMap<i64, i32>,
    url_to_tweet_map: HashMap<i32, Vec<TweetInfo>>,
) {
    // Reset news feed URL scores
    reset_news_feed_url_url_scores(db_pool).await.unwrap();
    // URLS with shared by author count + score
    for url_id in url_to_tweet_map.keys() {
        let tweet_info_vec = url_to_tweet_map.get(url_id).unwrap();
        // Sum total score for url based on author scores
        let url_score: i32 = tweet_info_vec
            .iter()
            .map(|tweet_info| author_score_map.get(&tweet_info.author_id).unwrap())
            .sum();

        // Find the first tweet that shared the url
        let first_tweet: &TweetInfo = tweet_info_vec
            .iter()
            .min_by_key(|tweet_info| tweet_info.created_at)
            .unwrap();

        // TODO move logic into helper function
        let time_since_first_created = now_utc_timestamp() - first_tweet.created_at;
        let hours_since_first_created = time_since_first_created / seconds_in_hour();

        // Calculate url score factoring in time decay
        // A gravity of 0.4 is used in the ranking algorithm
        // Set it to 0.5 for newer results or 0.2 to older results
        let gravity = dec!(0.3);
        let time_decayed_url_score =
            time_decayed_url_score(gravity, url_score, hours_since_first_created);

        // Number of references/shares
        let num_references = tweet_info_vec.len() as i32;

        let news_feed_url_db = find_news_feed_url_by_url_id(db_pool, *url_id).await;
        if news_feed_url_db.is_none() {
            let news_feed_url = NewsFeedUrl {
                url_id: *url_id,
                url_score: time_decayed_url_score,
                num_references,
                first_referenced_by: first_tweet.author_id,
                created_at: first_tweet.created_at,
                created_at_str: datetime_to_str(datetime_from_unix_timestamp(
                    first_tweet.created_at,
                )),
            };
            insert_news_feed_url(db_pool, news_feed_url).await;
        } else {
            update_news_feed_url_url_score_and_num_references(
                db_pool,
                *url_id,
                url_score,
                num_references,
            )
            .await
            .unwrap();
        }
        // create query to export NewsFeedUrl to csv as training_data every day
        // add human_classification, gpt3_classification columns
    }
}
