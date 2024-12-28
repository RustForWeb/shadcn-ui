use leptos_node_ref::AnyNodeRef;
use tailwind_fuse::*;
use leptos::{attr::autocapitalize, prelude::*};
use leptos_style::Style;


#[component]
pub fn Textarea(
    // Global attributes
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<Style>,

    // Attributes from `textarea`
    #[prop(into, optional)] autocomplete: MaybeProp<String>,
    #[prop(into, optional)] autocapitalize: MaybeProp<String>,
    #[prop(into, optional)] autocorrect: MaybeProp<String>,
    #[prop(into, optional)] autofocus: MaybeProp<bool>,
    #[prop(into, optional)] spellcheck: MaybeProp<bool>,
    #[prop(into, optional)] cols: MaybeProp<String>,
    #[prop(into, optional)] dirname: MaybeProp<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] form: MaybeProp<String>,
    #[prop(into, optional)] maxlength: MaybeProp<String>,
    #[prop(into, optional)] minlength: MaybeProp<String>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] pattern: MaybeProp<String>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] rows: MaybeProp<String>,
    #[prop(into, optional)] value: MaybeProp<String>,


    // Event handler attributes
    #[prop(optional)] on_blur: MaybeCallback<FocusEvent>,
    #[prop(optional)] on_change: MaybeCallback<Event>,
    #[prop(optional)] on_focus: MaybeCallback<FocusEvent>,
    #[prop(optional)] on_input: MaybeCallback<InputEvent>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    view! {
        <textarea
            node_ref=node_ref

            class=move || tw_merge!(
                "flex min-h-[60px] w-full rounded-md border border-input bg-transparent px-3 py-2 text-base shadow-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
                class.get()
            )
            id=move || id.get()
            style=style

            autocapitalize=move ||autocapitalize.get()
            autocorrect=move ||autocorrect.get()
            autofocus=move ||autofocus.get()
            spellcheck=move ||spellcheck.get()
            
            
            autocomplete=move || autocomplete.get()
            cols=move || cols.get()
            dirname=move || dirname.get()
            disabled=move || disabled.get()
            form=move || form.get()
            maxlength=move || maxlength.get()
            minlength=move || minlength.get()
            name=move || name.get()
            pattern=move || pattern.get()
            placeholder=move || placeholder.get()
            readonly=move || false
            required=move || false
            rows=move || rows.get()
            value=move || value.get()

            onblur=Handler::from(on_blur)
            onchange=Handler::from(on_change)
            onfocus=Handler::from(on_focus)
            oninput=Handler::from(on_input)
        />
    }
}
