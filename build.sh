#!/bin/bash

# Build script for key-listener with different feature combinations

echo "Building key-listener with different configurations..."

# Default build with all features
echo "Building default (all features enabled)..."
cargo build --release

# Individual feature builds
echo "Building direct-only binary..."
cargo build --release --bin key-listener-direct --features direct --no-default-features

echo "Building complex-only binary..."
cargo build --release --bin key-listener-complex --features complex --no-default-features

echo "Building hold-and-release-only binary..."
cargo build --release --bin key-listener-hold-release --features hold_and_release --no-default-features

echo "Building simulation-only binary..."
cargo build --release --bin key-listener-simulation --features simulation --no-default-features

echo "Building mouse-only binary..."
cargo build --release --bin key-listener-mouse --features mouse --no-default-features

echo "All builds completed!"
echo ""
echo "Available binaries:"
echo "- key-listener (all features)"
echo "- key-listener-direct (direct mode only)"
echo "- key-listener-complex (complex mode only)"
echo "- key-listener-hold-release (hold and release mode only)"
echo "- key-listener-simulation (simulation mode only)"
echo "- key-listener-mouse (mouse mode only)"
