use crate::bluesky::api::{
    extract_created_at_from_record, get_actor_profile, get_author_feed, get_list, get_starter_pack,
    resolve_handle, GetListResponse,
};
use crate::bluesky::db::{
    parse_and_insert_bsky_post, parse_and_insert_bsky_user, parse_bsky_post_urls,
    parse_bsky_references,
};
use crate::bluesky::BlueskyAgent;
use crate::language::english_language_detector::EnglishLanguageDetector;
use crate::news_feed::constants::{MAX_BSKY_LIST_RESULTS, MAX_BSKY_POST_RESULTS};
use crate::util::{env_i64_or_default, env_usize, env_usize_or_default};
use anyhow::{anyhow, Result};
use db::sql::news_bsky_user::{
    find_all_news_bsky_users, update_news_bsky_user_last_checked_at,
    update_news_bsky_user_last_updated_at, update_news_bsky_user_stats,
};
use db::util::convert::now_utc_timestamp;
use db::util::time::now_formated;
use log::{error, info, warn};
use sqlx::PgPool;
use std::time::{Duration, Instant};

use super::constants::bsky_starter_pack_seeds;

const LIST_FETCH_MAX_ATTEMPTS: usize = 4;
const USER_PROGRESS_LOG_EVERY: usize = 25;
const RECENT_POST_MAX_AGE_HOURS: i64 = 24;
const DEFAULT_INITIAL_BACKFILL_MAX_PAGES: usize = 5;
const DEFAULT_INITIAL_BACKFILL_MAX_POST_AGE_DAYS: i64 = 30;

struct UserFeedFetchSummary {
    pages_fetched: usize,
    posts_seen: usize,
    reached_last_post: bool,
    reached_recent_cutoff: bool,
    initial_backfill: bool,
    reached_initial_backfill_age_cutoff: bool,
    reached_initial_backfill_page_cap: bool,
}

pub async fn get_all_bsky_posts(db_pool: &PgPool, bsky_agent: &BlueskyAgent) -> Result<()> {
    let started_at = Instant::now();
    info!("get_all_bsky_posts - {:?}", now_formated());
    fetch_bsky_users(db_pool, bsky_agent).await?;
    fetch_user_posts(db_pool, bsky_agent).await?;
    update_bsky_user_scores(db_pool).await?;
    info!(
        "get_all_bsky_posts complete - {:?} - elapsed_secs={}",
        now_formated(),
        started_at.elapsed().as_secs()
    );
    Ok(())
}

async fn fetch_bsky_users(db_pool: &PgPool, bsky_agent: &BlueskyAgent) -> Result<()> {
    info!("fetch_bsky_users");
    let started_at = Instant::now();
    let sources = bsky_starter_pack_seeds();
    let total_sources = sources.len();
    let mut total_users_upserted = 0usize;
    let mut source_failures = 0usize;

    for (source_idx, source) in sources.iter().enumerate() {
        info!(
            "fetch_bsky_users - source {}/{}: {}",
            source_idx + 1,
            total_sources,
            source.label
        );
        match fetch_users_from_starter_pack(
            db_pool,
            bsky_agent,
            source.label,
            source.starter_pack_url,
        )
        .await
        {
            Ok(num_users) => {
                total_users_upserted += num_users;
                info!(
                    "fetch_bsky_users - source: {} - added/updated {} users - cumulative_users={}",
                    source.label, num_users, total_users_upserted
                );
            }
            Err(err) => {
                source_failures += 1;
                error!(
                    "fetch_bsky_users - failed to read {} ({}): {:?}",
                    source.label, source.starter_pack_url, err
                );
            }
        }
    }

    info!(
        "fetch_bsky_users complete - sources={} failures={} users_added_or_updated={} elapsed_secs={}",
        total_sources,
        source_failures,
        total_users_upserted,
        started_at.elapsed().as_secs()
    );

    Ok(())
}

async fn fetch_users_from_starter_pack(
    db_pool: &PgPool,
    bsky_agent: &BlueskyAgent,
    source_label: &str,
    starter_pack_url: &str,
) -> Result<usize> {
    let starter_pack_uri = starter_pack_url_to_at_uri(bsky_agent, starter_pack_url).await?;
    let starter_pack_response = get_starter_pack(bsky_agent, &starter_pack_uri).await?;

    let list_uri = starter_pack_response
        .starter_pack
        .record
        .get("list")
        .and_then(|v| {
            if let Some(uri) = v.as_str() {
                return Some(uri.to_string());
            }

            v.get("uri")
                .and_then(|value| value.as_str())
                .map(|uri| uri.to_string())
        })
        .ok_or_else(|| {
            anyhow!(
                "Starter pack record is missing list URI: {}",
                starter_pack_uri
            )
        })?
        .to_string();

    let mut cursor: Option<String> = None;
    let mut total_users = 0;
    let mut page_num = 0usize;
    let started_at = Instant::now();

    loop {
        page_num += 1;
        let response = get_list_with_retries(
            bsky_agent,
            &list_uri,
            cursor.as_deref(),
            Some(MAX_BSKY_LIST_RESULTS),
        )
        .await?;

        let page_users = response.items.len();

        for item in &response.items {
            // List payloads often return a basic actor view without count stats.
            // Hydrate from getProfile so follower/following/post counts are filled.
            let actor = match get_actor_profile(bsky_agent, &item.subject.did).await {
                Ok(profile) => profile,
                Err(err) => {
                    error!(
                        "fetch_users_from_starter_pack - get_actor_profile failed for {}: {:?}",
                        item.subject.did, err
                    );
                    item.subject.clone()
                }
            };

            parse_and_insert_bsky_user(db_pool, &actor).await?;
            total_users += 1;
        }

        info!(
            "fetch_bsky_users - source: {} - list_page={} page_users={} cumulative_users={} cursor_present={}",
            source_label,
            page_num,
            page_users,
            total_users,
            response.cursor.is_some()
        );

        let next_cursor = response.cursor.clone();
        if response.items.is_empty() || (next_cursor.is_some() && next_cursor == cursor) {
            break;
        }

        match next_cursor {
            Some(next_cursor) => cursor = Some(next_cursor),
            None => break,
        }
    }

    info!(
        "fetch_bsky_users - source: {} - complete pages={} users={} elapsed_secs={}",
        source_label,
        page_num,
        total_users,
        started_at.elapsed().as_secs()
    );

    Ok(total_users)
}

async fn get_list_with_retries(
    bsky_agent: &BlueskyAgent,
    list_uri: &str,
    cursor: Option<&str>,
    limit: Option<u32>,
) -> Result<GetListResponse> {
    let mut attempt = 0;
    loop {
        attempt += 1;
        match get_list(bsky_agent, list_uri, cursor, limit).await {
            Ok(response) => return Ok(response),
            Err(err) => {
                if attempt >= LIST_FETCH_MAX_ATTEMPTS {
                    return Err(err);
                }

                let backoff_secs = attempt as u64;
                error!(
                    "get_list attempt {}/{} failed for {} (cursor={:?}): {:?}; retrying in {}s",
                    attempt, LIST_FETCH_MAX_ATTEMPTS, list_uri, cursor, err, backoff_secs
                );
                tokio::time::sleep(Duration::from_secs(backoff_secs)).await;
            }
        }
    }
}

async fn starter_pack_url_to_at_uri(
    bsky_agent: &BlueskyAgent,
    starter_pack_url: &str,
) -> Result<String> {
    if starter_pack_url.starts_with("at://") {
        return Ok(starter_pack_url.to_string());
    }

    let cleaned_url = starter_pack_url.trim_end_matches('/');
    let parts: Vec<&str> = cleaned_url.split('/').collect();
    let Some(starter_pack_idx) = parts.iter().position(|part| *part == "starter-pack") else {
        return Err(anyhow!("Invalid starter pack URL: {}", starter_pack_url));
    };

    if starter_pack_idx + 2 >= parts.len() {
        return Err(anyhow!("Invalid starter pack URL: {}", starter_pack_url));
    }

    let owner = parts[starter_pack_idx + 1];
    let rkey = parts[starter_pack_idx + 2].split('?').next().unwrap_or("");

    if rkey.is_empty() {
        return Err(anyhow!(
            "Invalid starter pack URL (missing rkey): {}",
            starter_pack_url
        ));
    }

    let owner_did = if owner.starts_with("did:") {
        owner.to_string()
    } else {
        resolve_handle(bsky_agent, owner).await?
    };

    Ok(format!(
        "at://{}/app.bsky.graph.starterpack/{}",
        owner_did, rkey
    ))
}

async fn fetch_user_posts(db_pool: &PgPool, bsky_agent: &BlueskyAgent) -> Result<()> {
    info!("fetch_user_posts - {:?}", now_formated());
    let started_at = Instant::now();
    let english_detector = EnglishLanguageDetector::init();

    let users = find_all_news_bsky_users(db_pool).await?;
    let total_users = users.len();
    let max_users_per_run = env_usize("BSKY_MAX_USERS_PER_RUN");
    let users_to_process = max_users_per_run
        .map(|limit| limit.min(total_users))
        .unwrap_or(total_users);

    if let Some(limit) = max_users_per_run {
        info!(
            "fetch_user_posts - user cap enabled via BSKY_MAX_USERS_PER_RUN={} (processing {} of {} users)",
            limit,
            users_to_process,
            total_users
        );
    }

    info!(
        "fetch_user_posts - users_available={} users_processing={}",
        total_users, users_to_process
    );

    let mut succeeded = 0usize;
    let mut failed = 0usize;
    let mut total_posts_seen = 0usize;
    let mut total_pages_fetched = 0usize;

    for (idx, user) in users.iter().take(users_to_process).enumerate() {
        let user_started_at = Instant::now();
        match fetch_user_feed(db_pool, bsky_agent, &english_detector, user).await {
            Ok(summary) => {
                succeeded += 1;
                total_posts_seen += summary.posts_seen;
                total_pages_fetched += summary.pages_fetched;
                info!(
                    "fetch_user_posts - user {}/{} handle={} pages={} posts_seen={} reached_last_post={} recent_cutoff_hit={} initial_backfill={} age_cutoff_hit={} page_cap_hit={} elapsed_ms={}",
                    idx + 1,
                    total_users,
                    user.handle,
                    summary.pages_fetched,
                    summary.posts_seen,
                    summary.reached_last_post,
                    summary.reached_recent_cutoff,
                    summary.initial_backfill,
                    summary.reached_initial_backfill_age_cutoff,
                    summary.reached_initial_backfill_page_cap,
                    user_started_at.elapsed().as_millis()
                );
            }
            Err(err) => {
                failed += 1;
                error!("fetch_user_feed failed for {}: {:?}", user.handle, err);
            }
        }
        update_news_bsky_user_last_checked_at(db_pool, user.did.clone(), now_utc_timestamp())
            .await?;

        let processed = idx + 1;
        if processed % USER_PROGRESS_LOG_EVERY == 0 || processed == users_to_process {
            info!(
                "fetch_user_posts progress - processed={}/{} succeeded={} failed={} pages={} posts_seen={} elapsed_secs={}",
                processed,
                users_to_process,
                succeeded,
                failed,
                total_pages_fetched,
                total_posts_seen,
                started_at.elapsed().as_secs()
            );
        }
    }

    info!(
        "fetch_user_posts complete - users_processed={} users_available={} succeeded={} failed={} pages={} posts_seen={} elapsed_secs={}",
        users_to_process,
        total_users,
        succeeded,
        failed,
        total_pages_fetched,
        total_posts_seen,
        started_at.elapsed().as_secs()
    );

    Ok(())
}

async fn fetch_user_feed(
    db_pool: &PgPool,
    bsky_agent: &BlueskyAgent,
    english_detector: &EnglishLanguageDetector,
    user: &db::models::news_bsky_user::NewsBskyUser,
) -> Result<UserFeedFetchSummary> {
    let last_post_cid = user.last_post_cid.clone();
    let recent_cutoff_ts = chrono::Utc::now().timestamp() - (RECENT_POST_MAX_AGE_HOURS * 60 * 60);
    let initial_backfill = last_post_cid.is_none();
    let initial_backfill_max_pages = env_usize_or_default(
        "BSKY_INITIAL_BACKFILL_MAX_PAGES",
        DEFAULT_INITIAL_BACKFILL_MAX_PAGES,
    );
    let initial_backfill_max_post_age_days = env_i64_or_default(
        "BSKY_INITIAL_BACKFILL_MAX_POST_AGE_DAYS",
        DEFAULT_INITIAL_BACKFILL_MAX_POST_AGE_DAYS,
    );
    let initial_backfill_cutoff_ts =
        chrono::Utc::now().timestamp() - (initial_backfill_max_post_age_days * 24 * 60 * 60);

    let mut cursor: Option<String> = None;
    let mut newest_cid: Option<String> = None;
    let mut reached_last_post = false;
    let mut reached_recent_cutoff = false;
    let mut reached_initial_backfill_age_cutoff = false;
    let mut reached_initial_backfill_page_cap = false;
    let mut pages_fetched = 0usize;
    let mut posts_seen = 0usize;

    loop {
        let response = get_author_feed(
            bsky_agent,
            &user.did,
            cursor.as_deref(),
            Some(MAX_BSKY_POST_RESULTS),
        )
        .await?;
        pages_fetched += 1;

        for feed_item in &response.feed {
            if let Some(record) = &feed_item.post.record {
                if let Some(created_at) = extract_created_at_from_record(record) {
                    if let Ok(created_at_dt) = chrono::DateTime::parse_from_rfc3339(&created_at) {
                        if created_at_dt.timestamp() < recent_cutoff_ts {
                            reached_recent_cutoff = true;
                            break;
                        }
                    }
                }
            }

            if initial_backfill {
                if let Some(record) = &feed_item.post.record {
                    if let Some(created_at) = extract_created_at_from_record(record) {
                        if let Ok(created_at_dt) = chrono::DateTime::parse_from_rfc3339(&created_at)
                        {
                            if created_at_dt.timestamp() < initial_backfill_cutoff_ts {
                                reached_initial_backfill_age_cutoff = true;
                                break;
                            }
                        }
                    }
                }
            }

            posts_seen += 1;
            if newest_cid.is_none() {
                if let Some(cid) = &feed_item.post.cid {
                    newest_cid = Some(cid.clone());
                }
            }

            parse_and_insert_bsky_user(db_pool, &feed_item.post.author).await?;
            parse_and_insert_bsky_post(db_pool, &feed_item.post).await?;
            parse_bsky_post_urls(db_pool, &feed_item.post, english_detector).await?;
            parse_bsky_references(db_pool, feed_item).await?;

            // Stop paginating once we reach the last post we already have
            if let Some(last_cid) = &last_post_cid {
                if feed_item.post.cid.as_ref() == Some(last_cid) {
                    reached_last_post = true;
                    break;
                }
            }
        }

        if initial_backfill && pages_fetched >= initial_backfill_max_pages {
            reached_initial_backfill_page_cap = true;
            break;
        }

        if reached_initial_backfill_age_cutoff {
            break;
        }

        if reached_recent_cutoff {
            break;
        }

        // Guard against an infinite loop if the API returns a non-advancing
        // cursor or an empty feed
        let next_cursor = response.cursor.clone();
        if response.feed.is_empty() || (next_cursor.is_some() && next_cursor == cursor) {
            break;
        }

        match next_cursor {
            Some(next_cursor) => cursor = Some(next_cursor),
            None => break,
        }

        if reached_last_post {
            break;
        }
    }

    // Advance the incremental fetch watermark to the newest post seen
    if let Some(cid) = newest_cid {
        update_news_bsky_user_last_updated_at(
            db_pool,
            user.did.clone(),
            Some(cid),
            now_utc_timestamp(),
        )
        .await?;
    }

    if reached_initial_backfill_age_cutoff {
        info!(
            "fetch_user_feed - initial_backfill age cutoff reached for handle={} cutoff_days={} pages={} posts_seen={}",
            user.handle,
            initial_backfill_max_post_age_days,
            pages_fetched,
            posts_seen
        );
    }

    if reached_recent_cutoff {
        info!(
            "fetch_user_feed - recent cutoff reached for handle={} cutoff_hours={} pages={} posts_seen={}",
            user.handle,
            RECENT_POST_MAX_AGE_HOURS,
            pages_fetched,
            posts_seen
        );
    }

    if reached_initial_backfill_page_cap {
        info!(
            "fetch_user_feed - initial_backfill page cap reached for handle={} page_cap={} pages={} posts_seen={}",
            user.handle,
            initial_backfill_max_pages,
            pages_fetched,
            posts_seen
        );
    }

    Ok(UserFeedFetchSummary {
        pages_fetched,
        posts_seen,
        reached_last_post,
        reached_recent_cutoff,
        initial_backfill,
        reached_initial_backfill_age_cutoff,
        reached_initial_backfill_page_cap,
    })
}

async fn update_bsky_user_scores(db_pool: &PgPool) -> Result<()> {
    info!("update_bsky_user_scores started - {:?}", now_formated());
    let users = find_all_news_bsky_users(db_pool).await?;
    info!("update_bsky_user_scores - users_to_update={}", users.len());

    let mut updated = 0usize;
    let mut failed = 0usize;

    for user in users {
        let score = (user.followers_count / 1000).max(1);
        match update_news_bsky_user_stats(db_pool, user.did.clone(), score).await {
            Ok(_) => {
                updated += 1;
            }
            Err(err) => {
                failed += 1;
                warn!(
                    "update_bsky_user_scores - failed did={} handle={} err={:?}",
                    user.did, user.handle, err
                );
            }
        }
    }

    info!(
        "update_bsky_user_scores complete - updated={} failed={} at {:?}",
        updated,
        failed,
        now_formated()
    );

    if failed > 0 {
        return Err(anyhow!(
            "update_bsky_user_scores failed for {} users",
            failed
        ));
    }

    Ok(())
}
