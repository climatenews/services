// #[cfg(test)]
pub mod test_util {
    use crate::models::news_bsky_post::NewsBskyPost;
    use crate::models::news_bsky_post_url::NewsBskyPostUrl;
    use crate::models::news_bsky_reference::NewsBskyReference;
    use crate::models::news_bsky_referenced_post_url::NewsBskyReferencedPostUrl;
    use crate::models::news_bsky_user::NewsBskyUser;
    use crate::models::news_cron_job::NewsCronJob;
    use crate::models::news_feed_url::NewsFeedUrl;
    use crate::sql::news_bsky_post::{insert_news_bsky_post, truncate_news_bsky_post};
    use crate::sql::news_bsky_post_url::{insert_news_bsky_post_url, truncate_news_bsky_post_url};
    use crate::sql::news_bsky_reference::{insert_news_bsky_reference, truncate_news_bsky_reference};
    use crate::sql::news_bsky_referenced_post_url::{
        insert_news_bsky_referenced_post_url, truncate_news_bsky_referenced_post_url,
    };
    use crate::sql::news_bsky_user::{insert_news_bsky_user, truncate_news_bsky_user};
    use crate::sql::news_cron_job::{insert_news_cron_job, truncate_news_cron_job};
    use crate::sql::news_feed_url::{insert_news_feed_url, truncate_news_feed_url};
    use crate::util::convert::datetime_to_str;
    use sqlx::PgPool;
    use time::OffsetDateTime;

    pub async fn create_fake_news_bsky_post_url(db_pool: &PgPool, created_at_timestamp: i64) {
        truncate_news_bsky_post_url(db_pool).await.unwrap();
        let news_bsky_post_url = NewsBskyPostUrl {
            url_id: 0,
            url: String::from("https://example.com/link"),
            expanded_url: String::from("https://example.com/link"),
            expanded_url_parsed: String::from("https://example.com/link"),
            expanded_url_host: String::from("example.com"),
            display_url: Some(String::from("example.com/link")),
            is_bsky_url: Some(false),
            is_english: Some(true),
            title: Some(String::from("example title")),
            description: Some(String::from("description")),
            preview_image_thumbnail_url: Some(String::from("preview_image_thumbnail_url")),
            preview_image_url: Some(String::from("preview_image_url")),
            created_at: created_at_timestamp,
            created_at_str: String::from("created_at_str"),
        };
        insert_news_bsky_post_url(db_pool, news_bsky_post_url)
            .await
            .unwrap();
    }

    pub async fn create_fake_news_bsky_user(db_pool: &PgPool) {
        truncate_news_bsky_user(db_pool).await.unwrap();
        let news_bsky_user = NewsBskyUser {
            did: String::from("did:plc:user1"),
            handle: String::from("user1.bsky.social"),
            display_name: Some(String::from("User One")),
            avatar_url: Some(String::from("avatar_url")),
            description: Some(String::from("description")),
            followers_count: 100,
            follows_count: 100,
            posts_count: 100,
            user_score: Some(200),
            last_post_cid: None,
            last_updated_at: 0,
            last_checked_at: 0,
        };
        insert_news_bsky_user(db_pool, news_bsky_user).await.unwrap();

        let news_bsky_user_retweeted = NewsBskyUser {
            did: String::from("did:plc:reposter"),
            handle: String::from("reposter.bsky.social"),
            display_name: Some(String::from("Reposter")),
            avatar_url: Some(String::from("avatar_url")),
            description: Some(String::from("description")),
            followers_count: 100,
            follows_count: 100,
            posts_count: 100,
            user_score: Some(200),
            last_post_cid: None,
            last_updated_at: 0,
            last_checked_at: 0,
        };
        insert_news_bsky_user(db_pool, news_bsky_user_retweeted)
            .await
            .unwrap();
    }

    pub async fn create_fake_news_bsky_post(db_pool: &PgPool, created_at_timestamp: i64) {
        truncate_news_bsky_post(db_pool).await.unwrap();
        let news_bsky_post = NewsBskyPost {
            post_uri: String::from("at://did:plc:user1/app.bsky.feed.post/rkey1"),
            cid: String::from("cid1"),
            text: String::from("post_text"),
            author_did: String::from("did:plc:user1"),
            reply_parent_uri: None,
            reply_root_uri: None,
            created_at: created_at_timestamp,
            created_at_str: String::from("created_at_str"),
        };
        insert_news_bsky_post(db_pool, news_bsky_post).await.unwrap();

        let news_bsky_post_quoted = NewsBskyPost {
            post_uri: String::from("at://did:plc:user1/app.bsky.feed.post/rkey3"),
            cid: String::from("cid3"),
            text: String::from("quoted_post_text"),
            author_did: String::from("did:plc:user1"),
            reply_parent_uri: None,
            reply_root_uri: None,
            created_at: created_at_timestamp - 100,
            created_at_str: String::from("created_at_str"),
        };
        insert_news_bsky_post(db_pool, news_bsky_post_quoted)
            .await
            .unwrap();
    }

    pub async fn create_fake_news_bsky_referenced_post_url(db_pool: &PgPool) {
        truncate_news_bsky_referenced_post_url(db_pool)
            .await
            .unwrap();

        let news_bsky_referenced_post_url = NewsBskyReferencedPostUrl {
            post_uri: String::from("at://did:plc:user1/app.bsky.feed.post/rkey1"),
            url_id: 1,
        };
        insert_news_bsky_referenced_post_url(db_pool, news_bsky_referenced_post_url)
            .await
            .unwrap();

        let news_bsky_referenced_post_url_quoted = NewsBskyReferencedPostUrl {
            post_uri: String::from("at://did:plc:user1/app.bsky.feed.post/rkey3"),
            url_id: 1,
        };
        insert_news_bsky_referenced_post_url(db_pool, news_bsky_referenced_post_url_quoted)
            .await
            .unwrap();
    }

    pub async fn create_fake_news_bsky_references(db_pool: &PgPool) {
        truncate_news_bsky_reference(db_pool).await.unwrap();

        let news_bsky_reference_repost = NewsBskyReference {
            post_uri: String::from("at://did:plc:user1/app.bsky.feed.post/rkey1"),
            ref_post_uri: String::from("did:plc:reposter"),
            ref_kind: String::from("repost"),
        };
        insert_news_bsky_reference(db_pool, news_bsky_reference_repost)
            .await
            .unwrap();
    }

    pub async fn create_fake_news_feed_url(
        db_pool: &PgPool,
        url_slug: String,
        url_id: i32,
        created_at_timestamp: i64,
        truncate: bool,
    ) {
        if truncate {
            truncate_news_feed_url(db_pool).await.unwrap();
        }
        let news_feed_url = NewsFeedUrl {
            url_slug,
            url_id,
            url_score: 90,
            num_references: 2,
            first_referenced_by: String::from("did:plc:user1"),
            is_climate_related: Some(true),
            created_at: created_at_timestamp,
            created_at_str: String::from("created_at_str"),
            bsky_posted_at: None,
            bsky_posted_at_str: None,
        };
        insert_news_feed_url(db_pool, news_feed_url).await.unwrap();
    }

    pub async fn create_fake_news_cron_job(db_pool: &PgPool, start_datetime: OffsetDateTime) {
        truncate_news_cron_job(db_pool).await.unwrap();

        let news_cron_job = NewsCronJob {
            cron_type: String::from("Main"),
            started_at: start_datetime.unix_timestamp(),
            started_at_str: datetime_to_str(start_datetime),
            completed_at: Some(start_datetime.unix_timestamp()),
            completed_at_str: Some(datetime_to_str(start_datetime)),
            error: None,
        };

        insert_news_cron_job(db_pool, news_cron_job).await.unwrap();
    }
}
