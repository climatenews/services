#!/usr/bin/env bash

set -euo pipefail

if [[ "${1:-}" == "--reset-db" ]]; then
  echo "Stopping containers and resetting database volume..."
  docker-compose --env-file ".env.dev" down -v
fi

# Start local Postgres in Docker Compose, then run Procfile apps with Overmind.
docker-compose --env-file ".env.dev" up -d

export DATABASE_URL="postgres://climate_news:climate_news@localhost:5432/climate_news"
overmind start