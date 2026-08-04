use crate::bluesky::BlueskyAgent;
use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct FeedViewPost {
    pub post: PostView,
    pub reply: Option<ReplyRef>,
    pub reason: Option<ReasonRepost>,
}

#[derive(Debug, Deserialize)]
pub struct ReplyRef {
    pub parent: PostView,
    pub root: PostView,
}

#[derive(Debug, Deserialize)]
pub struct ReasonRepost {
    #[serde(rename = "by")]
    pub by: ActorBasic,
}

#[derive(Debug, Deserialize)]
pub struct PostView {
    pub uri: String,
    pub cid: String,
    pub author: ActorBasic,
    pub record: Option<serde_json::Value>,
    pub indexed_at: Option<String>,
    pub like_count: Option<i32>,
    pub repost_count: Option<i32>,
    pub reply_count: Option<i32>,
    pub embed: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ActorBasic {
    pub did: String,
    pub handle: String,
    pub display_name: Option<String>,
    pub avatar: Option<String>,
    pub description: Option<String>,
    pub followers_count: Option<i32>,
    pub follows_count: Option<i32>,
    pub posts_count: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct AuthorFeedResponse {
    pub feed: Vec<FeedViewPost>,
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FeedResponse {
    pub feed: Vec<FeedViewPost>,
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PostThreadResponse {
    pub thread: ThreadViewPost,
}

#[derive(Debug, Deserialize)]
pub struct ThreadViewPost {
    pub post: PostView,
    pub replies: Option<Vec<ThreadViewPost>>,
}

#[derive(Debug, Deserialize)]
pub struct ResolveHandleResponse {
    pub did: String,
}

#[derive(Debug, Deserialize)]
pub struct ActorProfile {
    pub did: String,
    pub handle: String,
    pub display_name: Option<String>,
    pub avatar: Option<String>,
    pub description: Option<String>,
    pub followers_count: Option<i32>,
    pub follows_count: Option<i32>,
    pub posts_count: Option<i32>,
}

pub async fn get_author_feed(
    agent: &BlueskyAgent,
    actor: &str,
    cursor: Option<&str>,
    limit: Option<u32>,
) -> Result<AuthorFeedResponse> {
    let mut url = format!(
        "{}/xrpc/app.bsky.feed.getAuthorFeed?actor={}",
        agent.service, actor
    );
    if let Some(c) = cursor {
        url.push_str(&format!("&cursor={}", c));
    }
    if let Some(l) = limit {
        url.push_str(&format!("&limit={}", l));
    }

    let resp: AuthorFeedResponse = agent
        .client
        .get(&url)
        .bearer_auth(&agent.session.access_jwt)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(resp)
}

pub async fn get_feed(
    agent: &BlueskyAgent,
    feed_uri: &str,
    cursor: Option<&str>,
    limit: Option<u32>,
) -> Result<FeedResponse> {
    let mut url = format!(
        "{}/xrpc/app.bsky.feed.getFeed?feed={}",
        agent.service, feed_uri
    );
    if let Some(c) = cursor {
        url.push_str(&format!("&cursor={}", c));
    }
    if let Some(l) = limit {
        url.push_str(&format!("&limit={}", l));
    }

    let resp: FeedResponse = agent
        .client
        .get(&url)
        .bearer_auth(&agent.session.access_jwt)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(resp)
}

pub async fn get_post_thread(agent: &BlueskyAgent, uri: &str) -> Result<PostThreadResponse> {
    let url = format!(
        "{}/xrpc/app.bsky.feed.getPostThread?uri={}",
        agent.service, uri
    );

    let resp: PostThreadResponse = agent
        .client
        .get(&url)
        .bearer_auth(&agent.session.access_jwt)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(resp)
}

pub async fn resolve_handle(agent: &BlueskyAgent, handle: &str) -> Result<String> {
    let url = format!(
        "{}/xrpc/com.atproto.identity.resolveHandle?handle={}",
        agent.service, handle
    );

    let resp: ResolveHandleResponse = agent
        .client
        .get(&url)
        .bearer_auth(&agent.session.access_jwt)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(resp.did)
}

pub async fn get_actor_profile(agent: &BlueskyAgent, actor: &str) -> Result<ActorProfile> {
    let url = format!(
        "{}/xrpc/app.bsky.actor.getProfile?actor={}",
        agent.service, actor
    );

    let resp: ActorProfile = agent
        .client
        .get(&url)
        .bearer_auth(&agent.session.access_jwt)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(resp)
}

#[derive(Serialize)]
struct CreateRecordRequest {
    repo: String,
    collection: String,
    record: serde_json::Value,
}

#[derive(Deserialize)]
struct CreateRecordResponse {
    uri: String,
    cid: String,
}

pub async fn create_post(
    agent: &BlueskyAgent,
    text: String,
    embed: Option<serde_json::Value>,
) -> Result<(String, String)> {
    let url = format!("{}/xrpc/com.atproto.repo.createRecord", agent.service);

    let mut record = serde_json::json!({
        "text": text,
        "createdAt": chrono::Utc::now().to_rfc3339(),
    });

    if let Some(e) = embed {
        record["embed"] = e;
    }

    let body = CreateRecordRequest {
        repo: agent.session.did.clone(),
        collection: "app.bsky.feed.post".to_string(),
        record,
    };

    let resp: CreateRecordResponse = agent
        .client
        .post(&url)
        .bearer_auth(&agent.session.access_jwt)
        .json(&body)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    info!("Created post: {}", resp.uri);
    Ok((resp.uri, resp.cid))
}

pub fn extract_facets_from_record(record: &serde_json::Value) -> Vec<FacetLink> {
    let mut links = vec![];
    if let Some(facets) = record.get("facets").and_then(|f| f.as_array()) {
        for facet in facets {
            if let Some(features) = facet.get("features").and_then(|f| f.as_array()) {
                for feature in features {
                    if let Some(feature_type) = feature.get("$type").and_then(|t| t.as_str()) {
                        if feature_type == "app.bsky.richtext.facet#link" {
                            if let Some(uri) = feature.get("uri").and_then(|u| u.as_str()) {
                                links.push(FacetLink {
                                    uri: uri.to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    links
}

#[derive(Debug, Clone)]
pub struct FacetLink {
    pub uri: String,
}

pub fn extract_post_text_from_record(record: &serde_json::Value) -> Option<String> {
    record.get("text").and_then(|t| t.as_str()).map(|s| s.to_string())
}

pub fn extract_created_at_from_record(record: &serde_json::Value) -> Option<String> {
    record.get("createdAt").and_then(|t| t.as_str()).map(|s| s.to_string())
}

pub fn extract_embed_from_post(post: &PostView) -> Option<serde_json::Value> {
    post.embed.clone()
}

#[derive(Debug, Deserialize)]
pub struct StarterPackResponse {
    pub starter_pack: StarterPackView,
}

#[derive(Debug, Deserialize)]
pub struct StarterPackView {
    pub uri: String,
    pub cid: String,
    pub record: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct GetListResponse {
    pub cursor: Option<String>,
    pub items: Vec<ListItemView>,
}

#[derive(Debug, Deserialize)]
pub struct ListItemView {
    pub subject: ActorBasic,
}

pub async fn get_starter_pack(agent: &BlueskyAgent, starter_pack: &str) -> Result<StarterPackResponse> {
    let url = format!(
        "{}/xrpc/app.bsky.graph.getStarterPack?starterPack={}",
        agent.service, starter_pack
    );

    let resp: StarterPackResponse = agent
        .client
        .get(&url)
        .bearer_auth(&agent.session.access_jwt)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(resp)
}

pub async fn get_list(
    agent: &BlueskyAgent,
    list: &str,
    cursor: Option<&str>,
    limit: Option<u32>,
) -> Result<GetListResponse> {
    let mut url = format!(
        "{}/xrpc/app.bsky.graph.getList?list={}",
        agent.service, list
    );
    if let Some(c) = cursor {
        url.push_str(&format!("&cursor={}", c));
    }
    if let Some(l) = limit {
        url.push_str(&format!("&limit={}", l));
    }

    let resp: GetListResponse = agent
        .client
        .get(&url)
        .bearer_auth(&agent.session.access_jwt)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(resp)
}
