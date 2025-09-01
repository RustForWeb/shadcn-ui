#!/bin/bash

# Tooltip Component Test Runner
# Comprehensive test suite for Tooltip component across all frameworks

set -e

echo "🧪 Running Tooltip Component Tests"
echo "=================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m' 
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Function to run a test with proper error handling
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    echo -e "${BLUE}ℹ️  INFO: Running $test_name...${NC}"
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    if eval "$test_command"; then
        echo -e "${GREEN}✅ PASS: $test_name completed successfully${NC}"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo -e "${RED}❌ FAIL: $test_name failed${NC}"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        return 1
    fi
}

# Function to print test results
print_results() {
    echo ""
    echo "📊 Test Summary"
    echo "==============="
    echo "Total tests: $TOTAL_TESTS"
    echo -e "Passed: ${GREEN}$PASSED_TESTS${NC}"
    echo -e "Failed: ${RED}$FAILED_TESTS${NC}"
    
    if [ $FAILED_TESTS -eq 0 ]; then
        echo -e "${GREEN}✅ PASS: All Tooltip tests passed! 🎉${NC}"
        return 0
    else
        echo -e "${RED}❌ FAIL: $FAILED_TESTS test(s) failed${NC}"
        return 1
    fi
}

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo -e "${YELLOW}⚠️  WARNING: wasm-pack not found. Installing...${NC}"
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Check if required tools are available
echo -e "${BLUE}🔧 Checking prerequisites...${NC}"

# Check Rust and Cargo
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ ERROR: cargo not found. Please install Rust.${NC}"
    exit 1
fi

# Check Node.js for wasm-pack tests
if ! command -v node &> /dev/null; then
    echo -e "${YELLOW}⚠️  WARNING: Node.js not found. Some tests may not run.${NC}"
fi

echo -e "${GREEN}✅ Prerequisites check passed${NC}"
echo ""

# Unit Tests - Yew
echo -e "${BLUE}🦀 Running Yew Tooltip Unit Tests${NC}"
run_test "Yew Tooltip Unit Tests" "cargo test -p shadcn-ui-yew-tooltip --target wasm32-unknown-unknown"

# Unit Tests - Leptos  
echo -e "${BLUE}🦀 Running Leptos Tooltip Unit Tests${NC}"
run_test "Leptos Tooltip Unit Tests" "cargo test -p shadcn-ui-leptos-tooltip --target wasm32-unknown-unknown"

# Integration Tests
echo -e "${BLUE}🔗 Running integration tests${NC}"
run_test "Tooltip Integration Tests" "cargo test --test tooltip_integration_test"

# Build Tests
echo -e "${BLUE}🏗️  Running build tests${NC}"
run_test "Yew Tooltip Build Test" "cargo check -p shadcn-ui-yew-tooltip"
run_test "Leptos Tooltip Build Test" "cargo check -p shadcn-ui-leptos-tooltip"

# Feature Tests
echo -e "${BLUE}🎛️  Running feature tests${NC}"
run_test "Default Theme Build" "cargo check -p shadcn-ui-leptos-tooltip"
run_test "New York Theme Build" "cargo check -p shadcn-ui-leptos-tooltip --features new_york"
run_test "Yew Default Theme Build" "cargo check -p shadcn-ui-yew-tooltip" 
run_test "Yew New York Theme Build" "cargo check -p shadcn-ui-yew-tooltip --features new_york"

# Wasm Tests (if wasm-pack is available)
if command -v wasm-pack &> /dev/null; then
    echo -e "${BLUE}🕸️  Running WASM tests${NC}"
    
    # Test Yew in browser environment
    cd packages/yew/tooltip
    run_test "Yew WASM Tests" "wasm-pack test --headless --firefox"
    cd ../../..
    
    # Test Leptos in browser environment 
    cd packages/leptos/tooltip
    run_test "Leptos WASM Tests" "wasm-pack test --headless --firefox"
    cd ../../..
else
    echo -e "${YELLOW}⚠️  SKIPPING: WASM tests (wasm-pack not available)${NC}"
fi

# Linting Tests
echo -e "${BLUE}🧹 Running linting tests${NC}"
run_test "Tooltip Clippy Lints" "cargo clippy -p shadcn-ui-yew-tooltip -p shadcn-ui-leptos-tooltip -- -D warnings"

# Format Tests  
echo -e "${BLUE}📝 Running format tests${NC}"
run_test "Tooltip Format Check" "cargo fmt --check -p shadcn-ui-yew-tooltip -p shadcn-ui-leptos-tooltip"

# Documentation Tests
echo -e "${BLUE}📚 Running documentation tests${NC}"
run_test "Tooltip Documentation" "cargo doc --no-deps -p shadcn-ui-yew-tooltip -p shadcn-ui-leptos-tooltip"

# Example Tests (if examples exist)
if [ -d "book-examples" ]; then
    echo -e "${BLUE}📖 Running example tests${NC}"
    
    # Test Leptos examples
    if [ -d "book-examples/leptos" ]; then
        run_test "Leptos Example Build" "cd book-examples/leptos && cargo check"
    fi
    
    # Test Yew examples
    if [ -d "book-examples/yew" ]; then
        run_test "Yew Example Build" "cd book-examples/yew && cargo check"
    fi
fi

# Security Audit
echo -e "${BLUE}🔒 Running security audit${NC}"
if command -v cargo-audit &> /dev/null; then
    run_test "Security Audit" "cargo audit"
else
    echo -e "${YELLOW}⚠️  SKIPPING: Security audit (cargo-audit not installed)${NC}"
fi

# Final Results
print_results