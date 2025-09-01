use leptos::prelude::*;

use crate::default::components::ui::{
    dialog::{Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger},
    button::Button,
};

#[component]
pub fn DialogExample() -> impl IntoView {
    let (open, set_open) = create_signal(false);

    let handle_open_change = Callback::new(move |new_open: bool| {
        set_open.set(new_open);
    });

    view! {
        <Dialog open=open on_open_change=handle_open_change>
            <DialogTrigger as_child>
                <Button>"Open Dialog"</Button>
            </DialogTrigger>
            <DialogContent class="sm:max-w-[425px]">
                <DialogHeader>
                    <DialogTitle>"Edit profile"</DialogTitle>
                    <DialogDescription>
                        "Make changes to your profile here. Click save when you're done."
                    </DialogDescription>
                </DialogHeader>
                <div class="grid gap-4 py-4">
                    <div class="grid grid-cols-4 items-center gap-4">
                        <label class="text-right" for="name">"Name"</label>
                        <input
                            id="name"
                            value="Pedro Duarte"
                            class="col-span-3"
                        />
                    </div>
                    <div class="grid grid-cols-4 items-center gap-4">
                        <label class="text-right" for="username">"Username"</label>
                        <input
                            id="username"
                            value="@peduarte"
                            class="col-span-3"
                        />
                    </div>
                </div>
                <DialogFooter>
                    <Button>"Save changes"</Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    }
}
