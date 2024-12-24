use lucide_leptos::{ChevronLeft, ChevronRight, Ellipsis};
use shadcn_ui_leptos_button::default::{ButtonClass, ButtonSize, ButtonVariant};
use tailwind_fuse::*;
use leptos::prelude::*;
use leptos_style::Style;


#[component]
pub fn Pagination(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: Children,
) -> impl Intoview {
    view! {
        <nav
            node_ref=node_ref

            aria-label="pagination"
            role="navigation"
            class=move || tw_merge!("mx-auto flex w-full justify-center", class.get())
            id=id.get()
            style=style
        >
            {children()}
        </nav>
    }
}


#[component]
pub fn PaginationContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: Children,
) -> impl IntoView {
    view! {
        <ul
            node_ref=node_ref

            class=move || tw_merge!("flex flex-row items-center gap-1", class.get())
            id=id.get()
            style=style
        >
            {children()}
        </ul>
    }
}


#[component]
pub fn PaginationItem(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: Children,
) -> impl IntoView {
    view! {
        <li
            node_ref=node_ref

            class=move || tw_merge!("", class.get())
            id=id.get()
            style=style
        >
            {children()}
        </li>
    }
}

#[derive(PartialEq, Properties)]
pub struct PaginationLinkProps {
    #[prop_or(false)]
    pub is_active: bool,
    #[prop_or(ButtonSize::Icon)]
    pub size: ButtonSize,

    // Global attributes
    #[prop_or_default]
    pub aria_label: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    // Attributes from `a`
    #[prop_or_default]
    pub href: Option<String>,

    // Event handler attributes
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub children: Html,
}

#[component]
pub fn PaginationLink(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] children: Children,
) -> impl IntoView {
    let class = use_memo(
        (props.is_active, props.size, props.class.clone()),
        |(is_active, size, class)| {
            ButtonClass {
                variant: if *is_active {
                    ButtonVariant::Outline
                } else {
                    ButtonVariant::Ghost
                },
                size: *size,
            }
            .with_class(class.clone().unwrap_or_default())
        },
    );

    html! {
        <a
            ref={props.node_ref.clone()}

            aria-current={props.is_active.then_some("page")}
            aria-label={props.aria_label.clone()}
            class={(*class).clone()}
            id={props.id.clone()}
            style={props.style.clone()}

            href={props.href.clone()}

            onclick={props.on_click.clone()}
        >
            {props.children.clone()}
        </a>
    }
}

#[derive(PartialEq, Properties)]
pub struct PaginationPreviousProps {
    #[prop_or(false)]
    pub is_active: bool,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    // Attributes from `a`
    #[prop_or_default]
    pub href: Option<String>,

    // Event handler attributes
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,

    #[prop_or_default]
    pub node_ref: NodeRef,
}

#[function_component]
pub fn PaginationPrevious(props: &PaginationPreviousProps) -> Html {
    html! {
        <PaginationLink
            is_active={props.is_active}
            size={ButtonSize::Default}

            aria_label={"Go to previous page"}
            class={tw_merge!("gap-1 pl-2.5", &props.class)}
            id={props.id.clone()}
            style={props.style.clone()}

            href={props.href.clone()}

            on_click={props.on_click.clone()}

            node_ref={props.node_ref.clone()}
        >
            <ChevronLeft class="h-4 w-4" />
            <span>{"Previous"}</span>
        </PaginationLink>
    }
}

#[derive(PartialEq, Properties)]
pub struct PaginationNextProps {
    #[prop_or(false)]
    pub is_active: bool,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    // Attributes from `a`
    #[prop_or_default]
    pub href: Option<String>,

    // Event handler attributes
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,

    #[prop_or_default]
    pub node_ref: NodeRef,
}

#[function_component]
pub fn PaginationNext(props: &PaginationNextProps) -> Html {
    html! {
        <PaginationLink
            is_active={props.is_active}
            size={ButtonSize::Default}

            aria_label={"Go to next page"}
            class={tw_merge!("gap-1 pr-2.5", &props.class)}
            id={props.id.clone()}
            style={props.style.clone()}

            href={props.href.clone()}

            on_click={props.on_click.clone()}

            node_ref={props.node_ref.clone()}
            >
            <span>{"Next"}</span>
            <ChevronRight class="h-4 w-4" />
        </PaginationLink>
    }
}

#[derive(PartialEq, Properties)]
pub struct PaginationEllipsisProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
}

#[function_component]
pub fn PaginationEllipsis(props: &PaginationEllipsisProps) -> Html {
    html! {
        <span
            ref={props.node_ref.clone()}

            aria-hidden="true"
            class={tw_merge!("flex h-9 w-9 items-center justify-center", &props.class)}
            id={props.id.clone()}
            style={props.style.clone()}
        >
            <Ellipsis class="h-4 w-4" />
            <span class="sr-only">{"More pages"}</span>
        </span>
    }
}
