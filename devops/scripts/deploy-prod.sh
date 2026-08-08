#!/usr/bin/env bash
set -euo pipefail

APP_DIR="${APP_DIR:-/home/ubuntu/services}"
COMPOSE_FILE="${COMPOSE_FILE:-${APP_DIR}/docker-compose.prod.yaml}"
ENV_FILE="${ENV_FILE:-/home/ubuntu/.env.prod}"
RUN_HEALTH_CHECK="${RUN_HEALTH_CHECK:-true}"

if [[ ! -f "${COMPOSE_FILE}" ]]; then
  echo "[deploy] compose file not found: ${COMPOSE_FILE}"
  exit 1
fi

if [[ ! -f "${ENV_FILE}" ]]; then
  echo "[deploy] env file not found: ${ENV_FILE}"
  exit 1
fi

cd "${APP_DIR}"

echo "[deploy] pulling latest images"
docker compose --env-file "${ENV_FILE}" -f "${COMPOSE_FILE}" pull

echo "[deploy] starting services"
docker compose --env-file "${ENV_FILE}" -f "${COMPOSE_FILE}" up -d --remove-orphans

echo "[deploy] showing service status"
docker compose --env-file "${ENV_FILE}" -f "${COMPOSE_FILE}" ps

if [[ "${RUN_HEALTH_CHECK}" == "true" ]]; then
  if [[ -x /usr/local/bin/check-prod-health.sh ]]; then
    echo "[deploy] running health checks"
    /usr/local/bin/check-prod-health.sh
  else
    echo "[deploy] check-prod-health.sh not installed at /usr/local/bin, skipping"
  fi
fi

echo "[deploy] complete"
