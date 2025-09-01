# Component Generator Documentation

The Rust shadcn/ui component generator is a powerful tool for creating consistent, framework-specific UI components with full theme variant support.

## Overview

The component generator creates scaffolded components that follow established patterns and conventions across different Rust web frameworks. It handles the boilerplate code, file structure, and framework-specific implementations automatically.

## Architecture

### Core Components

```
packages/component-generator/
├── src/
│   ├── lib.rs              # Core types and generator
│   ├── generator.rs        # Generation logic
│   └── templates/          # Handlebars templates
│       ├── leptos_component.hbs
│       ├── yew_component.hbs
│       └── lib_rs.hbs
```

### Framework Support

| Framework | Status | Template | Features |
|-----------|---------|----------|----------|
| **Leptos** | ✅ Stable | `leptos_component.hbs` | Signal-based reactivity, SSR |
| **Yew** | ✅ Stable | `yew_component.hbs` | Component architecture, Virtual DOM |
| **Dioxus** | 🚧 Planned | `dioxus_component.hbs` | Cross-platform support |

## Configuration System

### ComponentConfig

```rust
pub struct ComponentConfig {
    pub name: String,                      // Component name (kebab-case)
    pub framework: Framework,              // Target framework
    pub theme_variants: Vec<String>,       // ["default", "new_york"]
    pub props: HashMap<String, PropConfig>, // Component properties
    pub dependencies: Vec<String>,         // External dependencies
}
```

### PropConfig

```rust
pub struct PropConfig {
    pub prop_type: String,           // Rust type (String, bool, Option<T>)
    pub optional: bool,              // Whether prop is optional
    pub default_value: Option<String>, // Default value expression
    pub description: Option<String>,  // Documentation
}
```

## Template System

### Handlebars Templates

The generator uses Handlebars templating engine for flexible, maintainable code generation:

```handlebars
{{!-- leptos_component.hbs example --}}
use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn {{name}}(
    {{#each props}}
    {{@key}}: {{prop_type}},
    {{/each}}
) -> impl IntoView {
    view! {
        <div class=move || {{name}}Class::default()>
            // Component implementation
        </div>
    }
}
```

### Theme Variants

Each component generates both theme variants:

- **Default Theme**: Clean, minimal design
- **New York Theme**: Sophisticated, enterprise styling

Templates automatically handle theme-specific styling through CSS class generation.

## CLI Integration

### Command Structure

```bash
cargo run -p shadcn -- generate [OPTIONS] --name <NAME>
```

### Available Options

| Flag | Type | Description | Default |
|------|------|-------------|---------|
| `-n, --name` | String | Component name (required) | - |
| `-f, --framework` | String | Target framework | `leptos` |
| `-c, --classes` | String | Base CSS classes | Auto-generated |
| `-t, --tag` | String | Root HTML tag | `div` |
| `-d, --description` | String | Component description | - |
| `--themes` | Bool | Generate both themes | `true` |

### Usage Examples

**Basic Component Generation:**
```bash
cargo run -p shadcn -- generate --name "tooltip" --framework "leptos"
```

**Component with Custom Styling:**
```bash
cargo run -p shadcn -- generate \
  --name "dialog" \
  --framework "yew" \
  --classes "rounded-md bg-background p-6 shadow-lg" \
  --description "A modal dialog component"
```

**Multi-Framework Generation:**
```bash
# Generate for Leptos
cargo run -p shadcn -- generate --name "slider" --framework "leptos"

# Generate for Yew
cargo run -p shadcn -- generate --name "slider" --framework "yew"
```

## File Generation Patterns

### Directory Structure

```
packages/{framework}/{component-name}/
├── Cargo.toml              # Package configuration
├── src/
│   ├── lib.rs             # Public exports
│   ├── default.rs         # Default theme variant
│   └── new_york.rs        # New York theme variant
└── README.md              # Component documentation
```

### Generated Files

**Package Configuration (Cargo.toml):**
```toml
[package]
name = "shadcn-ui-{framework}-{component}"
version = "0.1.0"
edition = "2021"

[dependencies]
tailwind_fuse = "0.3"
{framework-specific-deps}
```

**Library Entry Point (lib.rs):**
```rust
//! {Component} component for {Framework}

mod default;
mod new_york;

pub use default::*;
pub use new_york as new_york;
```

**Theme Variants:**
- `default.rs` - Clean, minimal styling
- `new_york.rs` - Sophisticated, enterprise styling

## Framework-Specific Features

### Leptos Components

**Key Features:**
- Signal-based reactivity with `create_signal()`
- Server-side rendering compatibility
- `view!` macro for declarative UI
- MaybeProp for optional properties

**Example Generated Structure:**
```rust
#[component]
pub fn Button(
    #[prop(optional)] variant: MaybeProp<ButtonVariant>,
    #[prop(optional)] size: MaybeProp<ButtonSize>,
    children: Children,
) -> impl IntoView {
    let classes = create_memo(move |_| {
        ButtonClass {
            variant: variant.get(),
            size: size.get(),
        }
    });
    
    view! {
        <button class=classes>
            {children()}
        </button>
    }
}
```

### Yew Components

**Key Features:**
- Properties with `#[derive(Properties, PartialEq)]`
- Component trait implementation
- Html return type for render
- Callback handling with `Callback<T>`

**Example Generated Structure:**
```rust
#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub size: ButtonSize,
    pub children: Children,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let classes = use_memo(
        |props| ButtonClass {
            variant: props.variant,
            size: props.size,
        },
        props.clone(),
    );

    html! {
        <button class={classes}>
            {props.children.clone()}
        </button>
    }
}
```

## Styling System

### TailwindCSS Integration

All generated components use `tailwind_fuse` for dynamic class composition:

```rust
#[derive(TwClass)]
#[tw(
    class = "inline-flex items-center justify-center rounded-md text-sm font-medium",
    variants(
        variant(
            primary = "bg-primary text-primary-foreground hover:bg-primary/90",
            secondary = "bg-secondary text-secondary-foreground hover:bg-secondary/80",
        ),
        size(
            default = "h-10 px-4 py-2",
            sm = "h-9 rounded-md px-3",
            lg = "h-11 rounded-md px-8",
        )
    )
)]
pub struct ButtonClass {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
}
```

### Theme System

**Design Tokens:**
- CSS custom properties for theming
- Consistent color palette across variants
- Responsive design utilities
- Accessibility-first approach

**Color Palette:**
```css
:root {
  --background: 0 0% 100%;
  --foreground: 222.2 84% 4.9%;
  --primary: 222.2 47.4% 11.2%;
  --primary-foreground: 210 40% 98%;
  --secondary: 210 40% 96%;
  --secondary-foreground: 222.2 47.4% 11.2%;
  /* ... */
}
```

## Extension Points

### Custom Templates

Add custom templates by registering them in the generator:

```rust
impl ComponentGenerator {
    pub fn new() -> Result<Self> {
        let mut handlebars = handlebars::Handlebars::new();
        
        // Register custom template
        handlebars.register_template_string(
            "custom_component",
            include_str!("templates/custom_component.hbs")
        )?;
        
        Ok(Self { template_engine: handlebars })
    }
}
```

### Framework Integration

Adding support for new frameworks:

1. **Add Framework Enum Value:**
```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Framework {
    Leptos,
    Yew,
    Dioxus,
    NewFramework, // Add here
}
```

2. **Create Template:**
```handlebars
{{!-- templates/newframework_component.hbs --}}
// Framework-specific component structure
```

3. **Register Template:**
```rust
handlebars.register_template_string(
    "newframework_component",
    include_str!("templates/newframework_component.hbs")
)?;
```

4. **Update Generation Logic:**
```rust
pub fn generate_component(&self, config: &ComponentConfig) -> Result<String> {
    let template_name = match config.framework {
        Framework::Leptos => "leptos_component",
        Framework::Yew => "yew_component",
        Framework::Dioxus => "dioxus_component",
        Framework::NewFramework => "newframework_component", // Add here
    };
    
    self.template_engine.render(template_name, config)
        .map_err(Into::into)
}
```

## Best Practices

### Component Design

1. **Consistent API:** Follow established prop patterns across frameworks
2. **Accessibility:** Include ARIA attributes and semantic HTML
3. **Performance:** Use memoization for expensive computations
4. **Styling:** Leverage theme system for consistent design

### Code Generation

1. **Template Clarity:** Keep templates readable and maintainable
2. **Type Safety:** Generate proper Rust types and trait bounds
3. **Documentation:** Include comprehensive doc comments
4. **Testing:** Generate test scaffolds for new components

### Framework Compatibility

1. **Version Pinning:** Use compatible dependency versions
2. **Feature Flags:** Support optional framework features
3. **API Consistency:** Maintain similar APIs across frameworks
4. **Migration Support:** Provide upgrade paths for breaking changes

## Troubleshooting

### Common Issues

**Template Compilation Errors:**
- Verify Handlebars syntax
- Check variable names match config
- Ensure proper escaping for Rust keywords

**Framework Compatibility:**
- Update dependency versions
- Check trait implementations
- Verify macro usage patterns

**Styling Problems:**
- Validate TailwindCSS classes
- Check theme variable references
- Ensure responsive design patterns

### Debug Mode

Enable debug output for template generation:

```bash
RUST_LOG=debug cargo run -p shadcn -- generate --name "test" --framework "leptos"
```

## Contributing

### Adding New Components

1. Study existing component patterns
2. Create templates following conventions
3. Test across all supported frameworks
4. Update documentation and examples
5. Submit PR with comprehensive tests

### Template Guidelines

- Use semantic HTML elements
- Include accessibility attributes
- Follow framework-specific patterns
- Support all theme variants
- Maintain consistent styling approach

For more information, see the [Contributing Guide](../CONTRIBUTING.md).