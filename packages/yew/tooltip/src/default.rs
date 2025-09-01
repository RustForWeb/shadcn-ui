use std::rc::Rc;
use yew::prelude::*;
use yew_struct_component::{StructComponent, struct_component};
use yew_style::Style;
use web_sys::{window, wasm_bindgen::{closure::Closure, JsCast}};

pub fn tooltip_content_class() -> &'static str {
    "z-50 overflow-hidden rounded-md border bg-popover px-3 py-1.5 text-sm text-popover-foreground shadow-md animate-in fade-in-0 zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2"
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

#[derive(Debug, Clone, PartialEq)]
pub struct TooltipContext {
    pub is_open: bool,
    pub set_open: Callback<bool>,
    pub delay_duration: u32,
}

#[derive(PartialEq, Properties)]
pub struct TooltipProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn TooltipProvider(props: &TooltipProviderProps) -> Html {
    props.children.clone()
}

#[derive(PartialEq, Properties)]
pub struct TooltipProps {
    #[prop_or(false)]
    pub open: bool,
    #[prop_or_default]
    pub on_open_change: Option<Callback<bool>>,
    #[prop_or(700)]
    pub delay_duration: u32,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Tooltip(props: &TooltipProps) -> Html {
    let is_open = use_state(|| props.open);
    
    let context = use_memo(
        (is_open.clone(), props.on_open_change.clone(), props.delay_duration),
        |(is_open, on_open_change, delay_duration)| {
            TooltipContext {
                is_open: **is_open,
                set_open: {
                    let is_open = is_open.clone();
                    let on_open_change = on_open_change.clone();
                    Callback::from(move |open| {
                        is_open.set(open);
                        if let Some(callback) = &on_open_change {
                            callback.emit(open);
                        }
                    })
                },
                delay_duration: *delay_duration,
            }
        },
    );

    // Update internal state when prop changes
    use_effect_with((props.open, is_open.clone()), |(open, is_open)| {
        if *open != **is_open {
            is_open.set(*open);
        }
    });

    html! {
        <ContextProvider<Rc<TooltipContext>> context={context.clone()}>
            {props.children.clone()}
        </ContextProvider<Rc<TooltipContext>>>
    }
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "div")]
pub struct TooltipTriggerChildProps {
    pub node_ref: NodeRef,
    pub class: String,
    pub id: Option<String>,
    pub style: Style,
    pub onmouseenter: Callback<MouseEvent>,
    pub onmouseleave: Callback<MouseEvent>,
}

#[derive(PartialEq, Properties)]
pub struct TooltipTriggerProps {
    #[prop_or_default]
    pub as_child: Option<Callback<TooltipTriggerChildProps, Html>>,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn TooltipTrigger(props: &TooltipTriggerProps) -> Html {
    let context = use_context::<Rc<TooltipContext>>()
        .expect("TooltipTrigger must be used within a Tooltip component");
    
    let timeout_handle = use_mut_ref(|| None::<i32>);

    let handle_mouse_enter = {
        let context = context.clone();
        let timeout_handle = timeout_handle.clone();
        Callback::from(move |_: MouseEvent| {
            // Clear any existing timeout
            if let Some(handle) = timeout_handle.borrow_mut().take() {
                if let Some(window) = window() {
                    window.clear_timeout_with_handle(handle);
                }
            }

            // Set new timeout
            let set_open = context.set_open.clone();
            let delay = context.delay_duration;
            
            if let Some(window) = window() {
                let closure = Closure::once_into_js(Box::new(move || {
                    set_open.emit(true);
                }) as Box<dyn FnOnce()>);
                
                if let Ok(handle) = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.unchecked_ref(),
                    delay as i32,
                ) {
                    *timeout_handle.borrow_mut() = Some(handle);
                }
            }
        })
    };

    let handle_mouse_leave = {
        let context = context.clone();
        let timeout_handle = timeout_handle.clone();
        Callback::from(move |_: MouseEvent| {
            // Clear any existing timeout
            if let Some(handle) = timeout_handle.borrow_mut().take() {
                if let Some(window) = window() {
                    window.clear_timeout_with_handle(handle);
                }
            }
            
            context.set_open.emit(false);
        })
    };

    let child_props = TooltipTriggerChildProps {
        node_ref: props.node_ref.clone(),
        class: props.class.clone().unwrap_or_default(),
        id: props.id.clone(),
        style: props.style.clone(),
        onmouseenter: handle_mouse_enter,
        onmouseleave: handle_mouse_leave,
    };

    if let Some(as_child) = &props.as_child {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "div")]
pub struct TooltipContentChildProps {
    pub node_ref: NodeRef,
    pub class: String,
    pub id: Option<String>,
    pub style: Style,
    pub data_side: String,
    pub data_state: String,
}

#[derive(PartialEq, Properties)]
pub struct TooltipContentProps {
    #[prop_or_default]
    pub side: TooltipSide,
    #[prop_or(4)]
    pub side_offset: i32,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub as_child: Option<Callback<TooltipContentChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn TooltipContent(props: &TooltipContentProps) -> Html {
    let context = use_context::<Rc<TooltipContext>>()
        .expect("TooltipContent must be used within a Tooltip component");

    let computed_class = use_memo(
        props.class.clone(),
        |class| {
            let base_class = tooltip_content_class();
            match class {
                Some(additional) => format!("{} {}", base_class, additional),
                None => base_class.to_string(),
            }
        }
    );

    let computed_style = use_memo(
        (props.style.clone(), props.side, props.side_offset),
        |(base_style, side, side_offset)| {
            let positioning = match side {
                TooltipSide::Top => format!("position: absolute; z-index: 50; bottom: {}px; left: 50%; transform: translateX(-50%);", side_offset),
                TooltipSide::Bottom => format!("position: absolute; z-index: 50; top: {}px; left: 50%; transform: translateX(-50%);", side_offset),
                TooltipSide::Left => format!("position: absolute; z-index: 50; right: {}px; top: 50%; transform: translateY(-50%);", side_offset),
                TooltipSide::Right => format!("position: absolute; z-index: 50; left: {}px; top: 50%; transform: translateY(-50%);", side_offset),
            };
            
            // Combine with base style if any
            if base_style.to_string().is_empty() {
                Style::from(positioning)
            } else {
                Style::from(format!("{}; {}", base_style.to_string(), positioning))
            }
        },
    );

    if !context.is_open {
        return html! {};
    }

    let child_props = TooltipContentChildProps {
        node_ref: props.node_ref.clone(),
        class: (*computed_class).clone(),
        id: props.id.clone(),
        style: (*computed_style).clone(),
        data_side: props.side.to_string(),
        data_state: if context.is_open { "open".to_string() } else { "closed".to_string() },
    };

    if let Some(as_child) = &props.as_child {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}