use leptos::{ev::MouseEvent, prelude::*};
use leptos_node_ref::AnyNodeRef;
use leptos_struct_component::{StructComponent, struct_component};
use leptos_style::Style;

// Dialog Root Provider
#[component]
pub fn Dialog(
    #[prop(into, optional)] open: Signal<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let internal_open = RwSignal::new(false);
    
    let open_state = Signal::derive(move || {
        if open.get() != internal_open.get() {
            open.get()
        } else {
            internal_open.get()
        }
    });

    let set_open = Callback::new(move |new_open: bool| {
        internal_open.set(new_open);
        if let Some(callback) = &on_open_change {
            callback.run(new_open);
        }
    });

    provide_context(DialogContextValue {
        open: open_state,
        set_open,
    });

    view! {
        <div>
            {children.map(|c| c())}
        </div>
    }
}

#[derive(Clone, Copy)]
pub struct DialogContextValue {
    pub open: Signal<bool>,
    pub set_open: Callback<bool>,
}

// Dialog Trigger
#[derive(Clone, StructComponent)]
#[struct_component(tag = "button")]
pub struct DialogTriggerChildProps {
    pub node_ref: AnyNodeRef,
    pub class: Signal<String>,
    pub id: MaybeProp<String>,
    pub style: Signal<Style>,
    pub disabled: Signal<bool>,
    pub r#type: MaybeProp<String>,
    pub onclick: Option<Callback<MouseEvent>>,
}

#[component]
pub fn DialogTrigger(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: Option<Callback<DialogTriggerChildProps, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = expect_context::<DialogContextValue>();
    
    let trigger_class = Signal::derive(move || {
        format!("{}", class.get().unwrap_or_default())
    });

    let handle_click = Callback::new(move |_: MouseEvent| {
        ctx.set_open.run(true);
    });

    let child_props = DialogTriggerChildProps {
        node_ref,
        class: trigger_class,
        id,
        style,
        disabled: Signal::derive(|| false),
        r#type: "button".to_string().into(),
        onclick: Some(handle_click),
    };

    if let Some(as_child) = as_child.as_ref() {
        as_child.run(child_props)
    } else {
        child_props.render(children)
    }
}

// Dialog Content
#[component]
pub fn DialogContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = expect_context::<DialogContextValue>();
    
    let content_class = Signal::derive(move || {
        format!("fixed inset-0 z-50 flex items-center justify-center bg-background/80 backdrop-blur-sm data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[state=closed]:slide-out-to-left-1/2 data-[state=closed]:slide-out-to-top-[48%] data-[state=open]:slide-in-from-left-1/2 data-[state=open]:slide-in-from-top-[48%] sm:rounded-lg {}", class.get().unwrap_or_default())
    });

    if ctx.open.get() {
        view! {
            <div 
                class="fixed inset-0 z-50"
                on:click=move |_| ctx.set_open.run(false)
            >
                <div
                    class={content_class}
                    style={move || style.get().to_string()}
                    on:click=|e: MouseEvent| e.stop_propagation()
                >
                    {children.map(|c| c())}
                </div>
            </div>
        }.into_any()
    } else {
        view! { <div></div> }.into_any()
    }
}

// Dialog Header
#[component]
pub fn DialogHeader(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let header_class = Signal::derive(move || {
        format!("flex flex-col space-y-1.5 text-center sm:text-left {}", class.get().unwrap_or_default())
    });

    view! {
        <div class={header_class}>
            {children.map(|c| c())}
        </div>
    }
}

// Dialog Title
#[component]
pub fn DialogTitle(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let title_class = Signal::derive(move || {
        format!("text-lg font-semibold leading-none tracking-tight {}", class.get().unwrap_or_default())
    });

    view! {
        <h2 class={title_class}>
            {children.map(|c| c())}
        </h2>
    }
}

// Dialog Description
#[component]
pub fn DialogDescription(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let description_class = Signal::derive(move || {
        format!("text-sm text-muted-foreground {}", class.get().unwrap_or_default())
    });

    view! {
        <p class={description_class}>
            {children.map(|c| c())}
        </p>
    }
}

// Dialog Footer
#[component]
pub fn DialogFooter(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let footer_class = Signal::derive(move || {
        format!("flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2 {}", class.get().unwrap_or_default())
    });

    view! {
        <div class={footer_class}>
            {children.map(|c| c())}
        </div>
    }
}

// Dialog Close
#[component]
pub fn DialogClose(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = expect_context::<DialogContextValue>();
    
    let close_class = Signal::derive(move || {
        format!("{}", class.get().unwrap_or_default())
    });

    view! {
        <button
            class={close_class}
            on:click=move |_| ctx.set_open.run(false)
        >
            {children.map(|c| c())}
        </button>
    }
}
