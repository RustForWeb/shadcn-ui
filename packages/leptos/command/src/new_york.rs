use leptos::prelude::*;
use tailwind_fuse::tw_merge;

const COMMAND_CLASS: &str = "flex h-full w-full flex-col overflow-hidden rounded-md bg-popover text-popover-foreground shadow-md";
const COMMAND_INPUT_CLASS: &str = "flex items-center border-b px-3";
const COMMAND_INPUT_WRAPPER_CLASS: &str = "flex h-10 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50";
const COMMAND_LIST_CLASS: &str = "max-h-[300px] overflow-y-auto overflow-x-hidden";
const COMMAND_EMPTY_CLASS: &str = "py-6 text-center text-sm";
const COMMAND_GROUP_CLASS: &str = "overflow-hidden p-1 text-foreground";
const COMMAND_GROUP_HEADING_CLASS: &str = "px-2 py-1.5 text-xs font-medium text-muted-foreground";
const COMMAND_ITEM_CLASS: &str = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none hover:bg-accent hover:text-accent-foreground data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50";
const COMMAND_SHORTCUT_CLASS: &str = "ml-auto text-xs tracking-widest text-muted-foreground";
const COMMAND_SEPARATOR_CLASS: &str = "-mx-1 h-px bg-border";

#[component]
pub fn Command(
    #[prop(optional)] value: MaybeProp<String>,
    #[prop(optional)] on_value_change: Option<Callback<String>>,
    #[prop(optional)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let search = RwSignal::new(String::new());
    let selected_value = RwSignal::new(value.get().unwrap_or_default());
    
    Effect::new(move |_| {
        if let Some(new_value) = value.get() {
            selected_value.set(new_value);
        }
    });
    
    let merged_class = tw_merge!(&format!("{} {}", 
        COMMAND_CLASS,
        class.get().unwrap_or_default()
    ));
    
    provide_context(CommandContext {
        search,
        selected_value,
        on_value_change,
    });
    
    view! {
        <div 
            class={merged_class}
            role="combobox"
            aria-expanded="true"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CommandInput(
    #[prop(optional)] placeholder: MaybeProp<String>,
    #[prop(optional)] value: MaybeProp<String>,
    #[prop(optional)] on_value_change: Option<Callback<String>>,
    #[prop(optional)] class: MaybeProp<String>,
) -> impl IntoView {
    let context = expect_context::<CommandContext>();
    let input_ref = NodeRef::<leptos::html::Input>::new();
    let input_value = RwSignal::new(value.get().unwrap_or_default());
    
    let merged_class = tw_merge!(&format!("{} {}", 
        COMMAND_INPUT_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <div class={merged_class}>
            <svg 
                width="15" 
                height="15" 
                viewBox="0 0 15 15" 
                fill="none" 
                xmlns="http://www.w3.org/2000/svg"
                class="mr-2 h-4 w-4 shrink-0 opacity-50"
            >
                <path 
                    d="M10 6.5C10 8.433 8.433 10 6.5 10C4.567 10 3 8.433 3 6.5C3 4.567 4.567 3 6.5 3C8.433 3 10 4.567 10 6.5ZM9.30884 10.0159C8.53901 10.6318 7.56251 11 6.5 11C4.01472 11 2 8.98528 2 6.5C2 4.01472 4.01472 2 6.5 2C8.98528 2 11 4.01472 11 6.5C11 7.56251 10.6318 8.53901 10.0159 9.30884L12.8536 12.1464C13.0488 12.3417 13.0488 12.6583 12.8536 12.8536C12.6583 13.0488 12.3417 13.0488 12.1464 12.8536L9.30884 10.0159Z" 
                    fill="currentColor" 
                    fill-rule="evenodd" 
                    clip-rule="evenodd"
                />
            </svg>
            <input
                node_ref=input_ref
                class=COMMAND_INPUT_WRAPPER_CLASS
                placeholder={placeholder.get().unwrap_or("Type a command or search...".to_string())}
                prop:value={input_value}
                on:input=move |evt| {
                    let value = event_target_value(&evt);
                    input_value.set(value.clone());
                    context.search.set(value.clone());
                    
                    if let Some(on_value_change) = on_value_change {
                        on_value_change.run(value);
                    }
                }
                autocomplete="off"
                spellcheck="false"
                aria-autocomplete="list"
                role="combobox"
                aria-expanded="true"
            />
        </div>
    }
}

#[component]
pub fn CommandList(
    #[prop(optional)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let merged_class = tw_merge!(&format!("{} {}", 
        COMMAND_LIST_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <div 
            class={merged_class}
            role="listbox"
            aria-label="Suggestions"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CommandEmpty(
    #[prop(optional)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let context = expect_context::<CommandContext>();
    let _search = context.search;
    
    let merged_class = tw_merge!(&format!("{} {}", 
        COMMAND_EMPTY_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <div class={merged_class}>
            {children()}
        </div>
    }
}

#[component]
pub fn CommandGroup(
    #[prop(optional)] heading: MaybeProp<String>,
    #[prop(optional)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let merged_class = tw_merge!(&format!("{} {}", 
        COMMAND_GROUP_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <div class={merged_class} role="group">
            {if let Some(heading_text) = heading.get() {
                view! {
                    <div class=COMMAND_GROUP_HEADING_CLASS role="presentation">
                        {heading_text}
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }}
            {children()}
        </div>
    }
}

#[component]
pub fn CommandItem(
    #[prop(optional)] value: MaybeProp<String>,
    #[prop(optional)] keywords: MaybeProp<Vec<String>>,
    #[prop(optional)] disabled: MaybeProp<bool>,
    #[prop(optional)] on_select: Option<Callback<String>>,
    #[prop(optional)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let context = expect_context::<CommandContext>();
    let search = context.search;
    let selected_value = context.selected_value;
    
    let item_value = value.get().unwrap_or_default();
    let item_keywords = keywords.get().unwrap_or_default();
    let is_disabled = disabled.get().unwrap_or(false);
    
    let item_value_for_search = item_value.clone();
    let item_keywords_for_search = item_keywords.clone();
    let matches_search = Memo::new(move |_| {
        let search_term = search.get();
        if search_term.is_empty() {
            return true;
        }
        
        let search_lower = search_term.to_lowercase();
        
        if item_value_for_search.to_lowercase().contains(&search_lower) {
            return true;
        }
        
        for keyword in &item_keywords_for_search {
            if keyword.to_lowercase().contains(&search_lower) {
                return true;
            }
        }
        
        false
    });
    
    let item_value_for_selected = item_value.clone();
    let is_selected = Memo::new(move |_| {
        selected_value.get() == item_value_for_selected
    });
    
    let merged_class = tw_merge!(&format!("{} {}", 
        COMMAND_ITEM_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <div
            class={merged_class}
            role="option"
            aria-selected={is_selected.get()}
            data-disabled={is_disabled}
            on:click=move |_evt| {
                if !is_disabled {
                    selected_value.set(item_value.clone());
                    
                    if let Some(on_select) = on_select {
                        on_select.run(item_value.clone());
                    }
                    
                    if let Some(on_value_change) = context.on_value_change {
                        on_value_change.run(item_value.clone());
                    }
                }
            }
            style=("display", if matches_search.get() { "flex" } else { "none" })
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CommandShortcut(
    #[prop(optional)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let merged_class = tw_merge!(&format!("{} {}", 
        COMMAND_SHORTCUT_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <span class={merged_class}>
            {children()}
        </span>
    }
}

#[component]
pub fn CommandSeparator(
    #[prop(optional)] class: MaybeProp<String>,
) -> impl IntoView {
    let merged_class = tw_merge!(&format!("{} {}", 
        COMMAND_SEPARATOR_CLASS,
        class.get().unwrap_or_default()
    ));
    
    view! {
        <div 
            class={merged_class} 
            role="separator" 
            aria-orientation="horizontal"
        />
    }
}

#[derive(Clone, Copy)]
struct CommandContext {
    search: RwSignal<String>,
    selected_value: RwSignal<String>,
    on_value_change: Option<Callback<String>>,
}