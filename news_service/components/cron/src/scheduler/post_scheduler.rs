use crate::bluesky::api::create_post;
use crate::bluesky::init_bluesky_agent;
use crate::slack::send_post_cron_message;
use anyhow::Result;
use chrono::Utc;
use db::constants::{
    MAX_POST_CHARACTER_COUNT, NEWS_FEED_MIN_NUM_SHARES_BEFORE_POSTING, NEWS_FEED_URLS_LIMIT,
    NEWS_FEED_URLS_NUM_DAYS, POST_URL_PLACEHOLDER_LENGTH,
};
use db::models::news_cron_job::{CronType, NewsCronJob};
use db::queries::news_feed_url_query::NewsFeedUrlQuery;
use db::queries::news_feed_url_references_query::NewsFeedUrlReferencesQuery;
use db::sql::news_cron_job::{
    insert_news_cron_job, update_news_cron_job_completed_at, update_news_cron_job_error,
};
use db::sql::news_feed_url::update_news_feed_url_bsky_posted_at;
use db::sql::news_feed_url_query::get_news_feed_urls;
use db::sql::news_feed_url_references_query::get_news_feed_url_references;
use db::util::convert::{datetime_to_str, now_utc_datetime};
use db::util::db::init_db_result;
use db::util::string::concat_string;
use db::util::time::{now_formated, past_days};
use itertools::Itertools;
use log::{debug, error, info, warn};
use sqlx::PgPool;
use tokio_schedule::{every, Job};

pub async fn start_post_scheduler() {
    let post_scheduler = every(2).hours().in_timezone(&Utc).perform(|| async {
        send_post_cron_message(format!("post_scheduler started - {:?}", now_formated())).await;
        match init_post_cron_job().await {
            Ok(_) => {
                send_post_cron_message(format!(
                    "init_post_cron_job success - {:?}",
                    now_formated()
                )).await;
            }
            Err(err) => {
                send_post_cron_message(format!("init_post_cron_job error - {:?}", err)).await;
            }
        }
    });
    post_scheduler.await;
}

pub async fn init_post_cron_job() -> Result<()> {
    let db_pool = init_db_result().await?;
    match start_post_cron_job(&db_pool).await {
        Ok(_) => {
            send_post_cron_message(format!("start_post_cron_job ended - {:?}", now_formated())).await;
        }
        Err(err) => {
            send_post_cron_message(format!("start_post_cron_job failed: {:?}", err)).await;
        }
    }
    db_pool.close().await;
    Ok(())
}

pub async fn start_post_cron_job(db_pool: &PgPool) -> anyhow::Result<()> {
    let start_datetime = now_utc_datetime();
    let news_cron_job = NewsCronJob {
        cron_type: CronType::Post.to_string(),
        started_at: start_datetime.unix_timestamp(),
        started_at_str: datetime_to_str(start_datetime),
        completed_at: None,
        completed_at_str: None,
        error: None,
    };

    let news_cron_job_db = insert_news_cron_job(db_pool, news_cron_job).await?;
    match post_cron_job(&db_pool).await {
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
            send_post_cron_message(format!("post_cron_job #2 failed: {:?}", err)).await;
        }
    }
    Ok(())
}

pub async fn post_cron_job(db_pool: &PgPool) -> Result<()> {
    info!("post_cron_job started - {:?}", now_formated());
    let recent_timestamp = past_days(NEWS_FEED_URLS_NUM_DAYS).unix_timestamp();

    match get_news_feed_urls(db_pool, recent_timestamp, NEWS_FEED_URLS_LIMIT).await {
        Ok(news_feed_urls) => {
            let news_feed_urls_not_posted: Vec<NewsFeedUrlQuery> = news_feed_urls
                .into_iter()
                .filter(|nfu| {
                    nfu.bsky_posted_at.is_none()
                        && nfu.num_references >= NEWS_FEED_MIN_NUM_SHARES_BEFORE_POSTING
                })
                .collect();
            match news_feed_urls_not_posted.first() {
                Some(news_feed_url) => {
                    info!(
                        "post_cron_job - news_feed_url found not shared to Bluesky - {:?}",
                        news_feed_url
                    );

                    let news_feed_url_references_result =
                        get_news_feed_url_references(db_pool, news_feed_url.url_slug.clone()).await;

                    if let Ok(news_feed_url_references_list) = news_feed_url_references_result {
                        let mut news_feed_url_references_list = news_feed_url_references_list;
                        news_feed_url_references_list
                            .sort_by(|a, b| a.created_at.partial_cmp(&b.created_at).unwrap());

                        let post_text: String = if get_post_text_long_len(
                            news_feed_url,
                            &news_feed_url_references_list,
                        ) <= MAX_POST_CHARACTER_COUNT
                        {
                            get_post_text_long(news_feed_url, &news_feed_url_references_list)
                        } else {
                            get_post_text_short(news_feed_url)
                        };

                        if cfg!(debug_assertions) {
                            // Don't post (or mark as posted) in debug mode so the queue is not drained
                            debug!("post_text - {}", post_text);
                        } else {
                            // Only post in release mode
                            match init_bluesky_agent().await {
                                Ok(bsky_agent) => {
                                    create_post(&bsky_agent, post_text, None).await?;
                                    let now = now_utc_datetime();
                                    update_news_feed_url_bsky_posted_at(
                                        db_pool,
                                        news_feed_url.url_id,
                                        now.unix_timestamp(),
                                        datetime_to_str(now),
                                    )
                                    .await?;
                                }
                                Err(e) => {
                                    error!("init_bluesky_agent error: {:?}", e);
                                }
                            }
                        }
                    } else {
                        error!("post_cron_job - news_feed_url_references not found");
                    }
                }
                None => {
                    warn!("post_cron_job - all news_feed_urls have been posted to Bluesky");
                }
            }
        }
        Err(e) => {
            info!("post_cron_job - no news_feed_urls found - {:?}", e);
        }
    }

    Ok(())
}

pub fn get_post_text_long_len(
    news_feed_url: &NewsFeedUrlQuery,
    news_feed_url_references: &Vec<NewsFeedUrlReferencesQuery>,
) -> usize {
    let post_text = format!(
        r#"{}

Posts:

{}

Article link: "#,
        news_feed_url.title.clone().unwrap_or_default(),
        post_shared_by_text(news_feed_url_references),
    );
    post_text.len() + (POST_URL_PLACEHOLDER_LENGTH * 2)
}

pub fn get_post_text_long(
    news_feed_url: &NewsFeedUrlQuery,
    news_feed_url_references: &Vec<NewsFeedUrlReferencesQuery>,
) -> String {
    format!(
        r#"{}

Posts: https://climatenews.app/news_feed/{}

{}

Article link: {}"#,
        news_feed_url.title.clone().unwrap_or_default(),
        news_feed_url.url_slug,
        post_shared_by_text(news_feed_url_references),
        news_feed_url.expanded_url_parsed
    )
}

pub fn get_post_text_short(news_feed_url: &NewsFeedUrlQuery) -> String {
    format!(
        r#"{}

Posts: https://climatenews.app/news_feed/{}

Article link: {}"#,
        news_feed_url.title.clone().unwrap_or_default(),
        news_feed_url.url_slug,
        news_feed_url.expanded_url_parsed
    )
}

// TODO avoid duplicating this logic on web and backend
// Post shared by text
// Examples:
// 1  Shared by @user1
// 2  Shared by @user1 and @user2
// 3  Shared by @user1, @user2 and @user3
// 3+ Shared by @user1, @user2, @user3 and 5 others
pub fn post_shared_by_text(news_feed_url_references: &Vec<NewsFeedUrlReferencesQuery>) -> String {
    let mut shared_by_text = String::from("");

    let unique_referenced_handles = get_unique_referenced_handles(news_feed_url_references);
    for (i, referenced_handle) in unique_referenced_handles.iter().enumerate() {
        match i {
            0 => {
                shared_by_text = concat_string(
                    shared_by_text,
                    format!("Shared by @{}", referenced_handle),
                );
            }
            1 => {
                let seperator = if unique_referenced_handles.len() == 2 {
                    String::from(" and @")
                } else {
                    String::from(", @")
                };
                shared_by_text = concat_string(
                    shared_by_text,
                    format!("{}{}", seperator, referenced_handle),
                );
            }
            2 => {
                let seperator = if unique_referenced_handles.len() == 3 {
                    String::from(" and @")
                } else {
                    String::from(", @")
                };
                let suffix = if unique_referenced_handles.len() == 4 {
                    String::from(" and 1 other")
                } else if unique_referenced_handles.len() > 4 {
                    format!(" and {} others", unique_referenced_handles.len() - 3)
                } else {
                    String::from("")
                };
                shared_by_text = concat_string(
                    shared_by_text,
                    format!("{}{}{}", seperator, referenced_handle, suffix),
                );
            }
            _ => {
                break;
            }
        }
    }

    shared_by_text
}

fn get_unique_referenced_handles(
    news_feed_url_references: &Vec<NewsFeedUrlReferencesQuery>,
) -> Vec<String> {
    news_feed_url_references
        .iter()
        .filter_map(|nfur| nfur.author_handle.clone())
        .unique()
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[derive(Debug, PartialEq)]
    pub struct TestStruct {
        pub name: Option<String>,
    }

    #[test]
    fn test_unique_struct_values() {
        // Test to ensure a list of unique names can be extracted from a vec of structs
        let data = vec![
            TestStruct {
                name: Some(String::from("a")),
            },
            TestStruct {
                name: Some(String::from("a")),
            },
        ];

        let unique_names: Vec<String> = data
            .iter()
            .map(|t| t.name.clone())
            .filter_map(|name| name)
            .unique()
            .collect();
        assert_eq!(unique_names, vec![String::from("a")]);
    }

    #[test]
    fn test_get_post_text() {
        let news_feed_url_query = NewsFeedUrlQuery {
            url_slug: String::from("example-slug"),
            url_id: 1,
            url_score: 100,
            num_references: 2,
            bsky_posted_at: None,
            first_referenced_by_username: String::from("user1.bsky.social"),
            created_at: 0,
            title: Some(String::from("Example Title")),
            description: Some(String::from("example description")),
            expanded_url_parsed: String::from("https://www.theguardian.com/environment/2022/dec/12/brazil-goldminers-carve-road-to-chaos-amazon-reserve"),
            expanded_url_host: String::from("www.theguardian.com"),
            display_url: Some(String::from("theguardian.com")),
            preview_image_thumbnail_url: None,
            preview_image_url: None,
        };

        // Shared by 1 user
        let mut news_feed_url_references_list = vec![NewsFeedUrlReferencesQuery {
            url_id: 1,
            text: String::from("Example Title"),
            post_uri: String::from("at://did:plc:user1/app.bsky.feed.post/rkey1"),
            author_did: String::from("did:plc:user1"),
            author_handle: Some(String::from("user1.bsky.social")),
            created_at: 0,
            created_at_str: String::from(""),
        }];

        assert_eq!(
            get_post_text_long(&news_feed_url_query, &news_feed_url_references_list),
            String::from("Example Title\n\nPosts: https://climatenews.app/news_feed/example-slug\n\nShared by @user1.bsky.social\n\nArticle link: https://www.theguardian.com/environment/2022/dec/12/brazil-goldminers-carve-road-to-chaos-amazon-reserve")
        );

        assert_eq!(
            get_post_text_long_len(&news_feed_url_query, &news_feed_url_references_list),
            107
        );

        // Shared by 2 users
        news_feed_url_references_list.push(NewsFeedUrlReferencesQuery {
            url_id: 1,
            text: String::from("Example Title"),
            post_uri: String::from("at://did:plc:user2/app.bsky.feed.post/rkey2"),
            author_did: String::from("did:plc:user2"),
            author_handle: Some(String::from("user2.bsky.social")),
            created_at: 0,
            created_at_str: String::from(""),
        });

        assert_eq!(
            get_post_text_long(&news_feed_url_query, &news_feed_url_references_list),
                    String::from("Example Title\n\nPosts: https://climatenews.app/news_feed/example-slug\n\nShared by @user1.bsky.social and @user2.bsky.social\n\nArticle link: https://www.theguardian.com/environment/2022/dec/12/brazil-goldminers-carve-road-to-chaos-amazon-reserve")
                );
        // Shared by 3 users
        news_feed_url_references_list.push(NewsFeedUrlReferencesQuery {
            url_id: 1,
            text: String::from("Example Title"),
            post_uri: String::from("at://did:plc:user3/app.bsky.feed.post/rkey3"),
            author_did: String::from("did:plc:user3"),
            author_handle: Some(String::from("user3.bsky.social")),
            created_at: 0,
            created_at_str: String::from(""),
        });

        assert_eq!(
            get_post_text_long(&news_feed_url_query, &news_feed_url_references_list),
                    String::from("Example Title\n\nPosts: https://climatenews.app/news_feed/example-slug\n\nShared by @user1.bsky.social, @user2.bsky.social and @user3.bsky.social\n\nArticle link: https://www.theguardian.com/environment/2022/dec/12/brazil-goldminers-carve-road-to-chaos-amazon-reserve")
                );

        // Shared by 4 users
        news_feed_url_references_list.push(NewsFeedUrlReferencesQuery {
            url_id: 1,
            text: String::from("Example Title"),
            post_uri: String::from("at://did:plc:user4/app.bsky.feed.post/rkey4"),
            author_did: String::from("did:plc:user4"),
            author_handle: Some(String::from("user4.bsky.social")),
            created_at: 0,
            created_at_str: String::from(""),
        });

        assert_eq!(
                get_post_text_long(&news_feed_url_query, &news_feed_url_references_list),
                    String::from("Example Title\n\nPosts: https://climatenews.app/news_feed/example-slug\n\nShared by @user1.bsky.social, @user2.bsky.social, @user3.bsky.social and 1 other\n\nArticle link: https://www.theguardian.com/environment/2022/dec/12/brazil-goldminers-carve-road-to-chaos-amazon-reserve")
                );

        // Shared by 5 users
        news_feed_url_references_list.push(NewsFeedUrlReferencesQuery {
            url_id: 1,
            text: String::from("Example Title"),
            post_uri: String::from("at://did:plc:user5/app.bsky.feed.post/rkey5"),
            author_did: String::from("did:plc:user5"),
            author_handle: Some(String::from("user5.bsky.social")),
            created_at: 0,
            created_at_str: String::from(""),
        });

        assert_eq!(
                    get_post_text_long(&news_feed_url_query, &news_feed_url_references_list),
                    String::from("Example Title\n\nPosts: https://climatenews.app/news_feed/example-slug\n\nShared by @user1.bsky.social, @user2.bsky.social, @user3.bsky.social and 2 others\n\nArticle link: https://www.theguardian.com/environment/2022/dec/12/brazil-goldminers-carve-road-to-chaos-amazon-reserve")
                );

        // Shared by 5 users with duplicate handle
        news_feed_url_references_list.push(NewsFeedUrlReferencesQuery {
            url_id: 1,
            text: String::from("Example Title"),
            post_uri: String::from("at://did:plc:user5/app.bsky.feed.post/rkey5"),
            author_did: String::from("did:plc:user5"),
            author_handle: Some(String::from("user5.bsky.social")),
            created_at: 0,
            created_at_str: String::from(""),
        });

        let unique_referenced_handles = get_unique_referenced_handles(&news_feed_url_references_list);
        assert_eq!(
            unique_referenced_handles,
            vec![
                String::from("user1.bsky.social"),
                String::from("user2.bsky.social"),
                String::from("user3.bsky.social"),
                String::from("user4.bsky.social"),
                String::from("user5.bsky.social")
            ]
        );
        assert_eq!(
                    get_post_text_long(&news_feed_url_query, &news_feed_url_references_list),
                    String::from("Example Title\n\nPosts: https://climatenews.app/news_feed/example-slug\n\nShared by @user1.bsky.social, @user2.bsky.social, @user3.bsky.social and 2 others\n\nArticle link: https://www.theguardian.com/environment/2022/dec/12/brazil-goldminers-carve-road-to-chaos-amazon-reserve")
                );
        assert_eq!(
            get_post_text_short(&news_feed_url_query),
            String::from("Example Title\n\nPosts: https://climatenews.app/news_feed/example-slug\n\nArticle link: https://www.theguardian.com/environment/2022/dec/12/brazil-goldminers-carve-road-to-chaos-amazon-reserve")
        );
    }
}
