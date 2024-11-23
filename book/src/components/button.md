# Button

Displays a button or a component that looks like a button.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button"]
files = ["src/default/button/button.rs"]
url_fragment = "#/default/"

[[file_replacements]]
find = "crate::default::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button"]
files = ["src/new_york/button/button.rs"]
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
rust-shadcn-ui add button
```

-   [View source](https://github.com/RustForWeb/shadcn-ui/tree/main/packages/yew/button)

{{#endtab }}
{{#endtabs }}

## Usage

{{#tabs global="framework" }}
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
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button"]
files = ["src/default/button/button.rs"]
url_fragment = "#/default/"

[[file_replacements]]
find = "crate::default::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button"]
files = ["src/new_york/button/button.rs"]
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
features = ["button"]
files = ["src/default/button/button_secondary.rs"]
url_fragment = "#/default/secondary"

[[file_replacements]]
find = "crate::new_york::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button"]
files = ["src/new_york/button/button_secondary.rs"]
url_fragment = "#/new-york/secondary"
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
features = ["button"]
files = ["src/default/button/button_destructive.rs"]
url_fragment = "#/default/destructive"

[[file_replacements]]
find = "crate::new_york::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button"]
files = ["src/new_york/button/button_destructive.rs"]
url_fragment = "#/new-york/destructive"
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
features = ["button"]
files = ["src/default/button/button_outline.rs"]
url_fragment = "#/default/outline"

[[file_replacements]]
find = "crate::new_york::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button"]
files = ["src/new_york/button/button_outline.rs"]
url_fragment = "#/new-york/outline"
```

{{#endtab }}
{{#endtabs }}

{{#endtab }}
{{#endtabs }}

### Ghost

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button"]
files = ["src/default/button/button_ghost.rs"]
url_fragment = "#/default/ghost"

[[file_replacements]]
find = "crate::new_york::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button"]
files = ["src/new_york/button/button_ghost.rs"]
url_fragment = "#/new-york/ghost"
```

{{#endtab }}
{{#endtabs }}

{{#endtab }}
{{#endtabs }}

### Link

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button"]
files = ["src/default/button/button_link.rs"]
url_fragment = "#/default/link"

[[file_replacements]]
find = "crate::new_york::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button"]
files = ["src/new_york/button/button_link.rs"]
url_fragment = "#/new-york/link"
```

{{#endtab }}
{{#endtabs }}

{{#endtab }}
{{#endtabs }}

### Icon

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button"]
files = ["src/default/button/button_icon.rs"]
url_fragment = "#/default/icon"

[[file_replacements]]
find = "crate::new_york::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button"]
files = ["src/new_york/button/button_icon.rs"]
url_fragment = "#/new-york/icon"
```

{{#endtab }}
{{#endtabs }}

{{#endtab }}
{{#endtabs }}

### With Icon

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button"]
files = ["src/default/button/button_with_icon.rs"]
url_fragment = "#/default/with-icon"

[[file_replacements]]
find = "crate::new_york::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button"]
files = ["src/new_york/button/button_with_icon.rs"]
url_fragment = "#/new-york/with-icon"
```

{{#endtab }}
{{#endtabs }}

{{#endtab }}
{{#endtabs }}

### Loading

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button"]
files = ["src/default/button/button_loading.rs"]
url_fragment = "#/default/loading"

[[file_replacements]]
find = "crate::new_york::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button"]
files = ["src/new_york/button/button_loading.rs"]
url_fragment = "#/new-york/loading"
```

{{#endtab }}
{{#endtabs }}

{{#endtab }}
{{#endtabs }}

### As Child

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button"]
files = ["src/default/button/button_as_child.rs"]
url_fragment = "#/default/as-child"

[[file_replacements]]
find = "crate::new_york::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["button"]
files = ["src/new_york/button/button_as_child.rs"]
url_fragment = "#/new-york/as-child"
```

{{#endtab }}
{{#endtabs }}

{{#endtab }}
{{#endtabs }}

## See Also

-   [shadcn/ui documentation](https://ui.shadcn.com/docs/components/button)
