use crate::slack::send_tweet_cron_message;
use crate::twitter::api::post_tweet;
use crate::twitter::oauth::get_api_user_ctx;
use anyhow::Result;
use chrono::Utc;
use db::constants::{
    NEWS_FEED_MIN_NUM_SHARES_BEFORE_TWEETING, NEWS_FEED_URLS_LIMIT, NEWS_FEED_URLS_NUM_DAYS,
};
use db::models::news_cron_job::{CronType, NewsCronJob};
use db::queries::news_feed_url_query::NewsFeedUrlQuery;
use db::queries::news_feed_url_references_query::NewsFeedUrlReferencesQuery;
use db::sql::news_cron_job::{
    insert_news_cron_job, update_news_cron_job_completed_at, update_news_cron_job_error,
};
use db::sql::news_feed_url::update_news_feed_url_tweeted_at;
use db::sql::news_feed_url_query::get_news_feed_urls;
use db::sql::news_feed_url_references_query::get_news_feed_url_references;
use db::util::convert::{datetime_to_str, now_utc_datetime};
use db::util::db::init_db;
use db::util::string::concat_string;
use db::util::time::{past_days, now_formated};
use log::{debug, error, info, warn};
use sqlx::PgPool;
use tokio_schedule::{every, Job};

pub async fn start_tweet_scheduler() {
    let db_pool = init_db().await;
    let tweet_scheduler = every(2).hours().in_timezone(&Utc).perform(|| async {
        match start_tweet_cron_job(&db_pool).await {
            Ok(_) => {
                send_tweet_cron_message(format!("tweet_cron_job ended - {:?}", now_formated()));
            }
            Err(err) => {
                error!("tweet_cron_job failed: {:?}", err);
            }
        }
    });
    tweet_scheduler.await;
}

pub async fn start_tweet_cron_job(db_pool: &PgPool) -> anyhow::Result<()> {
    let start_datetime = now_utc_datetime();
    let news_cron_job = NewsCronJob {
        cron_type: CronType::Tweet.to_string(),
        started_at: start_datetime.unix_timestamp(),
        started_at_str: datetime_to_str(start_datetime),
        completed_at: None,
        completed_at_str: None,
        error: None,
    };

    let news_cron_job_db = insert_news_cron_job(db_pool, news_cron_job).await?;
    match tweet_cron_job(&db_pool).await {
        Ok(_) => {
            let completed_datetime = now_utc_datetime();
            update_news_cron_job_completed_at(
                &db_pool,
                news_cron_job_db.id,
                completed_datetime.unix_timestamp(),
                datetime_to_str(completed_datetime),
            )
            .await?;
        }
        Err(err) => {
            update_news_cron_job_error(&db_pool, news_cron_job_db.id, err.to_string()).await?;
            send_tweet_cron_message(format!("tweet_cron_job failed: {:?}", err));
        }
    }
    Ok(())
}

pub async fn tweet_cron_job(db_pool: &PgPool) -> Result<()> {
    info!("tweet_cron_job started - {:?}", now_formated());
    let recent_timestamp = past_days(NEWS_FEED_URLS_NUM_DAYS).unix_timestamp();

    match get_news_feed_urls(db_pool, recent_timestamp, NEWS_FEED_URLS_LIMIT).await {
        Ok(news_feed_urls) => {
            // The NewsFeedUrls not shared on Twitter yet
            let news_feed_urls_not_tweeted: Vec<NewsFeedUrlQuery> = news_feed_urls
                .into_iter()
                .filter(|nfu| {
                    nfu.tweeted_at.is_none()
                        && nfu.num_references >= NEWS_FEED_MIN_NUM_SHARES_BEFORE_TWEETING
                })
                .collect();
            match news_feed_urls_not_tweeted.first() {
                Some(news_feed_url) => {
                    info!(
                        "tweet_cron_job - news_feed_url found not shared on Twitter- {:?}",
                        news_feed_url
                    );

                    let news_feed_url_references_result =
                        get_news_feed_url_references(db_pool, news_feed_url.url_slug.clone()).await;

                    if let Some(news_feed_url_references_list) = news_feed_url_references_result {
                        // Sort by tweet created_at
                        let mut news_feed_url_references_list = news_feed_url_references_list;
                        news_feed_url_references_list
                            .sort_by(|a, b| a.created_at.partial_cmp(&b.created_at).unwrap());

                        let tweet_text =
                            get_tweet_text(news_feed_url, &news_feed_url_references_list);
                        if cfg!(debug_assertions) {
                            debug!("tweet_text - {}", tweet_text);
                        } else {
                            // Only post tweets in release mode
                            let api_user_ctx = get_api_user_ctx().await;
                            post_tweet(&api_user_ctx, tweet_text).await?;
                        }

                        //Update tweeted_at value
                        let now_utc_datetime = now_utc_datetime();
                        update_news_feed_url_tweeted_at(
                            db_pool,
                            news_feed_url.url_id,
                            now_utc_datetime.unix_timestamp(),
                            datetime_to_str(now_utc_datetime),
                        )
                        .await?;
                    } else {
                        error!("tweet_cron_job - news_feed_url_references not found");
                    }
                }
                None => {
                    warn!("tweet_cron_job - all news_feed_urls have been shared on Twitter");
                }
            }
        }
        Err(e) => {
            info!("tweet_cron_job - no news_feed_urls found - {:?}", e);
        }
    }

    Ok(())
}

pub fn get_tweet_text(
    news_feed_url: &NewsFeedUrlQuery,
    news_feed_url_references: &Vec<NewsFeedUrlReferencesQuery>,
) -> String {
    format!(
        r#"{}

More info: https://climatenews.app/news_feed/{}

{}

Article link: {}
#ClimateNews"#,
        news_feed_url.title,
        news_feed_url.url_slug,
        tweet_shared_by_text(news_feed_url_references),
        news_feed_url.expanded_url_parsed
    )
}

// TODO avoid duplicating this logic on web and backend
// Tweet shared by text
// Examples:
// 1  Shared by @user1
// 2  Shared by @user1 and @user2
// 3  Shared by @user1, @user2 and @user3
// 3+ Shared by @user1, @user2, @user3 and 5 others
pub fn tweet_shared_by_text(news_feed_url_references: &Vec<NewsFeedUrlReferencesQuery>) -> String {
    let mut shared_by_text = String::from("");
    for (i, news_feed_url_reference) in news_feed_url_references.iter().enumerate() {
        match i {
            0 => {
                shared_by_text = concat_string(
                    shared_by_text,
                    format!(
                        "Shared by @{}",
                        news_feed_url_reference.referenced_username.as_ref().unwrap()
                    ),
                );
            }
            1 => {
                let seperator = if news_feed_url_references.len() == 2 {
                    String::from(" and @")
                } else {
                    String::from(", @")
                };
                shared_by_text = concat_string(
                    shared_by_text,
                    format!(
                        "{}{}",
                        seperator,
                        news_feed_url_reference.referenced_username.as_ref().unwrap()
                    ),
                );
            }
            2 => {
                let seperator = if news_feed_url_references.len() == 3 {
                    String::from(" and @")
                } else {
                    String::from(", @")
                };
                let suffix = if news_feed_url_references.len() == 4 {
                    String::from(" and 1 other")
                } else if news_feed_url_references.len() > 4 {
                    format!(" and {} others", news_feed_url_references.len() - 3)
                } else {
                    String::from("")
                };
                shared_by_text = concat_string(
                    shared_by_text,
                    format!(
                        "{}{}{}",
                        seperator,
                        news_feed_url_reference.referenced_username.as_ref().unwrap(),
                        suffix
                    ),
                );
            }
            _ => {
                break;
            }
        }
    }

    shared_by_text
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_tweet_text() {
        let news_feed_url_query = NewsFeedUrlQuery {
            url_slug: String::from("example-slug"),
            url_id: 1,
            url_score: 100,
            num_references: 2,
            tweeted_at: None,
            first_referenced_by_username: String::from("climatenews_app"),
            created_at: 0,
            title: String::from("Example Title"),
            description: String::from("example description"),
            expanded_url_parsed: String::from("https://www.theguardian.com/environment/2022/dec/12/brazil-goldminers-carve-road-to-chaos-amazon-reserve"),
            expanded_url_host: String::from("https://www.theguardian.com"),
            display_url: String::from("https://www.theguardian.com"),
             preview_image_thumbnail_url: None,
             preview_image_url: None,
        };

        // Shared by 1 user
        let mut news_feed_url_references_list = vec![NewsFeedUrlReferencesQuery {
            url_id: 1,
            text: String::from("Example Title"),
            tweet_id: 1,
            author_id: 1,
            created_at: 0,
            created_at_str: String::from(""),
            username: Some(String::from("user1")),
            referenced_username: None,
            referenced_tweet_id: None,
            referenced_tweet_kind: None,
        }];

        assert_eq!(
            get_tweet_text(&news_feed_url_query, &news_feed_url_references_list),
            String::from("Example Title\n\nMore info: https://climatenews.app/news_feed/example-slug\n\nShared by @user1\n\nArticle link: https://www.theguardian.com/environment/2022/dec/12/brazil-goldminers-carve-road-to-chaos-amazon-reserve\n#ClimateNews")
        );

        // Shared by 2 users
        news_feed_url_references_list.push(NewsFeedUrlReferencesQuery {
            url_id: 1,
            text: String::from("Example Title"),
            tweet_id: 2,
            author_id: 2,
            created_at: 0,
            created_at_str: String::from(""),
            username: Some(String::from("user2")),
            referenced_username: None,
            referenced_tweet_id: None,
            referenced_tweet_kind: None,
        });

        assert_eq!(
                    get_tweet_text(&news_feed_url_query, &news_feed_url_references_list),
                    String::from("Example Title\n\nMore info: https://climatenews.app/news_feed/example-slug\n\nShared by @user1 and @user2\n\nArticle link: https://www.theguardian.com/environment/2022/dec/12/brazil-goldminers-carve-road-to-chaos-amazon-reserve\n#ClimateNews")
                );
        // Shared by 3 users
        news_feed_url_references_list.push(NewsFeedUrlReferencesQuery {
            url_id: 1,
            text: String::from("Example Title"),
            tweet_id: 2,
            author_id: 2,
            created_at: 0,
            created_at_str: String::from(""),
            username: Some(String::from("user3")),
            referenced_username: None,
            referenced_tweet_id: None,
            referenced_tweet_kind: None,
        });

        assert_eq!(
                    get_tweet_text(&news_feed_url_query, &news_feed_url_references_list),
                    String::from("Example Title\n\nMore info: https://climatenews.app/news_feed/example-slug\n\nShared by @user1, @user2 and @user3\n\nArticle link: https://www.theguardian.com/environment/2022/dec/12/brazil-goldminers-carve-road-to-chaos-amazon-reserve\n#ClimateNews")
                );

        // Shared by 4 users
        news_feed_url_references_list.push(NewsFeedUrlReferencesQuery {
            url_id: 1,
            text: String::from("Example Title"),
            tweet_id: 2,
            author_id: 2,
            created_at: 0,
            created_at_str: String::from(""),
            username: Some(String::from("user4")),
            referenced_username: None,
            referenced_tweet_id: None,
            referenced_tweet_kind: None,
        });

        assert_eq!(
                    get_tweet_text(&news_feed_url_query, &news_feed_url_references_list),
                    String::from("Example Title\n\nMore info: https://climatenews.app/news_feed/example-slug\n\nShared by @user1, @user2, @user3 and 1 other\n\nArticle link: https://www.theguardian.com/environment/2022/dec/12/brazil-goldminers-carve-road-to-chaos-amazon-reserve\n#ClimateNews")
                );

        // Shared by 5 users
        news_feed_url_references_list.push(NewsFeedUrlReferencesQuery {
            url_id: 1,
            text: String::from("Example Title"),
            tweet_id: 2,
            author_id: 2,
            created_at: 0,
            created_at_str: String::from(""),
            username: Some(String::from("user5")),
            referenced_username: None,
            referenced_tweet_id: None,
            referenced_tweet_kind: None,
        });

        assert_eq!(
                    get_tweet_text(&news_feed_url_query, &news_feed_url_references_list),
                    String::from("Example Title\n\nMore info: https://climatenews.app/news_feed/example-slug\n\nShared by @user1, @user2, @user3 and 2 others\n\nArticle link: https://www.theguardian.com/environment/2022/dec/12/brazil-goldminers-carve-road-to-chaos-amazon-reserve\n#ClimateNews")
                );
    }
}
