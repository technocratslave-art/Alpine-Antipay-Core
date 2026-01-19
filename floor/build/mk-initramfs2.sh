#!/usr/bin/env bash
# mk-initramfs.sh — build Antipay Floor initramfs
# Output: antipay-floor.cpio.zst

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="$ROOT_DIR/dist"
INITRAMFS_DIR="$OUT_DIR/initramfs"

WARDEN_BIN="$ROOT_DIR/floor/warden/target/release/antipay-warden"

BUSYBOX_BIN="/usr/bin/busybox"   # must be static
KEXEC_BIN="/sbin/kexec"          # from kexec-tools (static preferred)

echo "[*] Building Antipay Floor initramfs"

rm -rf "$INITRAMFS_DIR"
mkdir -p "$INITRAMFS_DIR"

# -----------------------------------------------------------------------------
# Build Warden (static Rust PID 1)
# -----------------------------------------------------------------------------

echo "[*] Building Rust Warden"
cd "$ROOT_DIR/floor/warden"
cargo build --release

if ! file "$WARDEN_BIN" | grep -q "static"; then
    echo "[!] Warden is not static — aborting"
    exit 1
fi

# -----------------------------------------------------------------------------
# Initramfs layout
# -----------------------------------------------------------------------------

cd "$OUT_DIR"

mkdir -p "$INITRAMFS_DIR"/{bin,sbin,proc,sys,dev,run,tmp,capsules}

# PID 1
cp "$WARDEN_BIN" "$INITRAMFS_DIR/init"
chmod +x "$INITRAMFS_DIR/init"

# BusyBox (tooling only, not init)
cp "$BUSYBOX_BIN" "$INITRAMFS_DIR/bin/busybox"
chmod +x "$INITRAMFS_DIR/bin/busybox"

# Minimal symlinks
ln -s busybox "$INITRAMFS_DIR/bin/sh"
ln -s busybox "$INITRAMFS_DIR/bin/mount"
ln -s busybox "$INITRAMFS_DIR/bin/ls"
ln -s busybox "$INITRAMFS_DIR/bin/cat"
ln -s busybox "$INITRAMFS_DIR/bin/echo"
ln -s busybox "$INITRAMFS_DIR/bin/sleep"

# kexec
cp "$KEXEC_BIN" "$INITRAMFS_DIR/sbin/kexec"
chmod +x "$INITRAMFS_DIR/sbin/kexec"

# -----------------------------------------------------------------------------
# Device nodes (minimal)
# -----------------------------------------------------------------------------

echo "[*] Creating device nodes"

sudo mknod -m 600 "$INITRAMFS_DIR/dev/console" c 5 1
sudo mknod -m 666 "$INITRAMFS_DIR/dev/null" c 1 3
sudo mknod -m 666 "$INITRAMFS_DIR/dev/tty" c 5 0

# -----------------------------------------------------------------------------
# Pack initramfs
# -----------------------------------------------------------------------------

echo "[*] Packing initramfs"

cd "$INITRAMFS_DIR"

find . -print0 \
  | cpio --null -ov --format=newc \
  | zstd -19 -T0 \
  > "$OUT_DIR/antipay-floor.cpio.zst"

echo "[✓] Built: $OUT_DIR/antipay-floor.cpio.zst"
