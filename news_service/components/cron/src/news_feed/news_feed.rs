use std::collections::HashMap;

use crate::util::helpers::past_3_days;
use chrono::Local;
use db::models::news_feed_url::NewsFeedUrl;
use db::queries::news_referenced_url_query::NewsReferencedUrlQuery;
use db::sql::news_feed_url::{insert_news_feed_url, truncate_news_feed_url};
use db::sql::news_referenced_url_query::get_news_referenced_urls;
use db::util::convert::{
    datetime_from_unix_timestamp, datetime_to_str, now_utc_timestamp, seconds_in_hour,
};
use log::info;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use rust_decimal::MathematicalOps;
use rust_decimal_macros::dec;
use sqlx::PgPool;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
// Basic tweet info for score calculation
pub struct TweetInfo {
    pub author_id: i64,
    pub created_at: i64,
}

pub async fn populate_news_feed(db_pool: &PgPool) {
    info!("populate_news_feed - {:?}", Local::now());
    //TODO clear and update scores every 1 hour
    truncate_news_feed_url(db_pool).await.unwrap();
    let last_week_timestamp = past_3_days().unix_timestamp();

    // Direct & indirect references
    let news_referenced_urls = get_news_referenced_urls(db_pool, last_week_timestamp).await;

    let author_score_map: HashMap<i64, i32> = populate_author_score_map(&news_referenced_urls);

    let url_to_tweet_map: HashMap<i32, Vec<TweetInfo>> =
        populate_url_to_tweet_map(&news_referenced_urls);

    // Insert News feed urls
    populate_news_feed_urls(db_pool, author_score_map, url_to_tweet_map).await;
    info!("populate_news_feed complete - {:?}", Local::now());
}

// Populate a map of author_id to score
// author_id -> user_score
fn populate_author_score_map(
    news_referenced_urls: &Option<Vec<NewsReferencedUrlQuery>>,
) -> HashMap<i64, i32> {
    let mut author_score_map: HashMap<i64, i32> = HashMap::new();
    if let Some(news_referenced_urls) = news_referenced_urls {
        for news_referenced_url in news_referenced_urls {
            let author_id = news_referenced_url.author_id;
            let user_score = news_referenced_url.user_score.map_or_else(|| 0, |us| us);

            author_score_map.insert(author_id, user_score);
        }
    }
    author_score_map
}

// Populate a map Urls shared in tweets.
// url_id -> [TweetInfo, TweetInfo]
fn populate_url_to_tweet_map(
    news_referenced_urls: &Option<Vec<NewsReferencedUrlQuery>>,
) -> HashMap<i32, Vec<TweetInfo>> {
    let mut url_to_tweet_map: HashMap<i32, Vec<TweetInfo>> = HashMap::new();
    if let Some(news_referenced_urls) = news_referenced_urls {
        for news_referenced_url in news_referenced_urls {
            // Populate TweetInfo
            let url_id = news_referenced_url.url_id;
            let author_id = news_referenced_url.author_id;
            let created_at = news_referenced_url.created_at;
            let tweet_info = TweetInfo {
                author_id,
                created_at,
            };
            // check if url exists in the map
            if url_to_tweet_map.contains_key(&url_id) {
                let tweet_info_vec = url_to_tweet_map.get(&url_id).unwrap();
                let mut tweet_info_vec = tweet_info_vec.clone();
                // Ensures URL is not already added by same author
                if !tweet_info_vec
                    .iter().any(|ti| ti.author_id == author_id)
                {
                    tweet_info_vec.push(tweet_info);
                }
                url_to_tweet_map.insert(url_id, tweet_info_vec);
            } else {
                url_to_tweet_map.insert(url_id, vec![tweet_info]);
            }
        }
    }
    url_to_tweet_map
}

async fn populate_news_feed_urls(
    db_pool: &PgPool,
    author_score_map: HashMap<i64, i32>,
    url_to_tweet_map: HashMap<i32, Vec<TweetInfo>>,
) {
    // URLS with shared by author count + score
    for url_id in url_to_tweet_map.keys() {
        let tweet_info_vec = url_to_tweet_map.get(url_id).unwrap();
        // Sum total score for url based on author scores
        let url_score: i32 = tweet_info_vec
            .iter()
            .map(|tweet_info| author_score_map.get(&tweet_info.author_id).unwrap())
            .sum();

        // Find the first date the url was tweeted
        let first_created_at: i64 = tweet_info_vec
            .iter()
            .map(|tweet_info| tweet_info.created_at)
            .min()
            .unwrap();

        let first_created_at_datetime = datetime_from_unix_timestamp(first_created_at);
        let time_since_first_created = now_utc_timestamp() - first_created_at;
        let hours_since_first_created = time_since_first_created / seconds_in_hour();

        // Calculate url score factoring in time decay
        let time_decayed_url_score = time_decayed_url_score(url_score, hours_since_first_created);

        // Number of references/shares
        let num_references = tweet_info_vec.len() as i32;
        let news_feed_url = NewsFeedUrl {
            url_id: *url_id,
            url_score: time_decayed_url_score,
            num_references,
            created_at: first_created_at,
            created_at_str: datetime_to_str(first_created_at_datetime),
        };
        insert_news_feed_url(db_pool, news_feed_url).await;
    }
}

// 0.2 = too old
// 0.5 = too new
// url_score / (( hours_since_first_created +2 )^gravity)
fn time_decayed_url_score(url_score: i32, hours_since_first_created: i64) -> i32 {
    let gravity = dec!(0.4);
    let hour_addition = dec!(2);
    let url_score: Decimal = url_score.into();
    let hours_since_first_created: Decimal = hours_since_first_created.into();
    let time_value = hours_since_first_created
        .checked_add(hour_addition)
        .unwrap();

    let numerator: Decimal = url_score;
    let denominator: Decimal = time_value.checked_powd(gravity).unwrap();

    let time_decayed_url_score = numerator.checked_div(denominator).unwrap();
    time_decayed_url_score.to_i32().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn time_decayed_url_score_test_1() {
        let url_score = 600;
        let hours_since_first_created = 5;
        let time_decayed_url_score = time_decayed_url_score(url_score, hours_since_first_created);
        assert_eq!(time_decayed_url_score, 226);
    }

    #[test]
    fn time_decayed_url_score_test_2() {
        let url_score = 600;
        let hours_since_first_created = 24;
        let time_decayed_url_score = time_decayed_url_score(url_score, hours_since_first_created);
        assert_eq!(time_decayed_url_score, 117);
    }
}
