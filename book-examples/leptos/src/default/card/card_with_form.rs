use leptos::prelude::*;

use crate::default::components::ui::{
    button::{Button, ButtonVariant},
    card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle},
};

#[component]
pub fn CardWithForm() -> impl IntoView {
    view! {
        <Card class="w-[350px]">
            <CardHeader>
                <CardTitle>{"Create project"}</CardTitle>
                <CardDescription>{"Deploy your new project in one-click."}</CardDescription>
            </CardHeader>
            <CardContent>
                <form>
                    <div class="grid w-full items-center gap-4">
                        <div class="flex flex-col space-y-1.5">
                            // TODO
                            // <Label r#for="name">{"Name"}</Label>
                            // <Input id="name" placeholder="Name of your project" />
                        </div>
                        <div class="flex flex-col space-y-1.5">
                            // TODO
                            // <Label r#for="framework">{"Framework"}</Label>
                            // <Select>
                            //     <SelectTrigger id="framework">
                            //         <SelectValue placeholder="Select" />
                            //     </SelectTrigger>
                            //     <SelectContent position="popper">
                            //         <SelectItem value="next">{"Next.js"}</SelectItem>
                            //         <SelectItem value="sveltekit">{"SvelteKit"}</SelectItem>
                            //         <SelectItem value="astro">{"Astro"}</SelectItem>
                            //         <SelectItem value="nuxt">{"Nuxt.js"}</SelectItem>
                            //     </SelectContent>
                            // </Select>
                        </div>
                    </div>
                </form>
            </CardContent>
            <CardFooter class="flex justify-between">
                <Button variant={ButtonVariant::Outline}>{"Cancel"}</Button>
                <Button>{"Deploy"}</Button>
            </CardFooter>
        </Card>
    }
}
