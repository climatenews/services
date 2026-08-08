# Climate News - Services

[![web_push](https://github.com/climatenews/services/actions/workflows/news_service_web_push.yml/badge.svg)](https://github.com/climatenews/services/actions/workflows/news_service_web_push.yml) [![cron_push](https://github.com/climatenews/services/actions/workflows/news_service_cron_push.yml/badge.svg)](https://github.com/climatenews/services/actions/workflows/news_service_cron_push.yml) [![api_push](https://github.com/climatenews/services/actions/workflows/news_service_api_push.yml/badge.svg)](https://github.com/climatenews/services/actions/workflows/news_service_api_push.yml)


## Overview
`devops` - Terraform & Ansible deployment scripts

`news_service` - Rust Cron and API services

`web` - Next.js frontend

## Running locally
### Prerequisites
- Docker & Docker Compose
- [Bluesky account with an app password](https://bsky.app/settings/app-passwords) (used by the cron service to fetch feeds and publish posts)
- [OpenAI API key](https://openai.com/api/)

### Setting up the .env.dev file
```bash
# copy the sample .env file 
cp .env.sample .env.dev
```
Set the `OPENAI_API_KEY`, `BLUESKY_APP_PASSWORD`, and (optionally) the Slack `POST_CRON_WEBHOOK_URL` / `MAIN_CRON_WEBHOOK_URL` variables in `.env.dev`

For faster local testing, you can optionally limit ingestion to a small subset of
users per cron run:

```bash
BSKY_MAX_USERS_PER_RUN=10
```

### Test the app with Docker Compose
```bash
# Start local Postgres only (recommended for local dev)
docker-compose --env-file ".env.dev" up
```

### Run the app with the Procfile
Install [Overmind](https://github.com/DarthSim/overmind) to manage the local API,
cron, and web processes:

```bash
brew install overmind

# Ubuntu: https://gist.github.com/iagopuccini/12b594f6726a8ae85e1b1e32491bb12d
```

Start Postgres, load the development environment, then start the processes
defined in `Procfile`:

```bash
./run_dev.sh
```

Stop all Procfile processes with `overmind stop`.

### Troubleshooting local Postgres version mismatch

If you see errors like "database files are incompatible with server" and
"The data directory was initialized by PostgreSQL version 12", your local Docker
volume was created by an older Postgres major version.

For disposable local data, reset the local DB volume and start again:

```bash
docker compose --env-file .env.dev -f docker-compose.yaml down -v
docker volume rm services_db_data 2>/dev/null || true
./run_dev.sh
```

If you need to preserve local data, temporarily run a Postgres 12 container,
export a dump, then restore into Postgres 16.

This repo intentionally keeps local and production paths separate:

- Local dev: `Procfile` + `docker-compose.yaml` (DB only)
- Production: `docker-compose.prod.yaml`

## Deploying
### Deploy the stack with Docker Compose (recommended)
```bash
# Copy env vars for production
cp .env.sample .env.prod

# Edit .env.prod and set required secrets

# Pull latest images
docker compose --env-file .env.prod -f docker-compose.prod.yaml pull

# Start services
docker compose --env-file .env.prod -f docker-compose.prod.yaml up -d

# Verify health
docker compose --env-file .env.prod -f docker-compose.prod.yaml ps
docker compose --env-file .env.prod -f docker-compose.prod.yaml logs -f --since=1h
```

### Run scripts on the production host

After running the Ansible playbook, these scripts are available under `/usr/local/bin`:

```bash
# Deploy or update services
sudo /usr/local/bin/deploy-prod.sh

# Run health checks
sudo /usr/local/bin/check-prod-health.sh

# Check backup timers and recent logs
sudo /usr/local/bin/check-backup-timers.sh

# Manual backup and restore verification
sudo /usr/local/bin/postgres-maintenance.sh backup
sudo /usr/local/bin/postgres-maintenance.sh verify
```

### Legacy Docker Swarm path

The Docker Swarm deployment flow is legacy and no longer the primary deployment path.
Use the Compose production flow above and the scripts documented in `devops/README.md`.

# Triggering a new Docker image build
```bash

git tag -a v0.0.52 -m "logging update" && git push origin v0.0.52

```