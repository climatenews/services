-- Switch news_feed_url to Bluesky-only
ALTER TABLE news_feed_url
    ALTER COLUMN first_referenced_by TYPE TEXT USING first_referenced_by::TEXT,
    DROP COLUMN tweeted_at,
    DROP COLUMN tweeted_at_str;

-- Drop Twitter/X tables
DROP TABLE IF EXISTS news_referenced_tweet_url;
DROP TABLE IF EXISTS news_tweet_url;
DROP TABLE IF EXISTS news_referenced_tweet;
DROP TABLE IF EXISTS news_tweet;
DROP TABLE IF EXISTS news_twitter_referenced_user;
DROP TABLE IF EXISTS news_twitter_list;
DROP TABLE IF EXISTS news_twitter_user;

-- Enforce NOT NULL on Bluesky user stat columns (they default to 0)
ALTER TABLE news_bsky_user
    ALTER COLUMN followers_count SET NOT NULL,
    ALTER COLUMN follows_count SET NOT NULL,
    ALTER COLUMN posts_count SET NOT NULL;
