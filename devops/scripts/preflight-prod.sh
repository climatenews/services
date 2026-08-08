#!/usr/bin/env bash
set -euo pipefail

MODE="${1:-deploy}"
APP_DIR="${APP_DIR:-/home/ubuntu/services}"
COMPOSE_FILE="${COMPOSE_FILE:-${APP_DIR}/docker-compose.prod.yaml}"
ENV_FILE="${ENV_FILE:-/home/ubuntu/.env.prod}"

require_cmd() {
  local cmd="$1"
  if ! command -v "${cmd}" >/dev/null 2>&1; then
    echo "[preflight] missing required command: ${cmd}"
    exit 1
  fi
}

require_var() {
  local var_name="$1"
  if [[ -z "${!var_name:-}" ]]; then
    echo "[preflight] missing required env var: ${var_name}"
    exit 1
  fi
}

if [[ ! -f "${ENV_FILE}" ]]; then
  echo "[preflight] env file not found: ${ENV_FILE}"
  exit 1
fi

set -a
# shellcheck disable=SC1090
source "${ENV_FILE}"
set +a

require_cmd docker

if ! docker info >/dev/null 2>&1; then
  echo "[preflight] docker daemon is not reachable"
  exit 1
fi

case "${MODE}" in
  deploy)
    require_cmd curl

    if [[ ! -f "${COMPOSE_FILE}" ]]; then
      echo "[preflight] compose file not found: ${COMPOSE_FILE}"
      exit 1
    fi

    require_var PUBLIC_DOMAIN
    require_var DATABASE_URL
    require_var GRAPHQL_API_URL
    require_var POSTGRES_USER
    require_var POSTGRES_DB
    require_var POSTGRES_PASSWORD
    require_var OPENAI_API_KEY
    require_var BLUESKY_HANDLE
    require_var BLUESKY_APP_PASSWORD
    require_var BLUESKY_SERVICE

    echo "[preflight] validating compose configuration"
    docker compose --env-file "${ENV_FILE}" -f "${COMPOSE_FILE}" config >/dev/null
    ;;

  backup)
    require_var POSTGRES_USER
    require_var POSTGRES_DB

    if [[ -n "${S3_BACKUP_BUCKET:-}" ]]; then
      require_cmd aws

      endpoint_args=()
      if [[ -n "${S3_ENDPOINT_URL:-}" ]]; then
        endpoint_args=(--endpoint-url "${S3_ENDPOINT_URL}")
      fi

      echo "[preflight] checking object storage access to bucket ${S3_BACKUP_BUCKET}"
      aws "${endpoint_args[@]}" s3 ls "s3://${S3_BACKUP_BUCKET}" >/dev/null
    else
      echo "[preflight] S3_BACKUP_BUCKET not set; remote upload checks skipped"
    fi
    ;;

  *)
    echo "[preflight] invalid mode '${MODE}'. Use deploy|backup"
    exit 1
    ;;
esac

echo "[preflight] ${MODE} checks passed"
