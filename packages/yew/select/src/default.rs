use tailwind_fuse::*;
use yew::prelude::*;
use yew_style::Style;
use web_sys::MouseEvent;

// Select Root Provider
#[derive(PartialEq, Properties)]
pub struct SelectProps {
    #[prop_or_default]
    pub open: Option<bool>,
    #[prop_or_default]
    pub on_open_change: Option<Callback<bool>>,
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub on_value_change: Option<Callback<String>>,
    #[prop_or_default]
    pub default_value: Option<String>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq)]
pub struct SelectContext {
    pub open: bool,
    pub set_open: Callback<bool>,
    pub value: String,
    pub set_value: Callback<String>,
    pub disabled: bool,
    pub required: bool,
    pub name: Option<String>,
}

pub type SelectContextProvider = ContextProvider<SelectContext>;

#[function_component]
pub fn Select(props: &SelectProps) -> Html {
    let internal_open = use_state(|| props.open.unwrap_or(false));
    let internal_value = use_state(|| props.default_value.clone().unwrap_or_default());
    
    let open_state = if let Some(controlled_open) = props.open {
        controlled_open
    } else {
        *internal_open
    };
    
    let value_state = if let Some(controlled_value) = &props.value {
        controlled_value.clone()
    } else {
        (*internal_value).clone()
    };

    let set_open = {
        let internal_open = internal_open.clone();
        let on_open_change = props.on_open_change.clone();
        Callback::from(move |open: bool| {
            internal_open.set(open);
            if let Some(callback) = &on_open_change {
                callback.emit(open);
            }
        })
    };

    let set_value = {
        let internal_value = internal_value.clone();
        let on_value_change = props.on_value_change.clone();
        Callback::from(move |value: String| {
            internal_value.set(value.clone());
            if let Some(callback) = &on_value_change {
                callback.emit(value);
            }
        })
    };

    let context = SelectContext {
        open: open_state,
        set_open,
        value: value_state,
        set_value,
        disabled: props.disabled,
        required: props.required,
        name: props.name.clone(),
    };

    html! {
        <ContextProvider<SelectContext> context={context}>
            <div class="relative">
                {props.children.clone()}
            </div>
        </ContextProvider<SelectContext>>
    }
}

// Select Trigger

#[derive(PartialEq, Properties)]
pub struct SelectTriggerProps {
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectTrigger(props: &SelectTriggerProps) -> Html {
    let context = use_context::<SelectContext>().expect("SelectTrigger must be used within Select");
    
    let trigger_class = use_memo(
        props.class.clone(),
        |class| {
            tw_merge!(
                "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1",
                &class.clone().unwrap_or_default()
            )
        },
    );

    let handle_click = {
        let context = context.clone();
        Callback::from(move |_: MouseEvent| {
            if !context.disabled {
                context.set_open.emit(!context.open);
            }
        })
    };

    html! {
        <button
            ref={props.node_ref.clone()}
            class={(*trigger_class).clone()}
            id={props.id.clone()}
            style={props.style.clone()}
            disabled={context.disabled}
            type="button"
            role="combobox"
            aria-haspopup="listbox"
            aria-expanded={context.open.to_string()}
            onclick={handle_click}
        >
            {props.children.clone()}
        </button>
    }
}

// Select Value placeholder
#[derive(PartialEq, Properties)]
pub struct SelectValueProps {
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
}

#[function_component]
pub fn SelectValue(props: &SelectValueProps) -> Html {
    let context = use_context::<SelectContext>().expect("SelectValue must be used within Select");
    
    let display_text = if context.value.is_empty() {
        props.placeholder.clone().unwrap_or_default()
    } else {
        context.value.clone()
    };

    html! {
        <span class={props.class.clone()}>{display_text}</span>
    }
}

// Select Content (dropdown)

#[derive(PartialEq, Properties)]
pub struct SelectContentProps {
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Style,
    #[prop_or_default]
    pub position: Option<String>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectContent(props: &SelectContentProps) -> Html {
    let context = use_context::<SelectContext>().expect("SelectContent must be used within Select");
    
    let content_class = use_memo(
        props.class.clone(),
        |class| {
            tw_merge!(
                "relative z-50 max-h-96 min-w-[8rem] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
                &class.clone().unwrap_or_default()
            )
        },
    );

    let handle_backdrop_click = {
        let context = context.clone();
        Callback::from(move |_: MouseEvent| {
            context.set_open.emit(false);
        })
    };

    let handle_content_click = Callback::from(|e: MouseEvent| {
        e.stop_propagation();
    });

    if !context.open {
        return html! {};
    }

    html! {
        <div 
            class="fixed inset-0 z-50"
            onclick={handle_backdrop_click}
        >
            <div
                class={(*content_class).clone()}
                style={format!("position: absolute; {}", props.style.to_string())}
                role="listbox"
                onclick={handle_content_click}
            >
                {props.children.clone()}
            </div>
        </div>
    }
}

// Select Item

#[derive(PartialEq, Properties)]
pub struct SelectItemProps {
    pub value: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectItem(props: &SelectItemProps) -> Html {
    let context = use_context::<SelectContext>().expect("SelectItem must be used within Select");
    
    let item_class = use_memo(
        props.class.clone(),
        |class| {
            tw_merge!(
                "relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
                &class.clone().unwrap_or_default()
            )
        },
    );

    let is_selected = context.value == props.value;

    let handle_click = {
        let context = context.clone();
        let value = props.value.clone();
        let disabled = props.disabled;
        Callback::from(move |_: MouseEvent| {
            if !disabled {
                context.set_value.emit(value.clone());
                context.set_open.emit(false);
            }
        })
    };

    html! {
        <div
            class={(*item_class).clone()}
            role="option"
            aria-selected={is_selected.to_string()}
            data-disabled={if props.disabled { "true" } else { "false" }}
            onclick={handle_click}
        >
            if is_selected {
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
            }
            {props.children.clone()}
        </div>
    }
}

// Select Group (optional for organizing items)
#[derive(PartialEq, Properties)]
pub struct SelectGroupProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectGroup(props: &SelectGroupProps) -> Html {
    html! {
        <div role="group">
            {props.children.clone()}
        </div>
    }
}

// Select Label (for group labels)

#[derive(PartialEq, Properties)]
pub struct SelectLabelProps {
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectLabel(props: &SelectLabelProps) -> Html {
    let label_class = use_memo(
        props.class.clone(),
        |class| {
            tw_merge!(
                "py-1.5 pl-8 pr-2 text-sm font-semibold",
                &class.clone().unwrap_or_default()
            )
        },
    );

    html! {
        <div class={(*label_class).clone()} role="presentation">
            {props.children.clone()}
        </div>
    }
}

// Select Separator (for separating groups)

#[derive(PartialEq, Properties)]
pub struct SelectSeparatorProps {
    #[prop_or_default]
    pub class: Option<String>,
}

#[function_component]
pub fn SelectSeparator(props: &SelectSeparatorProps) -> Html {
    let separator_class = use_memo(
        props.class.clone(),
        |class| {
            tw_merge!(
                "-mx-1 my-1 h-px bg-muted",
                &class.clone().unwrap_or_default()
            )
        },
    );

    html! {
        <div class={(*separator_class).clone()} role="separator" aria-orientation="horizontal" />
    }
}