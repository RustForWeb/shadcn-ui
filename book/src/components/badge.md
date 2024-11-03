# Badge

Displays a badge or a component that looks like a badge.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["badge"]
files = ["src/default/badge/badge.rs"]
url_fragment = "#/default/"

[[file_replacements]]
find = "crate::default::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["badge"]
files = ["src/new_york/badge/badge.rs"]
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
rust-shadcn-ui add badge
```

-   [View source](https://github.com/RustForWeb/shadcn-ui/tree/main/packages/yew/badge)

{{#endtab }}
{{#endtabs }}

## Usage

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```rust,ignore
use yew::prelude::*;

use crate::components::ui::badge::{Badge, BadgeVariant};

#[component]
fn Usage() -> impl IntoView {
    view! {
        <Badge variant={BadgeVariant::Outline}>{"Badge"}>/Badge>
    }
}
```

{{#endtab }}
{{#endtabs }}

### Link

You can use the `BadgeVariant` helper to create a link that looks like a badge.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```rust,ignore
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::ui::badge::{Badge, BadgeClass, BadgeVariant};

#[component]
fn Usage() -> impl IntoView {
    view! {
        <Link class={BadgeClass::builder().variant(BadgeVariant::Outline).build().to_class()}>
            {"Badge"}
        </Link>
    }
}
```

{{#endtab }}
{{#endtabs }}

## Examples

### Default

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["badge"]
files = ["src/default/badge/badge.rs"]
url_fragment = "#/default/"

[[file_replacements]]
find = "crate::default::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["badge"]
files = ["src/new_york/badge/badge.rs"]
url_fragment = "#/new-york/"

[[file_replacements]]
find = "crate::new_york::"
replace = "crate::"
```

{{#endtab }}
{{#endtabs }}

{{#endtab }}
{{#endtabs }}

### Secondary

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["badge"]
files = ["src/default/badge/badge_secondary.rs"]
url_fragment = "#/default/secondary"

[[file_replacements]]
find = "crate::new_york::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["badge"]
files = ["src/new_york/badge/badge_secondary.rs"]
url_fragment = "#/new-york/secondary"
```

{{#endtab }}
{{#endtabs }}

{{#endtab }}
{{#endtabs }}

### Outline

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["badge"]
files = ["src/default/badge/badge_outline.rs"]
url_fragment = "#/default/outline"

[[file_replacements]]
find = "crate::new_york::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["badge"]
files = ["src/new_york/badge/badge_outline.rs"]
url_fragment = "#/new-york/outline"
```

{{#endtab }}
{{#endtabs }}

{{#endtab }}
{{#endtabs }}

### Destructive

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["badge"]
files = ["src/default/badge/badge_destructive.rs"]
url_fragment = "#/default/destructive"

[[file_replacements]]
find = "crate::new_york::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["badge"]
files = ["src/new_york/badge/badge_destructive.rs"]
url_fragment = "#/new-york/destructive"
```

{{#endtab }}
{{#endtabs }}

{{#endtab }}
{{#endtabs }}

## See Also

-   [shadcn/ui documentation](https://ui.shadcn.com/docs/components/badge)
