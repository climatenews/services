### Climate Action Collective - News feed backend service

### Installation Steps (Mac)

#### Install Rust
```sh
curl https://sh.rustup.rs -sSf | sh
rustup install stable
cargo clippy --fix
```

#### Install Postgres
```sh
brew install postgres
psql postgres
# create climate_action role
CREATE ROLE climate_action WITH LOGIN PASSWORD 'climate_action';
ALTER ROLE climate_action CREATEDB;
```

#### Setup Postgres Database
```sh
# install sqlx-cli
cargo install sqlx-cli --features postgres


# create database & run migrations
cd components/db
sqlx database create
sqlx migrate run
sqlx migrate revert
#test database
export DATABASE_URL=postgres://climate_action:climate_action@localhost:5432/climate_action_test 
 sqlx database drop -y && sqlx database create &&  sqlx migrate run


$ echo "cargo sqlx prepare > /dev/null 2>&1; git add sqlx-data.json > /dev/null" > .git/hooks/pre-commit 
# stop database

sudo service postgresql stop
systemctl status 'postgresql*'
#docker

sudo systemctl disable docker.service
sudo systemctl disable docker.socket
sudo systemctl stop docker
sudo service docker stop
```
Mozilla example: https://github.com/mozilla-services/cjms

# running tests
cargo test --package cron --bin cron -- twitter::db::tests::get_expanded_url_parsed_youtube_params_test --exact --nocapture 

# Running the services
cargo run --bin api
cargo run --bin cron

## NVM
```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.35.3/install.sh | bash
```

# Pre build steps
## sqlx offline mode
cargo clean && DATABASE_URL=postgres://climate_action:climate_action@localhost:5432/climate_action cargo sqlx prepare --merged


## docker compose
sudo docker-compose up -d --build
sudo docker-compose logs --tail="all" -f


# Twitter
curl "https://api.twitter.com/2/users/1136330689475993600/tweets?max_results=100&tweet.fields=author_id%2Ccreated_at%2Cin_reply_to_user_id%2Cconversation_id%2Centities%2Creferenced_tweets&start_time=2022-07-23T22%3A56%3A37Z&exclude=replies&pagination_token=7140dibdnow9c7btw4232twylvn0m00zo5526s4w27bcg" -H "Authorization: Bearer AAAAAAAAAAAAAAAAAAAAAMo6ggEAAAAArLoP%2FRAXVHU1jsuvyrAvIKKUWyg%3DE5FPLBkAJOIy680R341JWFM1Ri1hM8ymZqQ6xjW7jDKTJ4Z4np"