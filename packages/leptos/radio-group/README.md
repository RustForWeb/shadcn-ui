# shadcn-ui-leptos-radio-group

Leptos port of [shadcn/ui Radio Group](https://ui.shadcn.com/docs/components/radio-group).

A set of checkable buttons—known as radio buttons—where no more than one of the buttons can be checked at a time.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
shadcn-ui-leptos-radio-group = { git = "https://github.com/shadcn-ui/shadcn-ui-rust" }
```

## Usage

```rust
use leptos::prelude::*;
use shadcn_ui_leptos_radio_group::{RadioGroup, RadioGroupItem};

#[component]
pub fn MyComponent() -> impl IntoView {
    let (selected_value, set_selected_value) = create_signal(None::<String>);
    
    let on_value_change = Callback::from(move |value: String| {
        set_selected_value.set(Some(value));
    });
    
    view! {
        <RadioGroup
            value=selected_value
            on_value_change=on_value_change
        >
            <RadioGroupItem value="option1".to_string()>
                "Option 1"
            </RadioGroupItem>
            <RadioGroupItem value="option2".to_string()>
                "Option 2"
            </RadioGroupItem>
            <RadioGroupItem value="option3".to_string()>
                "Option 3"
            </RadioGroupItem>
        </RadioGroup>
    }
}
```

## Variants

### Default

The default styling variant.

```rust
use shadcn_ui_leptos_radio_group::{RadioGroup, RadioGroupItem};
```

### New York

A variant with subtle styling differences.

```rust
use shadcn_ui_leptos_radio_group::{RadioGroupNewYork as RadioGroup, RadioGroupItemNewYork as RadioGroupItem};
```

## Props

### RadioGroup

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `MaybeProp<String>` | `None` | Currently selected value |
| `on_value_change` | `Option<Callback<String>>` | `None` | Callback when value changes |
| `disabled` | `Signal<bool>` | `false` | Whether the radio group is disabled |
| `class` | `MaybeProp<String>` | `None` | Additional CSS classes |
| `id` | `MaybeProp<String>` | `None` | HTML id attribute |
| `style` | `Signal<Style>` | `Style::default()` | Inline styles |

### RadioGroupItem

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `String` | **Required** | The value of this radio item |
| `disabled` | `Signal<bool>` | `false` | Whether this item is disabled |
| `class` | `MaybeProp<String>` | `None` | Additional CSS classes |
| `id` | `MaybeProp<String>` | `None` | HTML id attribute |
| `style` | `Signal<Style>` | `Style::default()` | Inline styles |

## Accessibility

The component follows ARIA guidelines for radio groups:

- Uses `role="radiogroup"` on the container
- Uses `role="radio"` on each item
- Provides `aria-checked` attribute
- Includes proper focus management
- Supports keyboard navigation

## License

MIT
