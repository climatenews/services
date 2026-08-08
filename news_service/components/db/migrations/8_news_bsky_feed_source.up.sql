CREATE TABLE IF NOT EXISTS news_bsky_feed_source (
    source_uri TEXT PRIMARY KEY,
    source_type TEXT NOT NULL,
    last_checked_at BIGINT NOT NULL
);
