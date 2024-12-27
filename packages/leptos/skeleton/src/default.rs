use tailwind_fuse::*;
use leptos::prelude::*;
use leptos_style::Style;
use leptos_node_ref::AnyNodeRef;

#[component]
pub fn Skeleton(
    // Global attributes
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref

            class=move || tw_merge!("animate-pulse rounded-md bg-muted", class.get())
            id=id.get()
            style=style
        />
    }
}
