use leptos::prelude::*;
use shadcn_ui_leptos_radio_group::default::{RadioGroup, RadioGroupItem};

#[component]
pub fn RadioGroupExample() -> impl IntoView {
    let (selected_value, set_selected_value) = create_signal(None::<String>);
    
    let on_value_change = Callback::from(move |value: String| {
        set_selected_value.set(Some(value));
    });
    
    view! {
        <div class="space-y-4">
            <div class="space-y-2">
                <h3 class="text-lg font-medium">"Radio Group Example"</h3>
                <p class="text-sm text-muted-foreground">
                    "Select one option from the radio group below."
                </p>
            </div>
            
            <RadioGroup
                value=selected_value
                on_value_change=on_value_change
            >
                <div class="flex items-center space-x-2">
                    <RadioGroupItem value="option1".to_string() />
                    <label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                        "Option 1"
                    </label>
                </div>
                <div class="flex items-center space-x-2">
                    <RadioGroupItem value="option2".to_string() />
                    <label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                        "Option 2"
                    </label>
                </div>
                <div class="flex items-center space-x-2">
                    <RadioGroupItem value="option3".to_string() />
                    <label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                        "Option 3"
                    </label>
                </div>
            </RadioGroup>
            
            <div class="text-sm">
                <span class="font-medium">"Selected value: "</span>
                {move || selected_value.get().unwrap_or_else(|| "None".to_string())}
            </div>
        </div>
    }
}
