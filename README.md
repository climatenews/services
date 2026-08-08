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
docker compose --env-file ".env.dev" up
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
./run_dev.sh --reset-db
./run_dev.sh
```

If you need to preserve local data, temporarily run a Postgres 12 container,
export a dump, then restore into Postgres 16.

This repo intentionally keeps local and production paths separate:

- Local dev: `Procfile` + `docker-compose.yaml` (DB only)
- Production: `docker-compose.prod.yaml`

## Deploying
### High-level deployment flow

1. Provision infrastructure (optional): use OpenTofu in `devops/opentofu/hetzner`.
2. Configure the host: run Ansible from `devops/ansible` to install Docker,
   operational scripts, and systemd timers.
3. Configure runtime secrets: create `/home/<user>/.env.prod` on the host.
4. Deploy containers: run Compose with `docker-compose.prod.yaml`.
5. Validate operations: run health checks and verify backup timers.

### Host setup (local Ansible + host systemd)

If this repository is running on the target host, run Ansible locally to
install scripts and systemd units in a reproducible way:

```bash
cd devops/ansible
ansible-playbook -i "localhost," -c local -K playbooks/main.yml -e "ansible_user=$USER"
```

Notes:

- Use systemd for host-level automation (timers/services).
- Use Docker Compose for container lifecycle only.
- Keep runtime secrets in `/home/<user>/.env.prod` on the host.

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

See `devops/README.md` for detailed infrastructure and operational procedures.

## Static site deployment (Cloudflare Pages)

The public web site is built as static HTML on the host and published to
Cloudflare Pages. The API/DB/cron run privately on the host; only the static
output is uploaded.

```bash
# On the host, from the repo checkout
GRAPHQL_API_URL="http://localhost:8000/graphql" \
PUBLIC_DOMAIN="climatenews.app" \
CLOUDFLARE_API_TOKEN="..." \
CLOUDFLARE_ACCOUNT_ID="..." \
devops/scripts/static-build-and-publish.sh
```

The script:

1. Pulls the repo (`GIT_PULL=true`), installs deps, and builds the static site
   with `npm run build` (regenerates GraphQL codegen, sitemaps, then
   `next build && next export` into `web/out`).
2. Checks a private-content marker (`newsFeedStatus.completedAt`) and skips
   publish when unchanged (`SKIP_IF_UNCHANGED=true`, default).
3. Publishes `web/out` to Cloudflare Pages with `wrangler pages deploy` when
   content changed.

`web/out` includes `_headers` (security/cache headers applied by Cloudflare
Pages). `getStaticProps`/`getStaticPaths` and sitemaps query the private API
at build time, so `GRAPHQL_API_URL` must point to a reachable API instance
(e.g. localhost).

Automated publishing on the host:

```bash
sudo cp devops/systemd/climatenews-static-publish.{service,timer} /etc/systemd/system/
sudo install -d -m 0755 /etc/climatenews
sudo install -m 0640 /dev/null /etc/climatenews/static-publish.env   # then set vars
sudo systemctl daemon-reload && sudo systemctl enable --now climatenews-static-publish.timer
```

The timer runs hourly and exits quickly when content is unchanged.
See `devops/README.md` for details.