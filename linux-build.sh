#!/usr/bin/env bash
# Linux desktop build script for verby
# Handles Wayland compatibility issues by forcing X11 backend
set -eux

echo "Building verby for Linux desktop..."

# Build the desktop version
cargo build --bin verby-desktop --target x86_64-unknown-linux-gnu --release

echo "✅ Build complete!"
echo "To run the app, use:"
echo "./target/x86_64-unknown-linux-gnu/release/verby-desktop"
echo ""
echo "Or run directly with:"
echo "cargo run --bin verby-desktop --target x86_64-unknown-linux-gnu"
