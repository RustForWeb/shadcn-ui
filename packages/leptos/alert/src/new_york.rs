use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use leptos_style::Style;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(
    class = "relative w-full rounded-lg border px-4 py-3 text-sm [&>svg+div]:translate-y-[-3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-foreground [&>svg~*]:pl-7"
)]
pub struct AlertClass {
    pub variant: AlertVariant,
}

#[derive(PartialEq, TwVariant)]
pub enum AlertVariant {
    #[tw(default, class = "bg-background text-foreground")]
    Default,
    #[tw(
        class = "border-destructive/50 text-destructive dark:border-destructive [&>svg]:text-destructive"
    )]
    Destructive,
}

#[component]
pub fn Alert(
    #[prop(into, optional)] variant: Signal<AlertVariant>,

    // Global attributes
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| {
        AlertClass {
            variant: variant.get(),
        }
        .with_class(class.get().unwrap_or_default())
    });

    view! {
        <div
            node_ref=node_ref
            class=class
            id=move || id.get()
            style=style
        >{children()}
        </div>
    }
}

#[component]
pub fn AlertTitle(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: Children,
) -> impl IntoView {
    view! {
        <h5
            node_ref=node_ref
            class= move || tw_merge!("mb-1 font-medium leading-none tracking-tight", class.get())
            id=move || id.get()
            style=style
        >{children()}
        </h5>
    }
}

#[component]
pub fn AlertDescription(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || tw_merge!("text-sm [&_p]:leading-relaxed", class.get())
            id=move || id.get()
            style=style
        >{children()}
        </div>
    }
}
