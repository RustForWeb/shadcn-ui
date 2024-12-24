use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use leptos_style::Style;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(
    class = "inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
)]
pub struct BadgeClass {
    pub variant: BadgeVariant,
}

#[derive(PartialEq, TwVariant)]
pub enum BadgeVariant {
    #[tw(
        default,
        class = "border-transparent bg-primary text-primary-foreground hover:bg-primary/80"
    )]
    Default,
    #[tw(
        class = "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80"
    )]
    Secondary,
    #[tw(
        class = "border-transparent bg-destructive text-destructive-foreground hover:bg-destructive/80"
    )]
    Destructive,
    #[tw(class = "text-foreground")]
    Outline,
}

#[component]
pub fn Badge(
    #[prop(into, optional)] variant: Signal<BadgeVariant>,
    // Global attributes
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| {
        BadgeClass {
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
        >
            {children()}
        </div>
    }
}
