use crate::bluesky::api::{
    get_actor_profile, get_author_feed, get_list, get_starter_pack,
    resolve_handle, GetListResponse,
};
use crate::bluesky::db::{
    parse_and_insert_bsky_post, parse_and_insert_bsky_user, parse_bsky_post_urls,
    parse_bsky_references,
};
use crate::bluesky::BlueskyAgent;
use crate::language::english_language_detector::EnglishLanguageDetector;
use crate::news_feed::constants::{
    MAX_BSKY_LIST_RESULTS, MAX_BSKY_POST_RESULTS,
};
use anyhow::{anyhow, Result};
use db::sql::news_bsky_user::{
    find_all_news_bsky_users, update_news_bsky_user_last_checked_at,
    update_news_bsky_user_last_updated_at, update_news_bsky_user_stats,
};
use db::util::convert::now_utc_timestamp;
use db::util::time::now_formated;
use log::{error, info, warn};
use sqlx::PgPool;
use std::time::Duration;

use super::constants::bsky_starter_pack_seeds;

const LIST_FETCH_MAX_ATTEMPTS: usize = 4;

pub async fn get_all_bsky_posts(
    db_pool: &PgPool,
    bsky_agent: &BlueskyAgent,
) -> Result<()> {
    info!("get_all_bsky_posts - {:?}", now_formated());
    fetch_bsky_users(db_pool, bsky_agent).await?;
    fetch_user_posts(db_pool, bsky_agent).await?;
    update_bsky_user_scores(db_pool).await?;
    info!("get_all_bsky_posts complete - {:?}", now_formated());
    Ok(())
}

async fn fetch_bsky_users(
    db_pool: &PgPool,
    bsky_agent: &BlueskyAgent,
) -> Result<()> {
    info!("fetch_bsky_users");

    for source in bsky_starter_pack_seeds() {
        info!("fetch_bsky_users - source: {}", source.label);
        match fetch_users_from_starter_pack(db_pool, bsky_agent, source.starter_pack_url).await {
            Ok(num_users) => {
                info!(
                    "fetch_bsky_users - source: {} - added/updated {} users",
                    source.label, num_users
                );
            }
            Err(err) => {
                error!(
                    "fetch_bsky_users - failed to read {} ({}): {:?}",
                    source.label, source.starter_pack_url, err
                );
            }
        }
    }

    Ok(())
}

async fn fetch_users_from_starter_pack(
    db_pool: &PgPool,
    bsky_agent: &BlueskyAgent,
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
        .ok_or_else(|| anyhow!("Starter pack record is missing list URI: {}", starter_pack_uri))?
        .to_string();

    let mut cursor: Option<String> = None;
    let mut total_users = 0;

    loop {
        let response = get_list_with_retries(
            bsky_agent,
            &list_uri,
            cursor.as_deref(),
            Some(MAX_BSKY_LIST_RESULTS),
        )
        .await?;

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

        let next_cursor = response.cursor.clone();
        if response.items.is_empty() || (next_cursor.is_some() && next_cursor == cursor) {
            break;
        }

        match next_cursor {
            Some(next_cursor) => cursor = Some(next_cursor),
            None => break,
        }
    }

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
                    attempt,
                    LIST_FETCH_MAX_ATTEMPTS,
                    list_uri,
                    cursor,
                    err,
                    backoff_secs
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
        return Err(anyhow!("Invalid starter pack URL (missing rkey): {}", starter_pack_url));
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

async fn fetch_user_posts(
    db_pool: &PgPool,
    bsky_agent: &BlueskyAgent,
) -> Result<()> {
    info!("fetch_user_posts - {:?}", now_formated());
    let english_detector = EnglishLanguageDetector::init();

    let users = find_all_news_bsky_users(db_pool).await?;
    info!(
        "fetch_user_posts - num_users - {:?}",
        users.len()
    );

    for user in &users {
        match fetch_user_feed(db_pool, bsky_agent, &english_detector, user).await {
            Ok(_) => {}
            Err(err) => {
                error!("fetch_user_feed failed for {}: {:?}", user.handle, err);
            }
        }
        update_news_bsky_user_last_checked_at(
            db_pool,
            user.did.clone(),
            now_utc_timestamp(),
        )
        .await?;
    }
    Ok(())
}

async fn fetch_user_feed(
    db_pool: &PgPool,
    bsky_agent: &BlueskyAgent,
    english_detector: &EnglishLanguageDetector,
    user: &db::models::news_bsky_user::NewsBskyUser,
) -> Result<()> {
    let last_post_cid = user.last_post_cid.clone();
    let mut cursor: Option<String> = None;
    let mut newest_cid: Option<String> = None;
    let mut reached_last_post = false;

    loop {
        let response = get_author_feed(
            bsky_agent,
            &user.did,
            cursor.as_deref(),
            Some(MAX_BSKY_POST_RESULTS),
        )
        .await?;

        for feed_item in &response.feed {
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
    Ok(())
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
