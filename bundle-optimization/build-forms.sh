#!/bin/bash
# Build script for form-focused components

set -e

echo "Building forms bundle..."
cd book-examples/leptos

# Use form features
cargo build --release --features "essential,forms"

echo "Forms bundle built successfully!"
echo "Bundle location: target/wasm32-unknown-unknown/release/"
