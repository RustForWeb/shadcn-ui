#!/bin/bash

# Production Build Script for Leptos shadcn/ui
# Handles Tailwind CSS compilation and Leptos build

set -e

echo "=== 🏗️ PRODUCTION BUILD FOR LEPTOS SHADCN/UI ==="
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

print_status "Building Tailwind CSS..."

# Navigate to Leptos example directory
cd book-examples/leptos

# Debug: show current directory
print_status "Current directory: $(pwd)"
print_status "Checking for Tailwind CSS file: style/tailwind.css"
if [ -f "style/tailwind.css" ]; then
    print_success "Tailwind CSS file found"
else
    print_error "Tailwind CSS file not found at style/tailwind.css"
    ls -la style/
    exit 1
fi

# Check if we have the right tools
if command -v pnpm &> /dev/null; then
    print_status "Using pnpm for Tailwind CSS build..."
    
    # Install Tailwind CSS if not already installed
    if [ ! -f "node_modules/.bin/tailwindcss" ]; then
        print_status "Installing Tailwind CSS..."
        pnpm add -D tailwindcss
    fi
    
    # Build Tailwind CSS
    pnpm exec tailwindcss -i style/tailwind.css -o style/tailwind.output.css --minify
    print_success "Tailwind CSS built successfully!"
    
elif command -v npx &> /dev/null; then
    print_status "Using npx for Tailwind CSS build..."
    npx tailwindcss -i style/tailwind.css -o style/tailwind.output.css --minify
    print_success "Tailwind CSS built successfully!"
    
else
    print_warning "Neither pnpm nor npx found. Attempting to use system Tailwind CSS..."
    
    # Try to use system Tailwind CSS
    if command -v tailwindcss &> /dev/null; then
        tailwindcss -i style/tailwind.css -o style/tailwind.output.css --minify
        print_success "Tailwind CSS built successfully using system installation!"
    else
        print_error "No Tailwind CSS build tool found. Please install one of:"
        echo "  - pnpm add -D tailwindcss"
        echo "  - npm install -D tailwindcss"
        echo "  - cargo install tailwindcss-cli"
        exit 1
    fi
fi

# Verify Tailwind CSS was built
if [ ! -f "style/tailwind.output.css" ]; then
    print_error "Tailwind CSS build failed - output file not found"
    exit 1
fi

print_status "Building Leptos application with Trunk..."

# Clean previous builds
trunk clean

# Build with production optimizations
trunk build --release

# Check if build was successful
if [ ! -f "dist/index-*.js" ]; then
    print_error "Leptos build failed - no output files found"
    exit 1
fi

print_success "Production build completed successfully!"

# Show build output sizes
echo ""
echo "=== 📊 BUILD OUTPUT SIZES ==="

WASM_FILE=$(find dist -name "*.wasm" | head -1)
JS_FILE=$(find dist -name "index-*.js" | head -1)
CSS_FILE=$(find dist -name "*.css" | head -1)

if [ -n "$WASM_FILE" ]; then
    WASM_SIZE=$(du -h "$WASM_FILE" | cut -f1)
    echo "WASM Bundle: $WASM_SIZE"
fi

if [ -n "$JS_FILE" ]; then
    JS_SIZE=$(du -h "$JS_FILE" | cut -f1)
    echo "JavaScript Bundle: $JS_SIZE"
fi

if [ -n "$CSS_FILE" ]; then
    CSS_SIZE=$(du -h "$CSS_FILE" | cut -f1)
    echo "CSS Bundle: $CSS_SIZE"
fi

echo ""
print_success "Production build is ready!"
print_status "Output directory: dist/"
print_status "You can now deploy the contents of the dist/ directory"

# Return to project root
cd ../..

echo ""
echo "=== 🎯 NEXT STEPS ==="
echo "1. Test the production build: cd book-examples/leptos && trunk serve"
echo "2. Analyze bundle sizes: make analyze-bundle"
echo "3. Run E2E tests: make test-e2e"
echo "4. Move to Phase 2: Error Boundaries & Resilience"
