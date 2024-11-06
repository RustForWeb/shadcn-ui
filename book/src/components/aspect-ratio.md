# Aspect Ratio

Displays content within a desired ratio.

-   [Docs](https://radix.rustforweb.org/primitives/components/aspect-ratio.html)
-   [API Reference](https://radix.rustforweb.org/primitives/components/aspect-ratio.html#api-reference)

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["aspect-ratio"]
files = ["src/default/aspect_ratio/aspect_ratio.rs"]
url_fragment = "#/default/"

[[file_replacements]]
find = "crate::default::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["aspect-ratio"]
files = ["src/new_york/aspect_ratio/aspect_ratio.rs"]
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
rust-shadcn-ui add aspect-ratio
```

-   [View source](https://github.com/RustForWeb/shadcn-ui/tree/main/packages/yew/aspect-ratio)

{{#endtab }}
{{#endtabs }}

## Usage

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```rust,ignore
use yew::prelude::*;

use crate::components::ui::aspect_ratio::AspectRatio;

#[component]
fn Usage() -> impl IntoView {
    view! {
        <div class="w-[450px]">
            <AspectRatio ratio={16.0 / 9.0}>
                <img src="..." alt="Image" class="rounded-md object-cover" />
            </AspectRatio>
        </div>
    }
}
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [shadcn/ui documentation](https://ui.shadcn.com/docs/components/aspect-ratio)
