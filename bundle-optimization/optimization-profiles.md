# Bundle Optimization Profiles

## Profile 1: Essential (Minimal Bundle)
**Use case**: Landing pages, simple forms, basic UI
**Components**: button, input, label, card, separator
**Estimated size**: 0MB WASM
**Features**: --features essential

## Profile 2: Forms (Form-Focused)
**Use case**: Data entry, user registration, settings
**Components**: button, input, label, checkbox, radio-group, select, textarea, form
**Estimated size**: ~0MB WASM
**Features**: --features forms

## Profile 3: Layout (Content-Heavy)
**Use case**: Dashboards, content management, complex layouts
**Components**: card, separator, skeleton, tabs, table, pagination
**Estimated size**: ~0MB WASM
**Features**: --features layout

## Profile 4: Interactive (Feature-Rich)
**Use case**: Applications, tools, interactive experiences
**Components**: button, checkbox, radio-group, select, switch, tabs, dialog, tooltip
**Estimated size**: ~0MB WASM
**Features**: --features interactive

## Profile 5: Complete (All Components)
**Use case**: Component libraries, design systems, development
**Components**: All 47 components
**Estimated size**: 0MB WASM
**Features**: --features all

## Usage Examples

### Minimal Landing Page
```bash
cargo build --release --features essential
```

### Form-Heavy Application
```bash
cargo build --release --features "essential,forms"
```

### Dashboard Application
```bash
cargo build --release --features "essential,layout,data"
```

### Full-Featured App
```bash
cargo build --release --features all
```
