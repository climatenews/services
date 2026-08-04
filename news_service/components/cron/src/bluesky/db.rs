use crate::bluesky::api::{self, ActorBasic, FeedViewPost, PostView};
use crate::language::english_language_detector::EnglishLanguageDetector;
use anyhow::Result;
use db::models::news_bsky_post::NewsBskyPost;
use db::models::news_bsky_post_url::NewsBskyPostUrl;
use db::models::news_bsky_reference::NewsBskyReference;
use db::models::news_bsky_referenced_post_url::NewsBskyReferencedPostUrl;
use db::models::news_bsky_user::NewsBskyUser;
use db::sql::news_bsky_post::insert_news_bsky_post;
use db::sql::news_bsky_post_url::{
    find_news_bsky_post_url_by_expanded_url_parsed, insert_news_bsky_post_url,
    update_news_bsky_post_url_metadata,
};
use db::sql::news_bsky_reference::insert_news_bsky_reference;
use db::sql::news_bsky_referenced_post_url::insert_news_bsky_referenced_post_url;
use db::sql::news_bsky_user::insert_news_bsky_user;
use db::util::convert::now_utc_timestamp;
use log::info;
use sqlx::PgPool;
use std::time::Duration;
use url::Url;

pub fn parse_bsky_user(actor: &ActorBasic) -> NewsBskyUser {
    NewsBskyUser {
        did: actor.did.clone(),
        handle: actor.handle.clone(),
        display_name: actor.display_name.clone(),
        avatar_url: actor.avatar.clone(),
        description: actor.description.clone(),
        followers_count: actor.followers_count.unwrap_or(0),
        follows_count: actor.follows_count.unwrap_or(0),
        posts_count: actor.posts_count.unwrap_or(0),
        user_score: None,
        last_post_cid: None,
        last_updated_at: now_utc_timestamp(),
        last_checked_at: now_utc_timestamp(),
    }
}

pub fn parse_bsky_post(post_view: &PostView) -> Result<Option<NewsBskyPost>> {
    if let Some(ref record) = post_view.record {
        let text = api::extract_post_text_from_record(record).unwrap_or_default();
        let created_at_str = api::extract_created_at_from_record(record).unwrap_or_default();

        // Skip posts without a parseable createdAt so they don't pollute the
        // feed with a timestamp of 0
        let created_at = match chrono::DateTime::parse_from_rfc3339(&created_at_str) {
            Ok(datetime) => datetime.timestamp(),
            Err(err) => {
                info!(
                    "Skipping post with unparseable createdAt {:?} - {}",
                    created_at_str, err
                );
                return Ok(None);
            }
        };

        let (reply_parent_uri, reply_root_uri) =
            extract_reply_uris_from_record(record);

        Ok(Some(NewsBskyPost {
            post_uri: post_view.uri.clone(),
            cid: post_view.cid.clone(),
            text,
            author_did: post_view.author.did.clone(),
            reply_parent_uri,
            reply_root_uri,
            created_at,
            created_at_str,
        }))
    } else {
        Ok(None)
    }
}

fn extract_reply_uris_from_record(
    record: &serde_json::Value,
) -> (Option<String>, Option<String>) {
    let parent_uri = record
        .get("reply")
        .and_then(|r| r.get("parent"))
        .and_then(|p| p.get("uri"))
        .and_then(|u| u.as_str())
        .map(|s| s.to_string());
    let root_uri = record
        .get("reply")
        .and_then(|r| r.get("root"))
        .and_then(|p| p.get("uri"))
        .and_then(|u| u.as_str())
        .map(|s| s.to_string());
    (parent_uri, root_uri)
}

pub async fn parse_and_insert_bsky_user(
    db_pool: &PgPool,
    actor: &ActorBasic,
) -> Result<NewsBskyUser> {
    let user = parse_bsky_user(actor);
    Ok(insert_news_bsky_user(db_pool, user).await?)
}

pub async fn parse_and_insert_bsky_post(
    db_pool: &PgPool,
    post_view: &PostView,
) -> Result<Option<NewsBskyPost>> {
    if let Some(post) = parse_bsky_post(post_view)? {
        let result = insert_news_bsky_post(db_pool, post.clone()).await;
        match result {
            Ok(_) => Ok(Some(post)),
            Err(sqlx::Error::Database(ref e)) if e.constraint() == Some("news_bsky_post_pkey") => {
                info!("Post already exists: {}", post.post_uri);
                Ok(None)
            }
            Err(e) => Err(e.into()),
        }
    } else {
        Ok(None)
    }
}

pub async fn parse_bsky_post_urls(
    db_pool: &PgPool,
    post_view: &PostView,
    english_detector: &EnglishLanguageDetector,
) -> Result<()> {
    if let Some(ref record) = post_view.record {
        let links = api::extract_facets_from_record(record);
        let now_ts = now_utc_timestamp();
        let now_str = chrono::Utc::now().to_rfc3339();

        for link in &links {
            if let Ok(parsed_url) = Url::parse(&link.uri) {
                let expanded_url_host = parsed_url
                    .host_str()
                    .map(|h| h.replace("www.", ""))
                    .unwrap_or_default();
                let is_bsky_url = link.uri.starts_with("https://bsky.app");

                let post_url = NewsBskyPostUrl {
                    url_id: 0,
                    url: link.uri.clone(),
                    expanded_url: link.uri.clone(),
                    expanded_url_parsed: link.uri.clone(),
                    expanded_url_host,
                    display_url: Some(link.uri.clone()),
                    is_bsky_url: Some(is_bsky_url),
                    is_english: None,
                    title: None,
                    description: None,
                    preview_image_thumbnail_url: None,
                    preview_image_url: None,
                    created_at: now_ts,
                    created_at_str: now_str.clone(),
                };

                // Insert the URL, or fetch the existing row on conflict so the
                // post -> URL mapping is recorded for every sharing post
                let url_db = match insert_news_bsky_post_url(db_pool, post_url).await {
                    Ok(url_db) => Some(url_db),
                    Err(sqlx::Error::Database(ref e))
                        if e.constraint()
                            == Some("news_bsky_post_url_expanded_url_parsed_key") =>
                    {
                        info!("URL already exists: {}", link.uri);
                        find_news_bsky_post_url_by_expanded_url_parsed(db_pool, link.uri.clone())
                            .await
                            .ok()
                            .flatten()
                    }
                    Err(e) => {
                        log::error!("Error inserting Bsky URL: {}", e);
                        None
                    }
                };

                if let Some(url_db) = url_db {
                    let ref_url = NewsBskyReferencedPostUrl {
                        post_uri: post_view.uri.clone(),
                        url_id: url_db.url_id,
                    };
                    insert_news_bsky_referenced_post_url(db_pool, ref_url)
                        .await
                        .ok();

                    // Enrich metadata (title, description, language, preview
                    // image) for external URLs that don't have it yet
                    if !is_bsky_url && url_db.title.is_none() && url_db.is_english.is_none() {
                        enrich_post_url_metadata(db_pool, english_detector, &url_db, &link.uri)
                            .await;
                    }
                }
            }
        }
    }
    Ok(())
}

async fn enrich_post_url_metadata(
    db_pool: &PgPool,
    english_detector: &EnglishLanguageDetector,
    url_db: &NewsBskyPostUrl,
    uri: &str,
) {
    let metadata = fetch_url_metadata(uri).await;
    if let Ok(Some(metadata)) = metadata {
        let title_and_description = format!(
            "{} - {}",
            metadata.title.clone().unwrap_or_default(),
            metadata.description.clone().unwrap_or_default()
        );
        let is_english = english_detector.is_english_language(&title_and_description);

        if let Err(err) = update_news_bsky_post_url_metadata(
            db_pool,
            url_db.url_id,
            metadata.title,
            metadata.description,
            Some(is_english),
            None,
            metadata.preview_image_url,
        )
        .await
        {
            log::error!("Error updating Bsky URL metadata: {}", err);
        }
    }
}

#[derive(Debug)]
struct UrlMetadata {
    title: Option<String>,
    description: Option<String>,
    preview_image_url: Option<String>,
}

async fn fetch_url_metadata(url: &str) -> Result<Option<UrlMetadata>> {
    let client = reqwest::Client::builder()
        .user_agent(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        )
        .timeout(Duration::from_secs(15))
        .build()?;

    let response = match client.get(url).send().await {
        Ok(response) => response,
        Err(_) => return Ok(None),
    };
    if !response.status().is_success() {
        return Ok(None);
    }
    let bytes = match response.bytes().await {
        Ok(bytes) => bytes,
        Err(_) => return Ok(None),
    };

    // Only parse the start of the document where metadata lives
    let limit = bytes.len().min(2_000_000);
    let html = String::from_utf8_lossy(&bytes[..limit]).into_owned();

    let title = extract_title(&html)
        .or_else(|| meta_content_by_key(&html, "property", "og:title"));
    let description = meta_content_by_key(&html, "name", "description")
        .or_else(|| meta_content_by_key(&html, "property", "og:description"));
    let preview_image_url = meta_content_by_key(&html, "property", "og:image");

    if title.is_none() && description.is_none() && preview_image_url.is_none() {
        return Ok(None);
    }

    Ok(Some(UrlMetadata {
        title,
        description,
        preview_image_url,
    }))
}

fn extract_title(html: &str) -> Option<String> {
    let lower = html.to_lowercase();
    let start = lower.find("<title")?;
    let gt = lower[start..].find('>')? + start + 1;
    let end = lower[gt..].find("</title>")? + gt;
    let title = html[gt..end].trim();
    if title.is_empty() {
        None
    } else {
        Some(title.to_string())
    }
}

// Finds a <meta> tag containing `key="value"` and returns its `content` value
fn meta_content_by_key(html: &str, key: &str, value: &str) -> Option<String> {
    let html_lower = html.to_lowercase();
    let key_lower = key.to_lowercase();
    let value_lower = value.to_lowercase();
    let double_quoted = format!("{}=\"{}\"", key_lower, value_lower);
    let single_quoted = format!("{}='{}'", key_lower, value_lower);

    let mut pos = 0;
    while let Some(relative_start) = html_lower[pos..].find("<meta") {
        let block_start = pos + relative_start;
        let block_end = html_lower[block_start..]
            .find('>')
            .map(|e| block_start + e)
            .unwrap_or(html.len());
        let block_lower = &html_lower[block_start..block_end];

        if block_lower.contains(&double_quoted) || block_lower.contains(&single_quoted) {
            if let Some(content) = extract_attr_content(&html[block_start..block_end], "content") {
                return Some(content);
            }
        }
        pos = block_end;
    }
    None
}

fn extract_attr_content(tag: &str, attr: &str) -> Option<String> {
    let tag_lower = tag.to_lowercase();
    let attr_lower = attr.to_lowercase();
    let mut pos = 0;
    loop {
        let relative = tag_lower[pos..].find(&format!("{}=", attr_lower))?;
        let start = pos + relative + attr.len() + 1;
        let after = &tag[start..];
        if let Some(rest) = after.strip_prefix('"') {
            let end = rest.find('"')?;
            let value = rest[..end].trim().to_string();
            if value.is_empty() {
                pos = start;
                continue;
            }
            return Some(value);
        } else if let Some(rest) = after.strip_prefix('\'') {
            let end = rest.find('\'')?;
            let value = rest[..end].trim().to_string();
            if value.is_empty() {
                pos = start;
                continue;
            }
            return Some(value);
        }
        pos = start;
    }
}

pub async fn parse_bsky_references(db_pool: &PgPool, feed_view: &FeedViewPost) -> Result<()> {
    if let Some(ref reply) = feed_view.reply {
        let reference = NewsBskyReference {
            post_uri: feed_view.post.uri.clone(),
            ref_post_uri: reply.parent.uri.clone(),
            ref_kind: "reply_to".to_string(),
        };
        insert_news_bsky_reference(db_pool, reference).await.ok();
    }

    if let Some(ref reason) = feed_view.reason {
        // The reposted post is feed_view.post; ref_post_uri stores the
        // reposter's DID so reposted-by handles can be looked up
        let reference = NewsBskyReference {
            post_uri: feed_view.post.uri.clone(),
            ref_post_uri: reason.by.did.clone(),
            ref_kind: "repost".to_string(),
        };
        insert_news_bsky_reference(db_pool, reference).await.ok();
    }

    Ok(())
}
