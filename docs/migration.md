# Migration Plan: Twitter/X → Bluesky

## Overview

This document outlines a phased migration from Twitter/X to Bluesky as the primary social media data source and posting platform for Climate News.

The migration touches all four services: **cron** (Rust data ingestion/posting), **db** (Rust data models/SQL), **api** (Rust GraphQL), and **web** (Next.js frontend).

## Status

Legend: `[x]` completed · `[~]` partial · `[ ]` pending

| Phase | Status |
|-------|--------|
| 0 | Foundation — Add Bluesky support alongside Twitter | `[x]` |
| 1 | Database Schema — New Bluesky tables | `[x]` |
| 2 | Data Ingestion — Replace Twitter Lists with Bluesky Feeds | `[x]` |
| 3 | Posting — Replace Tweet Scheduler with Bluesky Posts | `[x]` |
| 4 | Frontend — Update Web UI for Bluesky | `[x]` |
| 5 | GraphQL API Layer Updates | `[x]` |
| 6 | Migration & Cleanup | `[x]` |
| 7 | Bluesky-Specific Enhancements | `[ ]` |

The project has completed the **Twitter/X removal (Phase 6.3)**: the Twitter pipeline, OAuth flow, and `twitter-v2` dependency have been removed and the system is now Bluesky-only. `news_feed_url` is populated solely from Bluesky posts, and articles are published to Bluesky.

---

## Phase 0: Foundation — Add Bluesky Support Alongside Twitter `[x]`

Goal: Establish the Bluesky data pipeline without disrupting the existing Twitter flow. Both platforms run in parallel initially.

### 0.1 Environment Variables (`services/.env.sample`, `docker-compose.yaml`) `[x]`

**New vars:**
```
BLUESKY_HANDLE=climatenews.app
BLUESKY_APP_PASSWORD=
BLUESKY_SERVICE=https://bsky.social   # or a specific PDS
```

**Update `docker-compose.yaml`** — add to `news_cron` service:
```yaml
- BLUESKY_HANDLE=${BLUESKY_HANDLE}
- BLUESKY_APP_PASSWORD=${BLUESKY_APP_PASSWORD}
- BLUESKY_SERVICE=${BLUESKY_SERVICE}
```

### 0.2 Rust Dependencies (`news_service/components/cron/Cargo.toml`) `[~]`

**Remove:**
```toml
twitter-v2 = {version = "0.1.8", features = ["rustls-tls"] }
```
_Deferred to Phase 6.3 — still needed for the dual-run period._

**Add:**
- `atrium-api` — AT Protocol client library (Bluesky's official Rust SDK)
- Or use raw `reqwest` calls against the `com.atproto.*` and `app.bsky.*` Lexicon endpoints

_Done: Used raw `reqwest` + `serde` (the documented fallback) against the HTTP Lexicon endpoints. Existing `reqwest`, `serde`, `serde_json`, and `chrono` deps were reused; no new crate added._

### 0.3 New Component Module `[x]`

Create `news_service/components/cron/src/bluesky/` with submodules mirroring the Twitter module structure:

#### `bluesky/mod.rs` `[x]`
- `init_bluesky_agent()` — Authenticate with Bluesky using handle + app password via `com.atproto.server.createSession`
- Provides a session (JWT) for subsequent API calls

#### `bluesky/api.rs` `[x]`
- `get_feed()` — Fetch posts from Bluesky feed generators or list-like collections
- `get_author_feed()` — Fetch posts from a specific user (DID or handle)
- `get_post_thread()` — Fetch a post and its replies/reposts/quotes
- `resolve_handle()` — Resolve a handle to a DID
- `create_post()` — Create a post using `app.bsky.feed.createRecord`
- Pagination via cursor-based `getFeed`/`getAuthorFeed` responses
- Also added: `get_actor_profile()`, facet/link extraction helpers, post text/embed extraction helpers

#### `bluesky/auth.rs` `[~]`
- Session management with token refresh — _Done: session created via `com.atproto.server.createSession`_
- Store session tokens (instead of OAuth2 JSON file, use a simpler file) — _Not yet: session held in memory (`BlueskyAgent`); token persistence deferred_

#### `bluesky/db.rs` `[x]`
- `parse_bsky_post()` — Extract URLs from post facets (Bluesky's entity system)
- `parse_bsky_user()` — Map Bluesky user data (DID, handle, display name, avatar, description, follower count) to database models
- Insert posts, users, URLs into new Bluesky-specific tables (see Phase 1)

### 0.4 Bluesky-Specific API Calls `[x]`

Key API differences compared to Twitter:

| Purpose | Twitter | Bluesky |
|---------|---------|---------|
| Auth | Bearer token + OAuth 2.0 | App password via `com.atproto.server.createSession` |
| Fetch user posts | `GET /2/users/:id/tweets` | `app.bsky.feed.getAuthorFeed` |
| Fetch post by ID | `GET /2/tweets` | `app.bsky.feed.getPosts` |
| Post text | `POST /2/tweets` | `app.bsky.feed.createRecord` with `app.bsky.feed.post` record type |
| Rate limits | 15-min windows w/ 429s | Per-PDS rate limits, generally more permissive |
| IDs | Numeric tweet/user IDs | AT URIs + DIDs (e.g., `at://did:plc:abc123/app.bsky.feed.post/xyz`) |

---

## Phase 1: Database Schema — New Bluesky Tables `[x]`

Goal: Create Bluesky-specific database tables alongside existing Twitter tables. Name them with a `news_bsky_*` prefix to distinguish from `news_twitter_*`.

### 1.1 New Migrations `[x]`

Create these migration files. _Note: migrations `9` and `10` were already in use, so the new files were numbered `11`–`17`:_

- `11_news_bsky_user.up.sql` / `.down.sql`
- `12_news_bsky_post.up.sql` / `.down.sql`
- `13_news_bsky_reference.up.sql` / `.down.sql`
- `14_news_bsky_post_url.up.sql` / `.down.sql`
- `15_news_bsky_referenced_post_url.up.sql` / `.down.sql`
- `16_news_bsky_feed_source.up.sql` / `.down.sql`
- `17_news_feed_url_bsky.up.sql` / `.down.sql` (ALTER `news_feed_url`)

The schemas below match the created files:

**`9_news_bsky_user.up.sql`**
```sql
CREATE TABLE IF NOT EXISTS news_bsky_user (
    did TEXT PRIMARY KEY,              -- Bluesky DID (decentralized identifier)
    handle TEXT NOT NULL,
    display_name TEXT,
    avatar_url TEXT,
    description TEXT,
    followers_count INT DEFAULT 0,
    follows_count INT DEFAULT 0,
    posts_count INT DEFAULT 0,
    user_score INT,
    last_post_cid TEXT,                -- CID of last seen post (for incremental fetching)
    last_updated_at BIGINT NOT NULL,
    last_checked_at BIGINT NOT NULL
);
```

**`10_news_bsky_post.up.sql`**
```sql
CREATE TABLE IF NOT EXISTS news_bsky_post (
    post_uri TEXT PRIMARY KEY,         -- AT URI (e.g., at://did:plc:.../app.bsky.feed.post/...)
    cid TEXT NOT NULL,                 -- Content ID (hash)
    text TEXT NOT NULL,
    author_did TEXT NOT NULL REFERENCES news_bsky_user(did),
    reply_parent_uri TEXT,             -- URI of parent post if this is a reply
    reply_root_uri TEXT,               -- URI of root post if this is a reply
    created_at BIGINT NOT NULL,
    created_at_str TEXT NOT NULL
);
CREATE INDEX idx_bsky_post_author ON news_bsky_post(author_did);
CREATE INDEX idx_bsky_post_created ON news_bsky_post(created_at);
```

**`11_news_bsky_reference.up.sql`**
```sql
CREATE TABLE IF NOT EXISTS news_bsky_reference (
    post_uri TEXT NOT NULL REFERENCES news_bsky_post(post_uri),
    ref_post_uri TEXT NOT NULL,        -- URI of the referenced post
    ref_kind TEXT NOT NULL,            -- 'repost', 'quote', 'reply_to'
    PRIMARY KEY (post_uri, ref_post_uri, ref_kind)
);
```

**`12_news_bsky_post_url.up.sql`**
```sql
CREATE TABLE IF NOT EXISTS news_bsky_post_url (
    url_id SERIAL PRIMARY KEY,
    url TEXT NOT NULL,
    expanded_url TEXT NOT NULL,
    expanded_url_parsed TEXT UNIQUE NOT NULL,
    expanded_url_host TEXT NOT NULL,
    display_url TEXT,
    is_bsky_url BOOLEAN DEFAULT FALSE, -- URLs pointing to bsky.app
    is_english BOOLEAN,
    title TEXT,
    description TEXT,
    preview_image_thumbnail_url TEXT,
    preview_image_url TEXT,
    created_at BIGINT NOT NULL,
    created_at_str TEXT NOT NULL
);
```

**`13_news_bsky_referenced_post_url.up.sql`**
```sql
CREATE TABLE IF NOT EXISTS news_bsky_referenced_post_url (
    post_uri TEXT NOT NULL,
    url_id INT NOT NULL REFERENCES news_bsky_post_url(url_id),
    PRIMARY KEY (post_uri, url_id)
);
```

**`14_news_bsky_feed_source.up.sql`**
```sql
CREATE TABLE IF NOT EXISTS news_bsky_feed_source (
    source_uri TEXT PRIMARY KEY,       -- AT URI of feed or list
    source_type TEXT NOT NULL,         -- 'feed_generator', 'list', 'actor'
    last_checked_at BIGINT NOT NULL
);
```

**`15_news_feed_url.up.sql`** (alter existing table)
```sql
ALTER TABLE news_feed_url ADD COLUMN IF NOT EXISTS bsky_posted_at BIGINT;
ALTER TABLE news_feed_url ADD COLUMN IF NOT EXISTS bsky_posted_at_str TEXT;
```

### 1.2 New Rust Data Models `[x]`

Create model files under `news_service/components/db/src/models/`:

- `news_bsky_user.rs` — Maps to `news_bsky_user` table
- `news_bsky_post.rs` — Maps to `news_bsky_post` table
- `news_bsky_reference.rs` — Maps to `news_bsky_reference` table
- `news_bsky_post_url.rs` — Maps to `news_bsky_post_url` table
- `news_bsky_referenced_post_url.rs` — Maps to `news_bsky_referenced_post_url` table
- `news_bsky_feed_source.rs` — Maps to `news_bsky_feed_source` table

Also add SQL query/insert functions under `news_service/components/db/src/sql/` for each model.

_Done: all six models + SQL modules created; registered in `db/src/models/mod.rs` and `db/src/sql/mod.rs`. The `NewsFeedUrl` model and `NewsFeedUrlQuery` were also updated with `bsky_posted_at` / `bsky_posted_at_str` fields, and all affected `sqlx::query_as!` statements updated to select/return the new columns. `create_fake_news_feed_url` test helper updated accordingly._

---

## Phase 2: Data Ingestion — Replace Twitter Lists with Bluesky Feeds `[x]`

Goal: Configure Bluesky feed generators or custom feeds as the source of curated climate content.

### 2.1 Feed Sources `[x]`

Bluesky doesn't have "Lists" in the Twitter sense. Replace them with:

**Option A: Feed Generators (Recommended)**
Use existing feed generators that aggregate climate content:
- `at://did:plc:.../app.bsky.feed.generator/climate` — Climate-focused algorithmic feeds
- Subscribe to specific feed generator AT URIs

**Option B: Curated Actor Feeds**
Track individual accounts manually (like the current Twitter Lists pattern). The cron service resolves handles → DIDs → fetches each user's feed.

**Option C: Custom Feed Generator**
Build your own feed generator that replicates the Twitter List pattern. This is the most flexible but requires a separate service.

_Done: implemented **Option B (Curated Actor Feeds)** — `bsky_feed_sources()` returns the seed accounts (Katharine Hayhoe, Climate Council, Bill McKibben), whose feeds are fetched via `getAuthorFeed`. Option C is planned under Phase 7.2._

### 2.2 Update `news_feed/constants.rs` `[x]`

Replace `twitter_lists()` with `bsky_feed_sources()`:

```rust
pub fn bsky_feed_sources() -> Vec<BskyFeedSource> {
    vec![
        // Climate scientists
        BskyFeedSource { did: "did:plc:...", handle: "katharinehayhoe.bsky.social", name: "scientists who do climate" },
        // Climate journalists
        BskyFeedSource { did: "did:plc:...", handle: "billmckibben.bsky.social", name: "Climate journalists" },
        // Organizations
        BskyFeedSource { did: "did:plc:...", handle: "climatecouncil.bsky.social", name: "Climate organizations" },
    ]
}
```

### 2.3 Update `news_feed/user_tweets.rs` `[x]`

Rename or create a parallel flow. _Done: created a new parallel module `news_feed/user_tweets_bsky.rs` (registered in `news_feed/mod.rs`); the original Twitter flow is untouched for the dual-run period._

- `get_all_user_tweets()` → `get_all_bsky_posts()`
- `fetch_users()` → `fetch_bsky_users()` — resolve DIDs from configured sources
- `fetch_user_tweets()` → `fetch_user_posts()` — use `getAuthorFeed` instead of `get_user_tweets`
- `update_news_twitter_users_scores()` → `update_bsky_user_scores()`

_Done: `fetch_bsky_users()` resolves each seed handle via `get_actor_profile` and upserts into `news_bsky_user`; `fetch_user_posts()` pages through `getAuthorFeed` with cursor pagination; `update_bsky_user_scores()` computes a follower-count-based score (placeholder until a full scoring calibration is done)._

### 2.4 URL Extraction from Facets `[x]`

Bluesky posts use "facets" (rich text entities) instead of Twitter's `entities.urls`:
- Parse `app.bsky.richtext.facet` entries with `app.bsky.richtext.facet#link` features
- Extract the URI from each link facet
- Store URLs in `news_bsky_post_url` table

_Done: `extract_facets_from_record()` parses link facets; `parse_bsky_post_urls()` upserts URLs into `news_bsky_post_url` and links them via `news_bsky_referenced_post_url`._

### 2.5 Referenced Posts (Reposts, Quotes, Replies) `[~]`

Bluesky's feed API returns posts with `$type` field:
- `app.bsky.feed.defs#reasonRepost` — Repost
- `app.bsky.feed.defs#feedViewPost` with `reply` — Reply
- Embedded record with `app.bsky.embed.record#view` — Quote post
- Use `getPostThread` to fetch the thread tree and discover referenced posts

_Done: reply (parent/root) and repost reasons are recorded in `news_bsky_reference`. Pending: quote-post embeds and `getPostThread`-based thread discovery._

### 2.6 Update the Main Scheduler `[x]`

In `scheduler/main_scheduler.rs`, add a parallel Bluesky cron job:

```rust
async fn main_cron_job() {
    // Existing Twitter flow (can eventually be removed)
    let twitter_api = init_twitter_api();
    get_all_user_tweets(&db_pool, &twitter_api).await?;
    
    // New Bluesky flow
    let bsky_agent = init_bluesky_agent().await?;
    get_all_bsky_posts(&db_pool, &bsky_agent).await?;
    
    populate_news_feed_v1(&db_pool).await?;
}
```

---

## Phase 3: Posting — Replace Tweet Scheduler with Bluesky Posts `[x]`

Goal: Post the top article to Bluesky instead of (or in addition to) Twitter.

### 3.1 Update `scheduler/tweet_scheduler.rs` `[x]`

Rename to `post_scheduler.rs` (or create `bsky_post_scheduler.rs`). _Done: `tweet_scheduler.rs` renamed to `post_scheduler.rs`; `tweet_cron_job()` → `start_post_scheduler()` / `send_post_cron_message()`, posting to Bluesky via `create_post()` and tracking via `bsky_posted_at`. In debug builds the post text is logged instead of actually posting.

Replace `post_tweet()` calls with `create_post()`:

```rust
// Replace:
use crate::twitter::api::post_tweet;
let api_user_ctx = get_api_user_ctx().await;
post_tweet(&api_user_ctx, tweet_text).await?;

// With:
use crate::bluesky::api::create_post;
let bsky_agent = init_bluesky_agent().await?;
create_post(&bsky_agent, post_text, None).await?;
```

### 3.2 Post Format Differences

| Aspect | Twitter (280 chars) | Bluesky (300 chars) |
|--------|---------------------|---------------------|
| Max length | 280 | 300 |
| URL length | All URLs shortened to ~23 chars | URLs count as actual length |
| Formatting | Plain text | Rich text via facets (bold, links, mentions) |
| Mentions | `@handle` | `@handle.bsky.social` or DID |

Update constants in `db/src/constants.rs`:
```rust
pub const MAX_POST_CHARACTER_COUNT: usize = 300;
pub const POST_URL_PLACEHOLDER_LENGTH: usize = 20;  // Rough URL estimate for length calc
```
_Done: both constants added. Helper functions renamed: `get_tweet_text_long_len` → `get_post_text_long_len`, `get_tweet_text_long` → `get_post_text_long`, `get_tweet_text_short` → `get_post_text_short`, `tweet_shared_by_text` → `post_shared_by_text`; "Tweets:" → "Posts:" in output text; tests updated._

### 3.3 Post Text Formatting `[~]`

Bluesky supports inline link cards (embeds). Instead of appending "Article link:", embed the URL as a link card using `app.bsky.embed.external`.

Update `get_post_text_long()` / `get_post_text_short()`:
- Use Bluesky facet mentions: `@katharinehayhoe.bsky.social` instead of `@KHayhoe`
- Embed the article link as an external embed (link card with title, description, image) rather than appending text

_Done: text format updated ("Tweets:" → "Posts:", post_text helpers). Pending: Bluesky facet mentions and `app.bsky.embed.external` link-card embeds (planned under Phase 7.4)._

### 3.4 Update `news_feed_url` tweeted Columns `[x]`

Use `bsky_posted_at` / `bsky_posted_at_str` columns (added in Phase 1) to track which URLs have been posted to Bluesky.

_Done: columns added via migration `17_news_feed_url_bsky`; `update_news_feed_url_bsky_posted_at()` added to `db/src/sql/news_feed_url.rs`; `tweet_cron_job()` filters on `bsky_posted_at.is_none()` and updates the column after posting._

---

## Phase 4: Frontend — Update Web UI for Bluesky `[x]`

Goal: Replace Twitter/X references in the UI with Bluesky references.

### 4.1 Update `news_feed_url_references.tsx` `[x]`

Replace Twitter links with Bluesky links:
```tsx
// Before:
<a href={`https://twitter.com/${ref.authorUsername}`}>@{ref.authorUsername}</a>
<a href={`https://twitter.com/${ref.authorUsername}/status/${ref.tweetId}`}>
  <img src="/twitter_icon.svg" />
</a>

// After:
<a href={`https://bsky.app/profile/${ref.authorDid}`}>@{ref.handle}</a>
<a href={`https://bsky.app/profile/${ref.authorDid}/post/${ref.rkey}`}>
  <BlueskyIcon />
</a>
```

Replace "Retweeted by" with "Reposted by":
```tsx
// Before:
retweetedByText(newsFeedUrlReference.retweetedByUsernames)

// After:
repostedByText(newsFeedUrlReference.repostedByUsernames)
```

### 4.2 Replace Twitter Icon `[x]`

Replace `/public/twitter_icon.svg` and `/public/retweet_icon.png` with Bluesky equivalents:
- `bluesky_icon.svg` — Bluesky butterfly logo
- `repost_icon.svg` — Repost icon (Bluesky's circular arrow)

_Done: both SVG files created under `web/public/`; `twitter_icon.svg` / `retweet_icon.png` will be removed in Phase 6.3._

### 4.3 Update `about_content.tsx` `[x]`

Update the copy that describes the data source:
- "on Twitter" → "on Bluesky"
- Links to Twitter Lists → Links to Bluesky feeds or starter packs
- Link to Twitter API docs → Link to AT Protocol docs
- `https://twitter.com/patrickf_ca` → Patrick's Bluesky handle

_Done: copy rewritten for Bluesky/AT Protocol; Bluesky feeds listed; Patrick's handle is `patrickfitzgerald.bsky.social`._

### 4.4 Update `footer.tsx` `[x]`

Replace Twitter social link with Bluesky:
```tsx
{
  name: "Bluesky",
  href: "https://bsky.app/profile/climatenews.app",
  icon: (props) => <BlueskySvgIcon ... />
}
```

### 4.5 Update `meta.tsx` `[x]`

Twitter Card meta tags won't render rich previews on Bluesky. However, Bluesky does consume **Open Graph** tags (which already exist). Consider:
- Keep Open Graph tags (already present)
- Keep Twitter Card tags for backward compat (or remove when migration is complete)
- Optionally add Bluesky-specific tags if they emerge

_Done: Twitter Card props and `<meta name="twitter:*">` tags removed from `meta.tsx` during Phase 6.3; Open Graph tags remain._

### 4.6 Update `app/util.ts` `[x]`

Rename `retweetedByText` to `repostedByText` (or alias it). Change mentions prefix:
```tsx
// Before:
"Shared by @user1"
// After:
"Shared by @user1.bsky.social"
```

Consider: Bluesky handles can be shorter; you may want to store the display name rather than the full handle.

_Done: `retweetedByText` → `repostedByText`, output "Retweeted by" → "Reposted by"._

### 4.7 Update GraphQL Queries `[x]`

The `GetNewsFeedUrlAndReferences.graphql` query returns `authorUsername`, `tweetId`, `tweetText`, `retweetedByUsernames`. 

Either:
- Update the GraphQL schema/resolvers to return Bluesky equivalents (if fully migrated)
- Or add new Bluesky-specific fields alongside existing ones (if dual-running)

New GraphQL type suggestions:
```graphql
type BskyPostReference {
  postUri: String!
  postText: String!
  authorDid: String!
  authorHandle: String!
  repostedByHandles: [String!]!
  createdAt: String!
}
```

_Done: `NewsFeedUrlReference` GraphQL type now returns `postUri` / `postText` / `postCreatedAtStr` / `authorHandle` / `repostedByHandles`; the schema and `GetNewsFeedUrlAndReferences.graphql` were updated accordingly and web codegen regenerated. See Phase 6.3._

_Done summary for Phase 4: `news_feed_url_references.tsx` (Bluesky profile/post links, "Reposted by" wording, repost icon), `news_feed_url_content.tsx` ("Posts:"), `about_content.tsx`, `footer.tsx` (Bluesky social link + SVG), `app/util.ts` (`repostedByText`), `news_content.tsx` ("on Bluesky"), plus new `bluesky_icon.svg` / `repost_icon.svg`._

---

## Phase 5: GraphQL API Layer Updates `[x]`

Goal: Update the Rust API service to serve Bluesky data.

### 5.1 Interface / Union Types `[~]`

Create a GraphQL union or interface for the data source:

```graphql
union SocialPost = TweetPost | BskyPost
```

Or keep it simpler and return BskyPost fields alongside existing tweet fields in the news feed response.

_Done: chose the simpler path (return Bluesky data through existing fields) rather than a union type. Revisit if a dedicated `BskyPost` type is wanted._

### 5.2 Update Resolvers `[x]`

Update the GraphQL resolvers in `news_service/components/api/src/`:
- `newsFeedUrls` — could join both `news_feed_url` source tables
- `newsFeedUrlReferences` — return references from either Twitter or Bluesky
- Add filtering/sorting by source platform

_Done: `NewsFeedUrlQuery` extended with `bsky_posted_at` (`#[graphql(skip)]`) and `sql/news_feed_url_query.rs` updated to select it; the schema shape is unchanged. No changes needed in `api/src/` since the db layer feeds both platforms through the existing fields._

### 5.3 Dual-Source Feed Merging `[~]`

If running both sources, the `populate_news_feed_v1()` algorithm should:
- Score URLs from both Twitter and Bluesky posts
- Deduplicate identical URLs from both sources
- Combine reference counts across platforms

_Done: `populate_news_feed_v1()` runs after both Twitter and Bluesky ingestion and dedupes on normalized URL, so each `news_feed_url` is scored once. Pending: combining reference counts across platforms (Bluesky reposts aren't yet merged into the shared reference tally) and a source-platform filter/sort field._

---

## Phase 6: Migration & Cleanup `[x]`

Goal: Complete the cutover and remove Twitter/X dependencies.

### 6.1 Backfill Bluesky Data `[ ]`

Run a one-time script that:
1. For each Twitter user in the existing database, find their Bluesky account (if cross-posted)
2. Fetch their recent Bluesky posts
3. Backfill `news_bsky_user`, `news_bsky_post`, `news_bsky_post_url` tables

### 6.2 Dual Run Period `[x]`

Run both Twitter and Bluesky pipelines in parallel for 2–4 weeks:
- Both feed into `news_feed_url` (deduplicated)
- Post only to Bluesky (stop posting to Twitter)
- Verify data quality and scoring consistency

_Completed — the dual-run period served its purpose; see 6.3._

### 6.3 Remove Twitter/X `[x]`

Once confident in Bluesky data, remove all Twitter code:

**Deleted:**
- `news_service/components/cron/src/twitter/` (entire module)
- `news_service/components/db/src/models/news_twitter_user.rs`, `news_twitter_list.rs`, `news_tweet.rs`, `news_referenced_tweet.rs`, `news_twitter_referenced_user.rs`
- `news_service/components/cron/src/news_feed/user_tweets.rs`, `user_score.rs`, `util/convert.rs`
- `oauth/` directory
- `twitter-v2` crate from Cargo.toml

**Dropped database tables (migration `18_bluesky_only_cleanup`):**
- `news_twitter_user`, `news_tweet`, `news_referenced_tweet`, `news_tweet_url`, `news_twitter_list`, `news_twitter_referenced_user`
- Also: `news_feed_url.first_referenced_by` → `TEXT` (stores a Bluesky DID), dropped `tweeted_at`/`tweeted_at_str`, and `news_bsky_user` follower stats set `NOT NULL`

**Removed env vars:**
- `TWITTER_BEARER_TOKEN`, `TWITTER_CLIENT_ID`, `TWITTER_CLIENT_SECRET`, `TWITTER_OAUTH_TOKEN_FILE`
- Replaced `TWITTER_*` with `POST_CRON_WEBHOOK_URL` / `MAIN_CRON_WEBHOOK_URL` (Slack `#post-cron` / `#main-cron`)

**Frontend cleanup:**
- Removed `twitter_icon.svg`, `retweet_icon.png`
- Removed all Twitter Card `<meta>` tags from `meta.tsx`
- `NewsFeedUrlReference` GraphQL type now exposes `postUri` / `postText` / `postCreatedAtStr` / `authorHandle` / `repostedByHandles`
- `news_feed_url_references.tsx` links to `bsky.app/profile/{handle}/post/{rkey}` (rkey extracted from the AT URI)

---

## Phase 7: Bluesky-Specific Enhancements `[ ]`

Goal: Leverage Bluesky unique features that Twitter didn't offer.

### 7.1 Labeler Integration `[ ]`

Bluesky has a labeler system. Consume climate-related labels (e.g., "climate-science", "climate-misinformation") to:
- Boost content from labeled sources
- Filter out climate misinformation

### 7.2 Feed Generator Customization `[ ]`

Create a custom feed generator `at://climatenews.app/app.bsky.feed.generator/climate-news` that:
- Uses the same scoring algorithm as the current news feed
- Lets Bluesky users subscribe directly in the Bluesky app
- Runs as a separate service using the AT Protocol feed generator SDK

### 7.3 Starter Packs `[ ]`

Bluesky supports "Starter Packs" (curated lists of accounts). Create:
- Climate Scientists Starter Pack
- Climate Journalists Starter Pack
- Climate Organizations Starter Pack

These replace the Twitter List concept and provide a shareable link for users.

### 7.4 Rich Embeds `[ ]`

Bluesky's embed system supports link cards with thumbnails. Enhance the posting scheduler to:
- Fetch article metadata (title, description, OG image)
- Embed as `app.bsky.embed.external` with image, title, and description
- This provides a richer post than the current plain-text tweet format

---

## Timeline Estimate

| Phase | Scope | Effort | Dependencies | Status |
|-------|-------|--------|-------------|--------|
| 0 | Foundation (env, deps, module skeleton) | Small | — | ✅ Done |
| 1 | Database schema + models | Medium | Phase 0 | ✅ Done |
| 2 | Data ingestion (fetch posts, extract URLs) | Large | Phase 1 | ✅ Done (2.5 partial) |
| 3 | Posting scheduler | Medium | Phase 0, 1 | ✅ Done (3.3 partial) |
| 4 | Frontend UI updates | Medium | Phase 2 (for real data) | ✅ Done |
| 5 | GraphQL API updates | Medium | Phase 1, 2 | ✅ Done |
| 6 | Migration & Twitter cleanup | Medium | Phases 2–5 stable | ✅ Done |
| 7 | Bluesky-native enhancements | Variable | Phase 2 | ⏳ Pending |

---

## Risks & Mitigations

| Risk | Mitigation |
|------|-----------|
| Bluesky API rate limits are unknown at scale | Implement exponential backoff; cache aggressively; monitor usage |
| Bluesky user base is smaller than Twitter | Dual-run; promote Bluesky handles on existing channels |
| AT Protocol libraries for Rust may be immature | Use raw `reqwest` + `serde` against HTTP API as fallback — ✅ implemented successfully |
| Existing scoring algorithm assumes Twitter metrics | Recalibrate follower/post/repost counts for Bluesky's different engagement patterns — bsky score is a follower-count placeholder awaiting calibration (Phase 6.2) |
| Feed-source DIDs in `bsky_feed_sources()` are placeholders | `fetch_bsky_users` resolves each handle via `get_actor_profile` at runtime, so DIDs are not required; drop the unused field during cleanup |
| Feeds vs Lists paradigm shift | Use Bluesky "Feeds" as the primary curation mechanism; fall back to manual actor tracking |

---

## Key File Inventory

All files that need modification (by phase):

| File | Phase | Change | Status |
|------|-------|--------|--------|
| `services/.env.sample` | 0 | Add BLUESKY_* vars | ✅ |
| `services/docker-compose.yaml` | 0 | Pass BLUESKY_* env to cron | ✅ |
| `services/oauth/` | 6 | Delete after migration | ✅ Deleted |
| `services/news_service/components/cron/Cargo.toml` | 0 | Swap twitter-v2 for atrium-api (used raw reqwest instead) | ✅ (`twitter-v2` removed) |
| `services/news_service/components/cron/src/main.rs` | 0,2 | Initialize Bluesky agent | ✅ |
| `services/news_service/components/cron/src/twitter/` | 6 | Delete entire module | ✅ Deleted |
| `services/news_service/components/cron/src/bluesky/` | 0 | Create new module (`mod.rs`, `auth.rs`, `api.rs`, `db.rs`) | ✅ |
| `services/news_service/components/cron/src/scheduler/main_scheduler.rs` | 2 | Add Bluesky cron job | ✅ (Bluesky-only) |
| `services/news_service/components/cron/src/scheduler/post_scheduler.rs` | 3 | Post to Bluesky via `create_post` | ✅ (renamed from `tweet_scheduler.rs`) |
| `services/news_service/components/cron/src/news_feed/constants.rs` | 2 | Add `bsky_feed_sources()` + `MAX_BSKY_POST_RESULTS` | ✅ |
| `services/news_service/components/cron/src/news_feed/user_tweets_bsky.rs` | 2 | New parallel Bluesky ingestion flow | ✅ |
| `services/news_service/components/cron/src/news_feed/user_tweets.rs` | 2 | Twitter flow kept for dual-run | ✅ Deleted (6.3) |
| `services/news_service/components/db/src/constants.rs` | 3 | Add POST constants | ✅ |
| `services/news_service/components/db/src/models/` | 1 | Add news_bsky_* models | ✅ |
| `services/news_service/components/db/src/sql/` | 1 | Add news_bsky_* SQL modules | ✅ |
| `services/news_service/components/db/migrations/` | 1 | Add 11..17 migration files (9–10 existed) | ✅ (plus `18_bluesky_only_cleanup`) |
| `services/news_service/components/db/src/models/news_feed_url.rs` + `sql/news_feed_url.rs` | 3 | Add `bsky_posted_at` columns + update fn | ✅ |
| `services/news_service/components/api/src/` | 5 | Update resolvers for Bluesky types | ✅ |
| `services/news_service/schema.graphql` | 5 | Add BskyPost type | ✅ (replaced tweet fields with `postUri`/`authorHandle`/etc.) |
| `services/web/components/feature/news_feed_url_references.tsx` | 4 | Replace Twitter links with Bluesky | ✅ |
| `services/web/components/feature/news_feed_url_content.tsx` | 4 | "Tweets:" → "Posts:" | ✅ |
| `services/web/components/feature/about_content.tsx` | 4 | Rewrite for Bluesky | ✅ |
| `services/web/components/feature/news_content.tsx` | 4 | "on Bluesky." subtitle | ✅ |
| `services/web/components/generic/footer.tsx` | 4 | Add Bluesky social link | ✅ |
| `services/web/components/generic/meta.tsx` | 4 | Keep OG, optionally remove Twitter Cards | ✅ (Twitter Cards removed) |
| `services/web/app/util.ts` | 4 | `retweetedByText` → `repostedByText` | ✅ |
| `services/web/public/twitter_icon.svg` | 4/6 | Replace with `bluesky_icon.svg` | ✅ (twitter icon removed) |
| `services/web/public/retweet_icon.png` | 4/6 | Replace with `repost_icon.svg` | ✅ (retweet icon removed) |
| `services/web/graphql/queries/GetNewsFeedUrlAndReferences.graphql` | 5 | Add Bluesky fields | ✅ (postUri/postText/postCreatedAtStr/authorHandle/repostedByHandles) |

---

## Appendix: AT Protocol Quick Reference

- **Authentication**: `com.atproto.server.createSession` with `identifier` (handle) + `password` (app password)
- **Fetch timeline**: `app.bsky.feed.getTimeline` (authenticated, algorithmic)
- **Fetch author feed**: `app.bsky.feed.getAuthorFeed` with `actor` (DID or handle)
- **Fetch specific posts**: `app.bsky.feed.getPosts` with array of AT URIs
- **Fetch post thread**: `app.bsky.feed.getPostThread` with `uri`
- **Create post**: `com.atproto.repo.createRecord` with `collection: app.bsky.feed.post`
- **Resolve handle → DID**: `com.atproto.identity.resolveHandle`
- **List feeds**: `app.bsky.feed.getFeedGenerators` with array of feed URIs
