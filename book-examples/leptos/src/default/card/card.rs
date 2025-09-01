use leptos::prelude::*;

#[cfg(feature = "lucide-leptos")]
use lucide_leptos::{BellRing, Check};

#[cfg(not(feature = "lucide-leptos"))]
const BellRing: () = ();
#[cfg(not(feature = "lucide-leptos"))]
const Check: () = ();

use crate::default::components::ui::{
    button::Button,
    card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle},
};

struct Notification {
    id: usize,
    title: &'static str,
    description: &'static str,
}

fn notifications() -> Vec<Notification> {
    vec![
        Notification {
            id: 0,
            title: "Your call has been confirmed.",
            description: "1 hour ago",
        },
        Notification {
            id: 1,
            title: "You have a new message!",
            description: "1 hour ago",
        },
        Notification {
            id: 2,
            title: "Your subscription is expiring soon!",
            description: "2 hours ago",
        },
    ]
}

#[component]
pub fn CardDemo() -> impl IntoView {
    view! {
        <Card class="w-[380px]">
            <CardHeader>
                <CardTitle>{"Notifications"}</CardTitle>
                <CardDescription>{"You have 3 unread messages."}</CardDescription>
            </CardHeader>
            <CardContent class="grid gap-4">
                <div class=" flex items-center space-x-4 rounded-md border p-4">
                    {
                        #[cfg(feature = "lucide-leptos")]
                        {
                            view! { <BellRing /> }
                        }
                        #[cfg(not(feature = "lucide-leptos"))]
                        {
                            view! { <span class="h-4 w-4">"🔔"</span> }
                        }
                    }
                    <div class="flex-1 space-y-1">
                        <p class="text-sm font-medium leading-none">
                            {"Push Notifications"}
                        </p>
                        <p class="text-sm text-muted-foreground">
                            {"Send notifications to device."}
                        </p>
                    </div>
                </div>
                <div>
                    <For
                        each=move || notifications()
                        key=|notification| notification.id
                        children=move |notification: Notification| {
                    view! {
                        <div
                            class="mb-4 grid grid-cols-[25px_1fr] items-start pb-4 last:mb-0 last:pb-0"
                        >
                            <span class="flex h-2 w-2 translate-y-1 rounded-full bg-sky-500" />
                            <div class="space-y-1">
                                <p class="text-sm font-medium leading-none">
                                    {notification.title}
                                </p>
                                <p class="text-sm text-muted-foreground">
                                    {notification.description}
                                </p>
                            </div>
                        </div>
                    }
                }
                    />
                </div>
            </CardContent>
            <CardFooter>
                <Button class="w-full">
                    {
                        #[cfg(feature = "lucide-leptos")]
                        {
                            view! { <Check /> }
                        }
                        #[cfg(not(feature = "lucide-leptos"))]
                        {
                            view! { <span>"✓"</span> }
                        }
                    }
                    {" Mark all as read"}
                </Button>
            </CardFooter>
        </Card>
    }
}
