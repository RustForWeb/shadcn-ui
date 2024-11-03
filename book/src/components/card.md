# Card

Displays a card with header, content, and footer.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["card"]
files = ["src/default/card/card_with_form.rs"]
url_fragment = "#/default/with-form"

[[file_replacements]]
find = "crate::default::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["card"]
files = ["src/new_york/card/card_with_form.rs"]
url_fragment = "#/new-york/with-form"

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
rust-shadcn-ui add card
```

-   [View source](https://github.com/RustForWeb/shadcn-ui/tree/main/packages/yew/card)

{{#endtab }}
{{#endtabs }}

## Usage

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```rust,ignore
use yew::prelude::*;

use crate::components::ui::card::{
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
};

#[component]
fn Usage() -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <CardTitle>{"Card Title"}</CardTitle>
                <CardDescription>{"Card Description"}</CardDescription>
            </CardHeader>
            <CardContent>
                <p>{"Card Content"}</p>
            </CardContent>
            <CardFooter>
                <p>{"Card Footer"}</p>
            </CardFooter>
        </Card>
    }
}
```

{{#endtab }}
{{#endtabs }}

## Examples

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["card"]
files = ["src/default/card/card.rs"]
url_fragment = "#/default/"

[[file_replacements]]
find = "crate::default::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["card"]
files = ["src/new_york/card/card.rs"]
url_fragment = "#/new-york/"

[[file_replacements]]
find = "crate::new_york::"
replace = "crate::"
```

{{#endtab }}
{{#endtabs }}

{{#endtab }}
{{#endtabs }}

## See Also

-   [shadcn/ui documentation](https://ui.shadcn.com/docs/components/card)
