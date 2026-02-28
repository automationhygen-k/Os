#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${ROOT_DIR}/image/out"
INITRAMFS_STAGE="$(mktemp -d)"
trap 'rm -rf "$INITRAMFS_STAGE"' EXIT

KERNEL_RELEASE="${KERNEL_RELEASE:-$(uname -r)}"
KERNEL_SRC="${KERNEL_SRC:-/boot/vmlinuz-${KERNEL_RELEASE}}"
KERNEL_OUT="${OUT_DIR}/vmlinuz-aether"
INITRAMFS_OUT="${OUT_DIR}/initramfs-aether.img"
AETHER_BIN="${ROOT_DIR}/target/release/aether_os"

mkdir -p "$OUT_DIR"

if [[ ! -f "$KERNEL_SRC" ]]; then
  echo "Kernel image not found: ${KERNEL_SRC}"
  echo "Next step: set KERNEL_SRC to a valid Linux kernel image path."
  exit 1
fi

if [[ ! -f "$AETHER_BIN" ]]; then
  echo "Missing ${AETHER_BIN}"
  echo "Next step: cargo build --release"
  exit 1
fi

cp "$KERNEL_SRC" "$KERNEL_OUT"

mkdir -p "$INITRAMFS_STAGE"/{bin,proc,sys,dev,etc,var/log/aetheros}
cp "$ROOT_DIR/initramfs/init" "$INITRAMFS_STAGE/init"
cp "$ROOT_DIR/initramfs/boot_guard.sh" "$INITRAMFS_STAGE/bin/boot_guard.sh"
cp "$AETHER_BIN" "$INITRAMFS_STAGE/bin/aether_os"
cp /bin/sh "$INITRAMFS_STAGE/bin/sh"

for lib in $(ldd /bin/sh | awk '{print $3}' | grep '^/'); do
  target="$INITRAMFS_STAGE$(dirname "$lib")"
  mkdir -p "$target"
  cp "$lib" "$target/"
done

ldso="$(ldd /bin/sh | awk '/ld-linux|ld-musl/{print $1,$NF}' | awk '{print $NF}' | tail -n1)"
if [[ -n "${ldso}" && -f "${ldso}" ]]; then
  mkdir -p "$INITRAMFS_STAGE$(dirname "$ldso")"
  cp "$ldso" "$INITRAMFS_STAGE${ldso}"
fi

(
  cd "$INITRAMFS_STAGE"
  find . -print0 | cpio --null -ov --format=newc 2>/dev/null | gzip -9 > "$INITRAMFS_OUT"
)

echo "Built kernel bundle:"
echo "  Kernel:    $KERNEL_OUT"
echo "  Initramfs: $INITRAMFS_OUT"
