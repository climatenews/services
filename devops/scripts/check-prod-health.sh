#!/usr/bin/env bash
set -euo pipefail

APP_DIR="${APP_DIR:-/home/ubuntu/services}"
COMPOSE_FILE="${COMPOSE_FILE:-${APP_DIR}/docker-compose.prod.yaml}"
ENV_FILE="${ENV_FILE:-/home/ubuntu/.env.prod}"

if [[ ! -f "${COMPOSE_FILE}" ]]; then
  echo "[health] compose file not found: ${COMPOSE_FILE}"
  exit 1
fi

if [[ ! -f "${ENV_FILE}" ]]; then
  echo "[health] env file not found: ${ENV_FILE}"
  exit 1
fi

cd "${APP_DIR}"

echo "[health] checking container status"
docker compose --env-file "${ENV_FILE}" -f "${COMPOSE_FILE}" ps

echo "[health] checking database readiness"
docker compose --env-file "${ENV_FILE}" -f "${COMPOSE_FILE}" exec -T db \
  sh -lc 'pg_isready -U "$POSTGRES_USER"'

echo "[health] checking news_api /health"
docker compose --env-file "${ENV_FILE}" -f "${COMPOSE_FILE}" exec -T news_api \
  curl -fsS http://localhost:8000/health >/dev/null

echo "[health] checking news_api /graphql"
docker compose --env-file "${ENV_FILE}" -f "${COMPOSE_FILE}" exec -T news_api \
  curl -fsS -H "Content-Type: application/json" \
  --data '{"query":"{ __typename }"}' \
  http://localhost:8000/graphql >/dev/null

echo "[health] checking news_cron /health"
docker compose --env-file "${ENV_FILE}" -f "${COMPOSE_FILE}" exec -T news_cron \
  curl -fsS http://localhost:8001/health >/dev/null

echo "[health] checking web root"
docker compose --env-file "${ENV_FILE}" -f "${COMPOSE_FILE}" exec -T web \
  curl -fsS http://localhost:3000 >/dev/null

echo "[health] all checks passed"
