#!/bin/bash

# Leptos Component Test Runner
# Usage: ./scripts/run_tests.sh [component_name] [test_type]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}  Leptos Component Test Runner${NC}"
    echo -e "${BLUE}================================${NC}"
}

print_test_result() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}[PASS]${NC} $2"
    else
        echo -e "${RED}[FAIL]${NC} $2"
    fi
}

# Function to run tests for a specific component
run_component_tests() {
    local component_name=$1
    local component_dir="packages/leptos/$component_name"
    
    if [ ! -d "$component_dir" ]; then
        print_warning "Component $component_name not found, skipping..."
        return 1
    fi
    
    print_status "Running tests for $component_name..."
    
    # Run cargo test for the specific component
    if cargo test -p "shadcn-ui-leptos-$component_name" --lib; then
        print_test_result 0 "$component_name tests passed"
        return 0
    else
        print_test_result 1 "$component_name tests failed"
        return 1
    fi
}

# Function to run all component tests
run_all_tests() {
    print_status "Running tests for all Leptos components..."
    
    local total_components=0
    local passed_components=0
    local failed_components=0
    
    # Find all Leptos components
    for component_dir in packages/leptos/*/; do
        if [ -d "$component_dir" ]; then
            local component_name=$(basename "$component_dir")
            total_components=$((total_components + 1))
            
            if run_component_tests "$component_name"; then
                passed_components=$((passed_components + 1))
            else
                failed_components=$((failed_components + 1))
            fi
        fi
    done
    
    echo
    print_status "Test Summary:"
    print_status "  Total components: $total_components"
    print_status "  Passed: $passed_components"
    print_status "  Failed: $failed_components"
    
    if [ $failed_components -eq 0 ]; then
        print_status "All tests passed! 🎉"
        return 0
    else
        print_error "Some tests failed! ❌"
        return 1
    fi
}

# Function to run integration tests
run_integration_tests() {
    print_status "Running integration tests..."
    
    if cargo test --test integration_test; then
        print_test_result 0 "Integration tests passed"
        return 0
    else
        print_test_result 1 "Integration tests failed"
        return 1
    fi
}

# Function to run cross-framework parity tests
run_parity_tests() {
    print_status "Running cross-framework parity tests..."
    
    if cargo test --test tooltip_integration_test; then
        print_test_result 0 "Tooltip parity tests passed"
    else
        print_test_result 1 "Tooltip parity tests failed"
        return 1
    fi
    
    if cargo test --test radio_group_integration_test; then
        print_test_result 0 "Radio group parity tests passed"
    else
        print_test_result 1 "Radio group parity tests failed"
        return 1
    fi
    
    return 0
}

# Function to run test utilities tests
run_test_utils_tests() {
    print_status "Running test utilities tests..."
    
    if cargo test -p shadcn-ui-test-utils; then
        print_test_result 0 "Test utilities tests passed"
        return 0
    else
        print_test_result 1 "Test utilities tests failed"
        return 1
    fi
}

# Function to run workspace tests
run_workspace_tests() {
    print_status "Running workspace tests..."
    
    if cargo test --workspace; then
        print_test_result 0 "Workspace tests passed"
        return 0
    else
        print_test_result 1 "Workspace tests failed"
        return 1
    fi
}

# Function to generate test coverage report
generate_coverage_report() {
    print_status "Generating test coverage report..."
    
    # Check if cargo-tarpaulin is installed
    if ! command -v cargo-tarpaulin &> /dev/null; then
        print_warning "cargo-tarpaulin not found. Install with: cargo install cargo-tarpaulin"
        return 1
    fi
    
    # Generate coverage report
    if cargo tarpaulin --out Html --output-dir coverage; then
        print_status "Coverage report generated in coverage/ directory"
        return 0
    else
        print_error "Failed to generate coverage report"
        return 1
    fi
}

# Function to run specific test types
run_test_type() {
    local test_type=$1
    
    case $test_type in
        "unit")
            print_status "Running unit tests..."
            run_all_tests
            ;;
        "integration")
            print_status "Running integration tests..."
            run_integration_tests
            ;;
        "parity")
            print_status "Running parity tests..."
            run_parity_tests
            ;;
        "utils")
            print_status "Running test utilities tests..."
            run_test_utils_tests
            ;;
        "workspace")
            print_status "Running workspace tests..."
            run_workspace_tests
            ;;
        "coverage")
            print_status "Generating coverage report..."
            generate_coverage_report
            ;;
        "all")
            print_status "Running all tests..."
            run_workspace_tests
            ;;
        *)
            print_error "Unknown test type: $test_type"
            print_status "Available test types: unit, integration, parity, utils, workspace, coverage, all"
            exit 1
            ;;
    esac
}

# Main script logic
print_header

if [ $# -eq 0 ]; then
    # No arguments provided, run all tests
    print_status "No arguments provided, running all tests..."
    run_workspace_tests
elif [ $# -eq 1 ]; then
    # One argument provided - could be component name or test type
    if [[ "$1" == -* ]]; then
        # It's a test type
        run_test_type "${1#-}"
    else
        # It's a component name
        run_component_tests "$1"
    fi
elif [ $# -eq 2 ]; then
    # Two arguments provided - component name and test type
    local component_name=$1
    local test_type=$2
    
    print_status "Running $test_type tests for $component_name..."
    
    case $test_type in
        "unit")
            run_component_tests "$component_name"
            ;;
        "integration")
            run_integration_tests
            ;;
        "parity")
            run_parity_tests
            ;;
        *)
            print_error "Unknown test type: $test_type"
            print_status "Available test types: unit, integration, parity"
            exit 1
            ;;
    esac
else
    print_error "Too many arguments provided"
    print_status "Usage: $0 [component_name] [test_type]"
    print_status "Examples:"
    print_status "  $0                    # Run all tests"
    print_status "  $0 input              # Run tests for input component"
    print_status "  $0 -unit              # Run unit tests"
    print_status "  $0 -integration       # Run integration tests"
    print_status "  $0 -parity            # Run parity tests"
    print_status "  $0 -coverage          # Generate coverage report"
    print_status "  $0 input unit         # Run unit tests for input component"
    exit 1
fi

print_status "Test run completed!"
