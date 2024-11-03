# Label

Renders an accessible label associated with controls.

-   [Docs](https://radix.rustforweb.org/primitives/components/label.html)
-   [API Reference](https://radix.rustforweb.org/primitives/components/label.html#api-reference)

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["label"]
files = ["src/default/label/label.rs"]
url_fragment = "#/default/"

[[file_replacements]]
find = "crate::default::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["label"]
files = ["src/new_york/label/label.rs"]
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
rust-shadcn-ui add label
```

-   [View source](https://github.com/RustForWeb/shadcn-ui/tree/main/packages/yew/label)

{{#endtab }}
{{#endtabs }}

## Usage

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```rust,ignore
use yew::prelude::*;

use crate::components::ui::label::Label;

#[component]
fn Usage() -> impl IntoView {
    view! {
        <Label r#for="email">{"Your email address"}</Label>
    }
}
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [shadcn/ui documentation](https://ui.shadcn.com/docs/components/label)
