use leptos::{ev::Event, ev::FocusEvent, prelude::*};
use leptos_node_ref::AnyNodeRef;
use leptos_style::Style;
use shadcn_ui_leptos_utils::handlers::*;
use tailwind_fuse::*;

#[derive(Default, Clone)]
pub enum InputType {
    #[default]
    Text,
    Password,
    Search,
    Tel,
    Url,
    Email,
    Time,
    Date,
    DatetimeLocal,
    Month,
    Week,
    File,
}

impl InputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Password => "password",
            Self::Search => "search",
            Self::Tel => "tel",
            Self::Url => "url",
            Self::Email => "email",
            Self::Time => "time",
            Self::Date => "date",
            Self::DatetimeLocal => "datetime-local",
            Self::Month => "month",
            Self::Week => "week",
            Self::File => "file",
        }
    }
}

#[component]
pub fn Input(
    // Node reference
    #[prop(into, optional)] node_ref: AnyNodeRef,

    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] r#type: Signal<InputType>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,

    #[prop(into, optional)] on_input: Option<Callback<Event>>,
    #[prop(into, optional)] on_change: Option<Callback<Event>>,
    #[prop(into, optional)] on_blur: Option<Callback<FocusEvent>>,
    #[prop(into, optional)] on_focus: Option<Callback<FocusEvent>>,

    #[prop(into, optional)] value: Signal<String>,

    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] readonly: Signal<bool>,
) -> impl IntoView {
    view! {
        <input
            // Core Attributes
            node_ref=node_ref
            class=move || tw_merge!(
                "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
                class.get()
            )
            id=move || id.get()
            style=style

            // Input Attributes
            type=move || r#type.get().as_str()
            value=move || value.get()
            placeholder=move || placeholder.get()

            // Event Handlers
            on:blur= generate_handler(on_blur)
            on:focus = generate_handler(on_focus)
            on:input= generate_handler(on_input)
            on:change = generate_handler(on_change)

            disabled = disabled
            readonly = readonly
        />
    }
}
