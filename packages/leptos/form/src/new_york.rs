use leptos::prelude::*;
use leptos_style::Style;
use web_sys::{HtmlFormElement, SubmitEvent};
use wasm_bindgen::JsCast;
use crate::default::{FormData, FormValidation};

/// New York theme Form component
#[component]
pub fn Form(
    #[prop(into, optional)] on_submit: Option<Callback<FormData>>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let (_validation, _set_validation) = signal(FormValidation::new());
    
    let handle_submit = move |event: SubmitEvent| {
        event.prevent_default();
        
        if let Some(target) = event.target() {
            if let Ok(form) = target.dyn_into::<HtmlFormElement>() {
                let form_data = FormData::from_form(&form);
                
                if let Some(callback) = on_submit.as_ref() {
                    callback.run(form_data);
                }
            }
        }
    };
    
    let computed_class = Signal::derive(move || {
        let base_class = "space-y-6";
        if let Some(class) = class.get() {
            format!("{} {}", base_class, class)
        } else {
            base_class.to_string()
        }
    });
    
    view! {
        <form
            class=computed_class
            style=move || style.get().to_string()
            on:submit=handle_submit
        >
            {children.map(|c| c())}
        </form>
    }
}

/// Form field wrapper component (New York theme)
#[component]
pub fn FormField(
    #[prop(into)] name: String,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        let base_class = "space-y-2";
        if let Some(class) = class.get() {
            format!("{} {}", base_class, class)
        } else {
            base_class.to_string()
        }
    });
    
    view! {
        <div
            class=computed_class
            style=move || style.get().to_string()
            data-field=name
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Form item wrapper component (New York theme)
#[component]
pub fn FormItem(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        let base_class = "space-y-2";
        if let Some(class) = class.get() {
            format!("{} {}", base_class, class)
        } else {
            base_class.to_string()
        }
    });
    
    view! {
        <div
            class=computed_class
            style=move || style.get().to_string()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Form label component (New York theme)
#[component]
pub fn FormLabel(
    #[prop(into)] for_field: String,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        let base_class = "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70";
        if let Some(class) = class.get() {
            format!("{} {}", base_class, class)
        } else {
            base_class.to_string()
        }
    });
    
    view! {
        <label
            for=for_field
            class=computed_class
            style=move || style.get().to_string()
        >
            {children.map(|c| c())}
        </label>
    }
}

/// Form control wrapper component (New York theme)
#[component]
pub fn FormControl(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        let base_class = "peer";
        if let Some(class) = class.get() {
            format!("{} {}", base_class, class)
        } else {
            base_class.to_string()
        }
    });
    
    view! {
        <div
            class=computed_class
            style=move || style.get().to_string()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Form message component for displaying validation errors (New York theme)
#[component]
pub fn FormMessage(
    #[prop(into, optional)] message: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        let base_class = "text-sm font-medium text-destructive";
        if let Some(class) = class.get() {
            format!("{} {}", base_class, class)
        } else {
            base_class.to_string()
        }
    });
    
    view! {
        <p
            class=move || {
                if let Some(_message) = message.get() {
                    computed_class.get()
                } else {
                    "hidden".to_string()
                }
            }
            style=move || style.get().to_string()
        >
            {move || message.get().unwrap_or_default()}
        </p>
    }
}

/// Form description component (New York theme)
#[component]
pub fn FormDescription(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        let base_class = "text-sm text-muted-foreground";
        if let Some(class) = class.get() {
            format!("{} {}", base_class, class)
        } else {
            base_class.to_string()
        }
    });
    
    view! {
        <p
            class=computed_class
            style=move || style.get().to_string()
        >
            {children.map(|c| c())}
        </p>
    }
}
