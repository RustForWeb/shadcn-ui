use yew::prelude::*;
use shadcn_ui_yew_tooltip::*;

#[function_component]
pub fn TooltipExamples() -> Html {
    html! {
        <div class="w-full max-w-4xl mx-auto p-6 space-y-8">
            <div class="space-y-2">
                <h1 class="text-3xl font-bold tracking-tight">{"Tooltip"}</h1>
                <p class="text-lg text-muted-foreground">
                    {"A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it."}
                </p>
            </div>

            // Basic Example
            <section class="space-y-4">
                <h2 class="text-xl font-semibold">{"Basic Example"}</h2>
                <div class="flex items-center justify-center p-8 border rounded-lg">
                    <TooltipProvider>
                        <Tooltip>
                            <TooltipTrigger>
                                <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2">
                                    {"Hover me"}
                                </button>
                            </TooltipTrigger>
                            <TooltipContent>
                                <p>{"Add to library"}</p>
                            </TooltipContent>
                        </Tooltip>
                    </TooltipProvider>
                </div>
            </section>

            // Positioning Examples
            <section class="space-y-4">
                <h2 class="text-xl font-semibold">{"Positioning"}</h2>
                <div class="grid grid-cols-2 gap-4 p-8 border rounded-lg">
                    <TooltipProvider>
                        <div class="grid grid-cols-2 gap-4">
                            <Tooltip>
                                <TooltipTrigger>
                                    <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2">
                                        {"Top"}
                                    </button>
                                </TooltipTrigger>
                                <TooltipContent side={TooltipSide::Top}>
                                    <p>{"Tooltip on top"}</p>
                                </TooltipContent>
                            </Tooltip>

                            <Tooltip>
                                <TooltipTrigger>
                                    <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2">
                                        {"Right"}
                                    </button>
                                </TooltipTrigger>
                                <TooltipContent side={TooltipSide::Right}>
                                    <p>{"Tooltip on right"}</p>
                                </TooltipContent>
                            </Tooltip>

                            <Tooltip>
                                <TooltipTrigger>
                                    <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2">
                                        {"Bottom"}
                                    </button>
                                </TooltipTrigger>
                                <TooltipContent side={TooltipSide::Bottom}>
                                    <p>{"Tooltip on bottom"}</p>
                                </TooltipContent>
                            </Tooltip>

                            <Tooltip>
                                <TooltipTrigger>
                                    <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2">
                                        {"Left"}
                                    </button>
                                </TooltipTrigger>
                                <TooltipContent side={TooltipSide::Left}>
                                    <p>{"Tooltip on left"}</p>
                                </TooltipContent>
                            </Tooltip>
                        </div>
                    </TooltipProvider>
                </div>
            </section>

            // Controlled Example
            <section class="space-y-4">
                <h2 class="text-xl font-semibold">{"Controlled State"}</h2>
                <div class="flex items-center justify-center gap-4 p-8 border rounded-lg">
                    <ControlledTooltipExample />
                </div>
            </section>

            // Custom Styling
            <section class="space-y-4">
                <h2 class="text-xl font-semibold">{"Custom Styling"}</h2>
                <div class="flex items-center justify-center p-8 border rounded-lg">
                    <TooltipProvider>
                        <Tooltip>
                            <TooltipTrigger>
                                <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2">
                                    {"Custom styled"}
                                </button>
                            </TooltipTrigger>
                            <TooltipContent class="bg-red-500 text-white border-red-600">
                                <p>{"Custom red tooltip"}</p>
                            </TooltipContent>
                        </Tooltip>
                    </TooltipProvider>
                </div>
            </section>

            // Multiple Tooltips
            <section class="space-y-4">
                <h2 class="text-xl font-semibold">{"Multiple Tooltips"}</h2>
                <div class="flex items-center justify-center gap-4 p-8 border rounded-lg">
                    <TooltipProvider>
                        <Tooltip>
                            <TooltipTrigger>
                                <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2">
                                    {"Save"}
                                </button>
                            </TooltipTrigger>
                            <TooltipContent>
                                <p>{"Save your changes"}</p>
                            </TooltipContent>
                        </Tooltip>

                        <Tooltip>
                            <TooltipTrigger>
                                <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2">
                                    {"Load"}
                                </button>
                            </TooltipTrigger>
                            <TooltipContent>
                                <p>{"Load from file"}</p>
                            </TooltipContent>
                        </Tooltip>

                        <Tooltip>
                            <TooltipTrigger>
                                <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-destructive text-destructive-foreground hover:bg-destructive/90 h-10 px-4 py-2">
                                    {"Delete"}
                                </button>
                            </TooltipTrigger>
                            <TooltipContent>
                                <p>{"Delete permanently"}</p>
                            </TooltipContent>
                        </Tooltip>
                    </TooltipProvider>
                </div>
            </section>

            // Delay Example
            <section class="space-y-4">
                <h2 class="text-xl font-semibold">{"Custom Delay"}</h2>
                <div class="flex items-center justify-center gap-4 p-8 border rounded-lg">
                    <TooltipProvider>
                        <Tooltip delay_duration={1000}>
                            <TooltipTrigger>
                                <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2">
                                    {"1 second delay"}
                                </button>
                            </TooltipTrigger>
                            <TooltipContent>
                                <p>{"This tooltip appears after 1 second"}</p>
                            </TooltipContent>
                        </Tooltip>
                    </TooltipProvider>
                </div>
            </section>
        </div>
    }
}

#[function_component]
fn ControlledTooltipExample() -> Html {
    let is_open = use_state(|| false);
    let on_open_change = {
        let is_open = is_open.clone();
        Callback::from(move |open: bool| {
            is_open.set(open);
        })
    };

    let toggle_tooltip = {
        let is_open = is_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_open.set(!*is_open);
        })
    };

    html! {
        <div class="flex items-center gap-4">
            <TooltipProvider>
                <Tooltip 
                    open={*is_open}
                    on_open_change={on_open_change}
                >
                    <TooltipTrigger>
                        <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2">
                            {"Controlled Tooltip"}
                        </button>
                    </TooltipTrigger>
                    <TooltipContent>
                        <p>{"This tooltip is controlled programmatically"}</p>
                    </TooltipContent>
                </Tooltip>
            </TooltipProvider>
            <button 
                class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2"
                onclick={toggle_tooltip}
            >
                {if *is_open { "Hide Tooltip" } else { "Show Tooltip" }}
            </button>
        </div>
    }
}