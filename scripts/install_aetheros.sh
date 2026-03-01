#!/usr/bin/env bash
set -euo pipefail

PREFIX="${1:-/opt/aetheros}"
BIN_NAME="aether_os"

if [[ ! -f "./${BIN_NAME}" ]]; then
  echo "Missing ${BIN_NAME} in current directory."
  echo "Next step: run this script from extracted ISO content directory."
  exit 1
fi

sudo mkdir -p "${PREFIX}/bin"
sudo install -m 0755 "./${BIN_NAME}" "${PREFIX}/bin/${BIN_NAME}"

echo "Installed to ${PREFIX}/bin/${BIN_NAME}"
echo "Run with: ${PREFIX}/bin/${BIN_NAME}"
