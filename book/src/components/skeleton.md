# Skeleton

Use to show a placeholder while content is loading.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["skeleton"]
files = ["src/skeleton.rs"]
```

{{#endtab }}
{{#endtabs }}

## Installation

<div class="warning">

The CLI is not yet available. For now, manually copy the component source into your project.

</div>

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```shell
rust-shadcn-ui add skeleton
```

-   [View source](https://github.com/RustForWeb/shadcn-ui/tree/main/packages/yew/skeleton)

{{#endtab }}
{{#endtabs }}

## Usage

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```rust,ignore
use yew::prelude::*;

use crate::components::ui::button::{Skeleton};

#[component]
fn Usage() -> impl IntoView {
    view! {
        <Skeleton class="w-[100px] h-[20px] rounded-full" />
    }
}
```

{{#endtab }}
{{#endtabs }}

## Examples

### Card

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["skeleton-card"]
files = ["src/skeleton_card.rs"]
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [shadcn/ui documentation](https://ui.shadcn.com/docs/components/skeleton)
