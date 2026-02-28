#!/bin/sh
set -eu

LOG_DIR="/var/log/aetheros"
LOG_FILE="${LOG_DIR}/bootguard.log"
HISTORY_FILE="${LOG_DIR}/bootguard-history.log"

mkdir -p "${LOG_DIR}" || true

log() {
  msg="$1"
  ts="$(date +%s 2>/dev/null || echo 0)"
  echo "[$ts] $msg" >> "${LOG_FILE}"
  echo "[$ts] $msg" >> "${HISTORY_FILE}"
}

attempt_fix() {
  issue="$1"
  shift
  if "$@"; then
    log "FIXED: ${issue}"
  else
    log "UNRESOLVED: ${issue}"
  fi
}

log "BootGuard start"

[ -x /bin/sh ] || attempt_fix "missing /bin/sh" ln -sf /bin/busybox /bin/sh
[ -x /init ] || attempt_fix "init script not executable" chmod +x /init

if [ ! -x /bin/aether_os ]; then
  log "ERROR: /bin/aether_os missing"
else
  log "OK: /bin/aether_os present"
fi

for path in /proc /sys /dev; do
  [ -d "$path" ] || mkdir -p "$path"
done

log "BootGuard complete"
