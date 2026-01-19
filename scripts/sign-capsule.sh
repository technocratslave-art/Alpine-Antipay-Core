#!/usr/bin/env bash
set -euo pipefail

if [ $# -ne 2 ]; then
  echo "Usage: $0 <capsule-dir> <warden.key>"
  exit 1
fi

CAPSULE_DIR="$1"
KEY="$2"

MANIFEST="$CAPSULE_DIR/manifest.toml"
SIG="$CAPSULE_DIR/manifest.toml.sig"

if [ ! -f "$MANIFEST" ]; then
  echo "[x] manifest.toml not found in $CAPSULE_DIR"
  exit 1
fi

echo "[*] Signing capsule: $CAPSULE_DIR"

# Canonicalize manifest (strip comments + whitespace)
CANON="$(mktemp)"
grep -v '^[[:space:]]*#' "$MANIFEST" | sed '/^[[:space:]]*$/d' > "$CANON"

# Sign (raw Ed25519 signature, 64 bytes)
openssl pkeyutl \
  -sign \
  -inkey "$KEY" \
  -rawin \
  -in "$CANON" \
  -out "$SIG"

rm "$CANON"

echo "[✓] Signature written: $SIG"
