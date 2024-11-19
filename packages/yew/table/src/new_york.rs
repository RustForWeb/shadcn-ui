use tailwind_fuse::*;
use yew::prelude::*;
use yew_style::Style;

#[derive(PartialEq, Properties)]
pub struct TableProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Table(props: &TableProps) -> Html {
    html! {
        <div class="relative w-full overflow-auto">
            <table
                ref={props.node_ref.clone()}

                class={tw_merge!("w-full caption-bottom text-sm", &props.class)}
                id={props.id.clone()}
                style={props.style.clone()}
            >
                {props.children.clone()}
            </table>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct TableHeaderProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn TableHeader(props: &TableHeaderProps) -> Html {
    html! {
        <thead
            ref={props.node_ref.clone()}

            class={tw_merge!("[&_tr]:border-b", &props.class)}
            id={props.id.clone()}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </thead>
    }
}

#[derive(PartialEq, Properties)]
pub struct TableBodyProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn TableBody(props: &TableBodyProps) -> Html {
    html! {
        <tbody
            ref={props.node_ref.clone()}

            class={tw_merge!("[&_tr:last-child]:border-0", &props.class)}
            id={props.id.clone()}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </tbody>
    }
}

#[derive(PartialEq, Properties)]
pub struct TableFooterProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn TableFooter(props: &TableFooterProps) -> Html {
    html! {
        <tfoot
            ref={props.node_ref.clone()}

            class={tw_merge!("border-t bg-muted/50 font-medium [&>tr]:last:border-b-0", &props.class)}
            id={props.id.clone()}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </tfoot>
    }
}

#[derive(PartialEq, Properties)]
pub struct TableRowProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn TableRow(props: &TableRowProps) -> Html {
    html! {
        <tr
            ref={props.node_ref.clone()}

            class={tw_merge!("border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted", &props.class)}
            id={props.id.clone()}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </tr>
    }
}

#[derive(PartialEq, Properties)]
pub struct TableHeadProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    // Attributes from `th`
    #[prop_or_default]
    pub abbr: Option<String>,
    #[prop_or_default]
    pub colspan: Option<String>,
    #[prop_or_default]
    pub headers: Option<String>,
    #[prop_or_default]
    pub rowspan: Option<String>,
    #[prop_or_default]
    pub scope: Option<String>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn TableHead(props: &TableHeadProps) -> Html {
    html! {
        <th
            ref={props.node_ref.clone()}

            class={tw_merge!("h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]", &props.class)}
            id={props.id.clone()}
            style={props.style.clone()}

            abbr={props.abbr.clone()}
            colspan={props.colspan.clone()}
            headers={props.headers.clone()}
            rowspan={props.rowspan.clone()}
            scope={props.scope.clone()}
        >
            {props.children.clone()}
        </th>
    }
}

#[derive(PartialEq, Properties)]
pub struct TableCellProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    // Attributes from `td`
    #[prop_or_default]
    pub colspan: Option<String>,
    #[prop_or_default]
    pub headers: Option<String>,
    #[prop_or_default]
    pub rowspan: Option<String>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn TableCell(props: &TableCellProps) -> Html {
    html! {
        <td
            ref={props.node_ref.clone()}

            class={tw_merge!("p-2 align-middle [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]", &props.class)}
            id={props.id.clone()}
            style={props.style.clone()}

            colspan={props.colspan.clone()}
            headers={props.headers.clone()}
            rowspan={props.rowspan.clone()}
        >
            {props.children.clone()}
        </td>
    }
}

#[derive(PartialEq, Properties)]
pub struct TableCaptionProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn TableCaption(props: &TableCaptionProps) -> Html {
    html! {
        <caption
            ref={props.node_ref.clone()}

            class={tw_merge!("mt-4 text-sm text-muted-foreground", &props.class)}
            id={props.id.clone()}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </caption>
    }
}
