#!/bin/sh
set -eu

OUT="${1:-out}"
mkdir -p "$OUT"

ROOT="$OUT/tenant-root"
rm -rf "$ROOT"
mkdir -p "$ROOT"/{proc,sys}

cp tenant/dummy-init "$ROOT/init"
chmod +x "$ROOT/init"

( cd "$ROOT" && find . -print0 | cpio --null -ov --format=newc ) > "$OUT/tenant-initrd"
