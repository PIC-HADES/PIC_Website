#!/usr/bin/env sh
set -e

# Build Tailwind CSS
npm run build:css

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
