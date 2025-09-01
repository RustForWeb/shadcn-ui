use yew::prelude::*;
use shadcn_ui_yew_select::*;

#[function_component]
pub fn SelectExamples() -> Html {
    html! {
        <div class="w-full max-w-4xl mx-auto p-6 space-y-8">
            <div class="space-y-2">
                <h1 class="text-3xl font-bold tracking-tight">{"Select"}</h1>
                <p class="text-lg text-muted-foreground">
                    {"Displays a list of options for the user to pick from—triggered by a button."}
                </p>
            </div>

            // Basic Example
            <section class="space-y-4">
                <h2 class="text-xl font-semibold">{"Basic Example"}</h2>
                <div class="flex items-center justify-center p-8 border rounded-lg">
                    <Select default_value={"apple"}>
                        <SelectTrigger class="w-[180px]">
                            <SelectValue placeholder={"Select a fruit"} />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value={"apple"}>{"Apple"}</SelectItem>
                            <SelectItem value={"banana"}>{"Banana"}</SelectItem>
                            <SelectItem value={"blueberry"}>{"Blueberry"}</SelectItem>
                            <SelectItem value={"grapes"}>{"Grapes"}</SelectItem>
                            <SelectItem value={"pineapple"}>{"Pineapple"}</SelectItem>
                        </SelectContent>
                    </Select>
                </div>
            </section>

            // Disabled Example
            <section class="space-y-4">
                <h2 class="text-xl font-semibold">{"Disabled"}</h2>
                <div class="flex items-center justify-center p-8 border rounded-lg">
                    <Select disabled={true}>
                        <SelectTrigger class="w-[180px]">
                            <SelectValue placeholder={"Select a fruit"} />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value={"apple"}>{"Apple"}</SelectItem>
                            <SelectItem value={"banana"}>{"Banana"}</SelectItem>
                            <SelectItem value={"blueberry"}>{"Blueberry"}</SelectItem>
                        </SelectContent>
                    </Select>
                </div>
            </section>

            // With Groups
            <section class="space-y-4">
                <h2 class="text-xl font-semibold">{"Grouped Options"}</h2>
                <div class="flex items-center justify-center p-8 border rounded-lg">
                    <Select>
                        <SelectTrigger class="w-[180px]">
                            <SelectValue placeholder={"Select a timezone"} />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectGroup>
                                <SelectLabel>{"North America"}</SelectLabel>
                                <SelectItem value={"est"}>{"Eastern Standard Time (EST)"}</SelectItem>
                                <SelectItem value={"cst"}>{"Central Standard Time (CST)"}</SelectItem>
                                <SelectItem value={"mst"}>{"Mountain Standard Time (MST)"}</SelectItem>
                                <SelectItem value={"pst"}>{"Pacific Standard Time (PST)"}</SelectItem>
                                <SelectItem value={"akst"}>{"Alaska Standard Time (AKST)"}</SelectItem>
                                <SelectItem value={"hst"}>{"Hawaii Standard Time (HST)"}</SelectItem>
                            </SelectGroup>
                            <SelectSeparator />
                            <SelectGroup>
                                <SelectLabel>{"Europe & Africa"}</SelectLabel>
                                <SelectItem value={"gmt"}>{"Greenwich Mean Time (GMT)"}</SelectItem>
                                <SelectItem value={"cet"}>{"Central European Time (CET)"}</SelectItem>
                                <SelectItem value={"eet"}>{"Eastern European Time (EET)"}</SelectItem>
                                <SelectItem value={"west"}>{"Western European Summer Time (WEST)"}</SelectItem>
                                <SelectItem value={"cat"}>{"Central Africa Time (CAT)"}</SelectItem>
                                <SelectItem value={"eat"}>{"East Africa Time (EAT)"}</SelectItem>
                            </SelectGroup>
                        </SelectContent>
                    </Select>
                </div>
            </section>

            // Controlled Example
            <section class="space-y-4">
                <h2 class="text-xl font-semibold">{"Controlled"}</h2>
                <div class="flex items-center justify-center gap-4 p-8 border rounded-lg">
                    <ControlledSelectExample />
                </div>
            </section>

            // Form Example
            <section class="space-y-4">
                <h2 class="text-xl font-semibold">{"Form"}</h2>
                <div class="flex items-center justify-center p-8 border rounded-lg">
                    <FormSelectExample />
                </div>
            </section>
        </div>
    }
}

#[function_component]
fn ControlledSelectExample() -> Html {
    let selected_value = use_state(|| "apple".to_string());
    
    let on_value_change = {
        let selected_value = selected_value.clone();
        Callback::from(move |value: String| {
            selected_value.set(value);
        })
    };

    html! {
        <div class="space-y-4">
            <Select value={(*selected_value).clone()} on_value_change={on_value_change}>
                <SelectTrigger class="w-[180px]">
                    <SelectValue placeholder={"Select a fruit"} />
                </SelectTrigger>
                <SelectContent>
                    <SelectItem value={"apple"}>{"Apple"}</SelectItem>
                    <SelectItem value={"banana"}>{"Banana"}</SelectItem>
                    <SelectItem value={"blueberry"}>{"Blueberry"}</SelectItem>
                    <SelectItem value={"grapes"}>{"Grapes"}</SelectItem>
                    <SelectItem value={"pineapple"}>{"Pineapple"}</SelectItem>
                </SelectContent>
            </Select>
            <p class="text-sm text-muted-foreground">
                {format!("Selected: {}", *selected_value)}
            </p>
        </div>
    }
}

#[function_component]
fn FormSelectExample() -> Html {
    let form_value = use_state(|| "".to_string());
    let error_message = use_state(|| "".to_string());

    let on_value_change = {
        let form_value = form_value.clone();
        let error_message = error_message.clone();
        Callback::from(move |value: String| {
            form_value.set(value);
            if !value.is_empty() {
                error_message.set("".to_string());
            }
        })
    };

    let on_submit = {
        let form_value = form_value.clone();
        let error_message = error_message.clone();
        Callback::from(move |e: web_sys::Event| {
            e.prevent_default();
            if form_value.is_empty() {
                error_message.set("Please select an option".to_string());
            } else {
                // Handle form submission
                web_sys::console::log_1(&format!("Form submitted with: {}", *form_value).into());
            }
        })
    };

    html! {
        <form onsubmit={on_submit} class="space-y-4">
            <div class="space-y-2">
                <label class="text-sm font-medium">{"Favorite Framework"}</label>
                <Select value={(*form_value).clone()} on_value_change={on_value_change} required={true}>
                    <SelectTrigger class="w-[180px]">
                        <SelectValue placeholder={"Select framework"} />
                    </SelectTrigger>
                    <SelectContent>
                        <SelectItem value={"yew"}>{"Yew"}</SelectItem>
                        <SelectItem value={"leptos"}>{"Leptos"}</SelectItem>
                        <SelectItem value={"dioxus"}>{"Dioxus"}</SelectItem>
                    </SelectContent>
                </Select>
                if !error_message.is_empty() {
                    <p class="text-sm text-destructive">{(*error_message).clone()}</p>
                }
            </div>
            <button 
                type="submit" 
                class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2"
            >
                {"Submit"}
            </button>
        </form>
    }
}