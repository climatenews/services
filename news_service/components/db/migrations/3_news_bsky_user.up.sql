CREATE TABLE IF NOT EXISTS news_bsky_user (
    did TEXT PRIMARY KEY,
    handle TEXT NOT NULL,
    display_name TEXT,
    avatar_url TEXT,
    description TEXT,
    followers_count INT DEFAULT 0 NOT NULL,
    follows_count INT DEFAULT 0 NOT NULL,
    posts_count INT DEFAULT 0 NOT NULL,
    user_score INT,
    last_post_cid TEXT,
    last_updated_at BIGINT NOT NULL,
    last_checked_at BIGINT NOT NULL
);
