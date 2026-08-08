CREATE TABLE IF NOT EXISTS news_bsky_referenced_post_url (
    post_uri TEXT NOT NULL,
    url_id INT NOT NULL REFERENCES news_bsky_post_url(url_id),
    PRIMARY KEY (post_uri, url_id)
);
