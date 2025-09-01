use leptos::prelude::*;
use leptos_style::Style;
use web_sys::{HtmlInputElement, Event, KeyboardEvent, FocusEvent};
use wasm_bindgen::JsCast;

/// Props for a combobox option
#[derive(Clone, Debug)]
pub struct ComboboxOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl ComboboxOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Default theme Combobox component
#[component]
pub fn Combobox(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(optional)] on_change: Option<Callback<String>>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into)] options: Vec<ComboboxOption>,
    #[prop(into, optional)] open: Signal<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] _children: Option<Children>,
) -> impl IntoView {
    // Internal state
    let (is_open, set_is_open) = signal(false);
    let (input_value, set_input_value) = signal(String::new());
    let (selected_index, set_selected_index) = signal(0);
    let (filtered_options, set_filtered_options) = signal(options.clone());
    
    // Use external open state if provided, otherwise use internal
    let open_state = if open.get() { open } else { Signal::derive(move || is_open.get()) };
    let _set_open = on_open_change.unwrap_or_else(|| Callback::new(move |value| set_is_open.set(value)));
    
    // Use external value if provided, otherwise use internal
    let current_value = if let Some(val) = value.get() { Signal::derive(move || val.clone()) } else { Signal::derive(move || input_value.get()) };
    let set_value = on_change.unwrap_or_else(|| Callback::new(move |value| set_input_value.set(value)));
    
    // Filter options based on input value
    Effect::new(move |_| {
        let input = current_value.get();
        let filtered = if input.is_empty() {
            options.clone()
        } else {
            options
                .iter()
                .filter(|option| {
                    option.label.to_lowercase().contains(&input.to_lowercase()) ||
                    option.value.to_lowercase().contains(&input.to_lowercase())
                })
                .cloned()
                .collect()
        };
        set_filtered_options.set(filtered);
    });
    
    // Handle input change
    let handle_input_change = move |event: Event| {
        if let Some(target) = event.target() {
            if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                let value = input.value();
                set_value.run(value.clone());
                set_input_value.set(value);
                set_is_open.set(true);
                set_selected_index.set(0);
            }
        }
    };
    
    // Handle key navigation
    let handle_keydown = move |event: KeyboardEvent| {
        let options = filtered_options.get();
        let current_index = selected_index.get();
        
        match event.key().as_str() {
            "ArrowDown" => {
                event.prevent_default();
                let new_index = if current_index < options.len().saturating_sub(1) {
                    current_index + 1
                } else {
                    0
                };
                set_selected_index.set(new_index);
            }
            "ArrowUp" => {
                event.prevent_default();
                let new_index = if current_index > 0 {
                    current_index - 1
                } else {
                    options.len().saturating_sub(1)
                };
                set_selected_index.set(new_index);
            }
            "Enter" => {
                event.prevent_default();
                if let Some(option) = options.get(current_index) {
                    set_value.run(option.value.clone());
                    set_input_value.set(option.value.clone());
                    set_is_open.set(false);
                }
            }
            "Escape" => {
                event.prevent_default();
                set_is_open.set(false);
            }
            _ => {}
        }
    };
    
    // Handle option selection
    let handle_option_click = move |option: ComboboxOption| {
        set_value.run(option.value.clone());
        set_input_value.set(option.value.clone());
        set_is_open.set(false);
    };
    
    // Handle focus/blur
    let handle_focus = move |_: FocusEvent| {
        set_is_open.set(true);
    };
    
    let handle_blur = move |_: FocusEvent| {
        // Delay closing to allow for option clicks
        set_timeout(move || set_is_open.set(false), std::time::Duration::from_millis(150));
    };
    
    // Compute classes
    let computed_class = Signal::derive(move || {
        let base_class = "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";
        if let Some(class) = class.get() {
            format!("{} {}", base_class, class)
        } else {
            base_class.to_string()
        }
    });
    
    view! {
        <div class="relative w-full">
            <input
                type="text"
                class=computed_class
                id=id.get().unwrap_or_default()
                style=move || style.get().to_string()
                placeholder=placeholder.get().unwrap_or_default()
                disabled=disabled
                value=move || current_value.get()
                on:input=handle_input_change
                on:keydown=handle_keydown
                on:focus=handle_focus
                on:blur=handle_blur
            />
            
            <button
                type="button"
                class="absolute right-3 top-1/2 -translate-y-1/2"
                on:click=move |_| set_is_open.set(!is_open.get())
                disabled=disabled
            >
                <svg
                    class="h-4 w-4 opacity-50"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M19 9l-7 7-7-7"
                    />
                </svg>
            </button>
            
            <div class=move || {
                if open_state.get() && !filtered_options.get().is_empty() {
                    "absolute top-full left-0 right-0 z-50 mt-1 max-h-60 w-full overflow-auto rounded-md border bg-popover text-popover-foreground shadow-md"
                } else {
                    "absolute top-full left-0 right-0 z-50 mt-1 max-h-60 w-full overflow-auto rounded-md border bg-popover text-popover-foreground shadow-md hidden"
                }
            }>
                {move || {
                    if open_state.get() && !filtered_options.get().is_empty() {
                        filtered_options.get().into_iter().enumerate().map(|(index, option)| {
                            let is_selected = move || index == selected_index.get();
                            let is_disabled = option.disabled;
                            let option_clone = option.clone();
                            
                            view! {
                                <div
                                    class=move || {
                                        let base_class = "relative flex w-full cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none hover:bg-accent hover:text-accent-foreground";
                                        if is_selected() {
                                            format!("{} bg-accent text-accent-foreground", base_class)
                                        } else if is_disabled {
                                            format!("{} opacity-50 cursor-not-allowed", base_class)
                                        } else {
                                            base_class.to_string()
                                        }
                                    }
                                    on:click=move |_| {
                                        if !is_disabled {
                                            handle_option_click(option_clone.clone());
                                        }
                                    }
                                >
                                    {option.label}
                                </div>
                            }
                        }).collect::<Vec<_>>()
                    } else {
                        vec![]
                    }
                }}
            </div>
        </div>
    }
}
