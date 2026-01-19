#!/bin/sh
set -eu

OUT="${1:-out}"
mkdir -p "$OUT"

# Build static Rust PID1
( cd floor/warden && cargo build --release )
cp floor/warden/target/release/antipay-floor "$OUT/init"

chmod +x "$OUT/init"

# Minimal initramfs tree
ROOT="$OUT/initramfs"
rm -rf "$ROOT"
mkdir -p "$ROOT"/{proc,sys,dev,run,tenant}

cp "$OUT/init" "$ROOT/init"

# Drop in dummy tenant artifacts (for bring-up only)
cp "$OUT/tenant-vmlinuz" "$ROOT/tenant/vmlinuz"
cp "$OUT/tenant-initrd" "$ROOT/tenant/initrd"

# Pack cpio
( cd "$ROOT" && find . -print0 | cpio --null -ov --format=newc ) > "$OUT/floor.cpio"
