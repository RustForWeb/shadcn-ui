#!/usr/bin/env bash

# Test runner script for shadcn/ui Rust project
# Ensures tests auto-close and provides better output

set -e

echo "🧪 Running Playwright tests with auto-close..."

# Function to cleanup on exit
cleanup() {
    echo "🧹 Cleaning up test environment..."
    # Kill any remaining Playwright processes
    pkill -f "playwright" || true
    pkill -f "chromium" || true
    pkill -f "firefox" || true
    echo "✅ Cleanup complete"
}

# Set trap to cleanup on exit
trap cleanup EXIT

# Run tests with auto-close
echo "🚀 Starting tests..."
pnpm exec playwright test --reporter=line "$@"

# Tests completed, cleanup will happen automatically
echo "✅ All tests completed successfully!"
