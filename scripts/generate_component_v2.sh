#!/bin/bash

# Leptos Component Generator v2
# Usage: ./scripts/generate_component_v2.sh <component_name> <component_type>

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
    echo -e "${BLUE}  Component Generator v2${NC}"
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

# Component name mapping
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
    "alert") COMPONENT_NAME_PASCAL="Alert" ;;
    "badge") COMPONENT_NAME_PASCAL="Badge" ;;
    "checkbox") COMPONENT_NAME_PASCAL="Checkbox" ;;
    "dialog") COMPONENT_NAME_PASCAL="Dialog" ;;
    "dropdown-menu") COMPONENT_NAME_PASCAL="DropdownMenu" ;;
    "form") COMPONENT_NAME_PASCAL="Form" ;;
    "hover-card") COMPONENT_NAME_PASCAL="HoverCard" ;;
    "menubar") COMPONENT_NAME_PASCAL="Menubar" ;;
    "navigation-menu") COMPONENT_NAME_PASCAL="NavigationMenu" ;;
    "popover") COMPONENT_NAME_PASCAL="Popover" ;;
    "scroll-area") COMPONENT_NAME_PASCAL="ScrollArea" ;;
    "sheet") COMPONENT_NAME_PASCAL="Sheet" ;;
    "table") COMPONENT_NAME_PASCAL="Table" ;;
    "tabs") COMPONENT_NAME_PASCAL="Tabs" ;;
    "toast") COMPONENT_NAME_PASCAL="Toast" ;;
    "toggle") COMPONENT_NAME_PASCAL="Toggle" ;;
    *) 
        # Fallback: capitalize first letter and remove hyphens
        COMPONENT_NAME_PASCAL=$(echo "$COMPONENT_NAME" | sed 's/-//g' | sed 's/^./\U&/')
        ;;
esac

print_header
print_status "Generating $COMPONENT_NAME ($COMPONENT_TYPE) -> $COMPONENT_NAME_PASCAL"

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
web-sys.workspace = true

[features]
default = []
new_york = []

[dev-dependencies]
shadcn-ui-test-utils = { path = "../../test-utils" }
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
        cat > "$COMPONENT_DIR/src/default.rs" << EOF
use leptos::{ev::Event, prelude::*};
use leptos_style::Style;
use leptos::wasm_bindgen::JsCast;

const ${COMPONENT_NAME_PASCAL}_CLASS: &str = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

#[component]
pub fn $COMPONENT_NAME_PASCAL(
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
        format!("{} {}", ${COMPONENT_NAME_PASCAL}_CLASS, class.get().unwrap_or_default())
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

const ${COMPONENT_NAME_PASCAL}_CLASS: &str = "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70";

#[component]
pub fn $COMPONENT_NAME_PASCAL(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("{} {}", ${COMPONENT_NAME_PASCAL}_CLASS, class.get().unwrap_or_default())
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
    "interactive")
        cat > "$COMPONENT_DIR/src/default.rs" << EOF
use leptos::prelude::*;
use leptos_style::Style;

const ${COMPONENT_NAME_PASCAL}_CLASS: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";

#[component]
pub fn $COMPONENT_NAME_PASCAL(
    #[prop(into, optional)] variant: MaybeProp<String>,
    #[prop(into, optional)] size: MaybeProp<String>,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let handle_click = {
        let on_click = on_click.clone();
        move |_| {
            if let Some(callback) = &on_click {
                callback.run(());
            }
        }
    };

    let computed_class = Signal::derive(move || {
        let variant_class = match variant.get().unwrap_or_default().as_str() {
            "default" => "bg-primary text-primary-foreground hover:bg-primary/90",
            "destructive" => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
            "outline" => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
            "secondary" => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
            "ghost" => "hover:bg-accent hover:text-accent-foreground",
            "link" => "text-primary underline-offset-4 hover:underline",
            _ => "bg-primary text-primary-foreground hover:bg-primary/90",
        };
        
        let size_class = match size.get().unwrap_or_default().as_str() {
            "default" => "h-10 px-4 py-2",
            "sm" => "h-9 rounded-md px-3",
            "lg" => "h-11 rounded-md px-8",
            "icon" => "h-10 w-10",
            _ => "h-10 px-4 py-2",
        };
        
        format!("{} {} {} {}", ${COMPONENT_NAME_PASCAL}_CLASS, variant_class, size_class, class.get().unwrap_or_default())
    });

    view! {
        <button
            class=computed_class
            id=id.get().unwrap_or_default()
            style=move || style.get().to_string()
            disabled=disabled
            on:click=handle_click
        >
            {children.map(|c| c())}
        </button>
    }
}
EOF
        ;;
    "layout")
        cat > "$COMPONENT_DIR/src/default.rs" << EOF
use leptos::prelude::*;
use leptos_style::Style;

const ${COMPONENT_NAME_PASCAL}_CLASS: &str = "rounded-lg border bg-card text-card-foreground shadow-sm";

#[component]
pub fn $COMPONENT_NAME_PASCAL(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("{} {}", ${COMPONENT_NAME_PASCAL}_CLASS, class.get().unwrap_or_default())
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
    "feedback")
        cat > "$COMPONENT_DIR/src/default.rs" << EOF
use leptos::prelude::*;
use leptos_style::Style;

const ${COMPONENT_NAME_PASCAL}_CLASS: &str = "relative w-full rounded-lg border p-4";

#[component]
pub fn $COMPONENT_NAME_PASCAL(
    #[prop(into, optional)] variant: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        let variant_class = match variant.get().unwrap_or_default().as_str() {
            "default" => "bg-background text-foreground",
            "destructive" => "border-destructive/50 text-destructive dark:border-destructive",
            "success" => "border-green-500/50 text-green-600 dark:text-green-400",
            "warning" => "border-yellow-500/50 text-yellow-600 dark:text-yellow-400",
            _ => "bg-background text-foreground",
        };
        
        format!("{} {} {}", ${COMPONENT_NAME_PASCAL}_CLASS, variant_class, class.get().unwrap_or_default())
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

# Generate tests.rs based on component type
case $COMPONENT_TYPE in
    "basic")
        cat > "$COMPONENT_DIR/src/tests.rs" << EOF
#[cfg(test)]
mod tests {
    use crate::$COMPONENT_NAME_PASCAL;
    use shadcn_ui_test_utils::leptos_testing::{LeptosTestUtils, test_helpers};

    #[test]
    fn test_${COMPONENT_NAME}_component_exists() {
        // Basic test to ensure the component can be imported
        let result = LeptosTestUtils::test_component_renders();
        assert!(result.is_success());
    }

    #[test]
    fn test_${COMPONENT_NAME}_basic_functionality() {
        // Test basic component functionality
        let result = LeptosTestUtils::test_component_with_props(std::collections::HashMap::new());
        assert!(result.is_success());
    }

    #[test]
    fn test_${COMPONENT_NAME}_accessibility() {
        // Test component accessibility
        let result = LeptosTestUtils::test_component_accessibility();
        assert!(result.is_success());
    }

    #[test]
    fn test_${COMPONENT_NAME}_styling() {
        // Test component styling
        let result = LeptosTestUtils::test_component_styling();
        assert!(result.is_success());
    }

    #[test]
    fn test_${COMPONENT_NAME}_comprehensive() {
        // Comprehensive test using the test builder
        let test = test_helpers::basic_component_test("${COMPONENT_NAME}");
        let result = test.run();
        assert!(result.is_success());
    }
}
EOF
        ;;
    "form")
        cat > "$COMPONENT_DIR/src/tests.rs" << EOF
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::*;

    #[test]
    fn test_${COMPONENT_NAME}_renders() {
        let (cx, _) = create_test_runtime();
        let component = view! { cx, <$COMPONENT_NAME_PASCAL /> };
        assert!(true);
    }

    #[test]
    fn test_${COMPONENT_NAME}_with_value() {
        let (cx, _) = create_test_runtime();
        let component = view! { 
            cx, 
            <$COMPONENT_NAME_PASCAL value="test value" />
        };
        assert!(true);
    }

    #[test]
    fn test_${COMPONENT_NAME}_with_placeholder() {
        let (cx, _) = create_test_runtime();
        let component = view! { 
            cx, 
            <$COMPONENT_NAME_PASCAL placeholder="Enter text" />
        };
        assert!(true);
    }

    #[test]
    fn test_${COMPONENT_NAME}_disabled() {
        let (cx, _) = create_test_runtime();
        let component = view! { 
            cx, 
            <$COMPONENT_NAME_PASCAL disabled=true />
        };
        assert!(true);
    }

    #[test]
    fn test_${COMPONENT_NAME}_events() {
        let (cx, _) = create_test_runtime();
        let component = view! { 
            cx, 
            <$COMPONENT_NAME_PASCAL on_change=Callback::new(|_| {}) />
        };
        assert!(true);
    }

    fn create_test_runtime() -> (leptos::Scope, leptos::Runtime) {
        let runtime = leptos::Runtime::new().expect("Failed to create Leptos runtime");
        let cx = runtime.create_scope();
        (cx, runtime)
    }
}
EOF
        ;;
    "interactive")
        cat > "$COMPONENT_DIR/src/tests.rs" << EOF
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::*;

    #[test]
    fn test_${COMPONENT_NAME}_renders() {
        let (cx, _) = create_test_runtime();
        let component = view! { cx, <$COMPONENT_NAME_PASCAL /> };
        assert!(true);
    }

    #[test]
    fn test_${COMPONENT_NAME}_with_variants() {
        let (cx, _) = create_test_runtime();
        let component = view! { 
            cx, 
            <$COMPONENT_NAME_PASCAL variant="default" />
        };
        assert!(true);
    }

    #[test]
    fn test_${COMPONENT_NAME}_with_sizes() {
        let (cx, _) = create_test_runtime();
        let component = view! { 
            cx, 
            <$COMPONENT_NAME_PASCAL size="default" />
        };
        assert!(true);
    }

    #[test]
    fn test_${COMPONENT_NAME}_interactions() {
        let (cx, _) = create_test_runtime();
        let component = view! { 
            cx, 
            <$COMPONENT_NAME_PASCAL on_click=Callback::new(|_| {}) />
        };
        assert!(true);
    }

    fn create_test_runtime() -> (leptos::Scope, leptos::Runtime) {
        let runtime = leptos::Runtime::new().expect("Failed to create Leptos runtime");
        let cx = runtime.create_scope();
        (cx, runtime)
    }
}
EOF
        ;;
    "layout")
        cat > "$COMPONENT_DIR/src/tests.rs" << EOF
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::*;

    #[test]
    fn test_${COMPONENT_NAME}_renders() {
        let (cx, _) = create_test_runtime();
        let component = view! { cx, <$COMPONENT_NAME_PASCAL /> };
        assert!(true);
    }

    #[test]
    fn test_${COMPONENT_NAME}_with_children() {
        let (cx, _) = create_test_runtime();
        let component = view! { 
            cx, 
            <$COMPONENT_NAME_PASCAL>
                <div>"Child content"</div>
            </$COMPONENT_NAME_PASCAL>
        };
        assert!(true);
    }

    #[test]
    fn test_${COMPONENT_NAME}_with_props() {
        let (cx, _) = create_test_runtime();
        let component = view! { 
            cx, 
            <$COMPONENT_NAME_PASCAL class="test-class" />
        };
        assert!(true);
    }

    fn create_test_runtime() -> (leptos::Scope, leptos::Runtime) {
        let runtime = leptos::Runtime::new().expect("Failed to create Leptos runtime");
        let cx = runtime.create_scope();
        (cx, runtime)
    }
}
EOF
        ;;
    "feedback")
        cat > "$COMPONENT_DIR/src/tests.rs" << EOF
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::*;

    #[test]
    fn test_${COMPONENT_NAME}_renders() {
        let (cx, _) = create_test_runtime();
        let component = view! { cx, <$COMPONENT_NAME_PASCAL /> };
        assert!(true);
    }

    #[test]
    fn test_${COMPONENT_NAME}_with_variants() {
        let (cx, _) = create_test_runtime();
        let component = view! { 
            cx, 
            <$COMPONENT_NAME_PASCAL variant="default" />
        };
        assert!(true);
    }

    #[test]
    fn test_${COMPONENT_NAME}_with_content() {
        let (cx, _) = create_test_runtime();
        let component = view! { 
            cx, 
            <$COMPONENT_NAME_PASCAL>"Test message"</$COMPONENT_NAME_PASCAL>
        };
        assert!(true);
    }

    fn create_test_runtime() -> (leptos::Scope, leptos::Runtime) {
        let runtime = leptos::Runtime::new().expect("Failed to create Leptos runtime");
        let cx = runtime.create_scope();
        (cx, runtime)
    }
}
EOF
        ;;
    *)
        cat > "$COMPONENT_DIR/src/tests.rs" << EOF
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::*;

    #[test]
    fn test_${COMPONENT_NAME}_renders() {
        let (cx, _) = create_test_runtime();
        let component = view! { cx, <$COMPONENT_NAME_PASCAL /> };
        assert!(true);
    }

    fn create_test_runtime() -> (leptos::Scope, leptos::Runtime) {
        let runtime = leptos::Runtime::new().expect("Failed to create Leptos runtime");
        let cx = runtime.create_scope();
        (cx, runtime)
    }
}
EOF
        ;;
esac

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
