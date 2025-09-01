# 🚀 Development Guide

This guide covers the development setup and workflow for the shadcn/ui Rust project.

## 🛠️ Development Environment

We provide multiple ways to set up your development environment:

### Option 1: Nix Flake (Recommended)

For reproducible builds and consistent environments:

```bash
# Enter the development shell
nix develop

# Or with direnv
echo "use flake" > .envrc
direnv allow
```

The Nix environment provides:
- Rust toolchain with WebAssembly support
- Node.js 20 + pnpm
- Make and build tools
- Chromium and Firefox for testing
- All required development dependencies

### Option 2: Manual Setup

If you prefer manual setup:

```bash
# Install Rust with WebAssembly target
rustup target add wasm32-unknown-unknown

# Install Node.js and pnpm
# (Follow platform-specific instructions)

# Install development tools
cargo install cargo-watch wasm-pack trunk
```

## 📋 Available Commands

We use Make for task automation. Run `make help` to see all available commands:

### Development
- `make dev` - Start development with file watching
- `make dev-examples` - Run example applications
- `make dev:leptos` - Start Leptos examples (via pnpm)
- `make dev:yew` - Start Yew examples (via pnpm)

### Building
- `make build` - Build all packages
- `make build-release` - Build optimized release versions  
- `make build-examples` - Build example applications

### Testing
- `make test` - Run all tests (Rust + E2E)
- `make test-rust` - Run Rust tests only
- `make test-e2e` - Run Playwright E2E tests
- `make test-single COMPONENT=button` - Test specific component

### Quality & Linting
- `make check` - Run cargo check
- `make lint` - Run clippy linting
- `make fmt` - Format code
- `make audit` - Security audit

### Playwright Setup
- `make setup-playwright` - Complete Playwright setup
- `make test:headed` - Run E2E tests in headed mode
- `make test:ui` - Run E2E tests with Playwright UI

## 🧪 Testing Strategy

We have a multi-layer testing approach:

### 1. Rust Unit Tests
```bash
make test-rust
```
- Component logic testing
- WASM compatibility tests
- Property validation

### 2. Browser Integration Tests
```bash
cargo test --target wasm32-unknown-unknown
```
- WASM-bindgen tests running in browser
- DOM interaction testing
- Accessibility validation

### 3. End-to-End Tests
```bash
make test-e2e
```
- Cross-browser testing (Chrome, Firefox, Safari)
- Mobile responsiveness
- Component interaction flows
- Performance metrics

## 🏗️ Project Structure

```
.
├── packages/
│   ├── leptos/          # Leptos components (44/51 complete)
│   ├── yew/             # Yew components (20/51 complete)
│   ├── test-utils/      # Shared testing utilities
│   └── ...
├── tests/e2e/           # Playwright E2E tests
├── book-examples/       # Demo applications
├── flake.nix           # Nix development environment
├── Makefile            # Task automation
└── playwright.config.ts # E2E test configuration
```

## 🔄 Development Workflow

### Starting Development
```bash
# Enter Nix shell (if using Nix)
nix develop

# Start development environment
make dev

# In another terminal, start examples
make dev-examples
```

### Adding a New Component
```bash
# Generate component scaffold
make generate-component NAME=my-component FRAMEWORK=leptos

# Run tests for the new component
make test-single COMPONENT=my-component
```

### Before Committing
```bash
# Format code
make fmt

# Run all checks
make check lint test-rust

# Run E2E tests
make test-e2e
```

## 🎯 Component Status

### Leptos Components (44/51 - 86% complete)
✅ Complete: accordion, alert, badge, button, card, checkbox, dialog, input, label, separator, skeleton, switch, table, textarea, and 30 more...

❌ Missing: 7 components remain

### Yew Components (20/51 - 39% complete)  
✅ Complete: alert, badge, button, card, input, label, separator, skeleton, switch, table, and 10 more...

❌ Missing: 31 components remain

See [COMPONENT_STATUS.md](./COMPONENT_STATUS.md) for detailed status.

## 🐛 Troubleshooting

### Common Issues

**Compilation Errors:**
```bash
# Clean and rebuild
make clean
make build
```

**Test Failures:**
```bash
# Run tests with verbose output
make test-rust-verbose
```

**E2E Test Issues:**
```bash
# Reinstall Playwright browsers
make setup-playwright
```

**Nix Issues:**
```bash
# Update flake
nix flake update
# Garbage collect old versions
nix store gc
```

### Performance Tips

- Use `cargo watch` for fast iteration
- Run specific tests instead of full suite during development
- Use `trunk serve` for faster WASM development
- Enable debug symbols only when needed

## 📚 Additional Resources

- [Architecture Documentation](./docs/architecture.md)
- [Testing Strategy](./docs/testing-strategy.md)
- [Component Generation](./docs/component-generator.md)
- [Feature Parity Design](./docs/feature-parity-design.md)

## 🤝 Contributing

1. Set up development environment (preferably with Nix)
2. Create feature branch
3. Make changes with tests
4. Run full test suite
5. Submit pull request

For component contributions, ensure:
- [ ] Both Leptos and Yew implementations
- [ ] Comprehensive tests (unit + integration + E2E)
- [ ] Documentation and examples
- [ ] Accessibility compliance
- [ ] Performance benchmarks
