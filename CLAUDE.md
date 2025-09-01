# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is **Rust shadcn/ui** - a Rust port of the popular [shadcn/ui](https://ui.shadcn.com/) component library. It provides beautifully designed UI components for Rust web frameworks including Leptos and Yew.

## Development Commands

### Building and Testing
```bash
# Build the entire workspace
cargo build

# Check for compilation errors without building binaries
cargo check

# Run all tests in workspace
cargo test

# Run tests for specific package
cargo test -p shadcn-ui-leptos-button
cargo test -p shadcn-ui-yew-button

# Run tests with verbose output
cargo test -- --nocapture
```

### Working with Specific Components
```bash
# Build a specific component package
cargo build -p shadcn-ui-leptos-button
cargo build -p shadcn-ui-yew-button

# Check a specific component
cargo check -p shadcn-ui-leptos-alert
cargo check -p shadcn-ui-yew-alert
```

### CLI Tool Development
```bash
# Build the shadcn CLI tool
cargo build -p shadcn

# Run the CLI tool
cargo run -p shadcn -- --help

# Generate a new component scaffold
cargo run -p shadcn -- generate --name "my-component" --framework "leptos" --classes "custom-classes"

# Initialize a new project
cargo run -p shadcn -- init

# Add components to project (planned)
cargo run -p shadcn -- add button
```

## Architecture & Structure

### Workspace Organization
This is a Cargo workspace with the following key structure:

- **`packages/shadcn/`** - The main CLI tool (Rust port of shadcn CLI)
- **`packages/registry/`** - Component registry system 
- **`packages/leptos/`** - Leptos framework components (button, card, alert, etc.)
- **`packages/yew/`** - Yew framework components (button, card, alert, etc.)
- **`packages/component-generator/`** - Component generation utilities
- **`packages/test-utils/`** - Shared testing utilities
- **`book-examples/`** - Documentation examples for Leptos and Yew
- **`scripts/`** - Build and maintenance scripts

### Component Architecture
- Each UI component has separate implementations for **Leptos** and **Yew** frameworks
- Components follow naming convention: `shadcn-ui-{framework}-{component}`
- All components use **Tailwind CSS** with `tailwind_fuse` for dynamic styling
- Components leverage framework-specific utilities:
  - **Leptos**: `leptos-struct-component`, `leptos-style`, `leptos-node-ref`
  - **Yew**: `yew-struct-component`, `yew-style`

### Key Dependencies
- **Leptos 0.8.0** - Primary reactive framework
- **Yew 0.21.0** - Alternative reactive framework  
- **tailwind_fuse 0.3.0** - Dynamic CSS class composition
- **serde** - Serialization for component registry
- **clap** - CLI argument parsing for shadcn tool

### Framework Patterns
- Components are designed as **struct-based components** using framework-specific macros
- **Tailwind CSS** integration for styling with variant support
- **Reactive state management** using framework-native patterns
- **Icon integration** via Lucide icons for both frameworks

## Working with Components

### Adding New Components
1. Create directories in both `packages/leptos/` and `packages/yew/`
2. Add `Cargo.toml` with appropriate framework dependencies
3. Implement component following existing patterns in `src/lib.rs`
4. Add component to workspace members in root `Cargo.toml`
5. Update registry in `packages/registry/` if needed

### Framework-Specific Notes
- **Leptos components** use reactive signals and the component macro system
- **Yew components** use the functional component approach with hooks
- Both frameworks share similar prop structures but differ in implementation details
- Test files use `wasm-bindgen-test` for WASM compatibility

### Styling Guidelines
- Use `tailwind_fuse` for dynamic class composition
- Follow existing variant patterns for component customization
- Maintain consistency between Leptos and Yew implementations
- Leverage Tailwind's utility-first approach

## Registry System
The `packages/registry/` contains the component registry that tracks available components, their variants, and metadata. This system enables the CLI tool to manage component installation and updates.

## Component Generator
The `packages/component-generator/` provides automated scaffolding for new components:

### Features
- **Framework Support**: Generates components for Leptos, Yew, and Dioxus
- **Theme Variants**: Creates both `default` and `new_york` theme files
- **Template Engine**: Uses Handlebars templates for consistent code generation
- **CLI Integration**: Accessible via `cargo run -p shadcn -- generate`

### Usage Examples
```bash
# Generate a basic component
cargo run -p shadcn -- generate --name "dialog" --framework "leptos"

# Generate with custom styling
cargo run -p shadcn -- generate --name "tooltip" --classes "rounded-md bg-primary px-3 py-1" --description "A tooltip component"

# Generate for Yew framework
cargo run -p shadcn -- generate --name "select" --framework "yew" --tag "select"
```

### Template Customization
Templates are located in `packages/component-generator/src/templates/` and can be customized:
- `leptos_component.hbs` - Leptos component template
- `yew_component.hbs` - Yew component template
- `lib_rs.hbs` - Component library file template