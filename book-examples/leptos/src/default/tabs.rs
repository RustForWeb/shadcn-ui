use leptos::prelude::*;

use crate::default::components::ui::tabs::{Tabs, TabsContent, TabsList, TabsTrigger};

#[component]
pub fn TabsExample() -> impl IntoView {
    let (value, set_value) = create_signal("account".to_string());

    let handle_value_change = Callback::new(move |new_value: String| {
        set_value.set(new_value);
    });

    view! {
        <Tabs value=value on_value_change=handle_value_change class="w-[400px]">
            <TabsList>
                <TabsTrigger value="account">"Account"</TabsTrigger>
                <TabsTrigger value="password">"Password"</TabsTrigger>
            </TabsList>
            <TabsContent value="account">
                "Make changes to your account here."
            </TabsContent>
            <TabsContent value="password">
                "Change your password here."
            </TabsContent>
        </Tabs>
    }
}
