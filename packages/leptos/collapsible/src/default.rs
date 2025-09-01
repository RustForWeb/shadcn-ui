use leptos::prelude::*;
use web_sys::MouseEvent;
use web_sys::KeyboardEvent;

#[component]
pub fn Collapsible(
    #[prop(into, optional)] open: RwSignal<bool>,
    #[prop(into, optional)] default_open: bool,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Initialize open state with default if not set
    Effect::new(move |_| {
        if !open.get() && default_open {
            open.set(default_open);
        }
    });

    provide_context(open);
    provide_context(disabled);
    provide_context(on_open_change);

    let computed_class = Signal::derive(move || {
        format!("w-full {}", class.get().unwrap_or_default())
    });

    view! {
        <div class=computed_class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CollapsibleTrigger(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] as_child: Option<Callback<CollapsibleTriggerChildProps>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();
    let disabled = expect_context::<Signal<bool>>();
    let on_open_change = expect_context::<Option<Callback<bool>>>();

    let toggle = {
        let open = open.clone();
        let on_open_change = on_open_change.clone();
        move || {
            if disabled.get() {
                return;
            }

            let new_open = !open.get();
            open.set(new_open);
            if let Some(callback) = &on_open_change {
                callback.run(new_open);
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
            "flex w-full items-center justify-between py-4 font-medium transition-all hover:underline [&[data-state=open]>svg]:rotate-180 {}",
            class.get().unwrap_or_default()
        )
    });

    if let Some(as_child) = as_child {
        let child_props = CollapsibleTriggerChildProps {
            class: class.get().unwrap_or_default(),
            onclick: Some(Callback::new(move |_| {
                if disabled.get() {
                    return;
                }

                let new_open = !open.get();
                open.set(new_open);
                if let Some(callback) = &on_open_change {
                    callback.run(new_open);
                }
            })),
            onkeydown: Some(Callback::new(move |e| handle_keydown(e))),
            disabled: disabled.get(),
            open: open.get(),
        };
        as_child.run(child_props).into_any()
    } else {
        view! {
            <button
                class=computed_class
                data-state=move || if open.get() { "open" } else { "closed" }
                disabled=disabled
                aria-expanded=move || open.get()
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
pub struct CollapsibleTriggerChildProps {
    pub class: String,
    pub onclick: Option<Callback<()>>,
    pub onkeydown: Option<Callback<KeyboardEvent>>,
    pub disabled: bool,
    pub open: bool,
}

#[component]
pub fn CollapsibleContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] force_mount: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();

    let computed_class = Signal::derive(move || {
        format!(
            "overflow-hidden text-sm transition-all data-[state=closed]:animate-collapsible-up data-[state=open]:animate-collapsible-down {}",
            class.get().unwrap_or_default()
        )
    });

    let _should_render = Signal::derive(move || force_mount.get() || open.get());

    view! {
        <div
            class=computed_class
            data-state=move || if open.get() { "open" } else { "closed" }
        >
            <div>
                {children.map(|c| c())}
            </div>
        </div>
    }
}