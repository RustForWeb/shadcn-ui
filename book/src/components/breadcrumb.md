# Breadcrumb

Displays the path to the current resource using a hierarchy of links.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["breadcrumb"]
files = ["src/default/breadcrumb/breadcrumb.rs"]
url_fragment = "#/default/"

[[file_replacements]]
find = "crate::default::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["breadcrumb"]
files = ["src/new_york/breadcrumb/breadcrumb.rs"]
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
rust-shadcn-ui add breadcrumb
```

-   [View source](https://github.com/RustForWeb/shadcn-ui/tree/main/packages/yew/breadcrumb)

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
        <Breadcrumb>
            <BreadcrumbList>
                <BreadcrumbItem>
                    <BreadcrumbLink href="/">{"Home"}</BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                    <BreadcrumbLink href="/components">{"Components"}</BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                    <BreadcrumbPage>{"Breadcrumb"}</BreadcrumbPage>
                </BreadcrumbItem>
            </BreadcrumbList>
        </Breadcrumb>
    }
}
```

{{#endtab }}
{{#endtabs }}

## Examples

### Custom Separator

Use a custom component as children for `<BreadcrumbSeparator />` to create a custom separator.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["breadcrumb"]
files = ["src/default/breadcrumb/breadcrumb_separator.rs"]
url_fragment = "#/default/separator"

[[file_replacements]]
find = "crate::default::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["breadcrumb"]
files = ["src/new_york/breadcrumb/breadcrumb_separator.rs"]
url_fragment = "#/new-york/separator"

[[file_replacements]]
find = "crate::new_york::"
replace = "crate::"
```

{{#endtab }}
{{#endtabs }}

{{#endtab }}
{{#endtabs }}

### Dropdown

You can compose `<BreadcrumbItem />` with a `<DropdownMenu />` to create a dropdown in the breadcrumb.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["breadcrumb"]
files = ["src/default/breadcrumb/breadcrumb_dropdown.rs"]
url_fragment = "#/default/dropdown"

[[file_replacements]]
find = "crate::default::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["breadcrumb"]
files = ["src/new_york/breadcrumb/breadcrumb_dropdown.rs"]
url_fragment = "#/new-york/dropdown"

[[file_replacements]]
find = "crate::new_york::"
replace = "crate::"
```

{{#endtab }}
{{#endtabs }}

{{#endtab }}
{{#endtabs }}

### Collapsed

A `<BreadcrumbEllipsis />` component is provided to show a collapsed state when the breadcrumb is too long.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["breadcrumb"]
files = ["src/default/breadcrumb/breadcrumb_ellipsis.rs"]
url_fragment = "#/default/ellipsis"

[[file_replacements]]
find = "crate::default::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["breadcrumb"]
files = ["src/new_york/breadcrumb/breadcrumb_ellipsis.rs"]
url_fragment = "#/new-york/ellipsis"

[[file_replacements]]
find = "crate::new_york::"
replace = "crate::"
```

{{#endtab }}
{{#endtabs }}

{{#endtab }}
{{#endtabs }}

### Link Component

To use a custom link component from your routing library, you can use the `as_child` prop on `<BreadcrumbLink />`.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["breadcrumb"]
files = ["src/default/breadcrumb/breadcrumb_link.rs"]
url_fragment = "#/default/link"

[[file_replacements]]
find = "crate::default::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["breadcrumb"]
files = ["src/new_york/breadcrumb/breadcrumb_link.rs"]
url_fragment = "#/new-york/link"

[[file_replacements]]
find = "crate::new_york::"
replace = "crate::"
```

{{#endtab }}
{{#endtabs }}

{{#endtab }}
{{#endtabs }}

### Responsive

Here's an example of a responsive breadcrumb that composes `<BreadcrumbItem />` with `<BreadcrumbEllipsis />`, `<DropdownMenu />`, and `<Drawer />`.

It displays a dropdown on desktop and a drawer on mobile.

{{#tabs global="framework" }}
{{#tab name="Yew" }}

{{#tabs global="style" }}
{{#tab name="Default" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["breadcrumb"]
files = ["src/default/breadcrumb/breadcrumb_responsive.rs"]
url_fragment = "#/default/responsive"

[[file_replacements]]
find = "crate::default::"
replace = "crate::"
```

{{#endtab }}
{{#tab name="New York" }}

```toml,trunk
package = "shadcn-ui-yew-book"
features = ["breadcrumb"]
files = ["src/new_york/breadcrumb/breadcrumb_responsive.rs"]
url_fragment = "#/new-york/responsive"

[[file_replacements]]
find = "crate::new_york::"
replace = "crate::"
```

{{#endtab }}
{{#endtabs }}

{{#endtab }}
{{#endtabs }}

## See Also

-   [shadcn/ui documentation](https://ui.shadcn.com/docs/components/breadcrumb)
