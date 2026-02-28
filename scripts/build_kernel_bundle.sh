#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${ROOT_DIR}/image/out"
INITRAMFS_STAGE="$(mktemp -d)"
trap 'rm -rf "$INITRAMFS_STAGE"' EXIT

KERNEL_RELEASE="${KERNEL_RELEASE:-$(uname -r)}"
KERNEL_OUT="${OUT_DIR}/vmlinuz-aether"
INITRAMFS_OUT="${OUT_DIR}/initramfs-aether.img"
AETHER_BIN="${ROOT_DIR}/target/release/aether_os"
INCLUDE_HOST_MODULES="${INCLUDE_HOST_MODULES:-1}"

if ! command -v cpio >/dev/null 2>&1; then
  echo "Missing required tool: cpio"
  echo "Next step: install cpio and rerun."
  exit 1
fi

locate_kernel_image() {
  if [[ -n "${KERNEL_SRC:-}" ]]; then
    [[ -f "${KERNEL_SRC}" ]] && {
      printf '%s\n' "${KERNEL_SRC}"
      return 0
    }
    echo "KERNEL_SRC was provided but not found: ${KERNEL_SRC}" >&2
    return 1
  fi

  local candidates=(
    "/boot/vmlinuz-${KERNEL_RELEASE}"
    "/boot/vmlinuz"
    "/boot/bzImage-${KERNEL_RELEASE}"
  )

  local c
  for c in "${candidates[@]}"; do
    if [[ -f "$c" ]]; then
      printf '%s\n' "$c"
      return 0
    fi
  done

  c="$(compgen -G '/boot/vmlinuz-*' | head -n1 || true)"
  if [[ -n "$c" && -f "$c" ]]; then
    printf '%s\n' "$c"
    return 0
  fi

  return 1
}

mkdir -p "$OUT_DIR"

KERNEL_SRC_PATH="$(locate_kernel_image || true)"
if [[ -z "$KERNEL_SRC_PATH" ]]; then
  echo "Kernel image not found in /boot."
  echo "Next step: set KERNEL_SRC to a valid Linux kernel image path."
  exit 1
fi

if [[ ! -f "$AETHER_BIN" ]]; then
  echo "Missing ${AETHER_BIN}"
  echo "Next step: cargo build --release"
  exit 1
fi

cp "$KERNEL_SRC_PATH" "$KERNEL_OUT"

mkdir -p "$INITRAMFS_STAGE"/{bin,proc,sys,dev,etc,var/log/aetheros,lib/modules}
cp "$ROOT_DIR/initramfs/init" "$INITRAMFS_STAGE/init"
cp "$ROOT_DIR/initramfs/boot_guard.sh" "$INITRAMFS_STAGE/bin/boot_guard.sh"
cp "$AETHER_BIN" "$INITRAMFS_STAGE/bin/aether_os"
cp /bin/sh "$INITRAMFS_STAGE/bin/sh"

for lib in $(ldd /bin/sh | awk '{print $3}' | grep '^/' || true); do
  target="$INITRAMFS_STAGE$(dirname "$lib")"
  mkdir -p "$target"
  cp "$lib" "$target/"
done

ldso="$(ldd /bin/sh | awk '/ld-linux|ld-musl/ {for (i=1; i<=NF; i++) if ($i ~ /^\//) print $i}' | tail -n1 || true)"
if [[ -n "${ldso}" && -f "${ldso}" ]]; then
  mkdir -p "$INITRAMFS_STAGE$(dirname "$ldso")"
  cp "$ldso" "$INITRAMFS_STAGE${ldso}"
fi

if [[ "$INCLUDE_HOST_MODULES" == "1" ]]; then
  if [[ -d "/lib/modules/${KERNEL_RELEASE}" ]]; then
    cp -a "/lib/modules/${KERNEL_RELEASE}" "$INITRAMFS_STAGE/lib/modules/"
  else
    echo "Warning: /lib/modules/${KERNEL_RELEASE} not found; continuing without host modules."
  fi
fi

(
  cd "$INITRAMFS_STAGE"
  find . -print0 | cpio --null -ov --format=newc 2>/dev/null | gzip -9 > "$INITRAMFS_OUT"
)

echo "Built kernel bundle:"
echo "  Kernel source: $KERNEL_SRC_PATH"
echo "  Kernel out:    $KERNEL_OUT"
echo "  Initramfs:     $INITRAMFS_OUT"
