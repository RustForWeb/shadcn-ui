# Button

Displays a button or a component that looks like a button.

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["button"]
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
{{#endtabs }}

## Examples

### Primary

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["button"]
```

{{#endtab }}
{{#endtabs }}

### Secondary

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["button-secondary"]
```

{{#endtab }}
{{#endtabs }}

### Destructive

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["button-destructive"]
```

{{#endtab }}
{{#endtabs }}

### Outline

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["button-outline"]
```

{{#endtab }}
{{#endtabs }}

### Ghost

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["button-ghost"]
```

{{#endtab }}
{{#endtabs }}

### Link

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["button-link"]
```

{{#endtab }}
{{#endtabs }}

### Icon

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["button-icon"]
```

{{#endtab }}
{{#endtabs }}

### With Icon

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["button-with-icon"]
```

{{#endtab }}
{{#endtabs }}

### Loading

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["button-loading"]
```

{{#endtab }}
{{#endtabs }}

<!-- ### As Child

{{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["button-as-child"]
```

{{#endtab }}
{{#endtabs }} -->

## See Also

-   [shadcn/ui documentation](https://ui.shadcn.com/docs/components/button)
