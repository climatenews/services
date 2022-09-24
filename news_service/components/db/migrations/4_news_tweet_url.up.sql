CREATE TABLE news_tweet_url (
    id                  SERIAL,
    url                 TEXT        NOT NULL,
    expanded_url        TEXT        NOT NULL,
    parsed_expanded_url TEXT        NOT NULL UNIQUE,
    display_url         TEXT        NOT NULL,
    is_twitter_url      BOOLEAN     NOT NULL,
    title               TEXT,
    description         TEXT,
    created_at          BIGINT          NOT NULL,
    created_at_str      TEXT            NOT NULL
);


