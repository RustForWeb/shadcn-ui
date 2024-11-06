# Separator

Visually or semantically separates content.

-   [Docs](https://radix.rustforweb.org/primitives/components/separator.html)
-   [API Reference](https://radix.rustforweb.org/primitives/components/separator.html#api-reference)

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["separator"]
files = ["src/default/separator/separator.rs"]
url_fragment = "#/default/"

[[file_replacements]]
find = "crate::default::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["separator"]
files = ["src/new_york/separator/separator.rs"]
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
rust-shadcn-ui add separator
```

-   [View source](https://github.com/RustForWeb/shadcn-ui/tree/main/packages/yew/separator)

{{#endtab }}
{{#endtabs }}

## Usage

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```rust,ignore
use yew::prelude::*;

use crate::components::ui::separator::Separator;

#[component]
fn Usage() -> impl IntoView {
    view! {
        <Separator />
    }
}
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [shadcn/ui documentation](https://ui.shadcn.com/docs/components/separator)
