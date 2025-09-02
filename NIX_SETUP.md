# Nix Development Environment Setup

This project uses Nix to provide a reproducible development environment with Rust, Node.js, pnpm, and Playwright.

## Quick Start

### 1. Enter the Nix Development Environment

```bash
# Using direnv (recommended - automatically loads when you cd into the project)
cd /path/to/shadcn-ui
# Environment automatically loads

# Or manually enter the environment
nix develop
```

### 2. Available Commands

Once in the Nix environment, you'll have access to:

- **Rust tools**: `cargo`, `rustc`, `wasm-pack`, `trunk`
- **Node.js tools**: `node`, `pnpm`, `typescript`
- **Build tools**: `make`, `pkg-config`
- **Development utilities**: `git`, `jq`, `curl`

### 3. Running Tests

The Playwright tests are configured to automatically close after completion:

```bash
# Run all tests (auto-closing)
pnpm test

# Run specific test suites
pnpm test:dynamic-loading
pnpm test:bundle-optimization

# Run tests with browser visible
pnpm test:headed

# Run tests with Playwright UI
pnpm test:ui

# Run tests in debug mode
pnpm test:debug
```

### 4. Development Commands

```bash
# Start Leptos development server
pnpm dev:leptos

# Start Yew development server
pnpm dev:yew

# Build examples
pnpm build:examples

# Clean build artifacts
pnpm clean
```

## Environment Features

### Automatic Setup
- **Playwright browsers**: Automatically downloaded on first run
- **Dependencies**: Node.js dependencies installed with pnpm
- **Environment variables**: Properly configured for Rust and Playwright

### Auto-Closing Tests
- Tests automatically close after completion
- No more hanging HTML report servers
- Clean process termination
- Automatic cleanup of browser processes

### Cross-Platform Support
- Works on macOS (ARM64/x64), Linux, and Windows
- Appropriate browser dependencies for each platform
- Consistent development experience

## Troubleshooting

### Nix Issues
```bash
# If you get experimental features errors
export NIXPKGS_ALLOW_UNSUPPORTED_SYSTEM=1

# Clear Nix cache if needed
nix store gc
```

### Playwright Issues
```bash
# Reinstall Playwright browsers
pnpm exec playwright install --with-deps

# Clear Playwright cache
rm -rf .playwright-browsers
```

### Environment Issues
```bash
# Reload direnv
direnv reload

# Or manually reload
source .envrc
```

## Manual Installation (Alternative)

If you prefer not to use Nix, you can install dependencies manually:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js (v20+)
# Download from https://nodejs.org/

# Install pnpm
npm install -g pnpm

# Install project dependencies
pnpm install

# Install Playwright browsers
pnpm exec playwright install --with-deps
```

## Benefits of Nix Setup

1. **Reproducible**: Same environment for all developers
2. **Isolated**: Doesn't interfere with system packages
3. **Fast**: Efficient caching and dependency resolution
4. **Complete**: All tools and dependencies in one environment
5. **Auto-closing**: Tests don't hang after completion
6. **Cross-platform**: Works consistently across different systems
