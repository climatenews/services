CREATE TABLE IF NOT EXISTS news_bsky_post (
    post_uri TEXT PRIMARY KEY,
    cid TEXT NOT NULL,
    text TEXT NOT NULL,
    author_did TEXT NOT NULL REFERENCES news_bsky_user(did),
    reply_parent_uri TEXT,
    reply_root_uri TEXT,
    created_at BIGINT NOT NULL,
    created_at_str TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_bsky_post_author ON news_bsky_post(author_did);
CREATE INDEX IF NOT EXISTS idx_bsky_post_created ON news_bsky_post(created_at);
