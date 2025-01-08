use leptos_node_ref::AnyNodeRef;
use tailwind_fuse::*;
use leptos::prelude::*;
use leptos_style::Style;


#[component]
pub fn Table(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div class="relative w-full overflow-auto">
            <table
                node_ref=node_ref

                class=move || tw_merge!("w-full caption-bottom text-sm", class.get())
                id=move || id.get()
                style=style
            >
                {children.map(|children| children())}
            </table>
        </div>
    }
}



#[component]
pub fn TableHeader(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <thead
            node_ref=node_ref

            class=move || tw_merge!("[&_tr]:border-b", class.get())
            id=move || id.get()
            style=style
        >
            {children.map(|children| children())}
        </thead>
    }
}



#[component]
pub fn TableBody(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<Children>,

) -> impl IntoView {
    view! {
        <tbody
            node_ref=node_ref

            class=move || tw_merge!("[&_tr:last-child]:border-0", class.get())
            id=move || id.get()
            style=style
        >
            {children.map(|children| children())}
        </tbody>
    }
}



#[component]
pub fn TableFooter(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <tfoot
            node_ref=node_ref

            class=move || tw_merge!("border-t bg-muted/50 font-medium [&>tr]:last:border-b-0", class.get())
            id=move || id.get()
            style=style
        >
            {children.map(|children| children())}
        </tfoot>
    }
}

#[component]
pub fn TableRow(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<Children>,   
) -> impl IntoView {
    view! {
        <tr
            node_ref=node_ref

            class=move || tw_merge!("border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted", class.get())
            id=move || id.get()
            style=style
        >
            {children.map(|children| children())}
        </tr>
    }
}



#[component]
pub fn TableHead(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] abbr: MaybeProp<String>,
    #[prop(into, optional)] colspan: MaybeProp<String>,
    #[prop(into, optional)] headers: MaybeProp<String>,
    #[prop(into, optional)] rowspan: MaybeProp<String>,
    #[prop(into, optional)] scope: MaybeProp<String>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <th
            node_ref=node_ref

            class=move || tw_merge!("h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]", class.get())
            id=move || id.get()
            style=style

            abbr=move ||abbr.get()
            colspan=move || colspan.get()
            headers=move || headers.get()
            rowspan=move || rowspan.get()
            scope=move || scope.get()
        >
            {children.map(|children| children())}
        </th>
    }
}



#[component]
pub fn TableCell(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] colspan: MaybeProp<String>,
    #[prop(into, optional)] headers: MaybeProp<String>,
    #[prop(into, optional)] rowspan: MaybeProp<String>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <td
            node_ref=node_ref

            class=move || tw_merge!("p-2 align-middle [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]", class.get())
            id=move || id.get()
            style=style

            colspan=move || colspan.get()
            headers=move || headers.get()
            rowspan=move || rowspan.get()
        >
            {children.map(|children| children())}
        </td>
    }
}


#[component]
pub fn TableCaption(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,

    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <caption
            node_ref=node_ref

            class=move || tw_merge!("mt-4 text-sm text-muted-foreground", class.get())
            id=move || id.get()
            style=style
        >
            {children.map(|children| children())}
        </caption>
    }
}
