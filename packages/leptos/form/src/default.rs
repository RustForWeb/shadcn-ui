use leptos::prelude::*;
use leptos_style::Style;
use web_sys::{HtmlFormElement, HtmlInputElement, SubmitEvent};
use wasm_bindgen::JsCast;

/// Form validation error
#[derive(Clone, Debug)]
pub struct FormError {
    pub field: String,
    pub message: String,
}

/// Form validation result
#[derive(Clone, Debug)]
pub struct FormValidation {
    pub is_valid: bool,
    pub errors: Vec<FormError>,
}

impl FormValidation {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
        }
    }

    pub fn add_error(&mut self, field: impl Into<String>, message: impl Into<String>) {
        self.is_valid = false;
        self.errors.push(FormError {
            field: field.into(),
            message: message.into(),
        });
    }

    pub fn get_error(&self, field: &str) -> Option<&str> {
        self.errors
            .iter()
            .find(|error| error.field == field)
            .map(|error| error.message.as_str())
    }
}

/// Default theme Form component
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

/// Form field wrapper component
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

/// Form item wrapper component
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

/// Form label component
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

/// Form control wrapper component
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

/// Form message component for displaying validation errors
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

/// Form description component
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

/// Form data structure for handling form submissions
#[derive(Clone, Debug)]
pub struct FormData {
    pub fields: std::collections::HashMap<String, String>,
}

impl FormData {
    pub fn new() -> Self {
        Self {
            fields: std::collections::HashMap::new(),
        }
    }

    pub fn from_form(form: &HtmlFormElement) -> Self {
        let mut form_data = Self::new();
        
        // Get all form elements
        let elements = form.elements();
        for i in 0..elements.length() {
            if let Some(element) = elements.get_with_index(i) {
                if let Ok(input) = element.dyn_into::<HtmlInputElement>() {
                    let name = input.name();
                    let value = input.value();
                    
                    if !name.is_empty() {
                        form_data.fields.insert(name, value);
                    }
                }
            }
        }
        
        form_data
    }

    pub fn get(&self, field: &str) -> Option<&String> {
        self.fields.get(field)
    }

    pub fn get_or_default(&self, field: &str) -> String {
        self.fields.get(field).cloned().unwrap_or_default()
    }
}
