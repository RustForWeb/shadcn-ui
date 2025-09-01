use leptos::prelude::*;
use leptos_style::Style;
use web_sys::MouseEvent;
use wasm_bindgen::JsCast;

#[component]
pub fn ContextMenu(
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let position = RwSignal::new((0, 0));

    provide_context(open);
    provide_context(position);

    view! {
        <div class="relative">
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ContextMenuTrigger(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();
    let position = expect_context::<RwSignal<(i32, i32)>>();

    let handle_context_menu = move |e: MouseEvent| {
        e.prevent_default();
        let x = e.client_x();
        let y = e.client_y();
        position.set((x, y));
        open.set(true);
    };

    let handle_click = move |_| {
        open.set(false);
    };

    Effect::new(move |_| {
        if open.get() {
            let handle_click_outside = move |_: MouseEvent| {
                open.set(false);
            };
            
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(handle_click_outside) as Box<dyn Fn(MouseEvent)>);
                    let _ = document.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
                    closure.forget();
                }
            }
        }
    });

    view! {
        <div
            class=format!("select-none {}", class.get().unwrap_or_default())
            on:contextmenu=handle_context_menu
            on:click=handle_click
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ContextMenuContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();
    let position = expect_context::<RwSignal<(i32, i32)>>();

    let computed_class = Signal::derive(move || {
        format!(
            "z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 {}",
            class.get().unwrap_or_default()
        )
    });

    let computed_style = Signal::derive(move || {
        let (x, y) = position.get();
        format!(
            "position: fixed; left: {}px; top: {}px; {}",
            x,
            y,
            style.get().to_string()
        )
    });

    if open.get() {
        view! {
            <div
                class=computed_class
                style=computed_style
                data-state="open"
            >
                {children.map(|c| c())}
            </div>
        }.into_any()
    } else {
        view! {}.into_any()
    }
}

#[component]
pub fn ContextMenuItem(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
    #[prop(into, optional)] inset: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();

    let handle_click = move |_| {
        if !disabled.get() {
            if let Some(callback) = &on_click {
                callback.run(());
            }
            open.set(false);
        }
    };

    let computed_class = Signal::derive(move || {
        let base_class = if inset.get() {
            "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 pl-8 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50"
        } else {
            "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50"
        };
        
        format!("{} {}", base_class, class.get().unwrap_or_default())
    });

    view! {
        <div
            class=computed_class
            data-disabled=move || disabled.get()
            on:click=handle_click
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ContextMenuCheckboxItem(
    #[prop(into)] checked: RwSignal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] on_checked_change: Option<Callback<bool>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let handle_click = move |_| {
        if !disabled.get() {
            let new_checked = !checked.get();
            checked.set(new_checked);
            if let Some(callback) = &on_checked_change {
                callback.run(new_checked);
            }
        }
    };

    let computed_class = Signal::derive(move || {
        format!(
            "relative flex cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50 {}",
            class.get().unwrap_or_default()
        )
    });

    view! {
        <div
            class=computed_class
            data-disabled=move || disabled.get()
            on:click=handle_click
        >
            <span class="absolute left-2 flex h-3.5 w-3.5 items-center justify-center">
                <Show when=move || checked.get()>
                    <svg
                        class="h-4 w-4"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    >
                        <path d="M20 6 9 17l-5-5"/>
                    </svg>
                </Show>
            </span>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ContextMenuRadioGroup(
    #[prop(into)] value: RwSignal<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    provide_context(value);
    provide_context(on_value_change);

    view! {
        <div>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ContextMenuRadioItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let group_value = expect_context::<RwSignal<String>>();
    let on_value_change = expect_context::<Option<Callback<String>>>();

    let value_clone = value.clone();
    let handle_click = move |_| {
        if !disabled.get() {
            group_value.set(value_clone.clone());
            if let Some(callback) = &on_value_change {
                callback.run(value_clone.clone());
            }
        }
    };

    let is_selected = Signal::derive(move || group_value.get() == value);

    let computed_class = Signal::derive(move || {
        format!(
            "relative flex cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50 {}",
            class.get().unwrap_or_default()
        )
    });

    view! {
        <div
            class=computed_class
            data-disabled=move || disabled.get()
            on:click=handle_click
        >
            <span class="absolute left-2 flex h-3.5 w-3.5 items-center justify-center">
                <Show when=move || is_selected.get()>
                    <svg
                        class="h-2 w-2 fill-current"
                        viewBox="0 0 24 24"
                    >
                        <circle cx="12" cy="12" r="12"/>
                    </svg>
                </Show>
            </span>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ContextMenuLabel(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] inset: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        let base_class = if inset.get() {
            "px-2 py-1.5 pl-8 text-sm font-semibold"
        } else {
            "px-2 py-1.5 text-sm font-semibold"
        };
        
        format!("{} {}", base_class, class.get().unwrap_or_default())
    });

    view! {
        <div class=computed_class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ContextMenuSeparator(
    #[prop(into, optional)] class: MaybeProp<String>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("-mx-1 my-1 h-px bg-muted {}", class.get().unwrap_or_default())
    });

    view! {
        <div class=computed_class />
    }
}

#[component]
pub fn ContextMenuShortcut(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("ml-auto text-xs tracking-widest opacity-60 {}", class.get().unwrap_or_default())
    });

    view! {
        <span class=computed_class>
            {children.map(|c| c())}
        </span>
    }
}

#[component]
pub fn ContextMenuSub(
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let sub_open = RwSignal::new(false);
    provide_context(sub_open);

    view! {
        <div class="relative">
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ContextMenuSubTrigger(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] inset: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let sub_open = expect_context::<RwSignal<bool>>();

    let handle_mouse_enter = move |_| {
        sub_open.set(true);
    };

    let handle_mouse_leave = move |_| {
        sub_open.set(false);
    };

    let computed_class = Signal::derive(move || {
        let base_class = if inset.get() {
            "flex cursor-default select-none items-center rounded-sm px-2 py-1.5 pl-8 text-sm outline-none focus:bg-accent data-[state=open]:bg-accent"
        } else {
            "flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:bg-accent data-[state=open]:bg-accent"
        };
        
        format!("{} {}", base_class, class.get().unwrap_or_default())
    });

    view! {
        <div
            class=computed_class
            data-state=move || if sub_open.get() { "open" } else { "closed" }
            on:mouseenter=handle_mouse_enter
            on:mouseleave=handle_mouse_leave
        >
            {children.map(|c| c())}
            <svg
                class="ml-auto h-4 w-4"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <path d="m9 18 6-6-6-6"/>
            </svg>
        </div>
    }
}

#[component]
pub fn ContextMenuSubContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let sub_open = expect_context::<RwSignal<bool>>();

    let computed_class = Signal::derive(move || {
        format!(
            "z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-lg data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 {}",
            class.get().unwrap_or_default()
        )
    });

    if sub_open.get() {
        view! {
            <div
                class=computed_class
                data-state="open"
                style="position: absolute; left: 100%; top: 0;"
            >
                {children.map(|c| c())}
            </div>
        }.into_any()
    } else {
        view! {}.into_any()
    }
}