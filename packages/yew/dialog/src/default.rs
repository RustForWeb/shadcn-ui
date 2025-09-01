use yew::prelude::*;
use yew_style::Style;
use web_sys::{Element, KeyboardEvent, MouseEvent};

// Static classes for better compilation compatibility
const DIALOG_CLASS: &str = "fixed inset-0 z-50 bg-background/80 backdrop-blur-sm data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0";
const DIALOG_CONTENT_CLASS: &str = "fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border bg-background p-6 shadow-lg duration-200 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[state=closed]:slide-out-to-left-1/2 data-[state=closed]:slide-out-to-top-[48%] data-[state=open]:slide-in-from-left-1/2 data-[state=open]:slide-in-from-top-[48%] sm:rounded-lg";
const DIALOG_HEADER_CLASS: &str = "flex flex-col space-y-1.5 text-center sm:text-left";
const DIALOG_FOOTER_CLASS: &str = "flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2";
const DIALOG_TITLE_CLASS: &str = "text-lg font-semibold leading-none tracking-tight";
const DIALOG_DESCRIPTION_CLASS: &str = "text-sm text-muted-foreground";

#[derive(Clone, PartialEq, Properties)]
pub struct DialogProps {
    /// Whether the dialog is open
    #[prop_or(false)]
    pub open: bool,
    
    /// Callback when open state changes
    #[prop_or_default]
    pub on_open_change: Option<Callback<bool>>,
    
    /// Whether the dialog is disabled
    #[prop_or(false)]
    pub disabled: bool,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub children: Children,
}

#[derive(Clone, PartialEq, Properties)]
pub struct DialogTriggerProps {
    /// Whether the trigger is disabled
    #[prop_or(false)]
    pub disabled: bool,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub children: Children,
}

#[derive(Clone, PartialEq, Properties)]
pub struct DialogContentProps {
    /// Whether the content is disabled
    #[prop_or(false)]
    pub disabled: bool,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub children: Children,
}

#[derive(Clone, PartialEq, Properties)]
pub struct DialogHeaderProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub children: Children,
}

#[derive(Clone, PartialEq, Properties)]
pub struct DialogFooterProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub children: Children,
}

#[derive(Clone, PartialEq, Properties)]
pub struct DialogTitleProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub children: Children,
}

#[derive(Clone, PartialEq, Properties)]
pub struct DialogDescriptionProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub children: Children,
}

#[derive(Clone, PartialEq)]
struct DialogContext {
    open: UseStateHandle<bool>,
    on_open_change: Option<Callback<bool>>,
    disabled: bool,
}

#[function_component]
pub fn Dialog(props: &DialogProps) -> Html {
    let open = use_state(|| props.open);
    let on_open_change = props.on_open_change.clone();
    let disabled = props.disabled;
    
    let context = DialogContext {
        open: open.clone(),
        on_open_change,
        disabled,
    };
    
    let computed_class = format!(
        "{} {}",
        DIALOG_CLASS,
        props.class.as_deref().unwrap_or("")
    );
    
    html! {
        <ContextProvider<DialogContext> context={context}>
            <div
                class={computed_class}
                id={props.id.clone()}
                style={props.style.clone()}
                data-state={if *open { "open" } else { "closed" }}
                role="dialog"
                aria-modal="true"
            >
                {props.children.clone()}
            </div>
        </ContextProvider<DialogContext>>
    }
}

#[function_component]
pub fn DialogTrigger(props: &DialogTriggerProps) -> Html {
    let context = use_context::<DialogContext>().expect("DialogTrigger must be used within Dialog");
    
    let handle_click = {
        let open = context.open.clone();
        let on_open_change = context.on_open_change.clone();
        let disabled = context.disabled;
        
        Callback::from(move |_: MouseEvent| {
            if !disabled {
                open.set(true);
                if let Some(callback) = &on_open_change {
                    callback.emit(true);
                }
            }
        })
    };
    
    let computed_class = format!(
        "{} {}",
        props.class.as_deref().unwrap_or(""),
        if context.disabled { "cursor-not-allowed opacity-50" } else { "" }
    );
    
    html! {
        <button
            type="button"
            class={computed_class}
            id={props.id.clone()}
            style={props.style.clone()}
            disabled={context.disabled}
            onclick={handle_click}
        >
            {props.children.clone()}
        </button>
    }
}

#[function_component]
pub fn DialogContent(props: &DialogContentProps) -> Html {
    let context = use_context::<DialogContext>().expect("DialogContent must be used within Dialog");
    
    let handle_backdrop_click = {
        let open = context.open.clone();
        let on_open_change = context.on_open_change.clone();
        let disabled = context.disabled;
        
        Callback::from(move |event: MouseEvent| {
            if !disabled {
                // Only close if clicking the backdrop itself, not its children
                if let Some(target) = event.target_dyn_into::<Element>() {
                    if target.class_name().contains("fixed inset-0") {
                        open.set(false);
                        if let Some(callback) = &on_open_change {
                            callback.emit(false);
                        }
                    }
                }
            }
        })
    };
    
    let handle_keydown = {
        let open = context.open.clone();
        let on_open_change = context.on_open_change.clone();
        let disabled = context.disabled;
        
        Callback::from(move |event: KeyboardEvent| {
            if !disabled && event.key() == "Escape" {
                open.set(false);
                if let Some(callback) = &on_open_change {
                    callback.emit(false);
                }
            }
        })
    };
    
    let computed_class = format!(
        "{} {}",
        DIALOG_CONTENT_CLASS,
        props.class.as_deref().unwrap_or("")
    );
    
    if !*context.open {
        return html! {};
    }
    
    html! {
        <div
            class="fixed inset-0 z-50 bg-background/80 backdrop-blur-sm"
            onclick={handle_backdrop_click}
            onkeydown={handle_keydown}
        >
            <div
                class={computed_class}
                id={props.id.clone()}
                style={props.style.clone()}
                data-state="open"
                role="dialog"
                aria-modal="true"
            >
                {props.children.clone()}
            </div>
        </div>
    }
}

#[function_component]
pub fn DialogHeader(props: &DialogHeaderProps) -> Html {
    let computed_class = format!(
        "{} {}",
        DIALOG_HEADER_CLASS,
        props.class.as_deref().unwrap_or("")
    );
    
    html! {
        <div
            class={computed_class}
            id={props.id.clone()}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </div>
    }
}

#[function_component]
pub fn DialogFooter(props: &DialogFooterProps) -> Html {
    let computed_class = format!(
        "{} {}",
        DIALOG_FOOTER_CLASS,
        props.class.as_deref().unwrap_or("")
    );
    
    html! {
        <div
            class={computed_class}
            id={props.id.clone()}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </div>
    }
}

#[function_component]
pub fn DialogTitle(props: &DialogTitleProps) -> Html {
    let computed_class = format!(
        "{} {}",
        DIALOG_TITLE_CLASS,
        props.class.as_deref().unwrap_or("")
    );
    
    html! {
        <h2
            class={computed_class}
            id={props.id.clone()}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </h2>
    }
}

#[function_component]
pub fn DialogDescription(props: &DialogDescriptionProps) -> Html {
    let computed_class = format!(
        "{} {}",
        DIALOG_DESCRIPTION_CLASS,
        props.class.as_deref().unwrap_or("")
    );
    
    html! {
        <p
            class={computed_class}
            id={props.id.clone()}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </p>
    }
}
