CREATE TABLE IF NOT EXISTS news_bsky_reference (
    post_uri TEXT NOT NULL REFERENCES news_bsky_post(post_uri),
    ref_post_uri TEXT NOT NULL,
    ref_kind TEXT NOT NULL,
    PRIMARY KEY (post_uri, ref_post_uri, ref_kind)
);
