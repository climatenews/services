CREATE TABLE IF NOT EXISTS news_bsky_post_url (
    url_id SERIAL PRIMARY KEY,
    url TEXT NOT NULL,
    expanded_url TEXT NOT NULL,
    expanded_url_parsed TEXT UNIQUE NOT NULL,
    expanded_url_host TEXT NOT NULL,
    display_url TEXT,
    is_bsky_url BOOLEAN DEFAULT FALSE,
    is_english BOOLEAN,
    title TEXT,
    description TEXT,
    preview_image_thumbnail_url TEXT,
    preview_image_url TEXT,
    created_at BIGINT NOT NULL,
    created_at_str TEXT NOT NULL
);
