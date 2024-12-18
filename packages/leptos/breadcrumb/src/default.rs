use leptos_node_ref::AnyNodeRef;
use lucide_leptos::{ChevronRight, Ellipsis};
use tailwind_fuse::*;
use leptos::{ev::MouseEvent, prelude::*} ;
use leptos_struct_component::{struct_component, StructComponent};
use leptos_style::Style;



#[component]
pub fn Breadcrumb(
    // Global attributes
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: Children,
) -> impl IntoView {
    view!{
        <nav
            node_ref=node_ref
            aria-label="breadcrumb"
            class= move || class.get()
            id=move || id.get()
            style=move || style.get()
        >
            {children()}
        </nav>
    }
}


#[component]
pub fn BreadcrumbList(
    // Global attributes
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: Children,
) -> impl IntoView {
    view!{
        <ol
            node_ref=node_ref
        class= move || tw_merge!("flex flex-wrap items-center gap-1.5 break-words text-sm text-muted-foreground sm:gap-2.5",
                class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children()}
        </ol>
    }
}

#[component]
pub fn BreadcrumbItem(
    // Global attributes
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: Children,
) ->impl IntoView{
    view! {
        <li
            node_ref=node_ref
            class=move || tw_merge!("inline-flex items-center gap-1.5", class.get())
            id=move || id.get()
            style=style
        >
            {children()}
        </li>
    }
}

#[derive(Clone, StructComponent)]
#[struct_component(tag = "a")]
pub struct BreadcrumbLinkChildProps {
    pub node_ref: AnyNodeRef,

    // Global attributes
    pub class: Signal<String>,
    pub id: MaybeProp<String>,
    pub style: Signal<Style>,

    // Attributes from `a`
    pub href: MaybeProp<String>,

    // Event handler attributes
    pub onclick: Option<Callback<MouseEvent>>,
}

#[component]
pub fn BreadcrumbLink(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] href: MaybeProp<String>,
    #[prop(into, optional)] onclick: Option<Callback<MouseEvent>>,

    #[prop(into, optional)] as_child: Option<Callback<BreadcrumbLinkChildProps, AnyView>>,
    children: Option<Children>,
)-> impl IntoView {
    let child_props = BreadcrumbLinkChildProps {
        node_ref,

        // Global attributes
        class: tw_merge!("transition-colors hover:text-foreground", class.get()).into(),
        id,
        style,

        // Attributes from `a`
        href,

        // Event handler attributes
        onclick,
    };

    if let Some(as_child) = as_child.as_ref() {
        as_child.run(child_props)
    }else{
        child_props.render(children)
    }
}



#[component]
pub fn BreadcrumbPage(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Children,
)->impl IntoView{
    view!{
        <span
            node_ref=node_ref

            aria-disabled="true"
            aria-current="page"
            role="link"
            class=move || tw_merge!("font-normal text-foreground", class.get())
            id=move || id.get()
            style=style
        > 
            {children()}
        </span>
    }
}


#[component]
pub fn BreadcrumbSeparator(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFragmentFn,
) -> impl IntoView {
    if children().nodes.is_empty(){
        children()
        .nodes
        .push(  view!{<ChevronRight />}.into_any() );
    }
    view!{
        <li
            node_ref=node_ref
            aria-hidden="true"
            class=move || tw_merge!("[&>svg]:w-3.5 [&>svg]:h-3.5", class.get())
            id=move || id.get()
            role="presentation"
            style=style
        >
            {children().nodes.into_any()}
        </li>
    }
}


#[component]
pub fn BreadcrumbEllipsis(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    view! {
        <span
            node_ref=node_ref
            id=move || id.get()
            class=move || tw_merge!("flex h-9 w-9 items-center justify-center",class.get())
            style=style
            role="presentation"
            aria-hidden="true"
        >
            <Ellipsis attr:class="h-4 w-4" />
            <span class="sr-only">{"More"}</span>
        </span>
    }
}

