-- Recreate Twitter/X tables and restore tweet columns
ALTER TABLE news_feed_url
    ALTER COLUMN first_referenced_by TYPE BIGINT USING first_referenced_by::BIGINT,
    ADD COLUMN tweeted_at BIGINT,
    ADD COLUMN tweeted_at_str TEXT;

CREATE TABLE news_twitter_user (
    id                              SERIAL,
    user_id                         BIGINT      NOT NULL UNIQUE,
    username                        TEXT        NOT NULL UNIQUE,
    profile_image_url               TEXT,
    description                     TEXT,
    verified                        BOOLEAN,
    followers_count                 INTEGER     NOT NULL,
    listed_count                    INTEGER     NOT NULL,
    user_referenced_tweets_count    INTEGER,
    user_score                      INTEGER,
    last_tweet_id                   BIGINT,
    last_updated_at                 BIGINT      NOT NULL,
    last_checked_at                 BIGINT      NOT NULL
);

CREATE TABLE news_tweet (
    id                          SERIAL,
    tweet_id                    BIGINT          NOT NULL UNIQUE,
    text                        TEXT            NOT NULL,
    author_id                   BIGINT          NOT NULL,
    conversation_id             BIGINT,
    in_reply_to_user_id         BIGINT,
    created_at                  BIGINT          NOT NULL,
    created_at_str              TEXT            NOT NULL
);

CREATE TABLE news_referenced_tweet (
    id                          SERIAL,
    tweet_id                    BIGINT      NOT NULL,
    referenced_tweet_id         BIGINT      NOT NULL,
    referenced_tweet_kind       TEXT        NOT NULL
);
create index news_referenced_tweet_tweet_id_index on news_referenced_tweet (tweet_id);
create index news_referenced_tweet_referenced_tweet_id_index on news_referenced_tweet (referenced_tweet_id);
create index news_referenced_tweet_referenced_tweet_kind_index on news_referenced_tweet (referenced_tweet_kind);

CREATE TABLE news_tweet_url (
    id                          SERIAL,
    url                         TEXT        NOT NULL,
    expanded_url                TEXT        NOT NULL,
    expanded_url_parsed         TEXT        NOT NULL UNIQUE,
    expanded_url_host           TEXT        NOT NULL,
    display_url                 TEXT        NOT NULL,
    is_twitter_url              BOOLEAN     NOT NULL,
    is_english                  BOOLEAN     NOT NULL,
    title                       TEXT        NOT NULL,
    description                 TEXT        NOT NULL,
    preview_image_thumbnail_url TEXT,
    preview_image_url           TEXT,
    created_at                  BIGINT      NOT NULL,
    created_at_str              TEXT        NOT NULL
);

CREATE TABLE news_referenced_tweet_url (
    id                SERIAL,
    tweet_id          BIGINT        NOT NULL,
    url_id            INTEGER       NOT NULL
);
create index news_referenced_tweet_url_tweet_id_index on news_referenced_tweet_url (tweet_id);

CREATE TABLE news_twitter_list (
    id                              SERIAL,
    list_id                         BIGINT      NOT NULL UNIQUE,
    last_checked_at                 BIGINT      NOT NULL
);

CREATE TABLE news_twitter_referenced_user (
    id                              SERIAL,
    user_id                         BIGINT      NOT NULL UNIQUE,
    username                        TEXT        NOT NULL UNIQUE
);
