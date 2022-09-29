# Service
- [x] Add cron job every 1H
- [x] Create database using docker?
- [x] Add migrations to code "sqlx::migrate!("./migrations")"
- [x] Implement NewsFeedUrlSharesQuery
- [ ] Dedup news_feed_url_references
- [ ] use env logger
- [ ] use anyhow for errors, remove unwraps
- [ ] Fix API tests?
- [ ] set updated_at when not checked?
- [ ] Add domain (theverge.com)
- [ ] Parse bit.ly links https://stackoverflow.com/a/69944864
- [ ] Use decimal for url score or *100 



# Web

- [ ] Nav Menu
- [ ] Show 1 hour ago, 11 hours ago, 1 day ago on web or "7 shares | 13h"
- [ ] Use env variable for graphql host
- [ ] Add request caching / cache busting every hour - next.js website

# Backlog

- [ ] Incorporate benefit corporation
- [ ] Add news bot 
- [ ] Add User accounts
- [ ] Add comment section
- [ ] Display scores



- [x] Pagination support e.g [link](https://github.com/ekuinox/mikage/blob/7c96ae27021a6e9236a8408a05ea15efdf59f291/src/twitter.rs)
- [x] Parse multiple referenced tweets
- [x] Add join query to rust
- [x] Save last tweet_id with user to avoid multiple requests
- [x] Calculate user scores
- [x] Add NewsIndirectReferencedUrlQuery
- [x] Script to find most referenced climate scientists not on user list
- [x] Add more climate scientists to user list
- [x] User last_updated_at field, to avoid making requests if updated in the last hour
- [x] Add tweet URL created_at field using first tweet
- [x] Create API with news feed urls by score
- [x] Create API with news feed url details with references
- [x] Create web repo
- [x] Add News feed page
- [x] Find users with large num of references, but not on list (script or join query)
- [x] Implement time decay, similar to reddit
