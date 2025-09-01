# Bundle Analysis Report

Generated: Mon Sep  1 19:34:51 AEST 2025

## Bundle Sizes
- WASM: 3.4M
- JavaScript: 4.0K
- CSS: 4.0K

## Component Count
- Total Leptos Components: 62

## Build Configuration
- Trunk: Production optimized
- Cargo: Release profile with LTO
- WASM: Optimized with strip symbols

## Optimization Recommendations
- Consider code splitting for large WASM bundles
- Implement lazy loading for non-critical components
- Consider feature-based compilation to reduce bundle size
- Implement tree-shaking for unused components
- Enable gzip compression on web server
- Use CDN for static assets
- Implement service worker for caching
