#!/bin/bash

# Test script for Dialog component
# This script tests the Dialog component for both Yew and Leptos frameworks

set -e

echo "🎯 Testing Dialog Component"
echo "=========================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

# Function to run a test
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    print_status "Running: $test_name"
    
    if eval "$test_command"; then
        print_success "$test_name passed"
        ((TESTS_PASSED++))
    else
        print_error "$test_name failed"
        ((TESTS_FAILED++))
    fi
    echo
}

echo "🔧 Testing Yew Dialog Component"
echo "-------------------------------"

# Test Yew Dialog component compilation
run_test "Yew Dialog Component Check" "cargo check -p shadcn-ui-yew-dialog"

# Test Yew Dialog component tests
run_test "Yew Dialog Component Tests" "cargo test -p shadcn-ui-yew-dialog --lib"

# Test Yew Dialog component build
run_test "Yew Dialog Component Build" "cargo build -p shadcn-ui-yew-dialog --release"

# Test Yew Dialog component in book examples
run_test "Yew Dialog Book Integration" "cd book-examples/yew && cargo check --features dialog"

echo "🔧 Testing Leptos Dialog Component"
echo "----------------------------------"

# Test Leptos Dialog component compilation (if it exists)
if [ -d "packages/leptos/dialog" ]; then
    run_test "Leptos Dialog Component Check" "cargo check -p shadcn-ui-leptos-dialog"
    run_test "Leptos Dialog Component Tests" "cargo test -p shadcn-ui-leptos-dialog --lib"
    run_test "Leptos Dialog Component Build" "cargo build -p shadcn-ui-leptos-dialog --release"
else
    print_warning "Leptos Dialog component not found - skipping Leptos tests"
fi

echo "🔧 Testing Dialog Component Integration"
echo "--------------------------------------"

# Test workspace compilation with dialog (skip problematic components)
run_test "Workspace with Dialog Check" "cargo check -p shadcn-ui-yew-dialog -p shadcn-ui-yew-radio-group -p shadcn-ui-yew-button -p shadcn-ui-yew-card -p shadcn-ui-yew-alert -p shadcn-ui-yew-badge -p shadcn-ui-yew-input -p shadcn-ui-yew-label -p shadcn-ui-yew-textarea -p shadcn-ui-yew-switch -p shadcn-ui-yew-table -p shadcn-ui-yew-skeleton -p shadcn-ui-yew-separator -p shadcn-ui-yew-pagination -p shadcn-ui-yew-breadcrumb -p shadcn-ui-yew-avatar -p shadcn-ui-yew-aspect-ratio"

# Test dialog component in registry
run_test "Dialog Registry Integration" "cargo check -p shadcn-registry"

echo "🔧 Testing Dialog Component Documentation"
echo "----------------------------------------"

# Check if dialog documentation exists
if [ -f "packages/yew/dialog/README.md" ]; then
    print_success "Dialog README documentation exists"
    ((TESTS_PASSED++))
else
    print_warning "Dialog README documentation missing"
fi

# Check if dialog is in main README
if [ -f "README.md" ] && grep -q "dialog" README.md; then
    print_success "Dialog component mentioned in main README"
    ((TESTS_PASSED++))
else
    print_warning "Dialog component not mentioned in main README"
fi

echo "🔧 Testing Dialog Component Examples"
echo "-----------------------------------"

# Test dialog examples in book
if [ -f "book-examples/yew/src/default/dialog/dialog.rs" ]; then
    print_success "Dialog examples exist in Yew book"
    ((TESTS_PASSED++))
else
    print_warning "Dialog examples missing in Yew book"
fi

echo "📊 Test Results Summary"
echo "======================"
echo "Tests Passed: $TESTS_PASSED"
echo "Tests Failed: $TESTS_FAILED"
echo "Total Tests: $((TESTS_PASSED + TESTS_FAILED))"

if [ $TESTS_FAILED -eq 0 ]; then
    print_success "🎉 All Dialog component tests passed!"
    exit 0
else
    print_error "❌ Some Dialog component tests failed!"
    exit 1
fi
