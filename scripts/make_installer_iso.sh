#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${ROOT_DIR}/dist"
STAGE_DIR="${OUT_DIR}/iso-root"
ISO_PATH="${OUT_DIR}/AetherOS-installer.iso"
BIN_PATH="${ROOT_DIR}/target/release/aether_os"

mkdir -p "${OUT_DIR}"
rm -rf "${STAGE_DIR}"
mkdir -p "${STAGE_DIR}"

if [[ ! -f "${BIN_PATH}" ]]; then
  echo "Release binary not found at ${BIN_PATH}"
  echo "Next step: cargo build --release"
  exit 1
fi

cp "${BIN_PATH}" "${STAGE_DIR}/aether_os"
cp "${ROOT_DIR}/scripts/install_aetheros.sh" "${STAGE_DIR}/install_aetheros.sh"
cp "${ROOT_DIR}/README.md" "${STAGE_DIR}/README.md"

cat > "${STAGE_DIR}/ISO_NOTES.txt" <<'TXT'
AetherOS Installer ISO Artifact

This ISO packages the current AetherOS prototype binary and installer helper script.
It is not a standalone, bare-metal replacement operating system image yet.

Usage on an existing Linux system:
1) Mount ISO
2) Copy files to a writable folder
3) Run install_aetheros.sh
TXT

if command -v xorriso >/dev/null 2>&1; then
  xorriso -as mkisofs -V AETHEROS_INSTALLER -o "${ISO_PATH}" "${STAGE_DIR}"
elif command -v genisoimage >/dev/null 2>&1; then
  genisoimage -V AETHEROS_INSTALLER -o "${ISO_PATH}" -r "${STAGE_DIR}"
else
  echo "Neither xorriso nor genisoimage found."
  echo "Next step: install one of them, then rerun."
  exit 1
fi

echo "Built ISO: ${ISO_PATH}"
