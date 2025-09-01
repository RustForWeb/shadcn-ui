#!/bin/bash

# Advanced Leptos Component Generator Script
# Usage: ./scripts/generate_leptos_component_advanced.sh <component_name> <component_type> [description]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
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
    echo -e "${BLUE}  Advanced Leptos Generator${NC}"
    echo -e "${BLUE}================================${NC}"
}

print_component_type() {
    echo -e "${PURPLE}[COMPONENT TYPE]${NC} $1"
}

# Check if component name and type are provided
if [ $# -lt 2 ]; then
    print_error "Usage: $0 <component_name> <component_type> [description]"
    print_error "Component types: basic, form, interactive, layout, feedback"
    print_error "Example: $0 input form 'Form input component'"
    print_error "Example: $0 button basic 'Button component'"
    exit 1
fi

COMPONENT_NAME=$1
COMPONENT_TYPE=$2
DESCRIPTION=${3:-"Leptos port of shadcn/ui $COMPONENT_NAME"}
COMPONENT_DIR="packages/leptos/$COMPONENT_NAME"
COMPONENT_NAME_CAMEL=$(echo $COMPONENT_NAME | sed 's/-\([a-z]\)/\U\1/g')
COMPONENT_NAME_PASCAL=$(echo $COMPONENT_NAME | sed 's/-\([a-z]\)/\U\1/g' | sed 's/^\([a-z]\)/\U\1/')

print_header
print_status "Generating Leptos component: $COMPONENT_NAME"
print_component_type "$COMPONENT_TYPE"
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

# Function to generate component based on type
generate_component_by_type() {
    local component_type=$1
    local file_path=$2
    
    case $component_type in
        "basic")
            generate_basic_component "$file_path"
            ;;
        "form")
            generate_form_component "$file_path"
            ;;
        "interactive")
            generate_interactive_component "$file_path"
            ;;
        "layout")
            generate_layout_component "$file_path"
            ;;
        "feedback")
            generate_feedback_component "$file_path"
            ;;
        *)
            generate_basic_component "$file_path"
            ;;
    esac
}

# Basic component template
generate_basic_component() {
    local file_path=$1
    cat > "$file_path" << EOF
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
}

# Form component template
generate_form_component() {
    local file_path=$1
    cat > "$file_path" << 'EOF'
use leptos::{ev::MouseEvent, prelude::*};
use leptos_style::Style;

// Static classes for better compilation compatibility
const INPUT_CLASS: &str = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

#[component]
pub fn Input(
    /// The value of the input
    #[prop(into, optional)] value: MaybeProp<String>,
    
    /// Callback when value changes
    #[prop(into, optional)] on_change: Option<Callback<String>>,
    
    /// Placeholder text
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    
    /// Whether the input is disabled
    #[prop(into, optional)] disabled: Signal<bool>,
    
    /// Input type
    #[prop(into, optional)] input_type: MaybeProp<String>,

    // Global attributes
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
        format!(
            "{} {}",
            INPUT_CLASS,
            class.get().unwrap_or_default()
        )
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
}

# Interactive component template
generate_interactive_component() {
    local file_path=$1
    cat > "$file_path" << EOF
use leptos::{ev::MouseEvent, prelude::*};
use leptos_style::Style;

// Static classes for better compilation compatibility
const ${COMPONENT_NAME_CAMEL^^}_CLASS: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";

#[component]
pub fn $COMPONENT_NAME_PASCAL(
    /// Callback when clicked
    #[prop(into, optional)] on_click: Option<Callback<MouseEvent>>,
    
    /// Whether the component is disabled
    #[prop(into, optional)] disabled: Signal<bool>,
    
    /// Variant of the component
    #[prop(into, optional)] variant: MaybeProp<String>,

    // Global attributes
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let handle_click = {
        let on_click = on_click.clone();
        move |event: MouseEvent| {
            if let Some(callback) = &on_click {
                callback.run(event);
            }
        }
    };

    let computed_class = Signal::derive(move || {
        let variant_class = match variant.get().as_deref() {
            Some("default") => "bg-primary text-primary-foreground hover:bg-primary/90",
            Some("destructive") => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
            Some("outline") => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
            Some("secondary") => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
            Some("ghost") => "hover:bg-accent hover:text-accent-foreground",
            Some("link") => "text-primary underline-offset-4 hover:underline",
            _ => "bg-primary text-primary-foreground hover:bg-primary/90"
        };
        
        format!(
            "{} {} {}",
            ${COMPONENT_NAME_CAMEL^^}_CLASS,
            variant_class,
            class.get().unwrap_or_default()
        )
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
}

# Layout component template
generate_layout_component() {
    local file_path=$1
    cat > "$file_path" << EOF
use leptos::{ev::MouseEvent, prelude::*};
use leptos_style::Style;

// Static classes for better compilation compatibility
const ${COMPONENT_NAME_CAMEL^^}_CLASS: &str = "rounded-lg border bg-card text-card-foreground shadow-sm";

#[component]
pub fn $COMPONENT_NAME_PASCAL(
    /// The header content
    #[prop(optional)] header: Option<Children>,
    
    /// The footer content
    #[prop(optional)] footer: Option<Children>,

    // Global attributes
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
            {header.map(|h| view! {
                <div class="flex flex-col space-y-1.5 p-6">
                    {h()}
                </div>
            })}
            <div class="p-6 pt-0">
                {children.map(|c| c())}
            </div>
            {footer.map(|f| view! {
                <div class="flex items-center p-6 pt-0">
                    {f()}
                </div>
            })}
        </div>
    }
}
EOF
}

# Feedback component template
generate_feedback_component() {
    local file_path=$1
    cat > "$file_path" << EOF
use leptos::{ev::MouseEvent, prelude::*};
use leptos_style::Style;

// Static classes for better compilation compatibility
const ${COMPONENT_NAME_CAMEL^^}_CLASS: &str = "relative w-full rounded-lg border p-4";

#[component]
pub fn $COMPONENT_NAME_PASCAL(
    /// The variant of the component
    #[prop(into, optional)] variant: MaybeProp<String>,
    
    /// Whether the component can be dismissed
    #[prop(into, optional)] dismissible: Signal<bool>,
    
    /// Callback when dismissed
    #[prop(into, optional)] on_dismiss: Option<Callback<()>>,

    // Global attributes
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let handle_dismiss = {
        let on_dismiss = on_dismiss.clone();
        move |_: MouseEvent| {
            if let Some(callback) = &on_dismiss {
                callback.run(());
            }
        }
    };

    let computed_class = Signal::derive(move || {
        let variant_class = match variant.get().as_deref() {
            Some("default") => "bg-background text-foreground",
            Some("destructive") => "border-destructive/50 text-destructive dark:border-destructive",
            Some("success") => "border-green-500/50 text-green-600 dark:text-green-400",
            Some("warning") => "border-yellow-500/50 text-yellow-600 dark:text-yellow-400",
            _ => "bg-background text-foreground"
        };
        
        format!(
            "{} {} {}",
            ${COMPONENT_NAME_CAMEL^^}_CLASS,
            variant_class,
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
            {move || if dismissible.get() {
                view! {
                    <button
                        class="absolute right-2 top-2 rounded-md p-1 opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
                        on:click=handle_dismiss
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="16"
                            height="16"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        >
                            <path d="M18 6 6 18"/>
                            <path d="m6 6 12 12"/>
                        </svg>
                    </button>
                }
            } else {
                view! { <div></div> }
            }}
        </div>
    }
}
EOF
}

# Generate default.rs
generate_component_by_type "$COMPONENT_TYPE" "$COMPONENT_DIR/src/default.rs"

# Generate new_york.rs (same template for now, can be customized later)
generate_component_by_type "$COMPONENT_TYPE" "$COMPONENT_DIR/src/new_york.rs"

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

    #[test]
    fn test_${COMPONENT_NAME_CAMEL}_with_children() {
        let (cx, _) = leptos_testing::create_test_runtime();
        
        let component = view! { cx, 
            <$COMPONENT_NAME_PASCAL>
                "Test content"
            </$COMPONENT_NAME_PASCAL>
        };
        
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

EOF

# Add component-specific props to README based on type
case $COMPONENT_TYPE in
    "form")
        cat >> "$COMPONENT_DIR/README.md" << EOF
| value | MaybeProp<String> | None | The value of the input |
| on_change | Option<Callback<String>> | None | Callback when value changes |
| placeholder | MaybeProp<String> | None | Placeholder text |
| disabled | Signal<bool> | false | Whether the input is disabled |
| input_type | MaybeProp<String> | "text" | Input type (text, email, password, etc.) |

EOF
        ;;
    "interactive")
        cat >> "$COMPONENT_DIR/README.md" << EOF
| on_click | Option<Callback<MouseEvent>> | None | Callback when clicked |
| disabled | Signal<bool> | false | Whether the component is disabled |
| variant | MaybeProp<String> | "default" | Component variant (default, destructive, outline, etc.) |

EOF
        ;;
    "layout")
        cat >> "$COMPONENT_DIR/README.md" << EOF
| header | Option<Children> | None | Header content |
| footer | Option<Children> | None | Footer content |

EOF
        ;;
    "feedback")
        cat >> "$COMPONENT_DIR/README.md" << EOF
| variant | MaybeProp<String> | "default" | Component variant (default, destructive, success, warning) |
| dismissible | Signal<bool> | false | Whether the component can be dismissed |
| on_dismiss | Option<Callback<()>> | None | Callback when dismissed |

EOF
        ;;
esac

cat >> "$COMPONENT_DIR/README.md" << EOF
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
print_status "Component type: $COMPONENT_TYPE"
print_status "Generated with appropriate template and props"
print_status ""
print_status "Next steps:"
print_status "1. Update the component classes in $COMPONENT_DIR/src/default.rs"
print_status "2. Update the component classes in $COMPONENT_DIR/src/new_york.rs"
print_status "3. Customize the component logic and add specific props"
print_status "4. Run 'cargo check --workspace' to verify compilation"
print_status "5. Add specific tests in $COMPONENT_DIR/src/tests.rs"
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
