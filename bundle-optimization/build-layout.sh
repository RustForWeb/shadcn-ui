#!/bin/bash
# Build script for layout components

set -e

echo "Building layout bundle..."
cd book-examples/leptos

# Use layout features
cargo build --release --features "essential,layout"

echo "Layout bundle built successfully!"
echo "Bundle location: target/wasm32-unknown-unknown/release/"
