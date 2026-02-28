#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${ROOT_DIR}/dist"
ISO_ROOT="${OUT_DIR}/iso-boot"
ISO_PATH="${OUT_DIR}/AetherOS-bootable.iso"

mkdir -p "$OUT_DIR"
rm -rf "$ISO_ROOT"
mkdir -p "$ISO_ROOT/boot/grub"

bash "$ROOT_DIR/scripts/build_kernel_bundle.sh"
cp "$ROOT_DIR/image/out/vmlinuz-aether" "$ISO_ROOT/boot/vmlinuz-aether"
cp "$ROOT_DIR/image/out/initramfs-aether.img" "$ISO_ROOT/boot/initramfs-aether.img"
cp "$ROOT_DIR/boot/grub/grub.cfg" "$ISO_ROOT/boot/grub/grub.cfg"

if command -v grub-mkrescue >/dev/null 2>&1; then
  grub-mkrescue -o "$ISO_PATH" "$ISO_ROOT" >/dev/null 2>&1
elif command -v xorriso >/dev/null 2>&1; then
  echo "grub-mkrescue missing; cannot create bootable GRUB ISO with xorriso alone."
  echo "Next step: install grub-mkrescue (grub-common) and rerun."
  exit 1
else
  echo "Missing ISO/boot tooling (grub-mkrescue)."
  echo "Next step: install grub-mkrescue and xorriso, then rerun."
  exit 1
fi

echo "Built bootable ISO: $ISO_PATH"
