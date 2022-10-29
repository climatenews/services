// #[cfg(test)]
pub mod test_util {
    use crate::models::news_feed_url::NewsFeedUrl;
    use crate::models::news_tweet_url::NewsTweetUrl;
    use crate::models::news_twitter_user::NewsTwitterUser;
    use crate::sql::news_twitter_user::{insert_news_twitter_user, truncate_news_twitter_user};
    use crate::sql::news_feed_url::{insert_news_feed_url, truncate_news_feed_url};
    use crate::sql::news_tweet_url::{insert_news_tweet_url, truncate_news_tweet_url};
    use crate::util::convert::now_utc_timestamp;
    use sqlx::PgPool;

    pub async fn create_fake_news_tweet_url(db_pool: &PgPool){
        truncate_news_tweet_url(&db_pool).await.unwrap();
        let news_tweet_url = NewsTweetUrl {
            url: String::from("https://t.co/4HPNrqOnZj"),
            expanded_url: String::from("expanded_url"),
            expanded_url_parsed: String::from("expanded_url_parsed"),
            expanded_url_host: String::from("expanded_url_host"),
            display_url: String::from("display_url"),
            is_twitter_url: false,
            is_english: true,
            preview_image_url: Some(String::from("preview_image_url")),
            preview_image_thumbnail_url: Some(String::from("preview_image_thumbnail_url")),
            title: String::from("title"),
            description: String::from("description"),
            created_at: now_utc_timestamp(),
            created_at_str: String::from("created_at_str"),
        };
        insert_news_tweet_url(&db_pool, news_tweet_url)
            .await
            .unwrap();

    }

    pub async fn create_fake_news_feed_url(db_pool: &PgPool){
        let created_at_timestamp  = now_utc_timestamp() - 100; // to ensure news_feed_url is recent
        truncate_news_feed_url(&db_pool).await.unwrap();
        let news_feed_url = NewsFeedUrl {
            url_id: 1,
            url_score: 90,
            num_references: 2,
            first_referenced_by: 1,
            is_climate_related: true,
            created_at: created_at_timestamp,
            created_at_str: String::from("created_at_str"),
        };
        insert_news_feed_url(&db_pool, news_feed_url).await.unwrap();
    }

    pub async fn create_fake_news_twitter_user(db_pool: &PgPool){
        truncate_news_twitter_user(&db_pool).await.unwrap();
        let news_twitter_user = NewsTwitterUser {
            user_id: 1,
            username: String::from("username"),
            profile_image_url: Some(String::from("profile_image_url")),
            description: Some(String::from("description")),
            verified: Some(true),
            followers_count: 100,
            listed_count: 100,
            user_referenced_tweets_count: None,
            user_score: None,
            last_tweet_id: None,
            last_updated_at: 0,
            last_checked_at: 0,
        };
        insert_news_twitter_user(db_pool, news_twitter_user).await.unwrap();
    }

}