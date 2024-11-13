# Switch

A control that allows the user to toggle between checked and not checked.

-   [Docs](https://radix.rustforweb.org/primitives/components/switch.html)
-   [API Reference](https://radix.rustforweb.org/primitives/components/switch.html#api-reference)

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["switch"]
files = ["src/default/switch/switch.rs"]
url_fragment = "#/default/"

[[file_replacements]]
find = "crate::default::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["switch"]
files = ["src/new_york/switch/switch.rs"]
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
rust-shadcn-ui add switch
```

-   [View source](https://github.com/RustForWeb/shadcn-ui/tree/main/packages/yew/switch)

{{#endtab }}
{{#endtabs }}

## Usage

{{#tabs global="framework" }}
{{#tab name="Yew" }}

```rust,ignore
use yew::prelude::*;

use crate::components::ui::switch::Switch;

#[component]
fn Usage() -> impl IntoView {
    view! {
        <Switch />
    }
}
```

{{#endtab }}
{{#endtabs }}

## Examples

### Form

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["switch"]
files = ["src/default/switch/switch_form.rs"]
url_fragment = "#/default/form"

[[file_replacements]]
find = "crate::default::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["switch"]
files = ["src/new_york/switch/switch_form.rs"]
url_fragment = "#/new-york/form"

[[file_replacements]]
find = "crate::new_york::"
replace = "crate::"
```

{{#endtab }}
{{#endtabs }}

{{#endtab }}
{{#endtabs }}

## See Also

-   [shadcn/ui documentation](https://ui.shadcn.com/docs/components/switch)
