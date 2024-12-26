use leptos_node_ref::AnyNodeRef;
use lucide_leptos::{ChevronLeft, ChevronRight, Ellipsis};
use shadcn_ui_leptos_button::default::{ButtonClass, ButtonSize, ButtonVariant};
use shadcn_ui_leptos_utils::handlers::*;
use tailwind_fuse::*;
use leptos::{ev::MouseEvent, prelude::*};
use leptos_style::Style;


#[component]
pub fn Pagination(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: Children,
) -> impl IntoView {
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


#[component]
pub fn PaginationLink(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    
    #[prop(into, optional)] is_active: Signal<bool>,
    #[prop(into, optional)] size: Signal<ButtonSize>,

    #[prop(into, optional)] aria_label: MaybeProp<String>,

    #[prop(into, optional)] href: MaybeProp<String>,
    #[prop(into)] on_click: Option<Callback<MouseEvent>>,

    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = Memo::new(move |_| {
        ButtonClass {
            variant: if is_active.get() {
                ButtonVariant::Outline
            } else {
                ButtonVariant::Ghost
            },
            size:size.get(),
        }
        .with_class(class.get().unwrap_or_default())
    });
    
    view! {
        <a
            node_ref=node_ref

            aria-current=is_active.get().then_some("page")
            aria-label=aria_label.get()
            class=class
            id=id.get()
            style=style

            href=href.get()

            on:click=generate_handler(on_click)
        >
            {
                if let Some(children) = children {
                    children();
                }
            }
        </a>
    }
}


#[component]
pub fn PaginationPrevious(
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] href: MaybeProp<String>,
    #[prop(into)] on_click: Option<Callback<MouseEvent>>,

    #[prop(into, optional)] is_active: Signal<bool>,
) -> impl IntoView {
    view! {
        <PaginationLink
            is_active=is_active
            size=ButtonSize::Default

            aria_label="Go to previous page"
            class= tw_merge!("gap-1 pl-2.5", class.get())
            id=id.get()
            style=style

            href=href

            on_click=on_click

            node_ref=node_ref
        >
            <ChevronLeft/>
            <span>"Previous"</span>
        </PaginationLink>
    }
}


#[component]
pub fn PaginationNext(
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] href: MaybeProp<String>,
    #[prop(into)] on_click: Option<Callback<MouseEvent>>,

    #[prop(into, optional)] is_active: Signal<bool>,
) -> impl IntoView {
    view! {
        <PaginationLink
            is_active=is_active
            size={ButtonSize::Default}

            aria_label="Go to next page"
            class=tw_merge!("gap-1 pr-2.5", class.get())
            id=id.get()
            style=style

            href=href

            on_click=on_click

            node_ref= node_ref
            >
            <span>"Next"</span>
            <ChevronRight />
        </PaginationLink>
    }
}
 

#[component]
pub fn PaginationEllipsis(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] node_ref: AnyNodeRef
) -> impl IntoView {
    view! {
        <span
            node_ref=node_ref

            aria-hidden="true"
            class=move || tw_merge!("flex h-9 w-9 items-center justify-center", class.get())
            id=id.get()
            style=style
        >
            <Ellipsis />
            <span class="sr-only">"More pages"</span>
        </span>
    }
}
