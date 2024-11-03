# Alert

Displays a callout for user attention.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["alert"]
files = ["src/default/alert/alert.rs"]
url_fragment = "#/default/"

[[file_replacements]]
find = "crate::default::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["alert"]
files = ["src/new_york/alert/alert.rs"]
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

<!-- {{#tab name="Leptos" }}

```shell
rust-shadcn-ui add alert
```

-   [View source](https://github.com/RustForWeb/shadcn-ui/tree/main/packages/leptos/alert)

{{#endtab }} -->

{{#tab name="Yew" }}

```shell
rust-shadcn-ui add alert
```

-   [View source](https://github.com/RustForWeb/shadcn-ui/tree/main/packages/yew/alert)

{{#endtab }}
{{#endtabs }}

## Usage

{{#tabs global="framework" }}

<!-- {{#tab name="Leptos" }}

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

{{#endtab }} -->

{{#tab name="Yew" }}

```rust,ignore
use yew::prelude::*;

use crate::components::ui::alert::{Alert, AlertDescription, AlertTitle};

#[component]
fn Usage() -> impl IntoView {
    view! {
        <Alert>
            <Terminal class="h-4 w-4" />
            <AlertTitle>{"Heads up!"}</AlertTitle>
            <AlertDescription>
                {"You can add components and dependencies to your app using the cli."}
            </AlertDescription>
        </Alert>
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
features = ["alert"]
files = ["src/default/alert/alert.rs"]
url_fragment = "#/default/"

[[file_replacements]]
find = "crate::default::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["alert"]
files = ["src/new_york/alert/alert.rs"]
url_fragment = "#/new-york/"

[[file_replacements]]
find = "crate::new_york::"
replace = "crate::"
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
features = ["alert"]
files = ["src/default/alert/alert_destructive.rs"]
url_fragment = "#/default/destructive"

[[file_replacements]]
find = "crate::default::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["alert"]
files = ["src/new_york/alert/alert_destructive.rs"]
url_fragment = "#/new-york/destructive"

[[file_replacements]]
find = "crate::new_york::"
replace = "crate::"
```

{{#endtab }}
{{#endtabs }}

{{#endtab }}
{{#endtabs }}

## See Also

-   [shadcn/ui documentation](https://ui.shadcn.com/docs/components/alert)
