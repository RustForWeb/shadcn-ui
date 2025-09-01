.PHONY: help dev build test test-rust test-e2e clean install-deps setup-playwright lint fmt check docs

# Default target
help: ## Show this help message
	@echo "shadcn/ui Rust Development Commands"
	@echo "=================================="
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

# Development
dev: ## Start development environment with file watching
	@echo "🚀 Starting development environment..."
	cargo watch -x "check --workspace" -x "test --workspace --lib"

dev-examples: ## Run example applications
	@echo "🌟 Starting Leptos examples..."
	cd book-examples/leptos && trunk serve --open

# Building
build: ## Build all packages and examples
	@echo "🔨 Building all packages..."
	cargo build --workspace
	cargo build --workspace --target wasm32-unknown-unknown

build-release: ## Build optimized release versions
	@echo "🏗️ Building release versions..."
	cargo build --workspace --release
	cargo build --workspace --release --target wasm32-unknown-unknown

build-examples: ## Build example applications
	@echo "📦 Building examples..."
	cd book-examples/leptos && trunk build --release
	cd book-examples/yew && trunk build --release

# Testing
test: test-rust test-e2e ## Run all tests (Rust + E2E)

test-rust: ## Run Rust unit and integration tests
	@echo "🧪 Running Rust tests..."
	cargo test --workspace --lib
	cargo test --workspace --target wasm32-unknown-unknown

test-rust-verbose: ## Run Rust tests with verbose output
	@echo "🔍 Running Rust tests (verbose)..."
	RUST_LOG=debug cargo test --workspace --lib -- --nocapture

test-single: ## Run tests for a specific component (usage: make test-single COMPONENT=button)
	@if [ -z "$(COMPONENT)" ]; then \
		echo "❌ Please specify COMPONENT. Usage: make test-single COMPONENT=button"; \
		exit 1; \
	fi
	@echo "🎯 Testing $(COMPONENT) component..."
	cargo test -p shadcn-ui-leptos-$(COMPONENT)
	cargo test -p shadcn-ui-yew-$(COMPONENT)

test-e2e: install-playwright ## Run Playwright E2E tests
	@echo "🎭 Running Playwright E2E tests..."
	pnpm playwright test

test-e2e-headed: ## Run Playwright tests in headed mode
	@echo "🎭 Running Playwright E2E tests (headed)..."
	pnpm playwright test --headed

test-e2e-ui: ## Run Playwright E2E tests with UI
	@echo "🎭 Running Playwright E2E tests with UI..."
	pnpm playwright test --ui

test-e2e-debug: ## Run Playwright E2E tests in debug mode
	@echo "🐛 Running Playwright E2E tests in debug mode..."
	pnpm playwright test --debug

test-e2e-specific: ## Run specific E2E test file (usage: make test-e2e-specific FILE=filename)
	@echo "🎯 Running specific E2E test: $(FILE)..."
	pnpm playwright test $(FILE)

test-e2e-browser: ## Run E2E tests in specific browser (usage: make test-e2e-browser BROWSER=chromium)
	@echo "🌐 Running E2E tests in $(BROWSER)..."
	pnpm playwright test --project=$(BROWSER)

test-e2e-parallel: ## Run E2E tests in parallel
	@echo "⚡ Running E2E tests in parallel..."
	pnpm playwright test --workers=4

test-e2e-report: ## Generate E2E test report
	@echo "📊 Generating E2E test report..."
	pnpm playwright show-report

test-e2e-install: ## Install Playwright browsers
	@echo "📦 Installing Playwright browsers..."
	pnpm playwright install

test-e2e-codegen: ## Generate E2E test code
	@echo "🔄 Generating E2E test code..."
	pnpm playwright codegen http://127.0.0.1:8080

# Production Readiness
analyze-bundle: ## Analyze bundle sizes and optimization opportunities
	@echo "📦 Analyzing bundle sizes for production readiness..."
	./scripts/analyze_bundle.sh

build-production: ## Build production-optimized version
	@echo "🏗️ Building production-optimized version..."
	./scripts/build_production.sh

production-check: analyze-bundle build-production ## Complete production readiness check
	@echo "✅ Production readiness check complete!"

# Quality & Linting
check: ## Run cargo check on all packages
	@echo "✅ Checking all packages..."
	cargo check --workspace
	cargo check --workspace --target wasm32-unknown-unknown

lint: ## Run clippy linting
	@echo "📎 Running clippy..."
	cargo clippy --workspace -- -D warnings
	cargo clippy --workspace --target wasm32-unknown-unknown -- -D warnings

fmt: ## Format code with rustfmt
	@echo "🎨 Formatting code..."
	cargo fmt --all

fmt-check: ## Check if code is formatted
	@echo "🔍 Checking code formatting..."
	cargo fmt --all -- --check

audit: ## Run security audit
	@echo "🔒 Running security audit..."
	cargo audit

# Dependencies
install-deps: ## Install all dependencies
	@echo "📦 Installing dependencies..."
	pnpm install

install-playwright: ## Install Playwright and browsers
	@echo "🎭 Installing Playwright..."
	pnpm create playwright@latest --yes
	pnpm playwright install

setup-playwright: install-deps install-playwright ## Complete Playwright setup
	@echo "✅ Playwright setup complete!"

# Documentation
docs: ## Generate and open documentation
	@echo "📚 Generating documentation..."
	cargo doc --workspace --open

docs-book: ## Build and serve the documentation book
	@echo "📖 Building documentation book..."
	cd book && mdbook serve --open

# Maintenance
clean: ## Clean build artifacts
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	rm -rf book-examples/*/dist
	rm -rf node_modules
	rm -rf .playwright-browsers

clean-cache: ## Clean cargo cache and lock files
	@echo "🗑️ Cleaning caches..."
	cargo clean
	rm -f Cargo.lock
	rm -f package-lock.json

update-deps: ## Update all dependencies
	@echo "⬆️ Updating dependencies..."
	cargo update
	pnpm update

# Component Generation
generate-component: ## Generate a new component (usage: make generate-component NAME=my-component FRAMEWORK=leptos)
	@if [ -z "$(NAME)" ] || [ -z "$(FRAMEWORK)" ]; then \
		echo "❌ Please specify NAME and FRAMEWORK. Usage: make generate-component NAME=my-component FRAMEWORK=leptos"; \
		exit 1; \
	fi
	@echo "🏗️ Generating $(NAME) component for $(FRAMEWORK)..."
	cargo run --bin component-generator -- --name $(NAME) --framework $(FRAMEWORK)

# Nix integration
nix-develop: ## Enter Nix development shell
	@echo "❄️ Entering Nix development shell..."
	nix develop

nix-build: ## Build using Nix
	@echo "❄️ Building with Nix..."
	nix build

# Quick fixes
fix-permissions: ## Fix file permissions
	@echo "🔧 Fixing file permissions..."
	find . -name "*.rs" -type f -exec chmod 644 {} \;
	find . -name "*.toml" -type f -exec chmod 644 {} \;
	find scripts -name "*.sh" -type f -exec chmod +x {} \;

# Git hooks
install-git-hooks: ## Install git pre-commit hooks
	@echo "🪝 Installing git hooks..."
	echo '#!/bin/sh\nmake fmt-check && make check && make test-rust' > .git/hooks/pre-commit
	chmod +x .git/hooks/pre-commit
	echo "✅ Git hooks installed!"

# Environment info
env-info: ## Show environment information
	@echo "Environment Information:"
	@echo "======================="
	@echo "Rust version: $$(rustc --version 2>/dev/null || echo 'Not installed')"
	@echo "Cargo version: $$(cargo --version 2>/dev/null || echo 'Not installed')"
	@echo "Node.js version: $$(node --version 2>/dev/null || echo 'Not installed')"
	@echo "pnpm version: $$(pnpm --version 2>/dev/null || echo 'Not installed')"
	@echo "Make version: $$(make --version 2>/dev/null | head -n1 || echo 'Not installed')"
	@echo "Git version: $$(git --version 2>/dev/null || echo 'Not installed')"
	@if command -v nix >/dev/null 2>&1; then \
		echo "Nix version: $$(nix --version 2>/dev/null)"; \
	else \
		echo "Nix version: Not installed"; \
	fi
