#!/usr/bin/env bash
set -euo pipefail

OUT_DIR="${1:-image/out}"
KERNEL_RELEASE="${KERNEL_RELEASE:-$(uname -r)}"
DRIVER_IMAGE="${OUT_DIR}/drivers-${KERNEL_RELEASE}.cpio.gz"

mkdir -p "${OUT_DIR}"
STAGE="$(mktemp -d)"
trap 'rm -rf "$STAGE"' EXIT

if [[ ! -d "/lib/modules/${KERNEL_RELEASE}" ]]; then
  echo "Missing /lib/modules/${KERNEL_RELEASE}"
  echo "Next step: install matching kernel modules or set KERNEL_RELEASE."
  exit 1
fi

mkdir -p "${STAGE}/lib/modules"
cp -a "/lib/modules/${KERNEL_RELEASE}" "${STAGE}/lib/modules/"

(
  cd "$STAGE"
  find . -print0 | cpio --null -ov --format=newc 2>/dev/null | gzip -9 > "$DRIVER_IMAGE"
)

echo "Built driver image: ${DRIVER_IMAGE}"
