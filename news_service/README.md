### Climate Action Collective - News feed backend service

### Installation Steps (Mac)

#### Install Rust
```sh
curl https://sh.rustup.rs -sSf | sh
rustup install stable
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
sqlx database drop &&  
sqlx database create &&  
sqlx migrate run

# sqlx offline mode
export DATABASE_URL=postgres://climate_action:climate_action@localhost:5432/climate_action  cargo sqlx prepare 

cargo clean && DATABASE_URL=postgres://climate_action:climate_action@localhost:5432/climate_action cargo sqlx prepare --merged


$ echo "cargo sqlx prepare > /dev/null 2>&1; git add sqlx-data.json > /dev/null" > .git/hooks/pre-commit 
# stop database
sudo service postgresql stop
```
Mozilla example: https://github.com/mozilla-services/cjms

# Ubuntu

## NVM
```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.35.3/install.sh | bash
```