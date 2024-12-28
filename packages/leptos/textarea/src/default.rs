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
    #[prop(optional)] on_blur: MaybeCallback<FocusEvent>,
    #[prop(optional)] on_change: MaybeCallback<Event>,
    #[prop(optional)] on_focus: MaybeCallback<FocusEvent>,
    #[prop(optional)] on_input: MaybeCallback<Event>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    view! {
        <textarea
            node_ref=node_ref

            class=move || tw_merge!(
                "flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-base ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
                 tw_join!(class.get(), (is_error).get().then_some("border-destructive"))
            )
            id=move || id.get()
            style=style

            placeholder=move || placeholder.get()
            prop:value=value

            on:blur=Handler::from(on_blur)
            on:change=Handler::from(on_change)
            on:focus=Handler::from(on_focus)
            on:input=Handler::from(on_input)

            disabled=move || disabled.get()
            readonly=move || readonly.get()
            
        />
    }
}
