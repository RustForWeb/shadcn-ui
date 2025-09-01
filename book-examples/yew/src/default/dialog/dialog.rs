use yew::prelude::*;
use shadcn_ui_yew_dialog::*;

#[function_component]
pub fn DialogExample() -> Html {
    html! {
        <div class="space-y-8">
            <div>
                <h2 class="text-2xl font-bold tracking-tight">{"Dialog"}</h2>
                <p class="text-muted-foreground">
                    {"A window overlaid on either the primary window or another dialog window, rendering the content underneath inert."}
                </p>
            </div>

            <div class="space-y-4">
                <h3 class="text-lg font-medium">{"Basic Dialog"}</h3>
                <div class="flex items-center space-x-2">
                    <Dialog open=false>
                        <DialogTrigger class="btn btn-primary">
                            {"Open Dialog"}
                        </DialogTrigger>
                        <DialogContent>
                            <DialogHeader>
                                <DialogTitle>{"Are you sure?"}</DialogTitle>
                                <DialogDescription>
                                    {"This action cannot be undone."}
                                </DialogDescription>
                            </DialogHeader>
                            <DialogFooter>
                                <button type="button" class="btn btn-secondary">
                                    {"Cancel"}
                                </button>
                                <button type="button" class="btn btn-destructive">
                                    {"Delete"}
                                </button>
                            </DialogFooter>
                        </DialogContent>
                    </Dialog>
                </div>
            </div>

            <div class="space-y-4">
                <h3 class="text-lg font-medium">{"New York Variant"}</h3>
                <div class="flex items-center space-x-2">
                    <DialogNewYork open=false>
                        <DialogTriggerNewYork class="btn btn-primary">
                            {"Open Dialog (NY)"}
                        </DialogTriggerNewYork>
                        <DialogContentNewYork>
                            <DialogHeaderNewYork>
                                <DialogTitleNewYork>{"New York Style"}</DialogTitleNewYork>
                                <DialogDescriptionNewYork>
                                    {"This dialog uses the New York variant styling."}
                                </DialogDescriptionNewYork>
                            </DialogHeaderNewYork>
                            <DialogFooterNewYork>
                                <button type="button" class="btn btn-secondary">
                                    {"Cancel"}
                                </button>
                                <button type="button" class="btn btn-primary">
                                    {"Continue"}
                                </button>
                            </DialogFooterNewYork>
                        </DialogContentNewYork>
                    </DialogNewYork>
                </div>
            </div>
        </div>
    }
}
