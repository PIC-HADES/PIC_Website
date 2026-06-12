#!/usr/bin/env sh
set -e

# Build Tailwind CSS
npm run build:css

# Install Rust toolchain if missing
if ! which rustc >/dev/null 2>&1; then
    echo "Installing Rust toolchain..."
    RUSTUP_YES=true curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
        | sh -s -- --default-toolchain stable -y --no-modify-path 2>&1
    . "$HOME/.cargo/env"
fi
export PATH="$HOME/.cargo/bin:$PATH"

# Add wasm target if missing
if ! rustup target list --installed 2>/dev/null | grep -q wasm32-unknown-unknown; then
    rustup target add wasm32-unknown-unknown
fi

# Install trunk if not already available (download pre-built binary, ~2s vs 5min from source)
if ! which trunk >/dev/null 2>&1; then
    echo "Installing trunk..."
    CARGO_BIN="${CARGO_HOME:-$HOME/.cargo}/bin"
    mkdir -p "$CARGO_BIN"
    curl -fsSL "https://github.com/trunk-rs/trunk/releases/latest/download/trunk-x86_64-unknown-linux-gnu.tar.gz" \
        | tar xz -C "$CARGO_BIN"
fi

# Build Rust WASM
trunk build --release

# Copy SPA redirect rule for Cloudflare Pages
cp _redirects dist/_redirects
