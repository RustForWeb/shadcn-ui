use leptos::prelude::*;
use shadcn_ui_leptos_combobox::{Combobox, ComboboxOption};

#[component]
pub fn ComboboxExample() -> impl IntoView {
    let (selected_value, set_selected_value) = signal(String::new());
    
    // Sample options for the combobox
    let options = vec![
        ComboboxOption::new("react", "React"),
        ComboboxOption::new("vue", "Vue.js"),
        ComboboxOption::new("angular", "Angular"),
        ComboboxOption::new("svelte", "Svelte"),
        ComboboxOption::new("leptos", "Leptos"),
        ComboboxOption::new("yew", "Yew"),
        ComboboxOption::new("dioxus", "Dioxus"),
        ComboboxOption::new("solid", "Solid.js"),
        ComboboxOption::new("preact", "Preact"),
        ComboboxOption::new("alpine", "Alpine.js"),
    ];
    
    let handle_change = Callback::new(move |value: String| {
        set_selected_value.set(value.clone());
        log::info!("Selected value: {}", value);
    });
    
    view! {
        <div class="p-8 space-y-6">
            <div class="space-y-2">
                <h2 class="text-2xl font-bold">Combobox Example</h2>
                <p class="text-muted-foreground">
                    "Select a framework from the dropdown or type to filter options."
                </p>
            </div>
            
            <div class="space-y-4">
                <div class="space-y-2">
                    <label class="text-sm font-medium">"Framework"</label>
                    <Combobox
                        value=Signal::derive(move || selected_value.get())
                        on_change=handle_change
                        placeholder="Select a framework..."
                        options=options
                    />
                </div>
                
                <div class="p-4 bg-muted rounded-md">
                    <p class="text-sm">
                        <span class="font-medium">"Selected: "</span>
                        {move || {
                            let value = selected_value.get();
                            if value.is_empty() {
                                "None".to_string()
                            } else {
                                value
                            }
                        }}
                    </p>
                </div>
            </div>
            
            <div class="space-y-4">
                <h3 class="text-lg font-semibold">"Features Demonstrated:"</h3>
                <ul class="list-disc list-inside space-y-1 text-sm text-muted-foreground">
                    <li>"Keyboard navigation (Arrow keys, Enter, Escape)"</li>
                    <li>"Type to filter options"</li>
                    <li>"Click to select options"</li>
                    <li>"Accessible focus management"</li>
                    <li>"Responsive design"</li>
                </ul>
            </div>
        </div>
    }
}
