<p align="center">
    <a href="./logo.svg">
        <img src="./logo.svg" width="300" height="200" alt="Rust shadcn/ui Logo">
    </a>
</p>

<h1 align="center">Rust shadcn/ui</h1>

<p align="center">
    <strong>Beautifully designed components that you can copy and paste into your Rust web apps.</strong>
</p>

<p align="center">
    <a href="#quick-start">Quick Start</a> •
    <a href="#frameworks">Frameworks</a> •
    <a href="#components">Components</a> •
    <a href="#cli-tool">CLI Tool</a> •
    <a href="#contributing">Contributing</a>
</p>

---

**Rust shadcn/ui** is a comprehensive port of [shadcn/ui](https://ui.shadcn.com/) for Rust web frameworks. This project provides accessible, customizable, and beautifully designed UI components that integrate seamlessly with Tailwind CSS.

## ✨ Features

- 🎨 **Beautiful Components**: Faithfully ported from shadcn/ui with all styling and variants
- 🦀 **Rust Native**: Built specifically for Rust web frameworks with type safety
- 🔧 **CLI Tool**: Powerful command-line interface for component generation and management
- 🎭 **Theme Variants**: Support for default and New York design themes
- ⚡ **Performance**: Optimized components with minimal runtime overhead
- 🧪 **Type Safety**: Full TypeScript-equivalent type safety with Rust's type system
- 📱 **Responsive**: Mobile-first responsive design out of the box

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ with `wasm32-unknown-unknown` target
- Node.js 18+ (for Tailwind CSS)

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/RustForWeb/shadcn-ui.git
   cd shadcn-ui
   ```

2. **Build the workspace:**
   ```bash
   cargo build
   ```

3. **Test components:**
   ```bash
   cargo test -p shadcn-ui-leptos-button
   cargo test -p shadcn-ui-yew-button
   ```

## 🔧 CLI Tool

The included CLI tool provides powerful features for component management:

### Generate Components
```bash
# Generate a new component for Leptos
cargo run -p shadcn -- generate --name "dialog" --framework "leptos"

# Generate with custom styling
cargo run -p shadcn -- generate \
  --name "tooltip" \
  --framework "yew" \
  --classes "rounded-md bg-primary px-3 py-1 text-primary-foreground" \
  --description "A tooltip component"
```

### Available Commands
- `generate` - Generate new component scaffolds
- `init` - Initialize a new project with shadcn/ui
- `add` - Add components to your project *(planned)*
- `diff` - Check for component updates *(planned)*

### CLI Options
```bash
USAGE:
    rust-shadcn generate [OPTIONS] --name <NAME>

OPTIONS:
    -n, --name <NAME>                Name of the component to generate
    -f, --framework <FRAMEWORK>      Target framework [default: leptos]
    -c, --classes <CLASSES>          Base CSS classes for the component
    -t, --tag <TAG>                  HTML tag to use [default: div]
    -d, --description <DESCRIPTION>  Component description
        --themes                     Generate both themes [default: true]
```

## 🏗️ Frameworks

### Currently Supported

#### [Leptos](https://leptos.dev/) ![Leptos](https://img.shields.io/badge/status-near--complete-brightgreen)
- **Components**: 44/51 (86% coverage) - Near complete implementation!
- **Features**: Signal-based reactivity, server-side rendering, hydration
- **Status**: Production ready with comprehensive component library
- **Missing**: avatar, data-table, chart, resizable, sidebar, sonner, typography

#### [Yew](https://yew.rs/) ![Yew](https://img.shields.io/badge/status-stable-green)
- **Components**: 20/51 (39% coverage)
- **Features**: Component-based architecture, virtual DOM, WebAssembly  
- **Status**: Solid foundation, 25 components missing from Leptos parity
- **Available**: alert, aspect-ratio, avatar, badge, breadcrumb, button, card, checkbox, dialog, input, label, pagination, radio-group, select, separator, skeleton, switch, table, textarea, tooltip

### Under Development

#### [Dioxus](https://dioxuslabs.com/) ![Dioxus](https://img.shields.io/badge/status-planned-blue)
- **Status**: Framework support in planning phase
- **Timeline**: Component generator templates ready

## 📦 Components

### Available Components

| Component | Leptos | Yew | Description |
|-----------|:------:|:---:|-------------|
| Alert | ✅ | ✅ | Displays a callout for user attention |
| Aspect Ratio | ✅ | ✅ | Displays content within a desired ratio |
| Avatar | ❌ | ✅ | Represents a user or entity |
| Badge | ✅ | ✅ | Displays a badge or a component |
| Breadcrumb | ❌ | ✅ | Navigation aid showing page hierarchy |
| Button | ✅ | ✅ | Triggers an action or event |
| Card | ✅ | ✅ | Container for related information |
| Checkbox | ✅ | ✅ | Binary choice input |
| Input | ❌ | ✅ | Text input field |
| Label | ❌ | ✅ | Caption for form controls |
| Pagination | ❌ | ✅ | Navigate through pages |
| Radio Group | ✅ | ✅ | Single choice from options |
| Separator | ❌ | ✅ | Visual or semantic separator |
| Skeleton | ❌ | ✅ | Loading placeholder |
| Switch | ❌ | ✅ | Binary toggle control |
| Table | ❌ | ✅ | Structured data display |
| Textarea | ❌ | ✅ | Multi-line text input |

### Component Status Legend
- ✅ **Available**: Ready for production use
- ❌ **Planned**: In development roadmap
- 🚧 **In Progress**: Currently being developed

## 🏗️ Architecture

### Workspace Structure
```
shadcn-ui/
├── packages/
│   ├── shadcn/              # CLI tool
│   ├── registry/            # Component registry
│   ├── component-generator/ # Code generation
│   ├── test-utils/          # Testing utilities
│   ├── leptos/             # Leptos components
│   │   ├── button/         # Individual components
│   │   ├── card/
│   │   └── ...
│   └── yew/                # Yew components
│       ├── button/
│       ├── card/
│       └── ...
├── book-examples/          # Documentation examples
└── docs/                   # Additional documentation
```

### Design Principles

1. **Component Isolation**: Each component is a separate crate for modularity
2. **Framework Parity**: Consistent API across all supported frameworks  
3. **Theme Support**: Default and New York variants for all components
4. **Type Safety**: Leverage Rust's type system for component props
5. **Performance**: Minimal runtime overhead and optimal bundle sizes

## 🧪 Development

### Building Components
```bash
# Build all components
cargo build

# Build specific framework components
cargo build -p shadcn-ui-leptos-button
cargo build -p shadcn-ui-yew-button

# Run component tests
cargo test -p shadcn-ui-leptos-button
```

### Adding New Components

1. **Use the generator:**
   ```bash
   cargo run -p shadcn -- generate --name "new-component" --framework "leptos"
   ```

2. **Implement the component** following existing patterns

3. **Add tests** in the component directory

4. **Update documentation** and examples

See [CLAUDE.md](./CLAUDE.md) for detailed development guidelines.

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](./CONTRIBUTING.md) for details.

### Development Setup
1. Fork and clone the repository
2. Install Rust and required targets: `rustup target add wasm32-unknown-unknown`  
3. Build the project: `cargo build`
4. Run tests: `cargo test`
5. Make your changes and submit a PR

### Component Contributions
- Use the CLI generator for consistent scaffolding
- Follow established patterns from existing components
- Include tests and documentation
- Support both default and New York themes

## 📚 Documentation

- **[Book](https://shadcn-ui.rustforweb.org)**: Complete documentation and guides
- **[CLAUDE.md](./CLAUDE.md)**: Developer quick reference
- **[API Docs](./docs/api/)**: Generated API documentation
- **[Examples](./book-examples/)**: Framework-specific examples

## Credits

The logo is a combination of the [shadcn/ui logo](https://github.com/shadcn-ui/ui/blob/main/apps/www/components/icons.tsx) and [Ferris the Rustacean](https://rustacean.net/).

## License

This project is available under the [MIT license](LICENSE.md).

## Rust for Web

The Rust shadcn/ui project is part of [Rust for Web](https://github.com/RustForWeb).

[Rust for Web](https://github.com/RustForWeb) creates and ports web libraries for Rust. All projects are free and open source.
