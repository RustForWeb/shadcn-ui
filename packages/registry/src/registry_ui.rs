use std::{collections::HashMap, sync::LazyLock};

use crate::schema::{FrameworkName, Registry, RegistryEntry, RegistryItemType, RegistryItemFile};

/// Creates a complete registry entry for a UI component
fn create_ui_component(
    name: &str,
    description: &str,
    category: &str,
    dependencies: Vec<&str>,
) -> RegistryEntry {
    RegistryEntry {
        name: name.into(),
        r#type: RegistryItemType::Ui,
        description: Some(description.into()),
        dependencies: Some(dependencies.into_iter().map(|s| s.into()).collect()),
        dev_dependencies: Some(vec!["wasm-bindgen-test".into()]),
        registry_dependencies: None,
        files: Some(vec![
            RegistryItemFile {
                path: format!("ui/{}.rs", name),
                content: None,
                r#type: RegistryItemType::Ui,
                target: None,
            }
        ]),
        tailwind: None,
        css_vars: None,
        source: Some(format!("https://ui.shadcn.com/docs/components/{}", name)),
        category: Some(category.into()),
        subcategory: None,
        chunks: None,
        docs: Some(format!("https://shadcn-ui.rustforweb.org/components/{}.html", name)),
    }
}

/// Complete component registry for all shadcn/ui components
fn create_complete_registry() -> Registry {
    vec![
        // Form & Input Components (12 total)
        create_ui_component("button", "Displays a button or a component that looks like a button.", "forms", vec!["tailwind_fuse"]),
        create_ui_component("checkbox", "A control that allows the user to toggle between checked and not checked.", "forms", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("radio-group", "A set of checkable buttons—known as radio buttons—where no more than one of the buttons can be checked at a time.", "forms", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("select", "Displays a list of options for the user to pick from—triggered by a button.", "forms", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("combobox", "Autocomplete input and command palette with a list of suggestions.", "forms", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("form", "Building blocks for creating accessible forms.", "forms", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("input", "Displays a form input field or a component that looks like an input field.", "forms", vec!["tailwind_fuse"]),
        create_ui_component("label", "Renders an accessible label associated with controls.", "forms", vec!["tailwind_fuse"]),
        create_ui_component("textarea", "Displays a form textarea or a component that looks like a textarea.", "forms", vec!["tailwind_fuse"]),
        create_ui_component("slider", "An input where the user selects a value from within a given range.", "forms", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("switch", "A control that allows the user to toggle between checked and not checked.", "forms", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("toggle", "A two-state button that can be either on or off.", "forms", vec!["tailwind_fuse", "web-sys"]),

        // Navigation Components (7 total)
        create_ui_component("navigation-menu", "A collection of links for navigating websites.", "navigation", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("menubar", "A visually persistent menu common in desktop applications.", "navigation", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("tabs", "A set of layered sections of content—known as tab panels—that are displayed one at a time.", "navigation", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("breadcrumb", "Displays the path to the current resource using a hierarchy of links.", "navigation", vec!["tailwind_fuse"]),
        create_ui_component("pagination", "Pagination with page navigation, next and previous links.", "navigation", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("command", "Fast, composable, unstyled command menu for React.", "navigation", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("context-menu", "Displays a menu to the user — such as a set of actions or functions — triggered by a button.", "navigation", vec!["tailwind_fuse", "web-sys"]),

        // Overlay Components (8 total)
        create_ui_component("dialog", "A window overlaid on either the primary window or another dialog window.", "overlay", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("alert-dialog", "A modal dialog that interrupts the user with important content and expects a response.", "overlay", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("sheet", "Extends the Dialog component to display content that complements the main content of the screen.", "overlay", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("drawer", "A panel that slides out from the edge of the screen.", "overlay", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("dropdown-menu", "Displays a menu to the user — such as a set of actions or functions — triggered by a button.", "overlay", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("popover", "Displays rich content in a portal, triggered by a button.", "overlay", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("tooltip", "A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it.", "overlay", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("toast", "A succinct message that is displayed temporarily.", "overlay", vec!["tailwind_fuse", "web-sys"]),

        // Layout Components (7 total)
        create_ui_component("accordion", "A vertically stacked set of interactive headings that each reveal a section of content.", "layout", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("collapsible", "An interactive component which can be expanded/collapsed.", "layout", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("resizable", "Accessible resizable panel groups and layouts with keyboard support.", "layout", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("scroll-area", "Augments native scroll functionality for custom, cross-browser styling.", "layout", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("separator", "Visually or semantically separates content.", "layout", vec!["tailwind_fuse"]),
        create_ui_component("sidebar", "Composable, themeable, multi-level sidebar navigation component.", "layout", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("aspect-ratio", "Displays content within a desired ratio.", "layout", vec!["tailwind_fuse"]),

        // Display Components (8 total)
        create_ui_component("alert", "Displays a callout for user attention.", "display", vec!["tailwind_fuse"]),
        create_ui_component("avatar", "An image element with a fallback for representing the user.", "display", vec!["tailwind_fuse"]),
        create_ui_component("badge", "Displays a badge or a component that looks like a badge.", "display", vec!["tailwind_fuse"]),
        create_ui_component("card", "Displays a card with header, content, and footer.", "display", vec!["tailwind_fuse"]),
        create_ui_component("calendar", "A date field component that allows users to enter and edit date.", "display", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("progress", "Displays an indicator showing the completion progress of a task.", "display", vec!["tailwind_fuse"]),
        create_ui_component("skeleton", "Use to show a placeholder while content is loading.", "display", vec!["tailwind_fuse"]),
        create_ui_component("table", "A responsive table component.", "display", vec!["tailwind_fuse"]),

        // Advanced Components (9 total)
        create_ui_component("carousel", "A carousel with motion and swipe built using Embla.", "advanced", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("chart", "Recharts components built using Recharts and designed to work seamlessly with shadcn/ui.", "advanced", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("data-table", "Powerful table and datagrids built using TanStack Table.", "advanced", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("date-picker", "A date picker component with range and multiple selection.", "advanced", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("hover-card", "For sighted users to preview content available behind a link.", "advanced", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("input-otp", "Accessible one-time password component with copy paste functionality.", "advanced", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("sonner", "An opinionated toast component for React.", "advanced", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("toggle-group", "A set of two-state buttons that can be toggled on or off.", "advanced", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("typography", "Styles for headings, paragraphs, lists...etc", "advanced", vec!["tailwind_fuse"]),
    ]
}

pub static UI: LazyLock<HashMap<FrameworkName, Registry>> = LazyLock::new(|| {
    let complete_registry = create_complete_registry();
    
    // Create framework-specific registries with existing components
    let leptos_registry = vec![
        // Existing Leptos components
        create_ui_component("alert", "Displays a callout for user attention.", "display", vec!["tailwind_fuse"]),
        create_ui_component("badge", "Displays a badge or a component that looks like a badge.", "display", vec!["tailwind_fuse"]),
        create_ui_component("button", "Displays a button or a component that looks like a button.", "forms", vec!["tailwind_fuse"]),
        create_ui_component("card", "Displays a card with header, content, and footer.", "display", vec!["tailwind_fuse"]),
        create_ui_component("tooltip", "A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it.", "overlay", vec!["tailwind_fuse", "web-sys"]),
    ];
    
    let yew_registry = vec![
        // Existing Yew components
        create_ui_component("alert", "Displays a callout for user attention.", "display", vec!["tailwind_fuse"]),
        create_ui_component("aspect-ratio", "Displays content within a desired ratio.", "layout", vec!["tailwind_fuse"]),
        create_ui_component("avatar", "An image element with a fallback for representing the user.", "display", vec!["tailwind_fuse"]),
        create_ui_component("badge", "Displays a badge or a component that looks like a badge.", "display", vec!["tailwind_fuse"]),
        create_ui_component("breadcrumb", "Displays the path to the current resource using a hierarchy of links.", "navigation", vec!["tailwind_fuse"]),
        create_ui_component("button", "Displays a button or a component that looks like a button.", "forms", vec!["tailwind_fuse"]),
        create_ui_component("card", "Displays a card with header, content, and footer.", "display", vec!["tailwind_fuse"]),
        create_ui_component("input", "Displays a form input field or a component that looks like an input field.", "forms", vec!["tailwind_fuse"]),
        create_ui_component("label", "Renders an accessible label associated with controls.", "forms", vec!["tailwind_fuse"]),
        create_ui_component("pagination", "Pagination with page navigation, next and previous links.", "navigation", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("separator", "Visually or semantically separates content.", "layout", vec!["tailwind_fuse"]),
        create_ui_component("skeleton", "Use to show a placeholder while content is loading.", "display", vec!["tailwind_fuse"]),
        create_ui_component("switch", "A control that allows the user to toggle between checked and not checked.", "forms", vec!["tailwind_fuse", "web-sys"]),
        create_ui_component("table", "A responsive table component.", "display", vec!["tailwind_fuse"]),
        create_ui_component("textarea", "Displays a form textarea or a component that looks like a textarea.", "forms", vec!["tailwind_fuse"]),
        create_ui_component("tooltip", "A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it.", "overlay", vec!["tailwind_fuse", "web-sys"]),
    ];
    
    HashMap::from([
        (FrameworkName::Dioxus, complete_registry.clone()),
        (FrameworkName::Leptos, leptos_registry),
        (FrameworkName::Yew, yew_registry),
    ])
});
