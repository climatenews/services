use crate::bluesky::api::{get_author_feed, get_list, get_starter_pack};
use crate::bluesky::db::{
    parse_and_insert_bsky_post, parse_and_insert_bsky_user, parse_bsky_post_urls,
    parse_bsky_references,
};
use crate::bluesky::BlueskyAgent;
use crate::language::english_language_detector::EnglishLanguageDetector;
use crate::news_feed::constants::MAX_BSKY_POST_RESULTS;
use anyhow::Result;
use db::sql::news_bsky_user::{
    find_all_news_bsky_users, update_news_bsky_user_last_checked_at,
    update_news_bsky_user_last_updated_at, update_news_bsky_user_stats,
};
use db::util::convert::now_utc_timestamp;
use db::util::time::now_formated;
use log::{error, info};
use sqlx::PgPool;

use super::constants::{STARTER_PACK_DID, STARTER_PACK_RKEY};

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

    let list_uri = get_starter_pack_list_uri(bsky_agent).await?;
    info!("fetch_bsky_users - list_uri: {}", list_uri);

    let mut cursor: Option<String> = None;
    loop {
        let response = get_list(bsky_agent, &list_uri, cursor.as_deref(), Some(100)).await?;
        info!("fetch_bsky_users - fetched {} list items", response.items.len());

        for item in &response.items {
            parse_and_insert_bsky_user(db_pool, &item.subject).await?;
        }

        match response.cursor {
            Some(next_cursor)
                if !next_cursor.is_empty() && Some(&next_cursor) != cursor.as_ref() =>
            {
                cursor = Some(next_cursor);
            }
            _ => break,
        }
    }

    Ok(())
}

async fn get_starter_pack_list_uri(agent: &BlueskyAgent) -> Result<String> {
    let starter_pack_uri = format!(
        "at://{}/app.bsky.graph.starterpack/{}",
        STARTER_PACK_DID, STARTER_PACK_RKEY
    );
    let resp = get_starter_pack(agent, &starter_pack_uri).await?;
    let list_uri = resp
        .starter_pack
        .record
        .get("list")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("starter pack record missing 'list' field"))?
        .to_string();
    Ok(list_uri)
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
                newest_cid = Some(feed_item.post.cid.clone());
            }

            parse_and_insert_bsky_post(db_pool, &feed_item.post).await?;
            parse_bsky_post_urls(db_pool, &feed_item.post, english_detector).await?;
            parse_bsky_references(db_pool, feed_item).await?;

            // Stop paginating once we reach the last post we already have
            if let Some(last_cid) = &last_post_cid {
                if &feed_item.post.cid == last_cid {
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
    info!("update_bsky_user_scores - {:?}", now_formated());
    let users = find_all_news_bsky_users(db_pool).await?;

    for user in users {
        let score = (user.followers_count / 1000).max(1);
        update_news_bsky_user_stats(db_pool, user.did.clone(), score).await?;
    }
    Ok(())
}
