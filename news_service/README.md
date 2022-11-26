### Climate News - backend services


# Running the services
```bash
cargo run --bin api
cargo run --bin cron
```


# running tests
```bash
cargo test --package cron --bin cron -- twitter::db::tests::get_expanded_url_parsed_youtube_params_test --exact --nocapture 
```

# Pre build steps
## sqlx offline mode
```bash
cargo clean && DATABASE_URL=postgres://climate_action:climate_action@localhost:5432/climate_action cargo sqlx prepare --merged
```

## docker compose
```bash
sudo docker-compose up -d --build
sudo docker-compose logs --tail="all" -f
```


#### Setup Postgres Database
```sh
# Sqlx CLI
cargo install sqlx-cli 


# create database & run migrations
cd components/db
sqlx database create
sqlx migrate run
sqlx migrate revert

#test database
export DATABASE_URL=postgres://climate_action:climate_action@localhost:5432/climate_action_test 
 sqlx database drop -y && sqlx database create &&  sqlx migrate run

```
