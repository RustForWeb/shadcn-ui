# Architecture Documentation

This document provides a comprehensive overview of the Rust shadcn/ui architecture, design patterns, and implementation strategies.

## System Overview

Rust shadcn/ui is a modular, multi-framework UI component library that provides shadcn/ui components for Rust web applications. The architecture emphasizes type safety, performance, and framework flexibility while maintaining design consistency.

### Core Principles

1. **Component Isolation**: Each component is a separate crate for maximum modularity
2. **Framework Parity**: Consistent API across all supported frameworks
3. **Theme Support**: Default and New York variants for all components
4. **Type Safety**: Leverage Rust's type system for component properties
5. **Performance**: Minimal runtime overhead and optimal bundle sizes

## Workspace Architecture

### Cargo Workspace Structure

```
shadcn-ui/
├── Cargo.toml              # Workspace configuration
├── Cargo.lock              # Dependency lockfile
├── packages/               # Core packages
│   ├── shadcn/            # CLI tool
│   ├── registry/          # Component registry
│   ├── component-generator/ # Code generation
│   ├── test-utils/        # Testing utilities
│   ├── leptos/            # Leptos components
│   └── yew/               # Yew components
├── book-examples/         # Documentation examples
├── docs/                  # Project documentation
└── tests/                 # Integration tests
```

### Package Organization

#### Core Infrastructure

**packages/shadcn/**
- CLI tool for project management and component generation
- Built with clap for argument parsing
- Supports multiple subcommands (generate, init, add, diff)

**packages/registry/**
- Component metadata and registry management
- Centralizes component information across frameworks
- Provides component discovery and validation

**packages/component-generator/**
- Handlebars-based code generation system
- Framework-agnostic templates with specific implementations
- Supports multiple output formats and customization

**packages/test-utils/**
- Shared testing utilities across all components
- Framework-agnostic testing patterns
- Performance benchmarking tools

#### Framework Packages

**packages/leptos/**
```
leptos/
├── Cargo.toml           # Framework-level dependencies
├── src/
│   ├── lib.rs          # Re-exports all components
│   └── components/     # Shared utilities
└── {component}/        # Individual component crates
    ├── Cargo.toml
    ├── src/
    │   ├── lib.rs      # Public API
    │   ├── default.rs  # Default theme
    │   └── new_york.rs # New York theme
    └── README.md
```

**packages/yew/**
```
yew/
├── Cargo.toml           # Framework-level dependencies
├── src/
│   ├── lib.rs          # Re-exports all components
│   └── components/     # Shared utilities
└── {component}/        # Individual component crates
    ├── Cargo.toml
    ├── src/
    │   ├── lib.rs      # Public API
    │   ├── default.rs  # Default theme
    │   └── new_york.rs # New York theme
    └── README.md
```

## Component Architecture

### Design Patterns

#### 1. Component Isolation Pattern

Each component is implemented as a separate Cargo crate:

**Benefits:**
- Independent versioning and dependency management
- Granular imports (users only import what they need)
- Parallel compilation and development
- Clear separation of concerns

**Implementation:**
```rust
// packages/leptos/button/Cargo.toml
[package]
name = "shadcn-ui-leptos-button"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = "0.6"
tailwind_fuse = "0.3"
```

#### 2. Theme Variant Pattern

Components support multiple design themes through a dual-module approach:

```rust
// src/lib.rs - Public API
pub mod default;
pub mod new_york;

// Re-export default theme at crate root
pub use default::*;

// Named export for alternative theme
pub use new_york as new_york;
```

**Usage:**
```rust
// Default theme (implicit)
use shadcn_ui_leptos_button::Button;

// New York theme (explicit)
use shadcn_ui_leptos_button::new_york::Button;
```

#### 3. Styling Composition Pattern

Dynamic CSS class composition using `tailwind_fuse`:

```rust
#[derive(TwClass)]
#[tw(
    class = "inline-flex items-center justify-center rounded-md font-medium",
    variants(
        variant(
            default = "bg-primary text-primary-foreground hover:bg-primary/90",
            destructive = "bg-destructive text-destructive-foreground hover:bg-destructive/90",
            outline = "border border-input bg-background hover:bg-accent",
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

#### 4. Props Pattern

Consistent property handling across frameworks:

**Leptos:**
```rust
#[component]
pub fn Button(
    #[prop(optional)] variant: MaybeProp<ButtonVariant>,
    #[prop(optional)] size: MaybeProp<ButtonSize>,
    #[prop(optional)] disabled: MaybeProp<bool>,
    #[prop(optional)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView
```

**Yew:**
```rust
#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub size: ButtonSize,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: String,
    pub children: Children,
}
```

## Framework Integration

### Leptos Integration

**Key Features:**
- Signal-based reactivity
- Server-side rendering (SSR) support
- Hydration-friendly components
- `view!` macro for declarative UI

**Architecture Patterns:**

**Reactive State Management:**
```rust
#[component]
pub fn Toggle() -> impl IntoView {
    let (checked, set_checked) = create_signal(false);
    
    let toggle_class = create_memo(move |_| {
        ToggleClass {
            pressed: checked.get(),
            size: ToggleSize::Default,
        }
    });

    view! {
        <button
            type="button"
            role="switch"
            aria-checked=move || checked.get()
            class=toggle_class
            on:click=move |_| set_checked.update(|c| *c = !*c)
        >
            // Component implementation
        </button>
    }
}
```

**SSR Compatibility:**
```rust
// Ensure components work in SSR environments
#[component]
pub fn Component() -> impl IntoView {
    // Use create_effect for client-only side effects
    create_effect(move |_| {
        #[cfg(feature = "hydrate")]
        {
            // Client-only code
        }
    });

    view! { /* SSR-safe markup */ }
}
```

### Yew Integration

**Key Features:**
- Component-based architecture
- Virtual DOM diffing
- WebAssembly optimization
- Hook-based state management

**Architecture Patterns:**

**Component State:**
```rust
#[function_component]
pub fn Toggle() -> Html {
    let checked = use_state(|| false);
    
    let toggle_class = use_memo(
        {
            let checked = checked.clone();
            move |_| ToggleClass {
                pressed: *checked,
                size: ToggleSize::Default,
            }
        },
        (),
    );

    let onclick = {
        let checked = checked.clone();
        Callback::from(move |_| {
            checked.set(!*checked);
        })
    };

    html! {
        <button
            type="button"
            role="switch"
            aria-checked={checked.to_string()}
            class={classes!(&*toggle_class)}
            {onclick}
        >
            // Component implementation
        </button>
    }
}
```

**Performance Optimization:**
```rust
// Use memo for expensive computations
let complex_calculation = use_memo(
    |props| expensive_function(&props.data),
    props.clone(),
);

// Use callback for event handlers
let onclick = use_callback(
    |(event, props)| handle_click(event, &props.data),
    props.clone(),
);
```

## Styling Architecture

### TailwindCSS Integration

**Design System Foundation:**
- Consistent spacing scale (0.25rem increments)
- Type scale with semantic naming
- Color system with semantic tokens
- Component-specific utility classes

**CSS Custom Properties:**
```css
:root {
  --background: 0 0% 100%;
  --foreground: 222.2 84% 4.9%;
  
  --card: 0 0% 100%;
  --card-foreground: 222.2 84% 4.9%;
  
  --popover: 0 0% 100%;
  --popover-foreground: 222.2 84% 4.9%;
  
  --primary: 222.2 47.4% 11.2%;
  --primary-foreground: 210 40% 98%;
  
  --secondary: 210 40% 96%;
  --secondary-foreground: 222.2 47.4% 11.2%;
  
  --muted: 210 40% 96%;
  --muted-foreground: 215.4 16.3% 46.9%;
  
  --accent: 210 40% 96%;
  --accent-foreground: 222.2 47.4% 11.2%;
  
  --destructive: 0 84.2% 60.2%;
  --destructive-foreground: 210 40% 98%;
  
  --border: 214.3 31.8% 91.4%;
  --input: 214.3 31.8% 91.4%;
  --ring: 222.2 84% 4.9%;
  
  --radius: 0.5rem;
}
```

**Dark Mode Support:**
```css
.dark {
  --background: 222.2 84% 4.9%;
  --foreground: 210 40% 98%;
  /* ... */
}
```

### Component Styling Patterns

**Base Component Classes:**
```rust
#[derive(TwClass)]
#[tw(class = "base-classes")]
pub struct ComponentClass {
    // Variant properties
}
```

**Variant System:**
```rust
#[derive(TwClass)]
#[tw(
    class = "base-classes",
    variants(
        // Visual variants
        variant(
            default = "default-styles",
            secondary = "secondary-styles",
            destructive = "destructive-styles",
        ),
        // Size variants
        size(
            sm = "small-styles",
            default = "default-styles", 
            lg = "large-styles",
        )
    ),
    // Default variants
    defaults(
        variant = default,
        size = default
    )
)]
```

**Responsive Design:**
```rust
// Mobile-first responsive classes
"w-full sm:w-auto md:w-64 lg:w-80"

// Container queries (when supported)
"@container/sidebar:md:w-64"
```

## Code Generation System

### Template Architecture

**Handlebars Template System:**
```handlebars
{{!-- Base template structure --}}
use {{framework}}::*;
use tailwind_fuse::*;

{{#if has_props}}
#[derive({{prop_derives}})]
pub struct {{component_name}}Props {
    {{#each props}}
    {{#if optional}}#[prop_or_default]{{/if}}
    pub {{name}}: {{prop_type}},
    {{/each}}
}
{{/if}}

{{framework_component_macro}}
pub fn {{component_name}}(
    {{#each props}}
    {{#if @first}}{{else}},{{/if}}
    {{render_prop_parameter this}}
    {{/each}}
) -> {{return_type}} {
    {{render_component_body}}
}
```

**Framework-Specific Templates:**
- `leptos_component.hbs` - Leptos component structure
- `yew_component.hbs` - Yew component structure
- `dioxus_component.hbs` - Dioxus component structure (planned)

**Configuration System:**
```rust
pub struct ComponentConfig {
    pub name: String,
    pub framework: Framework,
    pub theme_variants: Vec<String>,
    pub props: HashMap<String, PropConfig>,
    pub dependencies: Vec<String>,
}
```

## Testing Architecture

### Framework Testing Patterns

**Leptos Testing:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;
    use leptos_dom::*;

    #[test]
    fn test_button_render() {
        let _ = create_runtime();
        
        let button = Button(ButtonProps {
            variant: ButtonVariant::Default,
            children: "Click me".into(),
            ..Default::default()
        });
        
        // Assert component properties
        assert!(button.is_ok());
    }
}
```

**Yew Testing:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use yew::prelude::*;
    use yew::html;

    #[test]
    fn test_button_render() {
        let props = ButtonProps {
            variant: ButtonVariant::Default,
            children: html! { "Click me" }.into(),
            ..Default::default()
        };
        
        let button = html! { <Button ..props /> };
        
        // Test rendering
        assert!(!button.is_empty());
    }
}
```

### Integration Testing

**Component Integration:**
```rust
// tests/integration/button_test.rs
use shadcn_ui_leptos_button::Button;
use shadcn_ui_yew_button::Button as YewButton;

#[test]
fn test_component_parity() {
    // Test that Leptos and Yew buttons have equivalent APIs
    // and render similar output structures
}
```

## Performance Considerations

### Bundle Size Optimization

**Tree Shaking:**
- Each component as separate crate enables fine-grained imports
- Users only bundle components they actually use
- Framework-specific optimizations applied automatically

**Lazy Loading:**
```rust
// Dynamic imports for large components
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Route 
                path="/dashboard"
                view=|| {
                    // Lazy load heavy dashboard components
                    provide_context(DashboardContext::new());
                    view! { <DashboardPage /> }
                }
            />
        </Router>
    }
}
```

### Runtime Performance

**Memoization Patterns:**
```rust
// Leptos
let expensive_calculation = create_memo(move |_| {
    expensive_function(props.data())
});

// Yew
let expensive_calculation = use_memo(
    |props| expensive_function(&props.data),
    props.clone(),
);
```

**Event Handler Optimization:**
```rust
// Avoid recreating handlers on every render
let onclick = create_callback(move |event| {
    // Handler logic
});
```

## Security Considerations

### XSS Prevention

**HTML Sanitization:**
```rust
// Always sanitize user-provided HTML content
use ammonia::clean;

let safe_html = clean(&user_input);
```

**Attribute Validation:**
```rust
// Validate and sanitize component attributes
pub fn sanitize_class_name(class: &str) -> String {
    class.chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect()
}
```

### Dependency Security

**Supply Chain Security:**
- Pin dependency versions in Cargo.lock
- Regular security audits with `cargo audit`
- Minimal dependency footprint
- Prefer well-maintained, popular crates

## Accessibility Architecture

### ARIA Integration

**Component Accessibility:**
```rust
view! {
    <button
        type="button"
        role="switch"
        aria-checked=move || checked.get()
        aria-describedby=description_id
        class=toggle_class
    >
        <span class="sr-only">
            {move || if checked.get() { "On" } else { "Off" }}
        </span>
    </button>
}
```

**Screen Reader Support:**
- Semantic HTML elements by default
- ARIA labels and descriptions
- Focus management
- Keyboard navigation support

### Keyboard Navigation

**Focus Management:**
```rust
use leptos_use::*;

#[component]
pub fn Dialog(show: ReadSignal<bool>) -> impl IntoView {
    let dialog_ref = create_node_ref::<html::Div>();
    
    // Trap focus within dialog when open
    let _ = use_focus_trap(dialog_ref, show);
    
    // Return focus to trigger when closed
    let _ = use_focus_return(dialog_ref, show);
    
    view! {
        <div node_ref=dialog_ref>
            // Dialog content
        </div>
    }
}
```

## Documentation Architecture

### API Documentation

**Rust Doc Standards:**
```rust
/// A button component that triggers an action or event.
/// 
/// The button component supports multiple variants and sizes, and can be
/// customized with additional CSS classes.
///
/// # Examples
///
/// ```rust
/// use shadcn_ui_leptos_button::*;
/// 
/// let button = view! {
///     <Button variant=ButtonVariant::Primary>
///         "Click me"
///     </Button>
/// };
/// ```
///
/// # Accessibility
///
/// The button component includes proper ARIA attributes and keyboard
/// navigation support by default.
#[component]
pub fn Button(/* ... */) -> impl IntoView {
    // Implementation
}
```

**Component Examples:**
```rust
/// # Component Variants
///
/// ```rust
/// // Primary button
/// view! { <Button variant=ButtonVariant::Primary>"Primary"</Button> }
///
/// // Secondary button  
/// view! { <Button variant=ButtonVariant::Secondary>"Secondary"</Button> }
///
/// // Disabled button
/// view! { <Button disabled=true>"Disabled"</Button> }
/// ```
```

### Living Documentation

**Storybook Integration:**
- Interactive component explorer
- Visual regression testing
- Design system documentation
- Accessibility testing integration

## Future Architecture Considerations

### Framework Expansion

**Dioxus Support:**
- Cross-platform component compilation
- Native mobile and desktop support
- Shared business logic patterns

**Web Components:**
- Framework-agnostic distribution
- Custom element registration
- Progressive enhancement

### Advanced Features

**Animation System:**
- CSS transition integration
- JavaScript animation coordination
- Performance-optimized animations

**Theme System Evolution:**
- Runtime theme switching
- Custom theme generation
- Design token automation

**Developer Experience:**
- Hot module replacement
- Component debugging tools
- Performance profiling integration

This architecture provides a solid foundation for scalable, maintainable, and performant UI component development across multiple Rust web frameworks.