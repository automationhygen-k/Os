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

require_tool() {
  local tool="$1"
  if ! command -v "$tool" >/dev/null 2>&1; then
    echo "Missing required tool: $tool"
    echo "Next step: install required build tools and rerun."
    exit 1
  fi
}

for t in cpio gzip find awk ldd; do
  require_tool "$t"
done

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
    "/lib/modules/${KERNEL_RELEASE}/vmlinuz"
    "/usr/lib/modules/${KERNEL_RELEASE}/vmlinuz"
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

copy_binary_with_deps() {
  local binary="$1"
  local destination_rel="$2"

  if [[ ! -f "$binary" ]]; then
    echo "Warning: binary missing, skipping dependency copy: $binary"
    return 0
  fi

  mkdir -p "$INITRAMFS_STAGE$(dirname "$destination_rel")"
  cp "$binary" "$INITRAMFS_STAGE$destination_rel"

  while IFS= read -r lib; do
    [[ -z "$lib" ]] && continue
    mkdir -p "$INITRAMFS_STAGE$(dirname "$lib")"
    cp "$lib" "$INITRAMFS_STAGE$lib"
  done < <(ldd "$binary" | awk '{for (i=1; i<=NF; i++) if ($i ~ /^\//) print $i}' | sort -u)
}

mkdir -p "$OUT_DIR"

KERNEL_SRC_PATH="$(locate_kernel_image || true)"
if [[ -z "$KERNEL_SRC_PATH" ]]; then
  echo "Kernel image not found in known locations."
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
chmod +x "$INITRAMFS_STAGE/init" "$INITRAMFS_STAGE/bin/boot_guard.sh"

copy_binary_with_deps "$AETHER_BIN" "/bin/aether_os"
copy_binary_with_deps "/bin/sh" "/bin/sh"

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
