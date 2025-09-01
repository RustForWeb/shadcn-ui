#!/bin/bash

# Batch Component Generator for Leptos
# Usage: ./scripts/generate_batch_components.sh [batch_name]

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_header() {
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}  Batch Component Generator${NC}"
    echo -e "${BLUE}================================${NC}"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to get component list for a batch
get_batch_components() {
    local batch_name=$1
    
    case $batch_name in
        "phase1")
            echo "separator:basic skeleton:basic textarea:form"
            ;;
        "phase2")
            echo "dropdown-menu:interactive hover-card:interactive menubar:interactive navigation-menu:interactive popover:interactive toggle:interactive"
            ;;
        "phase3")
            echo "scroll-area:layout sheet:layout table:layout tabs:layout"
            ;;
        "phase4")
            echo "dialog:feedback toast:feedback progress:feedback switch:form slider:form"
            ;;
        "all")
            echo "separator:basic skeleton:basic textarea:form switch:form slider:form dropdown-menu:interactive hover-card:interactive menubar:interactive navigation-menu:interactive popover:interactive toggle:interactive scroll-area:layout sheet:layout table:layout tabs:layout dialog:feedback toast:feedback progress:feedback"
            ;;
        *)
            echo ""
            ;;
    esac
}

print_usage() {
    echo "Usage: $0 [batch_name|component:type]"
    echo ""
    echo "Available batches:"
    echo "  phase1: separator:basic skeleton:basic textarea:form"
    echo "  phase2: dropdown-menu:interactive hover-card:interactive menubar:interactive navigation-menu:interactive popover:interactive toggle:interactive"
    echo "  phase3: scroll-area:layout sheet:layout table:layout tabs:layout"
    echo "  phase4: dialog:feedback toast:feedback progress:feedback switch:form slider:form"
    echo "  all: All components from all phases"
    echo ""
    echo "Available component types:"
    echo "  basic: badge separator skeleton"
    echo "  form: checkbox textarea switch slider"
    echo "  interactive: button dropdown-menu hover-card menubar navigation-menu popover toggle"
    echo "  layout: card aspect-ratio scroll-area sheet table tabs"
    echo "  feedback: alert dialog toast progress"
    echo ""
    echo "Examples:"
    echo "  $0 phase1                    # Generate Phase 1 components"
    echo "  $0 separator:basic           # Generate single component"
    echo "  $0 all                       # Generate all components"
}

if [ $# -eq 0 ]; then
    print_usage
    exit 1
fi

BATCH_NAME=$1

print_header

# Check if it's a predefined batch
COMPONENT_LIST=$(get_batch_components "$BATCH_NAME")
if [ -n "$COMPONENT_LIST" ]; then
    print_status "Generating batch: $BATCH_NAME"
elif [[ $BATCH_NAME == *":"* ]]; then
    # Single component specification
    print_status "Generating single component: $BATCH_NAME"
    COMPONENT_LIST="$BATCH_NAME"
else
    print_error "Unknown batch: $BATCH_NAME"
    print_usage
    exit 1
fi

# Track success and failures
SUCCESS_COUNT=0
FAILURE_COUNT=0
FAILED_COMPONENTS=""

# Generate components
for component_spec in $COMPONENT_LIST; do
    if [[ $component_spec == *":"* ]]; then
        IFS=':' read -r component_name component_type <<< "$component_spec"
        
        print_status "Generating $component_name ($component_type)..."
        
        if ./scripts/generate_component_v2.sh "$component_name" "$component_type"; then
            print_status "✅ $component_name generated successfully"
            SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
        else
            print_error "❌ Failed to generate $component_name"
            FAILURE_COUNT=$((FAILURE_COUNT + 1))
            FAILED_COMPONENTS="$FAILED_COMPONENTS $component_name"
        fi
    else
        print_warning "Invalid component specification: $component_spec"
        FAILURE_COUNT=$((FAILURE_COUNT + 1))
    fi
done

# Summary
echo ""
print_header
print_status "Batch generation completed!"
echo ""
print_status "Results:"
echo "  ✅ Success: $SUCCESS_COUNT"
echo "  ❌ Failures: $FAILURE_COUNT"

if [ -n "$FAILED_COMPONENTS" ]; then
    echo ""
    print_error "Failed components:"
    for component in $FAILED_COMPONENTS; do
        echo "  - $component"
    done
fi

echo ""
print_status "Next steps:"
echo "  1. Run 'cargo check --workspace' to verify compilation"
echo "  2. Run 'cargo test' to run component tests"
echo "  3. Update component classes and styles as needed"

if [ $FAILURE_COUNT -eq 0 ]; then
    echo ""
    print_status "🎉 All components generated successfully!"
    exit 0
else
    echo ""
    print_warning "Some components failed to generate. Check the errors above."
    exit 1
fi
