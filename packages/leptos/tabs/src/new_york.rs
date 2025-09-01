use leptos::{ev::MouseEvent, prelude::*};
use leptos_node_ref::AnyNodeRef;
use leptos_struct_component::{StructComponent, struct_component};
use leptos_style::Style;

// Tabs Root Provider
#[component]
pub fn Tabs(
    #[prop(into, optional)] value: Signal<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let internal_value = RwSignal::new(default_value.get().unwrap_or_default());
    
    let value_state = Signal::derive(move || {
        if !value.get().is_empty() && value.get() != internal_value.get() {
            value.get()
        } else {
            internal_value.get()
        }
    });

    let set_value = Callback::new(move |new_value: String| {
        internal_value.set(new_value.clone());
        if let Some(callback) = &on_value_change {
            callback.run(new_value);
        }
    });

    provide_context(TabsContextValue {
        value: value_state,
        set_value,
    });

    let tabs_class = Signal::derive(move || {
        format!("{}", class.get().unwrap_or_default())
    });

    view! {
        <div class={tabs_class}>
            {children.map(|c| c())}
        </div>
    }
}

#[derive(Clone, Copy)]
pub struct TabsContextValue {
    pub value: Signal<String>,
    pub set_value: Callback<String>,
}

// Tabs List
#[component]
pub fn TabsList(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let list_class = Signal::derive(move || {
        format!("inline-flex h-10 items-center justify-center rounded-md bg-muted p-1 text-muted-foreground {}", class.get().unwrap_or_default())
    });

    view! {
        <div class={list_class} role="tablist">
            {children.map(|c| c())}
        </div>
    }
}

// Tabs Trigger
#[derive(Clone, StructComponent)]
#[struct_component(tag = "button")]
pub struct TabsTriggerChildProps {
    pub node_ref: AnyNodeRef,
    pub class: Signal<String>,
    pub id: MaybeProp<String>,
    pub style: Signal<Style>,
    pub disabled: Signal<bool>,
    pub r#type: MaybeProp<String>,
    pub role: Signal<String>,
    pub aria_selected: Signal<String>,
    pub onclick: Option<Callback<MouseEvent>>,
}

#[component]
pub fn TabsTrigger(
    #[prop(into)] value: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: Option<Callback<TabsTriggerChildProps, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = expect_context::<TabsContextValue>();
    
    let is_selected = Signal::derive(move || {
        ctx.value.get() == value.get().unwrap_or_default()
    });

    let trigger_class = Signal::derive(move || {
        let base_class = "inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";
        let selected_class = if is_selected.get() {
            "bg-background text-foreground shadow-sm"
        } else {
            "hover:bg-background hover:text-foreground"
        };
        format!("{} {} {}", base_class, selected_class, class.get().unwrap_or_default())
    });

    let child_props = TabsTriggerChildProps {
        node_ref,
        class: trigger_class,
        id,
        style,
        disabled: Signal::derive(|| false),
        r#type: "button".to_string().into(),
        role: "tab".to_string().into(),
        aria_selected: Signal::derive(move || is_selected.get().to_string()).into(),
        onclick: Some(Callback::new({
            let ctx = ctx.clone();
            let value = value.clone();
            move |_: MouseEvent| {
                let val = value.get().unwrap_or_default();
                ctx.set_value.run(val);
            }
        })),
    };

    if let Some(as_child) = as_child.as_ref() {
        as_child.run(child_props)
    } else {
        child_props.render(children)
    }
}

// Tabs Content
#[component]
pub fn TabsContent(
    #[prop(into)] value: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = expect_context::<TabsContextValue>();
    
    let is_selected = Signal::derive(move || {
        ctx.value.get() == value.get().unwrap_or_default()
    });

    let content_class = Signal::derive(move || {
        format!("mt-2 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 {}", class.get().unwrap_or_default())
    });

    view! {
        <div
            class={content_class}
            role="tabpanel"
            aria-selected={move || is_selected.get().to_string()}
            style={move || if is_selected.get() { "" } else { "display: none;" }}
        >
            {children.map(|c| c())}
        </div>
    }
}
