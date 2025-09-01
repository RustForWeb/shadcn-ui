#!/bin/bash

# Leptos Component Generator Script
# Usage: ./scripts/generate_leptos_component.sh <component_name> [description]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}  Leptos Component Generator${NC}"
    echo -e "${BLUE}================================${NC}"
}

# Check if component name is provided
if [ $# -eq 0 ]; then
    print_error "Usage: $0 <component_name> [description]"
    print_error "Example: $0 input 'Form input component'"
    exit 1
fi

COMPONENT_NAME=$1
DESCRIPTION=${2:-"Leptos port of shadcn/ui $COMPONENT_NAME"}
COMPONENT_DIR="packages/leptos/$COMPONENT_NAME"
COMPONENT_NAME_CAMEL=$(echo $COMPONENT_NAME | sed 's/-\([a-z]\)/\U\1/g')
COMPONENT_NAME_PASCAL=$(echo $COMPONENT_NAME | sed 's/-\([a-z]\)/\U\1/g' | sed 's/^\([a-z]\)/\U\1/')

print_header
print_status "Generating Leptos component: $COMPONENT_NAME"
print_status "Description: $DESCRIPTION"
print_status "Component directory: $COMPONENT_DIR"

# Create component directory
mkdir -p "$COMPONENT_DIR/src"

# Generate Cargo.toml
cat > "$COMPONENT_DIR/Cargo.toml" << EOF
[package]
name = "shadcn-ui-leptos-$COMPONENT_NAME"
description = "$DESCRIPTION"
homepage = "https://shadcn-ui.rustforweb.org/components/$COMPONENT_NAME.html"
publish = false

authors.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
version.workspace = true

[dependencies]
leptos.workspace = true
leptos-node-ref.workspace = true
leptos-struct-component.workspace = true
leptos-style.workspace = true
tailwind_fuse.workspace = true

[features]
default = []
new_york = []

[dev-dependencies]
shadcn-ui-test-utils = { path = "../../test-utils", features = ["leptos-testing"] }
wasm-bindgen-test = { workspace = true }
EOF

# Generate lib.rs
cat > "$COMPONENT_DIR/src/lib.rs" << EOF
//! $DESCRIPTION
//!
//! See [the Rust shadcn/ui book](https://shadcn-ui.rustforweb.org/components/$COMPONENT_NAME.html) for more documentation.

pub mod default;
pub mod new_york;

// Re-export the components for easy access
pub use default::{$COMPONENT_NAME_PASCAL};
pub use new_york::{$COMPONENT_NAME_PASCAL as ${COMPONENT_NAME_PASCAL}NewYork};

#[cfg(test)]
mod tests;
EOF

# Generate default.rs template
cat > "$COMPONENT_DIR/src/default.rs" << EOF
use leptos::{ev::MouseEvent, prelude::*};
use leptos_style::Style;

// Static classes for better compilation compatibility
const ${COMPONENT_NAME_CAMEL^^}_CLASS: &str = "base-class-here";

#[component]
pub fn $COMPONENT_NAME_PASCAL(
    // Add your props here
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!(
            "{} {}",
            ${COMPONENT_NAME_CAMEL^^}_CLASS,
            class.get().unwrap_or_default()
        )
    });

    view! {
        <div
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
        >
            {children.map(|c| c())}
        </div>
    }
}
EOF

# Generate new_york.rs template
cat > "$COMPONENT_DIR/src/new_york.rs" << EOF
use leptos::{ev::MouseEvent, prelude::*};
use leptos_style::Style;

// Static classes for New York theme
const ${COMPONENT_NAME_CAMEL^^}_CLASS: &str = "base-class-here";

#[component]
pub fn $COMPONENT_NAME_PASCAL(
    // Add your props here
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!(
            "{} {}",
            ${COMPONENT_NAME_CAMEL^^}_CLASS,
            class.get().unwrap_or_default()
        )
    });

    view! {
        <div
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
        >
            {children.map(|c| c())}
        </div>
    }
}
EOF

# Generate tests.rs
cat > "$COMPONENT_DIR/src/tests.rs" << EOF
#[cfg(test)]
mod tests {
    use super::*;
    use shadcn_ui_test_utils::leptos_testing::*;

    #[test]
    fn test_${COMPONENT_NAME_CAMEL}_renders() {
        let (cx, _) = leptos_testing::create_test_runtime();
        
        let component = view! { cx, <$COMPONENT_NAME_PASCAL /> };
        
        // Add your test assertions here
        assert!(true); // Placeholder assertion
    }

    #[test]
    fn test_${COMPONENT_NAME_CAMEL}_with_class() {
        let (cx, _) = leptos_testing::create_test_runtime();
        
        let component = view! { cx, <$COMPONENT_NAME_PASCAL class="custom-class" /> };
        
        // Add your test assertions here
        assert!(true); // Placeholder assertion
    }
}
EOF

# Generate README.md
cat > "$COMPONENT_DIR/README.md" << EOF
# $COMPONENT_NAME_PASCAL

$DESCRIPTION

## Usage

\`\`\`rust
use shadcn_ui_leptos_$COMPONENT_NAME::$COMPONENT_NAME_PASCAL;

#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <$COMPONENT_NAME_PASCAL>
            "Content here"
        </$COMPONENT_NAME_PASCAL>
    }
}
\`\`\`

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| class | MaybeProp<String> | None | Additional CSS classes |
| id | MaybeProp<String> | None | HTML id attribute |
| style | Signal<Style> | Default | Inline styles |
| children | Option<Children> | None | Child elements |

## Themes

This component supports both default and New York themes:

- **Default**: \`shadcn_ui_leptos_$COMPONENT_NAME::$COMPONENT_NAME_PASCAL\`
- **New York**: \`shadcn_ui_leptos_$COMPONENT_NAME::${COMPONENT_NAME_PASCAL}NewYork\`

## Documentation

For more information, see the [shadcn/ui documentation](https://ui.shadcn.com/docs/components/$COMPONENT_NAME).
EOF

# Update workspace Cargo.toml to include the new component
WORKSPACE_CARGO_TOML="Cargo.toml"

# Check if the component is already in the workspace
if ! grep -q "packages/leptos/$COMPONENT_NAME" "$WORKSPACE_CARGO_TOML"; then
    print_status "Adding component to workspace Cargo.toml..."
    
    # Find the [workspace.members] section and add the new component
    # This is a simple approach - in a real scenario you might want to use a more robust method
    sed -i.bak "/packages\/leptos\/tooltip/a\\
    \"packages/leptos/$COMPONENT_NAME\"," "$WORKSPACE_CARGO_TOML"
    
    # Clean up backup file
    rm "${WORKSPACE_CARGO_TOML}.bak"
fi

# Update book examples if they exist
BOOK_EXAMPLES_DIR="book-examples/leptos/src"
if [ -d "$BOOK_EXAMPLES_DIR" ]; then
    print_status "Creating book examples..."
    
    # Create default theme example
    mkdir -p "$BOOK_EXAMPLES_DIR/default"
    cat > "$BOOK_EXAMPLES_DIR/default/${COMPONENT_NAME}.rs" << EOF
use leptos::prelude::*;
use shadcn_ui_leptos_$COMPONENT_NAME::$COMPONENT_NAME_PASCAL;

#[component]
pub fn ${COMPONENT_NAME_PASCAL}Example() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <div class="space-y-2">
                <h3 class="text-lg font-medium">"$COMPONENT_NAME_PASCAL Example"</h3>
                <p class="text-sm text-muted-foreground">
                    "This is an example of the $COMPONENT_NAME_PASCAL component."
                </p>
            </div>
            
            <div class="space-y-2">
                <$COMPONENT_NAME_PASCAL>
                    "Example content"
                </$COMPONENT_NAME_PASCAL>
            </div>
        </div>
    }
}

#[component(transparent)]
pub fn ${COMPONENT_NAME_PASCAL}Routes() -> impl IntoView {
    view! {
        <Route path="/$COMPONENT_NAME" view=${COMPONENT_NAME_PASCAL}Example />
    }
}
EOF

    # Create New York theme example
    mkdir -p "$BOOK_EXAMPLES_DIR/new_york"
    cat > "$BOOK_EXAMPLES_DIR/new_york/${COMPONENT_NAME}.rs" << EOF
use leptos::prelude::*;
use shadcn_ui_leptos_$COMPONENT_NAME::${COMPONENT_NAME_PASCAL}NewYork;

#[component]
pub fn ${COMPONENT_NAME_PASCAL}Example() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <div class="space-y-2">
                <h3 class="text-lg font-medium">"$COMPONENT_NAME_PASCAL Example (New York)"</h3>
                <p class="text-sm text-muted-foreground">
                    "This is an example of the $COMPONENT_NAME_PASCAL component with New York theme."
                </p>
            </div>
            
            <div class="space-y-2">
                <${COMPONENT_NAME_PASCAL}NewYork>
                    "Example content"
                </${COMPONENT_NAME_PASCAL}NewYork>
            </div>
        </div>
    }
}

#[component(transparent)]
pub fn ${COMPONENT_NAME_PASCAL}Routes() -> impl IntoView {
    view! {
        <Route path="/$COMPONENT_NAME" view=${COMPONENT_NAME_PASCAL}Example />
    }
}
EOF
fi

print_status "Component generation complete!"
print_status ""
print_status "Next steps:"
print_status "1. Update the component classes in $COMPONENT_DIR/src/default.rs"
print_status "2. Update the component classes in $COMPONENT_DIR/src/new_york.rs"
print_status "3. Add your component logic and props"
print_status "4. Run 'cargo check --workspace' to verify compilation"
print_status "5. Add tests in $COMPONENT_DIR/src/tests.rs"
print_status ""
print_status "Component files created:"
print_status "  - $COMPONENT_DIR/Cargo.toml"
print_status "  - $COMPONENT_DIR/src/lib.rs"
print_status "  - $COMPONENT_DIR/src/default.rs"
print_status "  - $COMPONENT_DIR/src/new_york.rs"
print_status "  - $COMPONENT_DIR/src/tests.rs"
print_status "  - $COMPONENT_DIR/README.md"

if [ -d "$BOOK_EXAMPLES_DIR" ]; then
    print_status "  - $BOOK_EXAMPLES_DIR/default/${COMPONENT_NAME}.rs"
    print_status "  - $BOOK_EXAMPLES_DIR/new_york/${COMPONENT_NAME}.rs"
fi

print_status ""
print_status "Happy coding! 🚀"
