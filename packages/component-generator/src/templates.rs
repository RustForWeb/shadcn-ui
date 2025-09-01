//! Template definitions for component generation.

pub const LEPTOS_COMPONENT_TEMPLATE: &str = r#"
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use leptos_struct_component::{StructComponent, component};
use leptos_style::Style;
use tailwind_fuse::{TwClass, TwVariant};

#[derive(TwClass)]
#[tw(class = "{{base_classes}}")]
pub struct {{component_name}}Class;

#[component]
pub fn {{component_name}}(
    {{#each props}}
    #[prop({{#if optional}}optional{{else}}default{{/if}})]
    {{name}}: {{prop_type}},
    {{/each}}
    
    // Global attributes
    #[prop(optional)]
    class: Option<String>,
    #[prop(optional)]
    id: Option<String>,
    #[prop(default)]
    style: Style,
    
    #[prop(optional)]
    node_ref: Option<AnyNodeRef>,
    children: Children,
) -> impl IntoView {
    let node_ref = node_ref.unwrap_or_default();
    
    let computed_class = move || {
        {{component_name}}Class.with_class(class.clone().unwrap_or_default())
    };
    
    view! {
        <{{html_tag}}
            node_ref=node_ref
            class=computed_class()
            id=id.clone()
            style=style.clone()
        >
            {children()}
        </{{html_tag}}>
    }
}
"#;

pub const YEW_COMPONENT_TEMPLATE: &str = r#"
use tailwind_fuse::{TwClass, TwVariant};
use yew::prelude::*;
use yew_struct_component::{StructComponent, struct_component};
use yew_style::Style;

#[derive(TwClass)]
#[tw(class = "{{base_classes}}")]
pub struct {{component_name}}Class;

#[derive(PartialEq, Properties)]
pub struct {{component_name}}Props {
    {{#each props}}
    #[prop_or_default]
    pub {{name}}: {{prop_type}},
    {{/each}}

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn {{component_name}}(props: &{{component_name}}Props) -> Html {
    let class = use_memo(
        props.class.clone(),
        |class| {
            {{component_name}}Class
                .with_class(class.clone().unwrap_or_default())
        },
    );

    html! {
        <{{html_tag}}
            ref={props.node_ref.clone()}
            class={(*class).clone()}
            id={props.id.clone()}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </{{html_tag}}>
    }
}
"#;

pub const LIB_RS_TEMPLATE: &str = r#"
//! {{framework}} port of [shadcn/ui {{component_title}}](https://ui.shadcn.com/docs/components/{{component_name}}).
//!
//! {{description}}
//!
//! See [the Rust shadcn/ui book](https://shadcn-ui.rustforweb.org/components/{{component_name}}.html) for more documentation.

pub mod default;
pub mod new_york;

// Re-export the components for easy access
pub use default::*;

#[cfg(feature = "new_york")]
pub use new_york as {{component_name}};
"#;