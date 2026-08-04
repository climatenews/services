# Backend services

### Running the services
```bash
cargo run --bin api
cargo run --bin cron
```
### running tests
```bash
# Setup a test database
export DATABASE_URL=postgres://climate_news:climate_news@localhost:5432/climate_news_test 
sqlx database drop -y && sqlx database create &&  sqlx migrate run
# all tests
export DATABASE_URL=postgres://climate_news:climate_news@localhost:5432/climate_news_test
cargo test -- --nocapture
# individual test
 cargo test --package cron --bin cron -- graphql::queries::news_feed_urls::tests::get_news_feed_urls_test --exact --nocapture 

```

## Pre build steps
### SQLX offline mode

```bash
# Ensure sqlx-cli version matches the version of sqlx installed (0.6.3)
cargo install sqlx-cli --version 0.6.3
# Generate a sqlx-data.json file (run from components/db, then move to the workspace root)
cd components/db
DATABASE_URL=postgres://climate_news:climate_news@localhost:5432/climate_news cargo sqlx prepare --database-url $DATABASE_URL
mv sqlx-data.json ../..
# Verify offline compile works without a live DB
cd ../..
env -u DATABASE_URL cargo check -p db
```
#### Setting up the database
```sh
# Sqlx CLI
cargo install sqlx-cli 

# create database & run migrations
cd components/db
createuser --superuser climate_news
sqlx database create --database-url postgres://climate_news:climate_news@localhost:5432/climate_news
sqlx migrate run --database-url postgres://climate_news:climate_news@localhost:5432/climate_news
sqlx migrate revert --database-url postgres://climate_news:climate_news@localhost:5432/climate_news
sqlx database drop --database-url postgres://climate_news:climate_news@localhost:5432/climate_news

```
