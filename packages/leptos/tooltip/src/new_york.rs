use leptos::{ev::MouseEvent, prelude::*};
use leptos_node_ref::AnyNodeRef;
use leptos_struct_component::{StructComponent, struct_component};
use leptos_style::Style;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(
    class = "z-50 overflow-hidden rounded-md border bg-popover px-3 py-1.5 text-sm text-popover-foreground shadow-md animate-in fade-in-0 zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2"
)]
pub struct TooltipContentClass {
    pub variant: TooltipVariant,
}

#[derive(PartialEq, TwVariant)]
pub enum TooltipVariant {
    #[tw(default, class = "")]
    Default,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TooltipSide {
    Top,
    Right,
    Bottom,
    Left,
}

impl TooltipSide {
    pub fn as_str(self) -> &'static str {
        match self {
            TooltipSide::Top => "top",
            TooltipSide::Right => "right", 
            TooltipSide::Bottom => "bottom",
            TooltipSide::Left => "left",
        }
    }
}

impl std::fmt::Display for TooltipSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for TooltipSide {
    fn default() -> Self {
        TooltipSide::Top
    }
}

#[derive(Clone, StructComponent)]
#[struct_component(tag = "div")]
pub struct TooltipContentChildProps {
    pub node_ref: AnyNodeRef,
    pub class: Signal<String>,
    pub id: MaybeProp<String>,
    pub style: Signal<Style>,
}

#[component]
pub fn TooltipProvider(#[prop(optional)] children: Option<Children>) -> impl IntoView {
    children.map(|children| children())
}

#[component]  
pub fn Tooltip(
    #[prop(into, optional)] open: Signal<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] delay_duration: Signal<u32>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let (is_open, set_is_open) = signal(open.get_untracked());
    
    Effect::new(move |_| {
        if open.get() != is_open.get() {
            set_is_open.set(open.get());
        }
    });

    provide_context((is_open, set_is_open, on_open_change, delay_duration));
    
    children.map(|children| children())
}

#[component]
pub fn TooltipTrigger(
    #[prop(into, optional)] as_child: Option<Callback<TooltipTriggerChildProps, AnyView>>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let (_is_open, set_is_open, on_open_change, _delay_duration) = 
        expect_context::<(ReadSignal<bool>, WriteSignal<bool>, Option<Callback<bool>>, Signal<u32>)>();

    let handle_mouse_enter = move |_: MouseEvent| {
        set_is_open.set(true);
        if let Some(callback) = on_open_change {
            callback.run(true);
        }
    };

    let handle_mouse_leave = move |_: MouseEvent| {
        set_is_open.set(false);
        if let Some(callback) = on_open_change {
            callback.run(false);
        }
    };

    let child_props = TooltipTriggerChildProps {
        node_ref,
        class: class.get().unwrap_or_default(),
        id,
        style,
        onmouseenter: Some(Callback::new(handle_mouse_enter)),
        onmouseleave: Some(Callback::new(handle_mouse_leave)),
    };

    if let Some(as_child) = as_child {
        as_child.run(child_props)
    } else {
        child_props.render(children)
    }
}

#[derive(Clone, StructComponent)]
#[struct_component(tag = "div")]
pub struct TooltipTriggerChildProps {
    pub node_ref: AnyNodeRef,
    pub class: String,
    pub id: MaybeProp<String>,
    pub style: Signal<Style>,
    pub onmouseenter: Option<Callback<MouseEvent>>,
    pub onmouseleave: Option<Callback<MouseEvent>>,
}

#[component]
pub fn TooltipContent(
    #[prop(into, optional)] _side: TooltipSide,
    #[prop(into, optional)] _side_offset: i32,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: Option<Callback<TooltipContentChildProps, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let (is_open, _, _, _) = 
        expect_context::<(ReadSignal<bool>, WriteSignal<bool>, Option<Callback<bool>>, Signal<u32>)>();

    let computed_class = Memo::new(move |_| {
        TooltipContentClass {
            variant: TooltipVariant::Default,
        }
        .with_class(class.get().unwrap_or_default())
    });

    let child_props = TooltipContentChildProps {
        node_ref,
        class: computed_class.into(),
        id,
        style,
    };

    if is_open.get() {
        if let Some(as_child) = as_child.as_ref() {
            as_child.run(child_props.clone())
        } else {
            child_props.render(children)
        }
    } else {
        view! { <div></div> }.into_any()
    }
}