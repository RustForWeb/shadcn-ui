# TypeScript Definitions for shadcn/ui Leptos Components

This package provides TypeScript definitions for all shadcn/ui Leptos components, enabling better development experience and type safety when integrating with TypeScript projects.

## Installation

```bash
npm install @shadcn-ui/leptos-types
# or
yarn add @shadcn-ui/leptos-types
# or
pnpm add @shadcn-ui/leptos-types
```

## Usage

### Basic Import

```typescript
import type { ButtonProps, InputProps, CardProps } from '@shadcn-ui/leptos-types';
```

### Component Props

```typescript
import type { ButtonProps } from '@shadcn-ui/leptos-types';

const buttonProps: ButtonProps = {
  variant: 'default',
  size: 'lg',
  disabled: false,
  on_click: (event) => console.log('Button clicked!'),
  class: 'custom-button-class'
};
```

### Form Integration

```typescript
import type { FormProps, InputProps, ButtonProps } from '@shadcn-ui/leptos-types';

const formProps: FormProps = {
  on_submit: (data) => {
    console.log('Form submitted:', data);
  }
};

const inputProps: InputProps = {
  type: 'email',
  placeholder: 'Enter your email',
  required: true,
  on_change: (event) => console.log('Input changed:', event.target.value)
};
```

### Event Handling

```typescript
import type { EventProps } from '@shadcn-ui/leptos-types';

const eventProps: EventProps = {
  on_click: (event) => console.log('Clicked'),
  on_change: (event) => console.log('Changed'),
  on_focus: (event) => console.log('Focused'),
  on_blur: (event) => console.log('Blurred'),
  on_keydown: (event) => console.log('Key pressed:', event.key),
  on_mouseenter: (event) => console.log('Mouse entered'),
  on_mouseleave: (event) => console.log('Mouse left')
};
```

## Available Components

### Form Components
- `Button` - Various button styles and sizes
- `Input` - Text input with validation support
- `Label` - Form labels with accessibility
- `Checkbox` - Checkbox input component
- `Select` - Dropdown selection component
- `RadioGroup` - Radio button group
- `Form` - Form container with validation

### Layout Components
- `Card` - Card container with header, content, footer
- `Dialog` - Modal dialog component
- `Tabs` - Tabbed interface component
- `Alert` - Alert/notification component
- `Badge` - Badge/tag component

### Data Components
- `Combobox` - Searchable dropdown
- `Table` - Data table component
- `Pagination` - Page navigation

## Type Definitions

### Common Props

All components extend these base interfaces:

```typescript
interface ComponentProps {
  class?: string;        // CSS classes
  style?: string;        // Inline styles
  id?: string;          // Element ID
  [key: string]: any;   // Additional props
}

interface ChildrenProps {
  children?: any;        // Child elements
}

interface EventProps {
  on_click?: (event: any) => void;
  on_change?: (event: any) => void;
  on_input?: (event: any) => void;
  on_submit?: (event: any) => void;
  on_focus?: (event: any) => void;
  on_blur?: (event: any) => void;
  on_keydown?: (event: any) => void;
  on_keyup?: (event: any) => void;
  on_mouseenter?: (event: any) => void;
  on_mouseleave?: (event: any) => void;
}
```

### Leptos Signal Types

```typescript
type Signal<T> = {
  get(): T;
  set(value: T): void;
};

type ReadSignal<T> = {
  get(): T;
};

type WriteSignal<T> = {
  set(value: T): void;
};
```

## Integration with Leptos

These types are designed to work seamlessly with Leptos components:

```typescript
import { Button } from 'shadcn-ui-leptos-button';
import type { ButtonProps } from '@shadcn-ui/leptos-types';

// Type-safe props
const button_props: ButtonProps = {
  variant: 'primary',
  size: 'lg',
  on_click: move |_| console.log('Clicked!')
};

// Use in Leptos view
view! {
  <Button ..button_props>
    "Click me!"
  </Button>
}
```

## Development

### Building Types

```bash
# Check TypeScript compilation
npx tsc --noEmit

# Build types (if needed)
npx tsc
```

### Contributing

When adding new components:

1. Add the component interface to `index.d.ts`
2. Extend from appropriate base interfaces
3. Include all props and event handlers
4. Add JSDoc comments for complex props
5. Update this README with usage examples

## License

MIT License - see the main project license for details.
