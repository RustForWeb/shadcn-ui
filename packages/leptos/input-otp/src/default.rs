use leptos::prelude::*;
use leptos::web_sys;
use tailwind_fuse::tw_merge;

const INPUT_OTP_CLASS: &str = "flex items-center gap-2";
const INPUT_OTP_SLOT_CLASS: &str = "relative flex h-10 w-10 items-center justify-center border-y border-r border-input text-sm transition-all first:rounded-l-md first:border-l last:rounded-r-md focus-within:z-10 focus-within:ring-2 focus-within:ring-ring focus-within:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

#[component]
pub fn InputOtp(
    #[prop(default = 6)] max_length: usize,
    #[prop(optional)] value: MaybeProp<String>,
    #[prop(optional)] on_change: Option<Callback<String>>,
    #[prop(optional)] on_complete: Option<Callback<String>>,
    #[prop(optional)] disabled: MaybeProp<bool>,
    #[prop(optional)] class: MaybeProp<String>,
) -> impl IntoView {
    let current_value = RwSignal::new(value.get().unwrap_or_default());
    let active_slot = RwSignal::new(0_usize);
    let is_disabled = disabled.get().unwrap_or(false);
    
    // Update value when prop changes
    Effect::new(move |_| {
        if let Some(new_value) = value.get() {
            current_value.set(new_value);
        }
    });
    
    let handle_input = {
        let current_value = current_value.clone();
        let active_slot = active_slot.clone();
        
        Callback::new(move |input_value: String| {
            let sanitized: String = input_value.chars()
                .filter(|c| c.is_ascii_alphanumeric())
                .take(max_length)
                .collect();
                
            current_value.set(sanitized.clone());
            
            // Update active slot
            let next_slot = sanitized.len().min(max_length - 1);
            active_slot.set(next_slot);
            
            // Call callbacks
            if let Some(on_change) = on_change {
                on_change.run(sanitized.clone());
            }
            
            if sanitized.len() == max_length {
                if let Some(on_complete) = on_complete {
                    on_complete.run(sanitized);
                }
            }
        })
    };
    
    let handle_keydown = {
        let current_value = current_value.clone();
        let active_slot = active_slot.clone();
        
        Callback::new(move |evt: web_sys::KeyboardEvent| {
            let key = evt.key();
            let current_val = current_value.get();
            let mut chars: Vec<char> = current_val.chars().collect();
            let active = active_slot.get();
            
            match key.as_str() {
                "Backspace" => {
                    if active > 0 && active <= chars.len() {
                        chars.remove(active - 1);
                        let new_value: String = chars.into_iter().collect();
                        handle_input.run(new_value);
                        active_slot.set((active - 1).max(0));
                    }
                    evt.prevent_default();
                }
                "ArrowLeft" => {
                    active_slot.set(active.saturating_sub(1));
                    evt.prevent_default();
                }
                "ArrowRight" => {
                    active_slot.set((active + 1).min(max_length - 1));
                    evt.prevent_default();
                }
                "Delete" => {
                    if active < chars.len() {
                        chars.remove(active);
                        let new_value: String = chars.into_iter().collect();
                        handle_input.run(new_value);
                    }
                    evt.prevent_default();
                }
                _ => {
                    if key.len() == 1 && key.chars().next().unwrap().is_ascii_alphanumeric() {
                        let new_char = key.chars().next().unwrap();
                        
                        if active < max_length {
                            if active >= chars.len() {
                                chars.push(new_char);
                            } else {
                                chars[active] = new_char;
                            }
                            let new_value: String = chars.into_iter().collect();
                            handle_input.run(new_value);
                        }
                        evt.prevent_default();
                    }
                }
            }
        })
    };
    
    let merged_class = tw_merge!(&format!("{} {}", 
        INPUT_OTP_CLASS,
        class.get().unwrap_or_default()
    ));
    
    let slots = move || {
        let val = current_value.get();
        let chars: Vec<char> = val.chars().collect();
        let active = active_slot.get();
        
        (0..max_length).map(|i| {
            let char_value = chars.get(i).map(|c| c.to_string()).unwrap_or_default();
            let is_active = i == active;
            
            let slot_class = if is_active {
                tw_merge!(&format!("{} {}", INPUT_OTP_SLOT_CLASS, "z-10 ring-2 ring-ring ring-offset-2"))
            } else {
                INPUT_OTP_SLOT_CLASS.to_string()
            };
            
            view! {
                <div class={slot_class}>
                    <input
                        type="text"
                        class="absolute inset-0 w-full h-full text-center bg-transparent border-none outline-none focus:outline-none"
                        value={char_value.clone()}
                        maxlength="1"
                        disabled=is_disabled
                        aria-label={format!("Digit {}", i + 1)}
                        on:keydown={
                            let handle_keydown = handle_keydown.clone();
                            move |evt| handle_keydown.run(evt)
                        }
                        readonly=true
                    />
                    <div class="pointer-events-none absolute inset-0 flex items-center justify-center">
                        {if char_value.is_empty() && is_active {
                            view! { <div class="h-4 w-px bg-foreground animate-pulse" /> }.into_any()
                        } else {
                            view! { <span>{char_value}</span> }.into_any()
                        }}
                    </div>
                </div>
            }
        }).collect::<Vec<_>>()
    };
    
    view! {
        <div 
            class={merged_class}
            role="group"
            aria-label="One-time password input"
        >
            {slots()}
        </div>
    }
}

#[component]
pub fn InputOtpSeparator(
    #[prop(optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let merged_class = tw_merge!(&format!("flex w-px items-center justify-center {}",
        class.get().unwrap_or_default()
    ));
    
    view! {
        <div class={merged_class}>
            {if let Some(children) = children {
                children().into_any()
            } else {
                view! { <div class="h-4 w-px bg-border" /> }.into_any()
            }}
        </div>
    }
}