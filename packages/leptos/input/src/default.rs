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

    #[prop(into, optional)] oninput: MaybeCallback<Event>,
    #[prop(into, optional)] onchange: MaybeCallback<Event>,
    #[prop(into, optional)] onblur: MaybeCallback<FocusEvent>,
    #[prop(into, optional)] onfocus: MaybeCallback<FocusEvent>,

    #[prop(into, optional)] value: Signal<String>,

    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] readonly: Signal<bool>,
    #[prop(into, optional)] is_error: Signal<bool>,
) -> impl IntoView {
    view! {
        <input
            // Core Attributes
            node_ref=node_ref
            class=move || tw_merge!(
                "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
                tw_join!(class.get(), (is_error).get().then_some("border-destructive"))
            )
            id=move || id.get()
            style=style

            // Input Attributes
            type=move || r#type.get().as_str()
            value=move || value.get()
            placeholder=move || placeholder.get()

            // Event Handlers
            on:blur= Handler::from(onblur)
            on:focus = Handler::from(onfocus)
            on:input= Handler::from(oninput)
            on:change = Handler::from(onchange)

            disabled = disabled
            readonly = readonly
        />
    }
}
