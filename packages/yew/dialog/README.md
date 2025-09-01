# Dialog

A window overlaid on either the primary window or another dialog window, rendering the content underneath inert.

## Installation

Add the dialog component to your `Cargo.toml`:

```toml
[dependencies]
shadcn-ui-yew-dialog = { path = "path/to/shadcn-ui/packages/yew/dialog" }
```

## Usage

```rust
use shadcn_ui_yew_dialog::*;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <Dialog open=false>
            <DialogTrigger>
                {"Open Dialog"}
            </DialogTrigger>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>{"Edit Profile"}</DialogTitle>
                    <DialogDescription>
                        {"Make changes to your profile here. Click save when you're done."}
                    </DialogDescription>
                </DialogHeader>
                <div class="grid gap-4 py-4">
                    <div class="grid grid-cols-4 items-center gap-4">
                        <label for="name" class="text-right">{"Name"}</label>
                        <input id="name" value="Pedro Duarte" class="col-span-3" />
                    </div>
                    <div class="grid grid-cols-4 items-center gap-4">
                        <label for="username" class="text-right">{"Username"}</label>
                        <input id="username" value="@peduarte" class="col-span-3" />
                    </div>
                </div>
                <DialogFooter>
                    <button type="button" class="btn btn-secondary">
                        {"Save changes"}
                    </button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    }
}
```

## Components

### Dialog

The main dialog container that manages the dialog state.

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `open` | `bool` | `false` | Whether the dialog is open |
| `on_open_change` | `Option<Callback<bool>>` | `None` | Callback when open state changes |
| `disabled` | `bool` | `false` | Whether the dialog is disabled |
| `class` | `Option<String>` | `None` | Additional CSS classes |
| `id` | `Option<String>` | `None` | HTML id attribute |
| `style` | `Style` | `Style::default()` | Inline styles |
| `children` | `Children` | `Children::default()` | Dialog content |

### DialogTrigger

The button that opens the dialog.

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `disabled` | `bool` | `false` | Whether the trigger is disabled |
| `class` | `Option<String>` | `None` | Additional CSS classes |
| `id` | `Option<String>` | `None` | HTML id attribute |
| `style` | `Style` | `Style::default()` | Inline styles |
| `children` | `Children` | `Children::default()` | Trigger content |

### DialogContent

The content container for the dialog.

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `disabled` | `bool` | `false` | Whether the content is disabled |
| `class` | `Option<String>` | `None` | Additional CSS classes |
| `id` | `Option<String>` | `None` | HTML id attribute |
| `style` | `Style` | `Style::default()` | Inline styles |
| `children` | `Children` | `Children::default()` | Content |

### DialogHeader

The header section of the dialog.

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `class` | `Option<String>` | `None` | Additional CSS classes |
| `id` | `Option<String>` | `None` | HTML id attribute |
| `style` | `Style` | `Style::default()` | Inline styles |
| `children` | `Children` | `Children::default()` | Header content |

### DialogFooter

The footer section of the dialog.

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `class` | `Option<String>` | `None` | Additional CSS classes |
| `id` | `Option<String>` | `None` | HTML id attribute |
| `style` | `Style` | `Style::default()` | Inline styles |
| `children` | `Children` | `Children::default()` | Footer content |

### DialogTitle

The title of the dialog.

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `class` | `Option<String>` | `None` | Additional CSS classes |
| `id` | `Option<String>` | `None` | HTML id attribute |
| `style` | `Style` | `Style::default()` | Inline styles |
| `children` | `Children` | `Children::default()` | Title text |

### DialogDescription

The description of the dialog.

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `class` | `Option<String>` | `None` | Additional CSS classes |
| `id` | `Option<String>` | `None` | HTML id attribute |
| `style` | `Style` | `Style::default()` | Inline styles |
| `children` | `Children` | `Children::default()` | Description text |

## Variants

### Default

The default dialog styling with standard animations and spacing.

### New York

The New York variant with subtle styling differences.

```rust
use shadcn_ui_yew_dialog::*;

// Use New York variants
<DialogNewYork>
    <DialogTriggerNewYork>{"Open Dialog"}</DialogTriggerNewYork>
    <DialogContentNewYork>
        <DialogHeaderNewYork>
            <DialogTitleNewYork>{"Title"}</DialogTitleNewYork>
            <DialogDescriptionNewYork>{"Description"}</DialogDescriptionNewYork>
        </DialogHeaderNewYork>
        <DialogFooterNewYork>
            {"Footer"}
        </DialogFooterNewYork>
    </DialogContentNewYork>
</DialogNewYork>
```

## Examples

### Basic Dialog

```rust
<Dialog open=false>
    <DialogTrigger>
        {"Open Dialog"}
    </DialogTrigger>
    <DialogContent>
        <DialogHeader>
            <DialogTitle>{"Are you sure?"}</DialogTitle>
            <DialogDescription>
                {"This action cannot be undone."}
            </DialogDescription>
        </DialogHeader>
        <DialogFooter>
            <button type="button" class="btn btn-secondary">
                {"Cancel"}
            </button>
            <button type="button" class="btn btn-destructive">
                {"Delete"}
            </button>
        </DialogFooter>
    </DialogContent>
</Dialog>
```

### Form Dialog

```rust
<Dialog open=false>
    <DialogTrigger>
        {"Edit Profile"}
    </DialogTrigger>
    <DialogContent>
        <DialogHeader>
            <DialogTitle>{"Edit Profile"}</DialogTitle>
            <DialogDescription>
                {"Make changes to your profile here. Click save when you're done."}
            </DialogDescription>
        </DialogHeader>
        <form>
            <div class="grid gap-4 py-4">
                <div class="grid grid-cols-4 items-center gap-4">
                    <label for="name" class="text-right">{"Name"}</label>
                    <input id="name" value="Pedro Duarte" class="col-span-3" />
                </div>
                <div class="grid grid-cols-4 items-center gap-4">
                    <label for="username" class="text-right">{"Username"}</label>
                    <input id="username" value="@peduarte" class="col-span-3" />
                </div>
            </div>
            <DialogFooter>
                <button type="submit" class="btn btn-primary">
                    {"Save changes"}
                </button>
            </DialogFooter>
        </form>
    </DialogContent>
</Dialog>
```

### Controlled Dialog

```rust
use yew::prelude::*;

#[function_component]
fn ControlledDialog() -> Html {
    let open = use_state(|| false);
    
    let on_open_change = {
        let open = open.clone();
        Callback::from(move |is_open: bool| {
            open.set(is_open);
        })
    };
    
    html! {
        <div>
            <button onclick={let open = open.clone(); Callback::from(move |_| open.set(true))}>
                {"Open Dialog"}
            </button>
            <Dialog open={*open} on_open_change={on_open_change}>
                <DialogContent>
                    <DialogHeader>
                        <DialogTitle>{"Controlled Dialog"}</DialogTitle>
                        <DialogDescription>
                            {"This dialog is controlled by external state."}
                        </DialogDescription>
                    </DialogHeader>
                    <DialogFooter>
                        <button onclick={let open = open.clone(); Callback::from(move |_| open.set(false))}>
                            {"Close"}
                        </button>
                    </DialogFooter>
                </DialogContent>
            </Dialog>
        </div>
    }
}
```

## Accessibility

The Dialog component includes the following accessibility features:

- **ARIA attributes**: Proper `role="dialog"` and `aria-modal="true"` attributes
- **Keyboard navigation**: ESC key closes the dialog
- **Focus management**: Focus is trapped within the dialog when open
- **Backdrop click**: Clicking outside the dialog closes it
- **Screen reader support**: Proper heading structure with `DialogTitle`

## Styling

The Dialog component uses Tailwind CSS classes and can be customized with additional classes:

```rust
<Dialog class="custom-dialog-class">
    <DialogContent class="custom-content-class">
        <DialogHeader class="custom-header-class">
            <DialogTitle class="custom-title-class">
                {"Custom Styled Dialog"}
            </DialogTitle>
        </DialogHeader>
    </DialogContent>
</Dialog>
```

## Testing

The Dialog component includes comprehensive tests:

```bash
cargo test -p shadcn-ui-yew-dialog
```

## License

MIT
