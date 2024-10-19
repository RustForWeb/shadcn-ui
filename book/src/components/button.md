# Button

Displays a button or a component that looks like a button.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["button"]
files = ["src/button.rs"]
```

{{#endtab }}
{{#tab name="Yew" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button"]
files = ["src/button.rs"]
```

{{#endtab }}
{{#endtabs }}

## Installation

<div class="warning">

The CLI is not yet available. For now, manually copy the component source into your project.

</div>

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```shell
rust-shadcn-ui add button
```

-   [View source](https://github.com/RustForWeb/shadcn-ui/tree/main/packages/leptos/button)

{{#endtab }}
{{#tab name="Yew" }}

```shell
rust-shadcn-ui add button
```

-   [View source](https://github.com/RustForWeb/shadcn-ui/tree/main/packages/yew/button)

{{#endtab }}
{{#endtabs }}

## Usage

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;

use crate::components::ui::button::{Button, ButtonVariant};

#[component]
fn Usage() -> impl IntoView {
    view! {
        <Button variant=ButtonVariant::Outline>Button>/Button>
    }
}
```

{{#endtab }}
{{#tab name="Yew" }}

```rust,ignore
use yew::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};

#[component]
fn Usage() -> impl IntoView {
    view! {
        <Button variant={ButtonVariant::Outline}>{"Button"}>/Button>
    }
}
```

{{#endtab }}
{{#endtabs }}

## Examples

### Primary

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["button"]
files = ["src/button.rs"]
```

{{#endtab }}
{{#tab name="Yew" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button"]
files = ["src/button.rs"]
```

{{#endtab }}
{{#endtabs }}

### Secondary

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["button-secondary"]
files = ["src/button_secondary.rs"]
```

{{#endtab }}
{{#tab name="Yew" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button-secondary"]
files = ["src/button_secondary.rs"]
```

{{#endtab }}
{{#endtabs }}

### Destructive

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["button-destructive"]
files = ["src/button_destructive.rs"]
```

{{#endtab }}
{{#tab name="Yew" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button-destructive"]
files = ["src/button_destructive.rs"]
```

{{#endtab }}
{{#endtabs }}

### Outline

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["button-outline"]
files = ["src/button_outline.rs"]
```

{{#endtab }}
{{#tab name="Yew" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button-outline"]
files = ["src/button_outline.rs"]
```

{{#endtab }}
{{#endtabs }}

### Ghost

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["button-ghost"]
files = ["src/button_ghost.rs"]
```

{{#endtab }}
{{#tab name="Yew" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button-ghost"]
files = ["src/button_ghost.rs"]
```

{{#endtab }}
{{#endtabs }}

### Link

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["button-link"]
files = ["src/button_link.rs"]
```

{{#endtab }}
{{#tab name="Yew" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button-link"]
files = ["src/button_link.rs"]
```

{{#endtab }}
{{#endtabs }}

### Icon

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["button-icon"]
files = ["src/button_icon.rs"]
```

{{#endtab }}
{{#tab name="Yew" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button-icon"]
files = ["src/button_icon.rs"]
```

{{#endtab }}
{{#endtabs }}

### With Icon

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["button-with-icon"]
files = ["src/button_with_icon.rs"]
```

{{#endtab }}
{{#tab name="Yew" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button-with-icon"]
files = ["src/button_with_icon.rs"]
```

{{#endtab }}
{{#endtabs }}

### Loading

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["button-loading"]
files = ["src/button_loading.rs"]
```

{{#endtab }}
{{#tab name="Yew" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button-loading"]
files = ["src/button_loading.rs"]
```

{{#endtab }}
{{#endtabs }}

### As Child

{{#tabs global="framework" }}

<!-- {{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["button-as-child"]
files = ["src/button_as_child.rs"]
```

{{#endtab }} -->

{{#tab name="Yew" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button-as-child"]
files = ["src/button_as_child.rs"]
```

{{#endtab }}
{{#endtabs }}

## See Also

-   [shadcn/ui documentation](https://ui.shadcn.com/docs/components/button)
