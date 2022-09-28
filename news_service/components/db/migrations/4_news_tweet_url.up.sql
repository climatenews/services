CREATE TABLE news_tweet_url (
    id                  SERIAL,
    url                 TEXT        NOT NULL,
    expanded_url        TEXT        NOT NULL,
    expanded_url_parsed TEXT        NOT NULL UNIQUE,
    expanded_url_host   TEXT        NOT NULL,
    display_url         TEXT        NOT NULL,
    is_twitter_url      BOOLEAN     NOT NULL,
    title               TEXT,
    description         TEXT,
    created_at          BIGINT          NOT NULL,
    created_at_str      TEXT            NOT NULL
);


