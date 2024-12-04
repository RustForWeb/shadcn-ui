use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use leptos_style::Style;
use tailwind_fuse::*;

#[component]
pub fn Card(
    // Global attributes
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] id: String,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=tw_merge!("rounded-xl border bg-card text-card-foreground shadow", class)
            id=id
            style=style
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CardHeader(
    // Global attributes
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] id: String,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=tw_merge!("flex flex-col space-y-1.5 p-6", class)
            id=id
            style=style
        >{children()}
        </div>
    }
}

#[component]
pub fn CardTitle(
    // Global attributes
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] id: String,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=tw_merge!("font-semibold leading-none tracking-tight", class)
            id=id
            style=style
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CardDescription(
    // Global attributes
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] id: String,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=tw_merge!("text-sm text-muted-foreground", class)
            id=id
            style=style
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CardContent(
    // Global attributes
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] id: String,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=tw_merge!("p-6 pt-0", class)
            id=id
            style=style
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CardFooter(
    // Global attributes
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] id: String,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=tw_merge!("flex items-center p-6 pt-0", class)
            id=id
            style=style
        >
            {children()}
        </div>
    }
}
