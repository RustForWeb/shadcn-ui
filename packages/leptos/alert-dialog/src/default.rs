use leptos::prelude::*;
use leptos_style::Style;
use web_sys::{KeyboardEvent, MouseEvent};
use wasm_bindgen::JsCast;

#[component]
pub fn AlertDialog(
    #[prop(into)] open: RwSignal<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    provide_context(open);
    provide_context(on_open_change);

    // Handle escape key
    Effect::new(move |_| {
        if open.get() {
            let handle_keydown = move |e: KeyboardEvent| {
                if e.key() == "Escape" {
                    open.set(false);
                    if let Some(callback) = &on_open_change {
                        callback.run(false);
                    }
                }
            };

            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(handle_keydown) as Box<dyn Fn(KeyboardEvent)>);
                    let _ = document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
                    closure.forget();
                }
            }
        }
    });

    view! {
        <div>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn AlertDialogTrigger(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] as_child: Option<Callback<AlertDialogTriggerChildProps, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();
    let on_open_change = expect_context::<Option<Callback<bool>>>();

    let handle_click = {
        let open = open.clone();
        let on_open_change = on_open_change.clone();
        move |_: MouseEvent| {
            open.set(true);
            if let Some(callback) = &on_open_change {
                callback.run(true);
            }
        }
    };

    if let Some(as_child) = as_child {
        let child_props = AlertDialogTriggerChildProps {
            class: class.get().unwrap_or_default(),
            onclick: Some(Callback::new({
                let open = open.clone();
                let on_open_change = on_open_change.clone();
                move |_| {
                    open.set(true);
                    if let Some(callback) = &on_open_change {
                        callback.run(true);
                    }
                }
            })),
        };
        as_child.run(child_props).into_any()
    } else {
        view! {
            <button
                class=class.get().unwrap_or_default()
                on:click=handle_click
            >
                {children.map(|c| c())}
            </button>
        }.into_any()
    }
}

#[derive(Debug, Clone)]
pub struct AlertDialogTriggerChildProps {
    pub class: String,
    pub onclick: Option<Callback<()>>,
}

#[component]
pub fn AlertDialogOverlay(
    #[prop(into, optional)] class: MaybeProp<String>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();

    let computed_class = Signal::derive(move || {
        format!(
            "fixed inset-0 z-50 bg-black/80 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 {}",
            class.get().unwrap_or_default()
        )
    });

    view! {
        <Show when=move || open.get()>
            <div
                class=computed_class
                data-state=move || if open.get() { "open" } else { "closed" }
            />
        </Show>
    }
}

#[component]
pub fn AlertDialogContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();
    let on_open_change = expect_context::<Option<Callback<bool>>>();

    let handle_overlay_click = move |e: MouseEvent| {
        // Close if clicking the overlay (not the content)
        if e.target() == e.current_target() {
            open.set(false);
            if let Some(callback) = &on_open_change {
                callback.run(false);
            }
        }
    };

    let computed_class = Signal::derive(move || {
        format!(
            "fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border bg-background p-6 shadow-lg duration-200 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[state=closed]:slide-out-to-left-1/2 data-[state=closed]:slide-out-to-top-[48%] data-[state=open]:slide-in-from-left-1/2 data-[state=open]:slide-in-from-top-[48%] sm:rounded-lg {}",
            class.get().unwrap_or_default()
        )
    });

    if open.get() {
        view! {
            <AlertDialogOverlay />
            <div
                class="fixed inset-0 z-50 flex items-center justify-center p-4"
                on:click=handle_overlay_click
            >
                <div
                    class=computed_class
                    style=move || style.get().to_string()
                    data-state="open"
                    on:click=move |e: MouseEvent| e.stop_propagation()
                >
                    {children.map(|c| c())}
                </div>
            </div>
        }.into_any()
    } else {
        view! {}.into_any()
    }
}

#[component]
pub fn AlertDialogHeader(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("flex flex-col space-y-2 text-center sm:text-left {}", class.get().unwrap_or_default())
    });

    view! {
        <div class=computed_class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn AlertDialogFooter(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2 {}", class.get().unwrap_or_default())
    });

    view! {
        <div class=computed_class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn AlertDialogTitle(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("text-lg font-semibold {}", class.get().unwrap_or_default())
    });

    view! {
        <h2 class=computed_class>
            {children.map(|c| c())}
        </h2>
    }
}

#[component]
pub fn AlertDialogDescription(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("text-sm text-muted-foreground {}", class.get().unwrap_or_default())
    });

    view! {
        <p class=computed_class>
            {children.map(|c| c())}
        </p>
    }
}

#[component]
pub fn AlertDialogAction(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] as_child: Option<Callback<AlertDialogActionChildProps, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();
    let on_open_change = expect_context::<Option<Callback<bool>>>();

    let handle_click = {
        let open = open.clone();
        let on_open_change = on_open_change.clone();
        let on_click = on_click.clone();
        move |_: MouseEvent| {
            if let Some(callback) = &on_click {
                callback.run(());
            }
            open.set(false);
            if let Some(callback) = &on_open_change {
                callback.run(false);
            }
        }
    };

    let computed_class = Signal::derive(move || {
        format!(
            "inline-flex h-10 items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-semibold text-primary-foreground ring-offset-background transition-colors hover:bg-primary/90 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 {}",
            class.get().unwrap_or_default()
        )
    });

    if let Some(as_child) = as_child {
        let child_props = AlertDialogActionChildProps {
            class: computed_class.get(),
            onclick: Some(Callback::new({
                let open = open.clone();
                let on_open_change = on_open_change.clone();
                let on_click = on_click.clone();
                move |_| {
                    if let Some(callback) = &on_click {
                        callback.run(());
                    }
                    open.set(false);
                    if let Some(callback) = &on_open_change {
                        callback.run(false);
                    }
                }
            })),
            disabled: disabled.get(),
        };
        as_child.run(child_props).into_any()
    } else {
        view! {
            <button
                class=computed_class
                disabled=disabled
                on:click=handle_click
            >
                {children.map(|c| c())}
            </button>
        }.into_any()
    }
}

#[derive(Debug, Clone)]
pub struct AlertDialogActionChildProps {
    pub class: String,
    pub onclick: Option<Callback<()>>,
    pub disabled: bool,
}

#[component]
pub fn AlertDialogCancel(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] as_child: Option<Callback<AlertDialogCancelChildProps, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();
    let on_open_change = expect_context::<Option<Callback<bool>>>();

    let handle_click = {
        let open = open.clone();
        let on_open_change = on_open_change.clone();
        let on_click = on_click.clone();
        move |_: MouseEvent| {
            if let Some(callback) = &on_click {
                callback.run(());
            }
            open.set(false);
            if let Some(callback) = &on_open_change {
                callback.run(false);
            }
        }
    };

    let computed_class = Signal::derive(move || {
        format!(
            "mt-2 inline-flex h-10 items-center justify-center rounded-md border border-input bg-background px-4 py-2 text-sm font-semibold ring-offset-background transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 sm:mt-0 {}",
            class.get().unwrap_or_default()
        )
    });

    if let Some(as_child) = as_child {
        let child_props = AlertDialogCancelChildProps {
            class: computed_class.get(),
            onclick: Some(Callback::new({
                let open = open.clone();
                let on_open_change = on_open_change.clone();
                let on_click = on_click.clone();
                move |_| {
                    if let Some(callback) = &on_click {
                        callback.run(());
                    }
                    open.set(false);
                    if let Some(callback) = &on_open_change {
                        callback.run(false);
                    }
                }
            })),
            disabled: disabled.get(),
        };
        as_child.run(child_props).into_any()
    } else {
        view! {
            <button
                class=computed_class
                disabled=disabled
                on:click=handle_click
            >
                {children.map(|c| c())}
            </button>
        }.into_any()
    }
}

#[derive(Debug, Clone)]
pub struct AlertDialogCancelChildProps {
    pub class: String,
    pub onclick: Option<Callback<()>>,
    pub disabled: bool,
}