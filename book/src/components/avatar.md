# Avatar

An image element with a fallback for representing the user.

-   [Docs](https://radix.rustforweb.org/primitives/components/avatar.html)
-   [API Reference](https://radix.rustforweb.org/primitives/components/avatar.html#api-reference)

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["avatar"]
files = ["src/default/avatar/avatar.rs"]
url_fragment = "#/default/"

[[file_replacements]]
find = "crate::default::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["avatar"]
files = ["src/new_york/avatar/avatar.rs"]
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
rust-shadcn-ui add avatar
```

-   [View source](https://github.com/RustForWeb/shadcn-ui/tree/main/packages/yew/avatar)

{{#endtab }}
{{#endtabs }}

## Usage

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```rust,ignore
use yew::prelude::*;

use crate::components::ui::pagination::{Avatar, AvatarFallback, AvatarImage};

#[component]
fn Usage() -> impl IntoView {
    view! {
        <Avatar>
            <AvatarImage src="https://github.com/shadcn.png" />
            <AvatarFallback>{"CN"}</AvatarFallback>
        </Avatar>
    }
}
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [shadcn/ui documentation](https://ui.shadcn.com/docs/components/avatar)
