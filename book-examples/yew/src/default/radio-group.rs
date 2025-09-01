use yew::prelude::*;
use shadcn_ui_yew_radio_group::default::{RadioGroup, RadioGroupItem};

#[function_component]
pub fn RadioGroupExample() -> Html {
    let selected_value = use_state(|| None::<String>);
    
    let on_value_change = {
        let selected_value = selected_value.clone();
        Callback::from(move |value: String| {
            selected_value.set(Some(value));
        })
    };
    
    html! {
        <div class="space-y-4">
            <div class="space-y-2">
                <h3 class="text-lg font-medium">{"Radio Group Example"}</h3>
                <p class="text-sm text-muted-foreground">
                    {"Select one option from the radio group below."}
                </p>
            </div>
            
            <RadioGroup
                value={(*selected_value).clone()}
                on_value_change={Some(on_value_change)}
            >
                <div class="flex items-center space-x-2">
                    <RadioGroupItem value="option1".to_string() />
                    <label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                        {"Option 1"}
                    </label>
                </div>
                <div class="flex items-center space-x-2">
                    <RadioGroupItem value="option2".to_string() />
                    <label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                        {"Option 2"}
                    </label>
                </div>
                <div class="flex items-center space-x-2">
                    <RadioGroupItem value="option3".to_string() />
                    <label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                        {"Option 3"}
                    </label>
                </div>
            </RadioGroup>
            
            <div class="text-sm">
                <span class="font-medium">{"Selected value: "}</span>
                {selected_value.as_ref().unwrap_or(&"None".to_string())}
            </div>
        </div>
    }
}
