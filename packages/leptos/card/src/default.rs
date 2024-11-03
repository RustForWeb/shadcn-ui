use leptos::prelude::*;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(class = "rounded-lg border bg-card text-card-foreground shadow-sm")]
pub struct CardClass {}

#[component]
pub fn Card(#[prop(into, optional)] class: Signal<String>, children: Children) -> impl IntoView {
    let class = Memo::new(move |_| CardClass {}.with_class(class.get()));

    view! {
        <div class=class>
            {children()}
        </div>
    }
}

#[derive(TwClass)]
#[tw(class = "flex flex-col space-y-1.5 p-6")]
pub struct CardHeaderClass {}

#[component]
pub fn CardHeader(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| CardHeaderClass {}.with_class(class.get()));

    view! {
        <div class=class>
            {children()}
        </div>
    }
}

#[derive(TwClass)]
#[tw(class = "text-2xl font-semibold leading-none tracking-tight")]
pub struct CardTitleClass {}

#[component]
pub fn CardTitle(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| CardTitleClass {}.with_class(class.get()));

    view! {
        <div class=class>
            {children()}
        </div>
    }
}

#[derive(TwClass)]
#[tw(class = "text-sm text-muted-foreground")]
pub struct CardDescriptionClass {}

#[component]
pub fn CardDescription(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| CardDescriptionClass {}.with_class(class.get()));

    view! {
        <div class=class>
            {children()}
        </div>
    }
}

#[derive(TwClass)]
#[tw(class = "p-6 pt-0")]
pub struct CardContentClass {}

#[component]
pub fn CardContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| CardContentClass {}.with_class(class.get()));

    view! {
        <div class=class>
            {children()}
        </div>
    }
}

#[derive(TwClass)]
#[tw(class = "flex items-center p-6 pt-0")]
pub struct CardFooterClass {}

#[component]
pub fn CardFooter(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| CardFooterClass {}.with_class(class.get()));

    view! {
        <div class=class>
            {children()}
        </div>
    }
}
