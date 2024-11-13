use radix_yew_icons::{ChevronRightIcon, DotsHorizontalIcon};
use tailwind_fuse::*;
use yew::{prelude::*, virtual_dom::VNode};
use yew_struct_component::{struct_component, StructComponent};

#[derive(PartialEq, Properties)]
pub struct BreadcrumbProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Breadcrumb(props: &BreadcrumbProps) -> Html {
    html! {
        <nav
            ref={props.node_ref.clone()}

            aria-label="breadcrumb"
            class={props.class.clone()}
            id={props.id.clone()}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </nav>
    }
}

#[derive(PartialEq, Properties)]
pub struct BreadcrumbListProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn BreadcrumbList(props: &BreadcrumbListProps) -> Html {
    html! {
        <ol
            ref={props.node_ref.clone()}

            class={tw_merge!(
                "flex flex-wrap items-center gap-1.5 break-words text-sm text-muted-foreground sm:gap-2.5",
                &props.class
            )}
            id={props.id.clone()}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </ol>
    }
}

#[derive(PartialEq, Properties)]
pub struct BreadcrumbItemProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn BreadcrumbItem(props: &BreadcrumbItemProps) -> Html {
    html! {
        <li
            ref={props.node_ref.clone()}

            class={tw_merge!("inline-flex items-center gap-1.5", &props.class)}
            id={props.id.clone()}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </li>
    }
}

#[derive(PartialEq, Properties)]
pub struct BreadcrumbLinkProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,

    // Attributes from `a`
    #[prop_or_default]
    pub href: Option<String>,

    // Event handler attributes
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub as_child: Option<Callback<BreadcrumbLinkChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "a")]
pub struct BreadcrumbLinkChildProps {
    pub node_ref: NodeRef,

    // Global attributes
    pub class: String,
    pub id: Option<String>,
    pub style: Option<String>,

    // Attributes from `a`
    pub href: Option<String>,

    // Event handler attributes
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn BreadcrumbLink(props: &BreadcrumbLinkProps) -> Html {
    let child_props = BreadcrumbLinkChildProps {
        node_ref: props.node_ref.clone(),

        // Global attributes
        class: tw_merge!("transition-colors hover:text-foreground", &props.class),
        id: props.id.clone(),
        style: props.style.clone(),

        // Attributes from `a`
        href: props.href.clone(),

        // Event handler attributes
        onclick: props.on_click.clone(),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}

#[derive(PartialEq, Properties)]
pub struct BreadcrumbPageProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn BreadcrumbPage(props: &BreadcrumbPageProps) -> Html {
    html! {
        <span
            ref={props.node_ref.clone()}

            aria-disabled="true"
            aria-current="page"
            class={tw_merge!("font-normal text-foreground", &props.class)}
            id={props.id.clone()}
            role="link"
            style={props.style.clone()}
        >
            {props.children.clone()}
        </span>
    }
}

#[derive(PartialEq, Properties)]
pub struct BreadcrumbSeparatorProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn BreadcrumbSeparator(props: &BreadcrumbSeparatorProps) -> Html {
    html! {
        <li
            ref={props.node_ref.clone()}

            aria-hidden="true"
            class={tw_merge!("[&>svg]:w-3.5 [&>svg]:h-3.5", &props.class)}
            id={props.id.clone()}
            role="presentation"
            style={props.style.clone()}
        >
            if matches!(&props.children, VNode::VList(list) if list.is_empty()) {
                <ChevronRightIcon />
            } else {
                {props.children.clone()}
            }
        </li>
    }
}

#[derive(PartialEq, Properties)]
pub struct BreadcrumbEllipsisProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub node_ref: NodeRef,
}

#[function_component]
pub fn BreadcrumbEllipsis(props: &BreadcrumbEllipsisProps) -> Html {
    html! {
        <span
            ref={props.node_ref.clone()}

            aria-hidden="true"
            class={tw_merge!("flex h-9 w-9 items-center justify-center", &props.class)}
            id={props.id.clone()}
            role="presentation"
            style={props.style.clone()}
        >
            <DotsHorizontalIcon class="h-4 w-4" />
            <span class="sr-only">{"More"}</span>
        </span>
    }
}
