#!/bin/bash
# Build script for essential components only

set -e

echo "Building essential components bundle..."
cd book-examples/leptos

# Use essential features only
cargo build --release --features essential

echo "Essential bundle built successfully!"
echo "Bundle location: target/wasm32-unknown-unknown/release/"
