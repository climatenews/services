ALTER TABLE news_feed_url ADD COLUMN IF NOT EXISTS bsky_posted_at BIGINT;
ALTER TABLE news_feed_url ADD COLUMN IF NOT EXISTS bsky_posted_at_str TEXT;
