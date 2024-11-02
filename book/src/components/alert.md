# Alert

Displays a callout for user attention.

<!-- {{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["alert"]
files = ["src/alert.rs"]
```

{{#endtab }}
{{#endtabs }} -->

## Installation

<div class="warning">

The CLI is not yet available. For now, manually copy the component source into your project.

</div>

<!-- {{#tabs global="framework" }}
{{#tab name="Leptos" }}

```shell
rust-shadcn-ui add alert
```

-   [View source](https://github.com/RustForWeb/shadcn-ui/tree/main/packages/leptos/alert)

{{#endtab }}
{{#endtabs }} -->

## Usage

<!-- {{#tabs global="framework" }}
{{#tab name="Leptos" }}

```rust,ignore
use leptos::*;
use crate::components::ui::alert::{Alert, AlertDescription, AlertTitle};

#[component]
fn Usage() -> impl IntoView {
    view! {
        <Alert>
            // <Terminal class="h-4 w-4" />
            <AlertTitle>Heads up!</AlertTitle>
            <AlertDescription>
                You can add components to your app using the cli.
            </AlertDescription>
        </Alert>
    }
}
```

{{#endtab }}
{{#endtabs }} -->

## Examples

### Default

<!-- {{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["alert"]
files = ["src/alert.rs"]
```

{{#endtab }}
{{#endtabs }} -->

### Destructive

<!-- {{#tabs global="framework" }}
{{#tab name="Leptos" }}

```toml,trunk
package = "shadcn-ui-leptos-book"
features = ["alert-destructive"]
files = ["src/alert_destructive.rs"]
```

{{#endtab }}
{{#endtabs }} -->

## See Also

-   [shadcn/ui documentation](https://ui.shadcn.com/docs/components/alert)
