#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

mkdir -p target/bug-bench
OUT="target/bug-bench/runtime.log"

cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
cargo run > "$OUT"

if rg -n "panic|thread '.*' panicked|ERROR" "$OUT" >/dev/null 2>&1; then
  echo "Bug bench detected runtime error markers."
  exit 1
fi

echo "Bug bench passed. Output saved to $OUT"
