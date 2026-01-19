#!/usr/bin/env bash
set -euo pipefail

OUT_DIR="certs"
mkdir -p "$OUT_DIR"

if [ -f "$OUT_DIR/warden.key" ]; then
  echo "[!] Keys already exist in $OUT_DIR"
  exit 0
fi

echo "[*] Generating Ed25519 Warden keypair"

# Raw 32-byte private key
openssl genpkey -algorithm ED25519 -out "$OUT_DIR/warden.key"

# Extract raw public key
openssl pkey -in "$OUT_DIR/warden.key" -pubout -out "$OUT_DIR/warden.pub"

# Dump hex for embedding into Floor
openssl pkey -in "$OUT_DIR/warden.pub" -pubin -outform DER \
  | tail -c 32 \
  | xxd -p -c 64 > "$OUT_DIR/warden.pub.hex"

echo "[✓] Keys generated:"
echo "  Private: $OUT_DIR/warden.key (KEEP SECRET)"
echo "  Public:  $OUT_DIR/warden.pub"
echo "  Hex:     $OUT_DIR/warden.pub.hex"
