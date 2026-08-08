#!/usr/bin/env bash
set -euo pipefail

# Verifies backup-related systemd timers and optionally performs a manual trigger
# for quick post-deploy validation.
#
# Usage:
#   check-backup-timers.sh
#   RUN_NOW=true check-backup-timers.sh

RUN_NOW="${RUN_NOW:-false}"
BACKUP_TIMER="climatenews-postgres-backup.timer"
VERIFY_TIMER="climatenews-postgres-restore-verify.timer"
BACKUP_SERVICE="climatenews-postgres-backup.service"
VERIFY_SERVICE="climatenews-postgres-restore-verify.service"

echo "[timers] listing relevant timers"
systemctl list-timers --all | grep -E "climatenews-postgres-(backup|restore-verify)" || true

echo "[timers] backup timer status"
systemctl status "${BACKUP_TIMER}" --no-pager || true

echo "[timers] restore verification timer status"
systemctl status "${VERIFY_TIMER}" --no-pager || true

if [[ "${RUN_NOW}" == "true" ]]; then
  echo "[timers] running backup service now"
  systemctl start "${BACKUP_SERVICE}"

  echo "[timers] running restore verification service now"
  systemctl start "${VERIFY_SERVICE}"
fi

echo "[timers] recent backup service logs"
journalctl -u "${BACKUP_SERVICE}" -n 80 --no-pager || true

echo "[timers] recent restore verification service logs"
journalctl -u "${VERIFY_SERVICE}" -n 80 --no-pager || true

echo "[timers] done"
