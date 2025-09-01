# Leptos Component Generation Scripts

This directory contains scripts to automate the creation of Leptos components for the shadcn/ui Rust port.

## Scripts Overview

### 1. `generate_leptos_component.sh` - Basic Component Generator
Simple script for generating individual components with basic templates.

**Usage:**
```bash
./scripts/generate_leptos_component.sh <component_name> [description]
```

**Example:**
```bash
./scripts/generate_leptos_component.sh input "Form input component"
```

### 2. `generate_leptos_component_advanced.sh` - Advanced Component Generator
Advanced script that generates components with specific templates based on component type.

**Usage:**
```bash
./scripts/generate_leptos_component_advanced.sh <component_name> <component_type> [description]
```

**Component Types:**
- `basic` - Simple components with children support
- `form` - Form input components with value handling
- `interactive` - Interactive components with click handlers and variants
- `layout` - Layout components with header/footer support
- `feedback` - Feedback components with variants and dismiss functionality

**Examples:**
```bash
# Generate a form input component
./scripts/generate_leptos_component_advanced.sh input form "Form input component"

# Generate an interactive button component
./scripts/generate_leptos_component_advanced.sh button interactive "Button component"

# Generate a layout card component
./scripts/generate_leptos_component_advanced.sh card layout "Card layout component"
```

### 3. `generate_batch_components.sh` - Batch Component Generator
Generates multiple components at once based on predefined phases.

**Usage:**
```bash
./scripts/generate_batch_components.sh <phase>
```

**Phases:**
- `1` - Core UI Components (input, label, textarea, separator, avatar, progress, skeleton, switch, slider, breadcrumb)
- `2` - Interactive Components (dialog, popover, dropdown-menu, accordion, collapsible, tabs, hover-card, alert-dialog, combobox, command)
- `3` - Advanced Components (form, table, data-table, calendar, date-picker, navigation-menu, context-menu, menubar, pagination, scroll-area)
- `4` - Complex Components (carousel, chart, drawer, sheet, sidebar, resizable, toggle, toggle-group, input-otp, toast, sonner, typography)
- `all` - Generate all phases at once

**Examples:**
```bash
# Generate all core UI components
./scripts/generate_batch_components.sh 1

# Generate all components
./scripts/generate_batch_components.sh all
```

## Generated Files

Each component generation creates the following structure:

```
packages/leptos/<component_name>/
├── Cargo.toml              # Package configuration
├── README.md               # Component documentation
└── src/
    ├── lib.rs              # Module exports
    ├── default.rs          # Default theme implementation
    ├── new_york.rs         # New York theme implementation
    └── tests.rs            # Test suite
```

Additionally, if the book examples directory exists:
```
book-examples/leptos/src/
├── default/<component_name>.rs    # Default theme example
└── new_york/<component_name>.rs   # New York theme example
```

## Component Templates

### Basic Template
Simple components with children support and standard props.

### Form Template
Components with:
- Value handling (`value`, `on_change`)
- Placeholder support
- Input type configuration
- Disabled state

### Interactive Template
Components with:
- Click event handling
- Variant support (default, destructive, outline, secondary, ghost, link)
- Disabled state

### Layout Template
Components with:
- Header and footer sections
- Structured layout with padding

### Feedback Template
Components with:
- Variant support (default, destructive, success, warning)
- Dismissible functionality
- Dismiss callback

## Customization

After generation, you'll need to:

1. **Update CSS Classes**: Replace placeholder classes in `default.rs` and `new_york.rs`
2. **Add Component Logic**: Implement specific component behavior
3. **Add Props**: Add component-specific props as needed
4. **Update Tests**: Add meaningful test assertions
5. **Verify Compilation**: Run `cargo check --workspace`

## Best Practices

1. **Start with Phase 1**: Begin with core UI components as they're used by others
2. **Test Incrementally**: Generate a few components and test before generating many
3. **Customize Templates**: Modify the script templates for your specific needs
4. **Follow Naming Conventions**: Use kebab-case for component names
5. **Document Changes**: Update README files with component-specific information

## Troubleshooting

### Component Already Exists
The batch script will skip components that already exist. To regenerate:
```bash
rm -rf packages/leptos/<component_name>
./scripts/generate_leptos_component_advanced.sh <component_name> <type>
```

### Compilation Errors
After generation, run:
```bash
cargo check --workspace
```

Common issues:
- Missing CSS classes (update the constants in the component files)
- Type mismatches (check prop types and imports)
- Missing dependencies (add to Cargo.toml if needed)

### Script Permissions
If scripts aren't executable:
```bash
chmod +x scripts/*.sh
```

## Contributing

When adding new component types or templates:

1. Update the `generate_component_by_type` function in the advanced script
2. Add new template functions following the existing pattern
3. Update the README documentation
4. Test with a sample component

## Examples

### Generate a Single Component
```bash
# Generate an input component
./scripts/generate_leptos_component_advanced.sh input form "Form input component"

# Check compilation
cargo check --workspace

# Update classes and test
```

### Generate Multiple Components
```bash
# Generate all core UI components
./scripts/generate_batch_components.sh 1

# Check compilation
cargo check --workspace

# Update classes and test incrementally
```

### Custom Workflow
```bash
# Generate specific components
./scripts/generate_leptos_component_advanced.sh input form
./scripts/generate_leptos_component_advanced.sh label basic
./scripts/generate_leptos_component_advanced.sh button interactive

# Test compilation
cargo check --workspace

# Commit changes
git add .
git commit -m "Add input, label, and button components"
```
