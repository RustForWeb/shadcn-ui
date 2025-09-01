use leptos::prelude::*;
use web_sys::MouseEvent;
use web_sys::KeyboardEvent;

#[derive(Debug, Clone, PartialEq)]
pub enum AccordionType {
    Single,
    Multiple,
}

impl Default for AccordionType {
    fn default() -> Self {
        AccordionType::Single
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AccordionOrientation {
    Vertical,
    Horizontal,
}

impl Default for AccordionOrientation {
    fn default() -> Self {
        AccordionOrientation::Vertical
    }
}

#[component]
pub fn Accordion(
    #[prop(into, optional)] r#type: Signal<AccordionType>,
    #[prop(into, optional)] orientation: Signal<AccordionOrientation>,
    #[prop(into, optional)] collapsible: Signal<bool>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] value: RwSignal<Vec<String>>,
    #[prop(into, optional)] default_value: Vec<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<Vec<String>>>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Initialize value with default if empty
    Effect::new(move |_| {
        if value.get().is_empty() && !default_value.is_empty() {
            value.set(default_value.clone());
        }
    });

    provide_context(r#type);
    provide_context(orientation);
    provide_context(collapsible);
    provide_context(disabled);
    provide_context(value);
    provide_context(on_value_change);

    let computed_class = Signal::derive(move || {
        format!("w-full {}", class.get().unwrap_or_default())
    });

    view! {
        <div
            class=computed_class
            data-orientation=move || match orientation.get() {
                AccordionOrientation::Vertical => "vertical",
                AccordionOrientation::Horizontal => "horizontal",
            }
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn AccordionItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let accordion_disabled = expect_context::<Signal<bool>>();
    let accordion_value = expect_context::<RwSignal<Vec<String>>>();
    
    let is_disabled = Signal::derive(move || disabled.get() || accordion_disabled.get());
    let value_clone = value.clone();
    let is_expanded = Signal::derive(move || accordion_value.get().contains(&value_clone));

    provide_context(value.clone());
    provide_context(is_disabled);
    provide_context(is_expanded);

    let computed_class = Signal::derive(move || {
        format!("border-b {}", class.get().unwrap_or_default())
    });

    view! {
        <div
            class=computed_class
            data-state=move || if is_expanded.get() { "open" } else { "closed" }
            data-disabled=move || is_disabled.get()
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn AccordionTrigger(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] as_child: Option<Callback<AccordionTriggerChildProps>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let item_value = expect_context::<String>();
    let is_disabled = expect_context::<Signal<bool>>();
    let is_expanded = expect_context::<Signal<bool>>();
    let accordion_type = expect_context::<Signal<AccordionType>>();
    let accordion_value = expect_context::<RwSignal<Vec<String>>>();
    let on_value_change = expect_context::<Option<Callback<Vec<String>>>>();
    let collapsible = expect_context::<Signal<bool>>();

    // Toggle logic extracted so it can be used by both click and keydown without moving closures
    let toggle = {
        let accordion_value = accordion_value.clone();
        let on_value_change = on_value_change.clone();
        let item_value = item_value.clone();
        move || {
            if is_disabled.get() {
                return;
            }

            let mut current_value = accordion_value.get();
            
            match accordion_type.get() {
                AccordionType::Single => {
                    if current_value.contains(&item_value) {
                        if collapsible.get() {
                            current_value.clear();
                        }
                    } else {
                        current_value.clear();
                        current_value.push(item_value.clone());
                    }
                }
                AccordionType::Multiple => {
                    if let Some(index) = current_value.iter().position(|v| v == &item_value) {
                        current_value.remove(index);
                    } else {
                        current_value.push(item_value.clone());
                    }
                }
            }

            accordion_value.set(current_value.clone());
            if let Some(callback) = &on_value_change {
                callback.run(current_value);
            }
        }
    };

    let handle_click = {
        let toggle = toggle.clone();
        move |_: MouseEvent| {
            toggle();
        }
    };

    let handle_keydown = {
        let toggle = toggle.clone();
        move |e: KeyboardEvent| {
            match e.key().as_str() {
                "Enter" | " " => {
                    e.prevent_default();
                    toggle();
                }
                _ => {}
            }
        }
    };

    let computed_class = Signal::derive(move || {
        format!(
            "flex flex-1 items-center justify-between py-4 font-medium transition-all hover:underline [&[data-state=open]>svg]:rotate-180 {}",
            class.get().unwrap_or_default()
        )
    });

    if let Some(as_child) = as_child {
        let child_props = AccordionTriggerChildProps {
            class: computed_class.get(),
            onclick: Some(Callback::new(move |_| {
                if is_disabled.get() {
                    return;
                }

                let mut current_value = accordion_value.get();
                
                match accordion_type.get() {
                    AccordionType::Single => {
                        if current_value.contains(&item_value) {
                            if collapsible.get() {
                                current_value.clear();
                            }
                        } else {
                            current_value.clear();
                            current_value.push(item_value.clone());
                        }
                    }
                    AccordionType::Multiple => {
                        if let Some(index) = current_value.iter().position(|v| v == &item_value) {
                            current_value.remove(index);
                        } else {
                            current_value.push(item_value.clone());
                        }
                    }
                }

                accordion_value.set(current_value.clone());
                if let Some(callback) = &on_value_change {
                    callback.run(current_value);
                }
            })),
            onkeydown: Some(Callback::new(move |e| handle_keydown(e))),
            disabled: is_disabled.get(),
            expanded: is_expanded.get(),
        };
        as_child.run(child_props).into_any()
    } else {
        view! {
            <button
                class=computed_class
                data-state=move || if is_expanded.get() { "open" } else { "closed" }
                disabled=is_disabled
                aria-expanded=move || is_expanded.get()
                on:click=handle_click
                on:keydown=handle_keydown
            >
                {children.map(|c| c())}
                <svg
                    class="h-4 w-4 shrink-0 transition-transform duration-200"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path d="m6 9 6 6 6-6"/>
                </svg>
            </button>
        }.into_any()
    }
}

#[derive(Debug, Clone)]
pub struct AccordionTriggerChildProps {
    pub class: String,
    pub onclick: Option<Callback<()>>,
    pub onkeydown: Option<Callback<KeyboardEvent>>,
    pub disabled: bool,
    pub expanded: bool,
}

#[component]
pub fn AccordionContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] force_mount: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let is_expanded = expect_context::<Signal<bool>>();

    let computed_class = Signal::derive(move || {
        format!(
            "overflow-hidden text-sm transition-all data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down {}",
            class.get().unwrap_or_default()
        )
    });

    let _should_render = Signal::derive(move || force_mount.get() || is_expanded.get());

    view! {
        <div
            class=computed_class
            data-state=move || if is_expanded.get() { "open" } else { "closed" }
        >
            <div class="pb-4 pt-0">
                {children.map(|c| c())}
            </div>
        </div>
    }
}