#!/bin/bash

# Bundle Analysis Script for Production Readiness
# Analyzes WASM bundle sizes and identifies optimization opportunities

set -e

echo "=== 📦 BUNDLE ANALYSIS FOR PRODUCTION READINESS ==="
echo ""

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

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Please run this script from the project root directory"
    exit 1
fi

# Create analysis directory
ANALYSIS_DIR="bundle-analysis"
mkdir -p "$ANALYSIS_DIR"

print_status "Starting bundle analysis..."

# Build production version
print_status "Building production version..."
cd book-examples/leptos

# Clean previous builds
trunk clean

# Build with production optimizations
print_status "Building with Trunk (production mode)..."
trunk build --release

# Check if build was successful
if [ ! "$(find dist -name "*.js" | wc -l)" -gt 0 ]; then
    print_error "Build failed - no output files found"
    exit 1
fi

print_success "Production build completed!"

# Analyze bundle sizes
print_status "Analyzing bundle sizes..."

# Get file sizes
WASM_FILE=$(find dist -name "*.wasm" | head -1)
JS_FILE=$(find dist -name "main-*.js" | head -1)
CSS_FILE=$(find dist -name "*.css" | head -1)

if [ -n "$WASM_FILE" ]; then
    WASM_SIZE=$(du -h "$WASM_FILE" | cut -f1)
    print_status "WASM Bundle: $WASM_SIZE"
    
    # Check WASM size thresholds
    WASM_SIZE_BYTES=$(stat -f%z "$WASM_FILE" 2>/dev/null || stat -c%s "$WASM_FILE" 2>/dev/null)
    if [ "$WASM_SIZE_BYTES" -gt 1048576 ]; then
        print_warning "WASM bundle is larger than 1MB - consider code splitting"
    elif [ "$WASM_SIZE_BYTES" -gt 524288 ]; then
        print_warning "WASM bundle is larger than 500KB - monitor for growth"
    else
        print_success "WASM bundle size is optimal"
    fi
else
    print_error "No WASM file found"
fi

if [ -n "$JS_FILE" ]; then
    JS_SIZE=$(du -h "$JS_FILE" | cut -f1)
    print_status "JavaScript Bundle: $JS_SIZE"
fi

if [ -n "$CSS_FILE" ]; then
    CSS_SIZE=$(du -h "$CSS_FILE" | cut -f1)
    print_status "CSS Bundle: $CSS_SIZE"
fi

# Analyze dependencies
print_status "Analyzing dependency tree..."
cd ../..
cargo tree --workspace --format "{p} {f}" > "$ANALYSIS_DIR/dependency-tree.txt"

# Count component dependencies
COMPONENT_COUNT=$(grep -c "shadcn-ui-leptos-" "$ANALYSIS_DIR/dependency-tree.txt" || echo "0")
print_status "Total Leptos components: $COMPONENT_COUNT"

# Analyze workspace packages
print_status "Analyzing workspace packages..."
cargo metadata --format-version 1 > "$ANALYSIS_DIR/workspace-metadata.json"

# Generate bundle report
print_status "Generating bundle report..."
cat > "$ANALYSIS_DIR/bundle-report.md" << EOF
# Bundle Analysis Report

Generated: $(date)

## Bundle Sizes
- WASM: ${WASM_SIZE:-"N/A"}
- JavaScript: ${JS_SIZE:-"N/A"}
- CSS: ${CSS_SIZE:-"N/A"}

## Component Count
- Total Leptos Components: $COMPONENT_COUNT

## Build Configuration
- Trunk: Production optimized
- Cargo: Release profile with LTO
- WASM: Optimized with strip symbols

## Optimization Recommendations
EOF

# Add optimization recommendations based on analysis
if [ "$WASM_SIZE_BYTES" -gt 1048576 ]; then
    echo "- Consider code splitting for large WASM bundles" >> "$ANALYSIS_DIR/bundle-report.md"
    echo "- Implement lazy loading for non-critical components" >> "$ANALYSIS_DIR/bundle-report.md"
fi

if [ "$COMPONENT_COUNT" -gt 20 ]; then
    echo "- Consider feature-based compilation to reduce bundle size" >> "$ANALYSIS_DIR/bundle-report.md"
    echo "- Implement tree-shaking for unused components" >> "$ANALYSIS_DIR/bundle-report.md"
fi

echo "- Enable gzip compression on web server" >> "$ANALYSIS_DIR/bundle-report.md"
echo "- Use CDN for static assets" >> "$ANALYSIS_DIR/bundle-report.md"
echo "- Implement service worker for caching" >> "$ANALYSIS_DIR/bundle-report.md"

print_success "Bundle analysis complete!"
print_status "Report saved to: $ANALYSIS_DIR/bundle-report.md"
print_status "Dependency tree saved to: $ANALYSIS_DIR/dependency-tree.txt"

# Show summary
echo ""
echo "=== 📊 BUNDLE ANALYSIS SUMMARY ==="
echo "WASM Size: ${WASM_SIZE:-"N/A"}"
echo "JS Size: ${JS_SIZE:-"N/A"}"
echo "CSS Size: ${CSS_SIZE:-"N/A"}"
echo "Components: $COMPONENT_COUNT"
echo "Report: $ANALYSIS_DIR/bundle-report.md"
echo ""

# Check for optimization opportunities
OPTIMIZATION_COUNT=0
if [ "$WASM_SIZE_BYTES" -gt 1048576 ]; then
    OPTIMIZATION_COUNT=$((OPTIMIZATION_COUNT + 1))
fi
if [ "$COMPONENT_COUNT" -gt 20 ]; then
    OPTIMIZATION_COUNT=$((OPTIMIZATION_COUNT + 1))
fi

if [ $OPTIMIZATION_COUNT -gt 0 ]; then
    print_warning "Found $OPTIMIZATION_COUNT optimization opportunities"
    print_status "Review the bundle report for detailed recommendations"
else
    print_success "Bundle is well-optimized for production!"
fi

echo ""
print_status "Next steps:"
echo "1. Review bundle report: $ANALYSIS_DIR/bundle-report.md"
echo "2. Implement recommended optimizations"
echo "3. Run bundle analysis again to measure improvements"
echo "4. Move to Phase 2: Error Boundaries & Resilience"
