#!/bin/bash
set -e

# Setup Rust if not already installed
if ! command -v rustup &> /dev/null; then
  echo "Installing Rust..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
fi

export PATH="$HOME/.cargo/bin:$PATH"

# Add WebAssembly target
echo "Adding wasm32-unknown-unknown target..."
rustup target add wasm32-unknown-unknown

# Download trunk if not in PATH
if ! command -v trunk &> /dev/null; then
  echo "Installing trunk..."
  curl -fsSL https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-musl.tar.gz | tar -xzf-
  chmod +x trunk
  export PATH="$PWD:$PATH"
fi

# Build release
echo "Building WebAssembly..."
trunk build --release

# Ensure _redirects is in dist for SPA routing
if [ -f "public/_redirects" ]; then
  cp public/_redirects dist/
  echo "Redirects configured"
fi

echo "✅ Build completed successfully!"
