use leptos_node_ref::AnyNodeRef;
use tailwind_fuse::*;
use leptos::{ev::{Event, FocusEvent}, prelude::*};
use leptos_style::Style;
use shadcn_ui_leptos_utils::handlers::*;

#[component]
pub fn Textarea(
    // Global attributes
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    // Attributes from `textarea`
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] value: Signal<String>,
    #[prop(into, optional)] readonly: MaybeProp<bool>,
    #[prop(into, optional)] is_error: Signal<bool>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,


    // Event handler attributes
    #[prop(optional)] onblur: MaybeCallback<FocusEvent>,
    #[prop(optional)] onchange: MaybeCallback<Event>,
    #[prop(optional)] onfocus: MaybeCallback<FocusEvent>,
    #[prop(optional)] oninput: MaybeCallback<Event>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    view! {
        <textarea
            node_ref=node_ref

            class=move || tw_merge!(
                "flex min-h-[60px] w-full rounded-md border border-input bg-transparent px-3 py-2 text-base shadow-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
                 tw_join!(class.get(), (is_error).get().then_some("border-destructive"))
            )
            id=move || id.get()
            style=style

            placeholder=move || placeholder.get()
            prop:value=value

            on:blur=Handler::from(onblur)
            on:change=Handler::from(onchange)
            on:focus=Handler::from(onfocus)
            on:input=Handler::from(oninput)

            disabled=move || disabled.get()
            readonly=move || readonly.get()
        />
    }
}
