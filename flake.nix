{
  description = "shadcn/ui Rust Development Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        # Rust toolchain with WebAssembly support
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        # Node.js and pnpm for Playwright and frontend tooling
        nodejs = pkgs.nodejs_20;
        pnpm = pkgs.nodePackages.pnpm;
        
        # Additional Node.js tools
        nodePackages = pkgs.nodePackages;

      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust ecosystem
            rustToolchain
            cargo-watch
            cargo-edit
            cargo-audit
            cargo-outdated
            
            # WebAssembly tools
            wasm-pack
            trunk
            
            # Node.js ecosystem for Playwright
            nodejs
            pnpm
            nodePackages.typescript
            
            # Build tools
            gnumake
            pkg-config
            
            # System dependencies for web development
            openssl
            cacert
            
            # Development utilities
            jq
            curl
            git
            
            # Browser automation (for Playwright) - will be installed by Playwright
          ];

          shellHook = ''
            echo "🦀 shadcn/ui Rust Development Environment"
            echo "======================================"
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo "Node.js version: $(node --version)"
            echo "pnpm version: $(pnpm --version)"
            echo "TypeScript version: $(tsc --version 2>/dev/null || echo 'Not installed')"
            echo "Make version: $(make --version | head -n1)"
            echo ""
            echo "Available commands:"
            echo "  pnpm test          - Run Playwright tests (auto-closing)"
            echo "  pnpm test:headed   - Run tests with browser visible"
            echo "  pnpm test:ui       - Run tests with Playwright UI"
            echo "  make help          - Show available make targets"
            echo "  make dev           - Start development environment"
            echo "  make test          - Run all tests"
            echo "  make test-rust     - Run Rust tests only"
            echo "  make test-e2e      - Run Playwright E2E tests"
            echo "  make build         - Build all components"
            echo "  make clean         - Clean build artifacts"
            echo ""
            
            # Set up environment variables
            export RUST_LOG=debug
            export PLAYWRIGHT_BROWSERS_PATH=$PWD/.playwright-browsers
            export PATH="$PWD/node_modules/.bin:$PATH"
            
            # Ensure pnpm is properly configured
            export PNPM_HOME="$HOME/.local/share/pnpm"
            export PATH="$PNPM_HOME:$PATH"
            
            # Create necessary directories
            mkdir -p .playwright-browsers
            mkdir -p tests/e2e
            mkdir -p test-results
            
            # Check if dependencies are installed
            if [ ! -d "node_modules" ] && [ -f "package.json" ]; then
              echo "📦 Installing Node.js dependencies with pnpm..."
              pnpm install
            fi
            
            # Install Playwright browsers if not already installed
            if [ ! -d ".playwright-browsers" ] || [ -z "$(ls -A .playwright-browsers 2>/dev/null)" ]; then
              echo "🎭 Installing Playwright browsers..."
              pnpm exec playwright install --with-deps
            fi
            
            echo "🚀 Environment ready! Run 'pnpm test' to start testing."
          '';

          # Environment variables
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD = "1";
        };

        # Formatter for nix files
        formatter = pkgs.nixpkgs-fmt;
      });
}
