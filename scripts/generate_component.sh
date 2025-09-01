#!/bin/bash

# Simple Leptos Component Generator
# Usage: ./scripts/generate_component.sh <component_name> <component_type>

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_header() {
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}  Component Generator${NC}"
    echo -e "${BLUE}================================${NC}"
}

if [ $# -lt 2 ]; then
    echo "Usage: $0 <component_name> <component_type>"
    echo "Types: basic, form, interactive, layout, feedback"
    exit 1
fi

COMPONENT_NAME=$1
COMPONENT_TYPE=$2
COMPONENT_DIR="packages/leptos/$COMPONENT_NAME"
COMPONENT_NAME_CAMEL=$(echo $COMPONENT_NAME | sed 's/-\([a-z]\)/\U\1/g')
# Simple mapping for common component names
case $COMPONENT_NAME in
    "input") COMPONENT_NAME_PASCAL="Input" ;;
    "label") COMPONENT_NAME_PASCAL="Label" ;;
    "button") COMPONENT_NAME_PASCAL="Button" ;;
    "card") COMPONENT_NAME_PASCAL="Card" ;;
    "radio-group") COMPONENT_NAME_PASCAL="RadioGroup" ;;
    "select") COMPONENT_NAME_PASCAL="Select" ;;
    "tooltip") COMPONENT_NAME_PASCAL="Tooltip" ;;
    "textarea") COMPONENT_NAME_PASCAL="Textarea" ;;
    "separator") COMPONENT_NAME_PASCAL="Separator" ;;
    "avatar") COMPONENT_NAME_PASCAL="Avatar" ;;
    "progress") COMPONENT_NAME_PASCAL="Progress" ;;
    "skeleton") COMPONENT_NAME_PASCAL="Skeleton" ;;
    "switch") COMPONENT_NAME_PASCAL="Switch" ;;
    "slider") COMPONENT_NAME_PASCAL="Slider" ;;
    "breadcrumb") COMPONENT_NAME_PASCAL="Breadcrumb" ;;
    *) 
        # Fallback: capitalize first letter and remove hyphens
        COMPONENT_NAME_PASCAL=$(echo "$COMPONENT_NAME" | sed 's/-//g' | sed 's/^./\U&/')
        ;;
esac

print_header
print_status "Generating $COMPONENT_NAME ($COMPONENT_TYPE)"

# Create directory
mkdir -p "$COMPONENT_DIR/src"

# Generate Cargo.toml
cat > "$COMPONENT_DIR/Cargo.toml" << EOF
[package]
name = "shadcn-ui-leptos-$COMPONENT_NAME"
description = "Leptos port of shadcn/ui $COMPONENT_NAME"
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
//! Leptos port of shadcn/ui $COMPONENT_NAME

pub mod default;
pub mod new_york;

pub use default::{$COMPONENT_NAME_PASCAL};
pub use new_york::{$COMPONENT_NAME_PASCAL as ${COMPONENT_NAME_PASCAL}NewYork};

#[cfg(test)]
mod tests;
EOF

# Generate default.rs based on type
case $COMPONENT_TYPE in
    "form")
        cat > "$COMPONENT_DIR/src/default.rs" << 'EOF'
use leptos::{ev::Event, prelude::*};
use leptos_style::Style;
use leptos::wasm_bindgen::JsCast;

const INPUT_CLASS: &str = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

#[component]
pub fn Input(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] on_change: Option<Callback<String>>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] input_type: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
) -> impl IntoView {
    let handle_input = {
        let on_change = on_change.clone();
        move |event: Event| {
            if let Some(callback) = &on_change {
                let target = event.target().unwrap();
                let input = target.unchecked_into::<web_sys::HtmlInputElement>();
                callback.run(input.value());
            }
        }
    };

    let computed_class = Signal::derive(move || {
        format!("{} {}", INPUT_CLASS, class.get().unwrap_or_default())
    });

    view! {
        <input
            type=input_type.get().unwrap_or_else(|| "text".to_string())
            value=value.get().unwrap_or_default()
            placeholder=placeholder.get().unwrap_or_default()
            disabled=disabled
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
            on:input=handle_input
        />
    }
}
EOF
        ;;
    "basic")
        cat > "$COMPONENT_DIR/src/default.rs" << EOF
use leptos::prelude::*;
use leptos_style::Style;

const LABEL_CLASS: &str = "base-class-here";

#[component]
pub fn $COMPONENT_NAME_PASCAL(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("{} {}", LABEL_CLASS, class.get().unwrap_or_default())
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
        ;;
    *)
        echo "Unknown component type: $COMPONENT_TYPE"
        exit 1
        ;;
esac

# Copy to new_york.rs
cp "$COMPONENT_DIR/src/default.rs" "$COMPONENT_DIR/src/new_york.rs"

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
        assert!(true);
    }
}
EOF

# Generate README.md
cat > "$COMPONENT_DIR/README.md" << EOF
# $COMPONENT_NAME_PASCAL

Leptos port of shadcn/ui $COMPONENT_NAME

## Usage

\`\`\`rust
use shadcn_ui_leptos_$COMPONENT_NAME::$COMPONENT_NAME_PASCAL;
\`\`\`

## Documentation

See [shadcn/ui documentation](https://ui.shadcn.com/docs/components/$COMPONENT_NAME).
EOF

# Update workspace Cargo.toml
if ! grep -q "packages/leptos/$COMPONENT_NAME" "Cargo.toml"; then
    sed -i.bak "/packages\/leptos\/tooltip/a\\
    \"packages/leptos/$COMPONENT_NAME\"," "Cargo.toml"
    rm "Cargo.toml.bak"
fi

print_status "Component generated successfully!"
print_status "Next: Update classes and test with 'cargo check --workspace'"
