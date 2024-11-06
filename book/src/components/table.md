# Table

A responsive table component.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["table"]
files = ["src/default/table/table.rs"]
url_fragment = "#/default/"

[[file_replacements]]
find = "crate::default::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["table"]
files = ["src/new_york/table/table.rs"]
url_fragment = "#/new-york/"

[[file_replacements]]
find = "crate::new_york::"
replace = "crate::"
```

{{#endtab }}
{{#endtabs }}

{{#endtab }}
{{#endtabs }}

## Installation

<div class="warning">

The CLI is not yet available. For now, manually copy the component source into your project.

</div>

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```shell
rust-shadcn-ui add table
```

-   [View source](https://github.com/RustForWeb/shadcn-ui/tree/main/packages/yew/table)

{{#endtab }}
{{#endtabs }}

## Usage

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```rust,ignore
use yew::prelude::*;

use crate::components::ui::table::{
    Table,
    TableBody,
    TableCaption,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
};

#[component]
fn Usage() -> impl IntoView {
    view! {
        <Table>
            <TableCaption>{"A list of your recent invoices."}</TableCaption>
            <TableHeader>
                <TableRow>
                    <TableHead class="w-[100px]">{"Invoice"}</TableHead>
                    <TableHead>{"Status"}</TableHead>
                    <TableHead>{"Method"}</TableHead>
                    <TableHead class="text-right">{"Amount"}</TableHead>
                </TableRow>
            </TableHeader>
            <TableBody>
                <TableRow>
                    <TableCell class="font-medium">{"INV001"}</TableCell>
                    <TableCell>{"Paid"}</TableCell>
                    <TableCell>{"Credit Card"}</TableCell>
                    <TableCell class="text-right">{"$250.00"}</TableCell>
                </TableRow>
            </TableBody>
        </Table>
    }
}
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [shadcn/ui documentation](https://ui.shadcn.com/docs/components/table)
