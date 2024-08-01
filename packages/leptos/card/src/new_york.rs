use leptos::*;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(class = "rounded-xl border bg-card text-card-foreground shadow")]
pub struct CardClass {}

#[component]
pub fn Card(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| CardClass {}.with_class(class.get()));

    view! {
        <div {..attrs} class=class>
            {children()}
        </div>
    }
}

#[derive(TwClass)]
#[tw(class = "flex flex-col space-y-1.5 p-6")]
pub struct CardHeaderClass {}

#[component]
pub fn CardHeader(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| CardHeaderClass {}.with_class(class.get()));

    view! {
        <div {..attrs} class=class>
            {children()}
        </div>
    }
}

#[derive(TwClass)]
#[tw(class = "font-semibold leading-none tracking-tight")]
pub struct CardTitleClass {}

#[component]
pub fn CardTitle(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| CardTitleClass {}.with_class(class.get()));

    view! {
        <div {..attrs} class=class>
            {children()}
        </div>
    }
}

#[derive(TwClass)]
#[tw(class = "text-sm text-muted-foreground")]
pub struct CardDescriptionClass {}

#[component]
pub fn CardDescription(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| CardDescriptionClass {}.with_class(class.get()));

    view! {
        <div {..attrs} class=class>
            {children()}
        </div>
    }
}

#[derive(TwClass)]
#[tw(class = "p-6 pt-0")]
pub struct CardContentClass {}

#[component]
pub fn CardContent(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| CardContentClass {}.with_class(class.get()));

    view! {
        <div {..attrs} class=class>
            {children()}
        </div>
    }
}

#[derive(TwClass)]
#[tw(class = "flex items-center p-6 pt-0")]
pub struct CardFooterClass {}

#[component]
pub fn CardFooter(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| CardFooterClass {}.with_class(class.get()));

    view! {
        <div {..attrs} class=class>
            {children()}
        </div>
    }
}
