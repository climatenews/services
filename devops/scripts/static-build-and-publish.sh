#!/usr/bin/env bash
set -euo pipefail

# Static build + publish for Cloudflare Pages.
# Builds the Next.js site against the private API and publishes the static output.
# Intended to run on the host via systemd timer (see devops/systemd).

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(dirname "$(dirname "${SCRIPT_DIR}")")"

APP_DIR="${APP_DIR:-${REPO_DIR}}"
WEB_DIR="${WEB_DIR:-${APP_DIR}/web}"

PUBLIC_DOMAIN="${PUBLIC_DOMAIN:-climatenews.app}"
GRAPHQL_API_URL="${GRAPHQL_API_URL:?GRAPHQL_API_URL is required (private API URL)}"
CF_PAGES_PROJECT_NAME="${CF_PAGES_PROJECT_NAME:-climatenews}"
CF_PAGES_BRANCH="${CF_PAGES_BRANCH:-main}"
CLOUDFLARE_API_TOKEN="${CLOUDFLARE_API_TOKEN:?CLOUDFLARE_API_TOKEN is required}"
CLOUDFLARE_ACCOUNT_ID="${CLOUDFLARE_ACCOUNT_ID:?CLOUDFLARE_ACCOUNT_ID is required}"
GIT_PULL="${GIT_PULL:-true}"
SKIP_IF_UNCHANGED="${SKIP_IF_UNCHANGED:-true}"
BUILD_DIR="${BUILD_DIR:-${WEB_DIR}/out}"
LOGS_DIR="${LOGS_DIR:-${APP_DIR}/logs}"
LOG_FILE="${LOG_FILE:-${LOGS_DIR}/static-publish.log}"
LOCK_FILE="${LOCK_FILE:-/tmp/climatenews-static-publish.lock}"
STATE_DIR="${STATE_DIR:-${APP_DIR}/.state}"
MARKER_FILE="${MARKER_FILE:-${STATE_DIR}/static-publish.marker}"

require_cmd() {
  local cmd="$1"
  if ! command -v "${cmd}" >/dev/null 2>&1; then
    echo "[static-publish] missing required command: ${cmd}"
    exit 1
  fi
}

log() {
  echo "[static-publish] $*"
  echo "$(date -Is) $*" >> "${LOG_FILE}"
}

mkdir -p "${LOGS_DIR}"
touch "${LOG_FILE}"
mkdir -p "${STATE_DIR}"

if [[ -e "${LOCK_FILE}" ]]; then
  log "another run is in progress (lock ${LOCK_FILE} exists); exiting"
  exit 0
fi
mkdir "${LOCK_FILE}"
trap 'rmdir "${LOCK_FILE}" 2>/dev/null || true' EXIT

require_cmd git
require_cmd node
require_cmd npm
require_cmd curl

if [[ ! -f "${WEB_DIR}/package.json" ]]; then
  log "no package.json found at ${WEB_DIR}"
  exit 1
fi

log "starting static build + publish (domain=${PUBLIC_DOMAIN}, api=${GRAPHQL_API_URL}, project=${CF_PAGES_PROJECT_NAME})"

current_marker=""
if [[ "${SKIP_IF_UNCHANGED}" == "true" ]]; then
  log "checking content marker via private GraphQL API"
  current_marker="$({
    curl -fsS \
      -H "Content-Type: application/json" \
      --data '{"query":"query{newsFeedStatus{completedAt}}"}' \
      "${GRAPHQL_API_URL}" \
      | node -e 'let d="";process.stdin.on("data",c=>d+=c);process.stdin.on("end",()=>{try{const j=JSON.parse(d);const v=j&&j.data&&j.data.newsFeedStatus?j.data.newsFeedStatus.completedAt:null;process.stdout.write(String(v ?? ""));}catch(_){process.stdout.write("");}})'
  } || true)"

  if [[ -n "${current_marker}" && -f "${MARKER_FILE}" ]]; then
    previous_marker="$(cat "${MARKER_FILE}")"
    if [[ "${current_marker}" == "${previous_marker}" ]]; then
      log "content unchanged (marker=${current_marker}); skipping publish"
      exit 0
    fi
  fi
fi

cd "${WEB_DIR}"

if [[ "${GIT_PULL}" == "true" ]]; then
  log "updating repo (git pull --ff-only)"
  git pull --ff-only
fi

log "installing dependencies (npm ci)"
npm ci

log "building static site (sitemaps + codegen + next build + next export)"
GRAPHQL_API_URL="${GRAPHQL_API_URL}" PUBLIC_DOMAIN="${PUBLIC_DOMAIN}" npm run build

if [[ ! -d "${BUILD_DIR}" ]]; then
  log "build output missing: ${BUILD_DIR}"
  exit 1
fi

log "publishing ${BUILD_DIR} to Cloudflare Pages"
npx wrangler pages deploy "${BUILD_DIR}" \
  --project-name="${CF_PAGES_PROJECT_NAME}" \
  --branch="${CF_PAGES_BRANCH}" \
  --commit-dirty=true

log "publish complete"

if [[ -n "${current_marker}" ]]; then
  echo "${current_marker}" > "${MARKER_FILE}"
  log "stored new content marker (${current_marker})"
fi
