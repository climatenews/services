#!/usr/bin/env bash

set -euo pipefail

# Start local Postgres in Docker Compose, then run Procfile apps with Overmind.
docker-compose --env-file ".env.dev" up -d

# Surface a common local issue: reusing a Postgres 12 data volume with a Postgres 16 image.
db_logs="$(docker compose --env-file .env.dev -f docker-compose.yaml logs db --tail=80 2>/dev/null || true)"
if echo "$db_logs" | grep -q "database files are incompatible with server"; then
	cat <<'EOF'
Postgres data volume version mismatch detected.
The local db volume was initialized by an older major version.

If this is disposable local data, reset with:
	docker compose --env-file .env.dev -f docker-compose.yaml down -v
	docker volume rm services_db_data 2>/dev/null || true

Then re-run:
	./run_dev.sh

If you need to keep data, pin Postgres to version 12 temporarily,
dump the data, then restore into Postgres 16.
EOF
	exit 1
fi

export DATABASE_URL="postgres://climate_news:climate_news@localhost:5432/climate_news"
overmind start