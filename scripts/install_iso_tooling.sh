#!/usr/bin/env bash
set -euo pipefail

# Best-effort installer for bootable ISO build dependencies.
# Supports common Linux package managers and retries transient failures.

require_tool() {
  command -v "$1" >/dev/null 2>&1
}

retry() {
  local attempts="$1"
  shift
  local i
  for i in $(seq 1 "$attempts"); do
    if "$@"; then
      return 0
    fi
    echo "Attempt ${i}/${attempts} failed: $*"
    sleep 3
  done
  return 1
}

have_all_tools() {
  local t
  for t in grub-mkrescue xorriso mformat cpio; do
    if ! require_tool "$t"; then
      return 1
    fi
  done
  return 0
}

if have_all_tools; then
  echo "ISO tooling already installed."
  exit 0
fi

if require_tool apt-get; then
  SUDO=""
  if [[ "${EUID}" -ne 0 ]]; then
    if require_tool sudo; then
      SUDO="sudo"
    else
      echo "Need root privileges or sudo for apt-get installs."
      exit 1
    fi
  fi
  retry 3 $SUDO apt-get update -o Acquire::Retries=3
  retry 3 $SUDO apt-get install -y --fix-missing grub-pc-bin grub-common xorriso cpio mtools
elif require_tool dnf; then
  SUDO=""
  if [[ "${EUID}" -ne 0 ]]; then SUDO=sudo; fi
  retry 3 $SUDO dnf install -y grub2-tools-minimal grub2-pc-modules xorriso cpio mtools
elif require_tool yum; then
  SUDO=""
  if [[ "${EUID}" -ne 0 ]]; then SUDO=sudo; fi
  retry 3 $SUDO yum install -y grub2-tools-minimal grub2-pc-modules xorriso cpio mtools
elif require_tool pacman; then
  SUDO=""
  if [[ "${EUID}" -ne 0 ]]; then SUDO=sudo; fi
  retry 3 $SUDO pacman -Sy --noconfirm grub xorriso cpio mtools
elif require_tool apk; then
  SUDO=""
  if [[ "${EUID}" -ne 0 ]]; then SUDO=sudo; fi
  retry 3 $SUDO apk add --no-cache grub grub-bios xorriso cpio mtools
else
  echo "No supported package manager found to auto-install ISO tools."
  exit 1
fi

if have_all_tools; then
  echo "ISO tooling installation complete."
  exit 0
fi

echo "Tooling install command completed but required tools are still missing."
exit 1
