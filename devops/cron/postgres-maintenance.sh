#!/usr/bin/env bash
set -euo pipefail

# Simple PostgreSQL maintenance helper.
# Usage:
#   postgres-maintenance.sh backup
#   postgres-maintenance.sh verify

ACTION="${1:-}"
COMPOSE_FILE="${COMPOSE_FILE:-/home/ubuntu/docker-compose.prod.yaml}"
ENV_FILE="${ENV_FILE:-/home/ubuntu/.env.prod}"
BACKUP_ROOT="${BACKUP_ROOT:-/backups/postgres}"
BACKUP_RETENTION_DAYS="${BACKUP_RETENTION_DAYS:-14}"
S3_BACKUP_BUCKET="${S3_BACKUP_BUCKET:-}"
S3_BACKUP_PREFIX="${S3_BACKUP_PREFIX:-postgres}"
S3_ENDPOINT_URL="${S3_ENDPOINT_URL:-}"
VERIFY_IMAGE="${VERIFY_IMAGE:-postgres:16-alpine}"
VERIFY_CONTAINER="postgres-restore-verify"
VERIFY_PASSWORD="verify_password"

require_var() {
  local name="$1"
  if [[ -z "${!name:-}" ]]; then
    echo "[postgres-maintenance] missing required env var: ${name}"
    exit 1
  fi
}

require_cmd() {
  local cmd="$1"
  if ! command -v "${cmd}" >/dev/null 2>&1; then
    echo "[postgres-maintenance] missing required command: ${cmd}"
    exit 1
  fi
}

ensure_prereqs() {
  require_cmd docker
  require_var POSTGRES_USER
  require_var POSTGRES_DB

  if [[ ! -f "${COMPOSE_FILE}" ]]; then
    echo "[postgres-maintenance] compose file not found: ${COMPOSE_FILE}"
    exit 1
  fi

  if [[ ! -f "${ENV_FILE}" ]]; then
    echo "[postgres-maintenance] env file not found: ${ENV_FILE}"
    exit 1
  fi
}

run_backup() {
  local timestamp backup_dir backup_file backup_path checksum_path s3_uri

  timestamp="$(date -u +%Y-%m-%dT%H-%M-%SZ)"
  backup_dir="${BACKUP_ROOT}/$(date -u +%Y-%m)"
  backup_file="${POSTGRES_DB}_${timestamp}.dump"
  backup_path="${backup_dir}/${backup_file}"
  checksum_path="${backup_path}.sha256"

  mkdir -p "${backup_dir}"
  cd /home/ubuntu

  echo "[postgres-maintenance] creating backup ${backup_path}"
  docker compose --env-file "${ENV_FILE}" -f "${COMPOSE_FILE}" exec -T db \
    pg_dump -U "${POSTGRES_USER}" -d "${POSTGRES_DB}" -Fc -Z 9 > "${backup_path}"

  sha256sum "${backup_path}" > "${checksum_path}"

  echo "[postgres-maintenance] pruning local files older than ${BACKUP_RETENTION_DAYS} days"
  find "${BACKUP_ROOT}" -type f \( -name "*.dump" -o -name "*.sha256" \) -mtime +"${BACKUP_RETENTION_DAYS}" -delete

  if [[ -n "${S3_BACKUP_BUCKET}" ]]; then
    require_cmd aws

    s3_uri="s3://${S3_BACKUP_BUCKET}/${S3_BACKUP_PREFIX}/$(date -u +%Y-%m)/"
    endpoint_args=()
    if [[ -n "${S3_ENDPOINT_URL}" ]]; then
      endpoint_args=(--endpoint-url "${S3_ENDPOINT_URL}")
    fi

    echo "[postgres-maintenance] checking object storage access"
    aws "${endpoint_args[@]}" s3 ls "s3://${S3_BACKUP_BUCKET}" >/dev/null

    echo "[postgres-maintenance] uploading to ${s3_uri}"
    aws "${endpoint_args[@]}" s3 cp "${backup_path}" "${s3_uri}${backup_file}"
    aws "${endpoint_args[@]}" s3 cp "${checksum_path}" "${s3_uri}${backup_file}.sha256"
  fi

  echo "[postgres-maintenance] backup complete"
}

run_verify() {
  local latest_backup

  latest_backup="$(find "${BACKUP_ROOT}" -type f -name "*.dump" | sort | tail -n 1)"

  if [[ -z "${latest_backup}" ]]; then
    echo "[postgres-maintenance] no backup files found in ${BACKUP_ROOT}"
    exit 1
  fi

  echo "[postgres-maintenance] verifying backup ${latest_backup}"

  if docker ps -a --format '{{.Names}}' | grep -q "^${VERIFY_CONTAINER}$"; then
    docker rm -f "${VERIFY_CONTAINER}" >/dev/null 2>&1 || true
  fi

  docker run -d --rm \
    --name "${VERIFY_CONTAINER}" \
    -e POSTGRES_PASSWORD="${VERIFY_PASSWORD}" \
    -e POSTGRES_USER="${POSTGRES_USER}" \
    -e POSTGRES_DB="${POSTGRES_DB}" \
    "${VERIFY_IMAGE}" >/dev/null

  cleanup() {
    docker rm -f "${VERIFY_CONTAINER}" >/dev/null 2>&1 || true
  }
  trap cleanup EXIT

  for _ in $(seq 1 30); do
    if docker exec "${VERIFY_CONTAINER}" pg_isready -U "${POSTGRES_USER}" >/dev/null 2>&1; then
      break
    fi
    sleep 2
  done

  cat "${latest_backup}" | docker exec -i "${VERIFY_CONTAINER}" \
    pg_restore -U "${POSTGRES_USER}" -d "${POSTGRES_DB}" --clean --if-exists

  docker exec "${VERIFY_CONTAINER}" psql -U "${POSTGRES_USER}" -d "${POSTGRES_DB}" \
    -c "SELECT now();" >/dev/null

  echo "[postgres-maintenance] restore verification complete"
}

ensure_prereqs

case "${ACTION}" in
  backup)
    run_backup
    ;;
  verify)
    run_verify
    ;;
  *)
    echo "Usage: $0 backup|verify"
    exit 1
    ;;
esac
