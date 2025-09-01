use leptos::prelude::*;
use leptos_style::Style;
use web_sys::{KeyboardEvent, MouseEvent, TouchEvent};
use wasm_bindgen::JsCast;

#[derive(Debug, Clone, PartialEq)]
pub enum DrawerDirection {
    Top,
    Bottom,
    Left,
    Right,
}

impl Default for DrawerDirection {
    fn default() -> Self {
        DrawerDirection::Bottom
    }
}

#[component]
pub fn Drawer(
    #[prop(into)] open: RwSignal<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] direction: Signal<DrawerDirection>,
    #[prop(into, optional)] should_scale_background: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    provide_context(open);
    provide_context(on_open_change);
    provide_context(direction);
    provide_context(should_scale_background);

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
pub fn DrawerTrigger(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] as_child: Option<Callback<DrawerTriggerChildProps, AnyView>>,
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
        let child_props = DrawerTriggerChildProps {
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
pub struct DrawerTriggerChildProps {
    pub class: String,
    pub onclick: Option<Callback<()>>,
}

#[component]
pub fn DrawerPortal(
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DrawerOverlay(
    #[prop(into, optional)] class: MaybeProp<String>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();
    let on_open_change = expect_context::<Option<Callback<bool>>>();

    let handle_click = move |_| {
        open.set(false);
        if let Some(callback) = &on_open_change {
            callback.run(false);
        }
    };

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
                on:click=handle_click
            />
        </Show>
    }
}

#[component]
pub fn DrawerContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();
    let direction = expect_context::<Signal<DrawerDirection>>();
    let should_scale_background = expect_context::<Signal<bool>>();

    // State for drag interactions
    let is_dragging = RwSignal::new(false);
    let drag_start_y = RwSignal::new(0);
    let drag_offset = RwSignal::new(0);

    let handle_mouse_down = move |e: MouseEvent| {
        is_dragging.set(true);
        drag_start_y.set(e.client_y());
        drag_offset.set(0);
    };

    let handle_touch_start = move |_e: TouchEvent| {
        // TouchEvent handling would need proper touch API access
        // For now, just disable touch functionality
    };

    let handle_mouse_move = move |e: MouseEvent| {
        if is_dragging.get() {
            let current_y = e.client_y();
            let offset = current_y - drag_start_y.get();
            
            match direction.get() {
                DrawerDirection::Bottom => {
                    if offset > 0 {
                        drag_offset.set(offset);
                    }
                }
                DrawerDirection::Top => {
                    if offset < 0 {
                        drag_offset.set(-offset);
                    }
                }
                _ => {}
            }
        }
    };

    let handle_mouse_up = move |_: MouseEvent| {
        if is_dragging.get() {
            is_dragging.set(false);
            
            // Close if dragged far enough
            if drag_offset.get().abs() > 100 {
                open.set(false);
            }
            drag_offset.set(0);
        }
    };

    let computed_class = Signal::derive(move || {
        let base_class = match direction.get() {
            DrawerDirection::Bottom => "fixed inset-x-0 bottom-0 z-50 mt-24 flex h-auto flex-col rounded-t-[10px] border bg-background",
            DrawerDirection::Top => "fixed inset-x-0 top-0 z-50 mb-24 flex h-auto flex-col rounded-b-[10px] border bg-background",
            DrawerDirection::Left => "fixed inset-y-0 left-0 z-50 h-full w-3/4 flex flex-col rounded-r-[10px] border bg-background sm:max-w-sm",
            DrawerDirection::Right => "fixed inset-y-0 right-0 z-50 h-full w-3/4 flex flex-col rounded-l-[10px] border bg-background sm:max-w-sm",
        };

        let animation_class = match direction.get() {
            DrawerDirection::Bottom => "data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:slide-out-to-bottom data-[state=open]:slide-in-from-bottom",
            DrawerDirection::Top => "data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:slide-out-to-top data-[state=open]:slide-in-from-top",
            DrawerDirection::Left => "data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:slide-out-to-left data-[state=open]:slide-in-from-left",
            DrawerDirection::Right => "data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:slide-out-to-right data-[state=open]:slide-in-from-right",
        };

        format!("{} {} {}", base_class, animation_class, class.get().unwrap_or_default())
    });

    let computed_style = Signal::derive(move || {
        let offset = drag_offset.get();
        let transform = if offset != 0 {
            match direction.get() {
                DrawerDirection::Bottom => format!("transform: translateY({}px);", offset),
                DrawerDirection::Top => format!("transform: translateY(-{}px);", offset),
                DrawerDirection::Left => format!("transform: translateX(-{}px);", offset),
                DrawerDirection::Right => format!("transform: translateX({}px);", offset),
            }
        } else {
            String::new()
        };

        format!("{} {}", style.get().to_string(), transform)
    });

    if open.get() {
        view! {
            <DrawerPortal>
                <DrawerOverlay />
                <div
                    class=computed_class
                    style=computed_style
                    data-state="open"
                    on:mousedown=handle_mouse_down
                    on:touchstart=handle_touch_start
                    on:mousemove=handle_mouse_move
                    on:mouseup=handle_mouse_up
                    on:click=move |e: MouseEvent| e.stop_propagation()
                >
                    {
                        if matches!(direction.get(), DrawerDirection::Bottom | DrawerDirection::Top) {
                            view! {
                                <div class="mx-auto mt-4 h-2 w-[100px] rounded-full bg-muted" />
                            }.into_any()
                        } else {
                            view! {}.into_any()
                        }
                    }
                    {children.map(|c| c())}
                </div>
            </DrawerPortal>
        }.into_any()
    } else {
        view! {}.into_any()
    }
}

#[component]
pub fn DrawerHeader(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("grid gap-1.5 p-4 text-center sm:text-left {}", class.get().unwrap_or_default())
    });

    view! {
        <div class=computed_class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DrawerFooter(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("mt-auto flex flex-col gap-2 p-4 {}", class.get().unwrap_or_default())
    });

    view! {
        <div class=computed_class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DrawerTitle(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let computed_class = Signal::derive(move || {
        format!("text-lg font-semibold leading-none tracking-tight {}", class.get().unwrap_or_default())
    });

    view! {
        <h2 class=computed_class>
            {children.map(|c| c())}
        </h2>
    }
}

#[component]
pub fn DrawerDescription(
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
pub fn DrawerClose(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
    #[prop(into, optional)] as_child: Option<Callback<DrawerCloseChildProps, AnyView>>,
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

    if let Some(as_child) = as_child {
        let child_props = DrawerCloseChildProps {
            class: class.get().unwrap_or_default(),
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
        };
        as_child.run(child_props).into_any()
    } else {
        let computed_class = Signal::derive(move || {
            format!(
                "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2 {}",
                class.get().unwrap_or_default()
            )
        });

        view! {
            <button
                class=computed_class
                on:click=handle_click
            >
                {children.map(|c| c())}
            </button>
        }.into_any()
    }
}

#[derive(Debug, Clone)]
pub struct DrawerCloseChildProps {
    pub class: String,
    pub onclick: Option<Callback<()>>,
}

#[component]
pub fn DrawerNestedRoot(
    #[prop(into)] open: RwSignal<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Nested drawer implementation - simpler version of main Drawer
    provide_context(open);
    provide_context(on_open_change);
    let direction = Signal::derive(|| DrawerDirection::Bottom);
    let should_scale_background = Signal::derive(|| false);
    provide_context(direction);
    provide_context(should_scale_background);

    view! {
        <div>
            {children.map(|c| c())}
        </div>
    }
}