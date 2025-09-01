#!/bin/bash

# Test runner script for radio-group component
# This script runs all tests related to the radio-group component

set -e

echo "🧪 Running RadioGroup Component Tests"
echo "====================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    local status=$1
    local message=$2
    case $status in
        "PASS")
            echo -e "${GREEN}✅ PASS${NC}: $message"
            ;;
        "FAIL")
            echo -e "${RED}❌ FAIL${NC}: $message"
            ;;
        "INFO")
            echo -e "${BLUE}ℹ️  INFO${NC}: $message"
            ;;
        "WARN")
            echo -e "${YELLOW}⚠️  WARN${NC}: $message"
            ;;
    esac
}

# Function to run tests and capture output
run_test() {
    local test_name=$1
    local test_command=$2
    
    print_status "INFO" "Running $test_name..."
    
    if eval "$test_command" 2>&1; then
        print_status "PASS" "$test_name completed successfully"
        return 0
    else
        print_status "FAIL" "$test_name failed"
        return 1
    fi
}

# Track test results
total_tests=0
passed_tests=0
failed_tests=0

# Test 1: Yew RadioGroup Unit Tests
print_status "INFO" "Testing Yew RadioGroup implementation..."
if run_test "Yew RadioGroup Unit Tests" "cargo test -p shadcn-ui-yew-radio-group"; then
    ((passed_tests++))
else
    ((failed_tests++))
fi
((total_tests++))

# Test 2: Leptos RadioGroup Unit Tests
print_status "INFO" "Testing Leptos RadioGroup implementation..."
if run_test "Leptos RadioGroup Unit Tests" "cargo test -p shadcn-ui-leptos-radio-group"; then
    ((passed_tests++))
else
    ((failed_tests++))
fi
((total_tests++))

# Test 3: Integration Tests
print_status "INFO" "Running integration tests..."
if run_test "RadioGroup Integration Tests" "cargo test --test radio_group_integration_test"; then
    ((passed_tests++))
else
    ((failed_tests++))
fi
((total_tests++))

# Test 4: Cross-framework Parity Tests
print_status "INFO" "Testing cross-framework parity..."
if run_test "Cross-framework Parity Tests" "cargo test test_radio_group_cross_framework_parity"; then
    ((passed_tests++))
else
    ((failed_tests++))
fi
((total_tests++))

# Test 5: Theme Consistency Tests
print_status "INFO" "Testing theme consistency..."
if run_test "Theme Consistency Tests" "cargo test test_radio_group_theme_consistency"; then
    ((passed_tests++))
else
    ((failed_tests++))
fi
((total_tests++))

# Test 6: Accessibility Tests
print_status "INFO" "Testing accessibility features..."
if run_test "Accessibility Tests" "cargo test test_radio_group_accessibility_features"; then
    ((passed_tests++))
else
    ((failed_tests++))
fi
((total_tests++))

# Test 7: Registry Integration Tests
print_status "INFO" "Testing registry integration..."
if run_test "Registry Integration Tests" "cargo test test_radio_group_registry_integration"; then
    ((passed_tests++))
else
    ((failed_tests++))
fi
((total_tests++))

# Test 8: Property Validation Tests
print_status "INFO" "Testing property validation..."
if run_test "Property Validation Tests" "cargo test test_radio_group_property_validation"; then
    ((passed_tests++))
else
    ((failed_tests++))
fi
((total_tests++))

# Test 9: Build Tests
print_status "INFO" "Testing component builds..."
if run_test "Yew RadioGroup Build" "cargo build -p shadcn-ui-yew-radio-group"; then
    ((passed_tests++))
else
    ((failed_tests++))
fi
((total_tests++))

if run_test "Leptos RadioGroup Build" "cargo build -p shadcn-ui-leptos-radio-group"; then
    ((passed_tests++))
else
    ((failed_tests++))
fi
((total_tests++))

# Test 10: Documentation Tests
print_status "INFO" "Testing documentation..."
if run_test "Documentation Tests" "cargo doc -p shadcn-ui-yew-radio-group --no-deps"; then
    ((passed_tests++))
else
    ((failed_tests++))
fi
((total_tests++))

if run_test "Documentation Tests" "cargo doc -p shadcn-ui-leptos-radio-group --no-deps"; then
    ((passed_tests++))
else
    ((failed_tests++))
fi
((total_tests++))

# Summary
echo ""
echo "📊 Test Summary"
echo "==============="
echo "Total tests: $total_tests"
echo -e "Passed: ${GREEN}$passed_tests${NC}"
echo -e "Failed: ${RED}$failed_tests${NC}"

if [ $failed_tests -eq 0 ]; then
    print_status "PASS" "All RadioGroup tests passed! 🎉"
    exit 0
else
    print_status "FAIL" "$failed_tests test(s) failed. Please check the output above."
    exit 1
fi
