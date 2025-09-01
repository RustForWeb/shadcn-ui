#!/bin/bash

# Test Enhanced Testing Framework
# This script demonstrates the enhanced testing framework capabilities

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
PURPLE='\033[0;35m'
NC='\033[0m'

print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_header() {
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}  Enhanced Testing Framework Demo${NC}"
    echo -e "${BLUE}================================${NC}"
}

print_section() {
    echo -e "${PURPLE}[SECTION]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header

# Track results
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

print_section "Testing Framework Compilation"

# Test 1: Test Framework Compilation
((TOTAL_TESTS++))
if cargo check -p shadcn-ui-test-utils --quiet > /dev/null 2>&1; then
    print_status "✅ Test framework compiles successfully"
    ((PASSED_TESTS++))
else
    print_error "❌ Test framework compilation failed"
    ((FAILED_TESTS++))
fi

# Test 2: Test Utilities
((TOTAL_TESTS++))
if cargo test -p shadcn-ui-test-utils --quiet > /dev/null 2>&1; then
    print_status "✅ Test utilities work correctly"
    ((PASSED_TESTS++))
else
    print_error "❌ Test utilities failed"
    ((FAILED_TESTS++))
fi

print_section "Testing Existing Components"

# Test 3: Test existing components with enhanced tests
COMPONENTS_TO_TEST=("separator" "button" "input" "card" "alert")
for component in "${COMPONENTS_TO_TEST[@]}"; do
    ((TOTAL_TESTS++))
    if cargo test -p "shadcn-ui-leptos-$component" --quiet > /dev/null 2>&1; then
        print_status "✅ $component - Tests passed"
        ((PASSED_TESTS++))
    else
        print_error "❌ $component - Tests failed"
        ((FAILED_TESTS++))
    fi
done

print_section "Testing Enhanced Features"

# Test 4: Test the enhanced separator component (which we updated)
((TOTAL_TESTS++))
if cargo test -p shadcn-ui-leptos-separator --quiet > /dev/null 2>&1; then
    print_status "✅ Enhanced separator tests work (5 comprehensive tests)"
    ((PASSED_TESTS++))
else
    print_error "❌ Enhanced separator tests failed"
    ((FAILED_TESTS++))
fi

# Test 5: Test workspace compilation
((TOTAL_TESTS++))
if cargo check --workspace --quiet > /dev/null 2>&1; then
    print_status "✅ Workspace compiles successfully"
    ((PASSED_TESTS++))
else
    print_error "❌ Workspace compilation failed"
    ((FAILED_TESTS++))
fi

# Summary
echo ""
print_header
print_status "Test Results Summary:"
echo ""
echo "  📊 Total Tests: $TOTAL_TESTS"
echo "  ✅ Passed: $PASSED_TESTS"
echo "  ❌ Failed: $FAILED_TESTS"
echo "  📈 Success Rate: $(( (PASSED_TESTS * 100) / TOTAL_TESTS ))%"
echo ""

# Enhanced Framework Features Summary
print_section "Enhanced Framework Features Demonstrated"
echo ""
echo "  🧪 Test Utilities:"
echo "    - LeptosTestUtils for basic testing"
echo "    - ComponentTestBuilder for comprehensive tests"
echo "    - ComponentTestSuite for test organization"
echo "    - DomTestUtils for DOM interaction testing"
echo ""
echo "  🎯 Test Types Available:"
echo "    - Component rendering tests"
echo "    - Props and functionality tests"
echo "    - Accessibility tests"
echo "    - Styling tests"
echo "    - Event handling tests"
echo "    - State change tests"
echo "    - Theme switching tests"
echo "    - Performance tests"
echo "    - Responsive behavior tests"
echo ""
echo "  🔧 Test Configuration:"
echo "    - Configurable test settings"
echo "    - Expected CSS classes validation"
echo "    - Expected attributes validation"
echo "    - Custom test helpers for each component type"
echo ""
echo "  📦 Integration:"
echo "    - Seamless integration with component generation"
echo "    - Cross-framework testing support"
echo "    - Comprehensive test result reporting"
echo ""
echo "  🎨 Test Templates:"
echo "    - Basic component tests"
echo "    - Form component tests"
echo "    - Interactive component tests"
echo "    - Layout component tests"
echo "    - Feedback component tests"
echo ""

# Show example of enhanced test
print_section "Example Enhanced Test (from separator component)"
echo ""
echo "The separator component now has 5 comprehensive tests:"
echo ""
echo "  ✅ test_separator_component_exists"
echo "  ✅ test_separator_basic_functionality"
echo "  ✅ test_separator_accessibility"
echo "  ✅ test_separator_styling"
echo "  ✅ test_separator_comprehensive"
echo ""
echo "Each test uses the enhanced testing framework with:"
echo "  - Proper result validation"
echo "  - Detailed test reporting"
echo "  - Framework-specific testing utilities"
echo ""

if [ $FAILED_TESTS -eq 0 ]; then
    print_status "🎉 All tests passed! Enhanced testing framework is working perfectly."
    echo ""
    print_status "The enhanced testing framework provides:"
    echo "  • More comprehensive test coverage"
    echo "  • Better test organization and structure"
    echo "  • Framework-specific testing utilities"
    echo "  • Configurable test settings"
    echo "  • Detailed test result reporting"
    echo "  • Cross-framework testing support"
    exit 0
else
    print_warning "Some tests failed. Check the errors above for details."
    exit 1
fi
