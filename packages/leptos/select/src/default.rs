use leptos::{ev::MouseEvent, prelude::*};
use leptos_node_ref::AnyNodeRef;
use leptos_struct_component::{StructComponent, struct_component};
use leptos_style::Style;
use tailwind_fuse::*;

// Select Root Provider
#[component]
pub fn Select(
    #[prop(into, optional)] open: Signal<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] value: Signal<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] required: Signal<bool>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let internal_open = RwSignal::new(false);
    let internal_value = RwSignal::new(default_value.get().unwrap_or_default());
    
    let open_state = Signal::derive(move || {
        if open.get() != internal_open.get() {
            open.get()
        } else {
            internal_open.get()
        }
    });
    
    let value_state = Signal::derive(move || {
        if !value.get().is_empty() && value.get() != internal_value.get() {
            value.get()
        } else {
            internal_value.get()
        }
    });

    let set_open = Callback::new(move |new_open: bool| {
        internal_open.set(new_open);
        if let Some(callback) = &on_open_change {
            callback.run(new_open);
        }
    });

    let set_value = Callback::new(move |new_value: String| {
        internal_value.set(new_value.clone());
        if let Some(callback) = &on_value_change {
            callback.run(new_value);
        }
    });

    provide_context(SelectContextValue {
        open: open_state,
        set_open,
        value: value_state,
        set_value,
        disabled,
        required,
        name,
    });

    view! {
        <div class="relative">
            {children.map(|c| c())}
        </div>
    }
}

#[derive(Clone, Copy)]
pub struct SelectContextValue {
    pub open: Signal<bool>,
    pub set_open: Callback<bool>,
    pub value: Signal<String>,
    pub set_value: Callback<String>,
    pub disabled: Signal<bool>,
    pub required: Signal<bool>,
    pub name: MaybeProp<String>,
}

// Select Trigger
#[derive(Clone, StructComponent)]
#[struct_component(tag = "button")]
pub struct SelectTriggerChildProps {
    pub node_ref: AnyNodeRef,
    pub class: Signal<String>,
    pub id: MaybeProp<String>,
    pub style: Signal<Style>,
    pub disabled: Signal<bool>,
    pub r#type: MaybeProp<String>,
    pub role: Signal<String>,
    pub aria_haspopup: Signal<String>,
    pub aria_expanded: Signal<String>,
    pub aria_controls: MaybeProp<String>,
    pub onclick: Option<Callback<MouseEvent>>,
}

#[component]
pub fn SelectTrigger(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: Option<Callback<SelectTriggerChildProps, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = expect_context::<SelectContextValue>();
    
    let trigger_class = Memo::new(move |_| {
        tw_merge!(
            "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1",
            &class.get().unwrap_or_default()
        )
    });

    let handle_click = Callback::new(move |_: MouseEvent| {
        if !ctx.disabled.get() {
            ctx.set_open.run(!ctx.open.get());
        }
    });

    let child_props = SelectTriggerChildProps {
        node_ref,
        class: trigger_class.into(),
        id,
        style,
        disabled: ctx.disabled,
        r#type: "button".to_string().into(),
        role: "combobox".to_string().into(),
        aria_haspopup: "listbox".to_string().into(),
        aria_expanded: Signal::derive(move || ctx.open.get().to_string()).into(),
        aria_controls: None::<String>.into(),
        onclick: Some(handle_click),
    };

    if let Some(as_child) = as_child.as_ref() {
        as_child.run(child_props)
    } else {
        child_props.render(children)
    }
}

// Select Value placeholder
#[component]
pub fn SelectValue(
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
) -> impl IntoView {
    let ctx = expect_context::<SelectContextValue>();
    
    let display_text = Signal::derive(move || {
        let value = ctx.value.get();
        if value.is_empty() {
            placeholder.get().unwrap_or_default()
        } else {
            value
        }
    });

    view! {
        <span class={class.get()}>{display_text}</span>
    }
}

// Select Content (dropdown)
#[component]
pub fn SelectContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] _position: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = expect_context::<SelectContextValue>();
    
    let content_class = Memo::new(move |_| {
        tw_merge!(
            "relative z-50 max-h-96 min-w-[8rem] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
            &class.get().unwrap_or_default()
        )
    });

    if ctx.open.get() {
        view! {
            <div 
                class="fixed inset-0 z-50"
                on:click=move |_| ctx.set_open.run(false)
            >
                <div
                    class={content_class}
                    style={move || format!("position: absolute; {}", style.get().to_string())}
                    role="listbox"
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

// Select Item
#[component]
pub fn SelectItem(
    #[prop(into)] value: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = expect_context::<SelectContextValue>();
    
    let item_class = Memo::new(move |_| {
        tw_merge!(
            "relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
            &class.get().unwrap_or_default()
        )
    });

    let is_selected = Signal::derive(move || {
        ctx.value.get() == value.get().unwrap_or_default()
    });

    let handle_click = {
        let ctx = ctx.clone();
        let value = value.clone();
        let disabled = disabled.clone();
        move |_: MouseEvent| {
            if !disabled.get() {
                let val = value.get().unwrap_or_default();
                ctx.set_value.run(val);
                ctx.set_open.run(false);
            }
        }
    };

    view! {
        <div
            class={item_class}
            role="option"
            aria-selected={move || is_selected.get().to_string()}
            data-disabled={move || if disabled.get() { "true" } else { "false" }}
            on:click=handle_click
        >
            <Show when=move || is_selected.get()>
                <span class="absolute left-2 flex h-3.5 w-3.5 items-center justify-center">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="h-4 w-4"
                    >
                        <path d="M20 6 9 17l-5-5"/>
                    </svg>
                </span>
            </Show>
            {children.map(|c| c())}
        </div>
    }
}

// Select Group (optional for organizing items)
#[component]
pub fn SelectGroup(
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div role="group">
            {children.map(|c| c())}
        </div>
    }
}

// Select Label (for group labels)
#[component]
pub fn SelectLabel(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let label_class = Memo::new(move |_| {
        tw_merge!(
            "py-1.5 pl-8 pr-2 text-sm font-semibold",
            &class.get().unwrap_or_default()
        )
    });

    view! {
        <div class={label_class} role="presentation">
            {children.map(|c| c())}
        </div>
    }
}

// Select Separator (for separating groups)
#[component]
pub fn SelectSeparator(
    #[prop(into, optional)] class: MaybeProp<String>,
) -> impl IntoView {
    let separator_class = Memo::new(move |_| {
        tw_merge!(
            "-mx-1 my-1 h-px bg-muted",
            &class.get().unwrap_or_default()
        )
    });

    view! {
        <div class={separator_class} role="separator" aria-orientation="horizontal" />
    }
}