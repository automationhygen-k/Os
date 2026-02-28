#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${ROOT_DIR}/dist"
ISO_ROOT="${OUT_DIR}/iso-boot"
ISO_PATH="${OUT_DIR}/AetherOS-bootable.iso"
DEBUG_LOG="${OUT_DIR}/bootable-iso-build.log"
FALLBACK_ARCHIVE="${OUT_DIR}/AetherOS-bootable-fallback.tar.gz"

mkdir -p "$OUT_DIR"
rm -rf "$ISO_ROOT"
mkdir -p "$ISO_ROOT/boot/grub"
: > "$DEBUG_LOG"

log() {
  echo "$1" | tee -a "$DEBUG_LOG"
}

require_tool() {
  local tool="$1"
  if ! command -v "$tool" >/dev/null 2>&1; then
    log "Missing required tool: $tool"
    log "Next step: install grub-common grub-pc-bin xorriso mtools cpio and rerun."
    exit 1
  fi
}

for tool in grub-mkrescue xorriso mformat tar; do
  require_tool "$tool"
done

log "Building kernel + initramfs bundle..."
if ! bash "$ROOT_DIR/scripts/build_kernel_bundle.sh" 2>&1 | tee -a "$DEBUG_LOG"; then
  log "Kernel bundle build failed."
  exit 1
fi

cp "$ROOT_DIR/image/out/vmlinuz-aether" "$ISO_ROOT/boot/vmlinuz-aether"
cp "$ROOT_DIR/image/out/initramfs-aether.img" "$ISO_ROOT/boot/initramfs-aether.img"
cp "$ROOT_DIR/boot/grub/grub.cfg" "$ISO_ROOT/boot/grub/grub.cfg"

build_with_retry() {
  local attempt="$1"
  log "grub-mkrescue attempt #${attempt}"
  grub-mkrescue -o "$ISO_PATH" "$ISO_ROOT" 2>&1 | tee -a "$DEBUG_LOG"
}

if ! build_with_retry 1; then
  log "First ISO build attempt failed. Cleaning partial artifact and retrying..."
  rm -f "$ISO_PATH"
  sync || true
  if ! build_with_retry 2; then
    log "ISO build failed after retry. Creating fallback archive for recovery/debug."
    tar -C "$OUT_DIR" -czf "$FALLBACK_ARCHIVE" "$(basename "$ISO_ROOT")"
    log "Fallback archive: $FALLBACK_ARCHIVE"
    log "Detailed log: $DEBUG_LOG"
    exit 1
  fi
fi

log "Built bootable ISO: $ISO_PATH"
log "Detailed log: $DEBUG_LOG"
