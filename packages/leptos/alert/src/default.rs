use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use leptos_struct_component::{struct_component, StructComponent};
use leptos_style::Style;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(
    class = "relative w-full rounded-lg border p-4 [&>svg~*]:pl-7 [&>svg+div]:translate-y-[-3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-foreground"
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

#[derive(Clone, StructComponent)]
#[struct_component(tag = "div")]
pub struct AlertChildProps {
    pub node_ref: AnyNodeRef,
    pub class: MaybeProp<String>,
    pub id: MaybeProp<String>,
    pub style: Signal<Style>,
}

#[component]
pub fn Alert(
    #[prop(into, optional)] variant: Signal<AlertVariant>,

    // Global attributes
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = Memo::new(move |_| {
        AlertClass {
            variant: variant.get(),
        }
        .with_class(class.get().unwrap_or_default())
    });

    let child_props = AlertChildProps {
        node_ref,
        class: class.into(),
        id,
        style,
    };

    child_props.render(children)
}

#[derive(Clone, StructComponent)]
#[struct_component(tag = "h5")]
pub struct AlertTitleChildProps {
    pub node_ref: AnyNodeRef,
    pub class: MaybeProp<String>,
    pub id: MaybeProp<String>,
    pub style: Signal<Style>,
}

#[component]
pub fn AlertTitle(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let child_props = AlertTitleChildProps {
        node_ref,
        class: tw_merge!("mb-1 font-medium leading-none tracking-tight", class.get()).into(),
        id,
        style,
    };
    child_props.render(children)
}

#[derive(Clone, StructComponent)]
#[struct_component(tag = "div")]
pub struct AlertDescriptioinChildProps {
    pub node_ref: AnyNodeRef,
    pub class: MaybeProp<String>,
    pub id: MaybeProp<String>,
    pub style: Signal<Style>,
}

#[component]
pub fn AlertDescription(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let child_props = AlertDescriptioinChildProps {
        node_ref,
        class: tw_merge!("text-sm [&_p]:leading-relaxed", class.get()).into(),
        id,
        style,
    };
    child_props.render(children)
}
