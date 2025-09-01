#!/bin/bash

# Dynamic Loading System Test Runner
# Comprehensive E2E testing for the enhanced lazy loading system

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test configuration
TEST_DIR="tests/e2e"
REPORT_DIR="test-results"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

echo -e "${BLUE}🚀 Dynamic Loading System Test Runner${NC}"
echo -e "${BLUE}=====================================${NC}"
echo ""

# Function to print colored output
print_status() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# Check if Playwright is installed
check_playwright() {
    if ! command -v npx &> /dev/null; then
        print_error "npx not found. Please install Node.js and npm."
        exit 1
    fi
    
    if ! npx playwright --version &> /dev/null; then
        print_warning "Playwright not found. Installing..."
        npm install -D @playwright/test
        npx playwright install
    fi
}

# Check if the development server is running
check_server() {
    print_info "Checking if development server is running..."
    
    if curl -s http://localhost:8080 > /dev/null 2>&1; then
        print_status "Development server is running on port 8080"
        return 0
    else
        print_warning "Development server not running on port 8080"
        return 1
    fi
}

# Start the development server
start_server() {
    print_info "Starting development server..."
    
    # Change to the correct directory
    cd book-examples/leptos
    
    # Start trunk serve in background with output
    print_info "Running: trunk serve"
    trunk serve > /tmp/trunk.log 2>&1 &
    SERVER_PID=$!
    
    # Store PID for cleanup
    echo $SERVER_PID > .server.pid
    
    # Wait for server to start with progress indicator
    print_info "Waiting for server to start..."
    for i in {1..30}; do
        if curl -s http://localhost:8080 > /dev/null 2>&1; then
            print_status "Server started successfully on port 8080"
            return 0
        fi
        
        # Show progress
        echo -n "."
        sleep 1
    done
    
    echo "" # New line after progress dots
    
    if ! curl -s http://localhost:8080 > /dev/null 2>&1; then
        print_error "Failed to start development server after 30 seconds"
        print_error "Check /tmp/trunk.log for server errors"
        print_info "You can start the server manually with: cd book-examples/leptos && trunk serve"
        return 1
    fi
}

# Clean up server on exit
cleanup() {
    if [ -f book-examples/leptos/.server.pid ]; then
        SERVER_PID=$(cat book-examples/leptos/.server.pid)
        if kill -0 $SERVER_PID 2>/dev/null; then
            print_info "Stopping development server (PID: $SERVER_PID)..."
            kill $SERVER_PID
            rm book-examples/leptos/.server.pid
        fi
    fi
}

# Set up trap for cleanup
trap cleanup EXIT

# Create report directory
setup_reports() {
    mkdir -p $REPORT_DIR
    print_status "Report directory created: $REPORT_DIR"
}

# Run specific test suite
run_test_suite() {
    local suite_name=$1
    local test_file=$2
    local browser=${3:-chromium}
    
    echo ""
    print_info "Running $suite_name tests on $browser..."
    echo "Test file: $test_file"
    echo "Browser: $browser"
    echo ""
    
    # Run the test with better output handling
    if npx playwright test $test_file --project=$browser --reporter=html,json,junit --timeout=30000; then
        print_status "$suite_name tests passed on $browser"
        return 0
    else
        print_error "$suite_name tests failed on $browser"
        return 1
    fi
}

# Run all tests
run_all_tests() {
    local browser=${1:-chromium}
    local failed_tests=0
    
    echo ""
    print_info "Running all test suites on $browser..."
    echo ""
    
    # Run dynamic loading tests
    if run_test_suite "Dynamic Loading System" "tests/e2e/dynamic-loading.spec.ts" $browser; then
        print_status "Dynamic Loading tests completed successfully"
    else
        failed_tests=$((failed_tests + 1))
    fi
    
    # Run bundle optimization tests
    if run_test_suite "Bundle Optimization" "tests/e2e/bundle-optimization.spec.ts" $browser; then
        print_status "Bundle Optimization tests completed successfully"
    else
        failed_tests=$((failed_tests + 1))
    fi
    
    # Run existing component tests
    if run_test_suite "Component Integration" "tests/e2e/component-integration.spec.ts" $browser; then
        print_status "Component Integration tests completed successfully"
    else
        failed_tests=$((failed_tests + 1))
    fi
    
    # Run performance tests
    if run_test_suite "Performance" "tests/e2e/performance.spec.ts" $browser; then
        print_status "Performance tests completed successfully"
    else
        failed_tests=$((failed_tests + 1))
    fi
    
    # Run accessibility tests
    if run_test_suite "Accessibility" "tests/e2e/accessibility.spec.ts" $browser; then
        print_status "Accessibility tests completed successfully"
    else
        failed_tests=$((failed_tests + 1))
    fi
    
    echo ""
    if [ $failed_tests -eq 0 ]; then
        print_status "All test suites completed successfully! 🎉"
    else
        print_error "$failed_tests test suite(s) failed"
    fi
    
    return $failed_tests
}

# Run tests with specific configuration
run_custom_tests() {
    local test_pattern=$1
    local browser=${2:-chromium}
    
    echo ""
    print_info "Running custom tests matching: $test_pattern"
    print_info "Browser: $browser"
    echo ""
    
    if npx playwright test --grep="$test_pattern" --project=$browser --reporter=html,json,junit --timeout=30000; then
        print_status "Custom tests completed successfully"
        return 0
    else
        print_error "Custom tests failed"
        return 1
    fi
}

# Generate test summary
generate_summary() {
    echo ""
    print_info "Generating test summary..."
    
    if [ -f "$REPORT_DIR/results.json" ]; then
        echo "Test results available in: $REPORT_DIR/results.json"
        echo "HTML report available in: $REPORT_DIR/playwright-report/index.html"
        echo "JUnit report available in: $REPORT_DIR/results.xml"
    fi
    
    # Count test results
    if command -v jq &> /dev/null; then
        local total_tests=$(jq '.stats.total' "$REPORT_DIR/results.json" 2>/dev/null || echo "unknown")
        local passed_tests=$(jq '.stats.passed' "$REPORT_DIR/results.json" 2>/dev/null || echo "unknown")
        local failed_tests=$(jq '.stats.failed' "$REPORT_DIR/results.json" 2>/dev/null || echo "unknown")
        
        echo ""
        echo "Test Summary:"
        echo "  Total Tests: $total_tests"
        echo "  Passed: $passed_tests"
        echo "  Failed: $failed_tests"
    fi
}

# Show help
show_help() {
    echo "Usage: $0 [OPTIONS] [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  all                    Run all test suites (default)"
    echo "  dynamic                Run only dynamic loading tests"
    echo "  bundle                 Run only bundle optimization tests"
    echo "  components             Run only component integration tests"
    echo "  performance            Run only performance tests"
    echo "  accessibility          Run only accessibility tests"
    echo "  custom <pattern>      Run tests matching pattern"
    echo "  help                   Show this help message"
    echo ""
    echo "Options:"
    echo "  --browser <browser>   Specify browser (chromium, firefox, webkit)"
    echo "  --headless            Run in headless mode"
    echo "  --debug               Run with debug output"
    echo "  --report              Generate detailed reports"
    echo "  --no-server           Don't start server (assume it's already running)"
    echo ""
    echo "Examples:"
    echo "  $0                    # Run all tests on chromium"
    echo "  $0 dynamic            # Run only dynamic loading tests"
    echo "  $0 --browser firefox # Run all tests on Firefox"
    echo "  $0 custom 'button'   # Run tests containing 'button'"
    echo "  $0 --no-server       # Run tests without starting server"
}

# Main execution
main() {
    local command="all"
    local browser="chromium"
    local headless=true
    local debug=false
    local generate_reports=true
    local start_server_flag=true
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --browser)
                browser="$2"
                shift 2
                ;;
            --headless)
                headless=true
                shift
                ;;
            --debug)
                debug=true
                shift
                ;;
            --report)
                generate_reports=true
                shift
                ;;
            --no-server)
                start_server_flag=false
                shift
                ;;
            help|--help|-h)
                show_help
                exit 0
                ;;
            *)
                command="$1"
                shift
                ;;
        esac
    done
    
    # Set environment variables
    if [ "$headless" = true ]; then
        export PLAYWRIGHT_HEADLESS=true
    else
        export PLAYWRIGHT_HEADLESS=false
    fi
    
    if [ "$debug" = true ]; then
        export DEBUG=pw:api
    fi
    
    echo -e "${BLUE}Configuration:${NC}"
    echo "  Command: $command"
    echo "  Browser: $browser"
    echo "  Headless: $headless"
    echo "  Debug: $debug"
    echo "  Reports: $generate_reports"
    echo "  Start Server: $start_server_flag"
    echo ""
    
    # Check prerequisites
    check_playwright
    
    # Setup
    setup_reports
    
    # Handle server startup
    if [ "$start_server_flag" = true ]; then
        if ! check_server; then
            if ! start_server; then
                print_error "Failed to start development server. Exiting."
                exit 1
            fi
        fi
    else
        if ! check_server; then
            print_error "Server not running and --no-server flag specified."
            print_info "Please start the server manually: cd book-examples/leptos && trunk serve"
            exit 1
        fi
    fi
    
    # Give server a moment to stabilize
    sleep 2
    
    # Run tests based on command
    case $command in
        all)
            run_all_tests $browser
            ;;
        dynamic)
            run_test_suite "Dynamic Loading System" "tests/e2e/dynamic-loading.spec.ts" $browser
            ;;
        bundle)
            run_test_suite "Bundle Optimization" "tests/e2e/bundle-optimization.spec.ts" $browser
            ;;
        components)
            run_test_suite "Component Integration" "tests/e2e/component-integration.spec.ts" $browser
            ;;
        performance)
            run_test_suite "Performance" "tests/e2e/performance.spec.ts" $browser
            ;;
        accessibility)
            run_test_suite "Accessibility" "tests/e2e/accessibility.spec.ts" $browser
            ;;
        custom)
            if [ -z "$2" ]; then
                print_error "Custom pattern required. Usage: $0 custom <pattern>"
                exit 1
            fi
            run_custom_tests "$2" $browser
            ;;
        *)
            print_error "Unknown command: $command"
            show_help
            exit 1
            ;;
    esac
    
    # Generate summary if requested
    if [ "$generate_reports" = true ]; then
        generate_summary
    fi
    
    echo ""
    print_info "Test execution completed!"
    print_info "Check $REPORT_DIR for detailed results"
}

# Run main function with all arguments
main "$@"
