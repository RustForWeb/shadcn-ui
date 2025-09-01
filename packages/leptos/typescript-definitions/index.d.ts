// TypeScript definitions for shadcn/ui Leptos components
// This file provides type definitions for better TypeScript integration

declare module 'shadcn-ui-leptos/*' {
  // Common types used across components
  export interface ComponentProps {
    class?: string;
    style?: string;
    id?: string;
    [key: string]: any;
  }

  export interface ChildrenProps {
    children?: any;
  }

  export interface EventProps {
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

  // Button component
  export interface ButtonProps extends ComponentProps, EventProps {
    variant?: 'default' | 'destructive' | 'outline' | 'secondary' | 'ghost' | 'link';
    size?: 'default' | 'sm' | 'lg' | 'icon';
    disabled?: boolean;
    loading?: boolean;
  }

  // Input component
  export interface InputProps extends ComponentProps, EventProps {
    type?: 'text' | 'email' | 'password' | 'number' | 'tel' | 'url' | 'search';
    placeholder?: string;
    value?: string;
    disabled?: boolean;
    required?: boolean;
    min?: number;
    max?: number;
    step?: number;
    pattern?: string;
    autocomplete?: string;
    readonly?: boolean;
  }

  // Label component
  export interface LabelProps extends ComponentProps, EventProps {
    for?: string;
    required?: boolean;
  }

  // Card components
  export interface CardProps extends ComponentProps, ChildrenProps {}
  export interface CardHeaderProps extends ComponentProps, ChildrenProps {}
  export interface CardTitleProps extends ComponentProps, ChildrenProps {}
  export interface CardDescriptionProps extends ComponentProps, ChildrenProps {}
  export interface CardContentProps extends ComponentProps, ChildrenProps {}
  export interface CardFooterProps extends ComponentProps, ChildrenProps {}

  // Dialog components
  export interface DialogProps extends ComponentProps, ChildrenProps {
    open?: boolean;
    on_open_change?: (open: boolean) => void;
  }
  export interface DialogTriggerProps extends ComponentProps, ChildrenProps {}
  export interface DialogContentProps extends ComponentProps, ChildrenProps {}
  export interface DialogHeaderProps extends ComponentProps, ChildrenProps {}
  export interface DialogTitleProps extends ComponentProps, ChildrenProps {}
  export interface DialogDescriptionProps extends ComponentProps, ChildrenProps {}
  export interface DialogFooterProps extends ComponentProps, ChildrenProps {}

  // Tabs components
  export interface TabsProps extends ComponentProps, ChildrenProps {
    value?: string;
    on_value_change?: (value: string) => void;
    orientation?: 'horizontal' | 'vertical';
  }
  export interface TabsListProps extends ComponentProps, ChildrenProps {}
  export interface TabsTriggerProps extends ComponentProps, EventProps {
    value: string;
    disabled?: boolean;
  }
  export interface TabsContentProps extends ComponentProps, ChildrenProps {
    value: string;
  }

  // Select components
  export interface SelectProps extends ComponentProps, ChildrenProps {
    value?: string;
    on_value_change?: (value: string) => void;
    disabled?: boolean;
    required?: boolean;
  }
  export interface SelectTriggerProps extends ComponentProps, ChildrenProps {}
  export interface SelectValueProps extends ComponentProps, ChildrenProps {}
  export interface SelectContentProps extends ComponentProps, ChildrenProps {}
  export interface SelectItemProps extends ComponentProps, EventProps {
    value: string;
    disabled?: boolean;
  }

  // Checkbox component
  export interface CheckboxProps extends ComponentProps, EventProps {
    checked?: boolean;
    disabled?: boolean;
    required?: boolean;
    indeterminate?: boolean;
  }

  // Radio Group components
  export interface RadioGroupProps extends ComponentProps, ChildrenProps {
    value?: string;
    on_value_change?: (value: string) => void;
    disabled?: boolean;
    required?: boolean;
  }
  export interface RadioGroupItemProps extends ComponentProps, EventProps {
    value: string;
    disabled?: boolean;
  }

  // Alert components
  export interface AlertProps extends ComponentProps, ChildrenProps {
    variant?: 'default' | 'destructive';
  }
  export interface AlertTitleProps extends ComponentProps, ChildrenProps {}
  export interface AlertDescriptionProps extends ComponentProps, ChildrenProps {}

  // Badge component
  export interface BadgeProps extends ComponentProps, ChildrenProps {
    variant?: 'default' | 'secondary' | 'destructive' | 'outline';
  }

  // Combobox components
  export interface ComboboxProps extends ComponentProps, ChildrenProps {
    value?: string;
    on_value_change?: (value: string) => void;
    placeholder?: string;
    disabled?: boolean;
    searchable?: boolean;
  }
  export interface ComboboxInputProps extends ComponentProps, EventProps {
    placeholder?: string;
    disabled?: boolean;
  }
  export interface ComboboxContentProps extends ComponentProps, ChildrenProps {}
  export interface ComboboxItemProps extends ComponentProps, EventProps {
    value: string;
    disabled?: boolean;
  }

  // Form components
  export interface FormProps extends ComponentProps, ChildrenProps {
    on_submit?: (data: any) => void;
    validation_schema?: any;
  }
  export interface FormFieldProps extends ComponentProps, ChildrenProps {
    name: string;
    rules?: any;
  }
  export interface FormItemProps extends ComponentProps, ChildrenProps {}
  export interface FormLabelProps extends ComponentProps, ChildrenProps {}
  export interface FormControlProps extends ComponentProps, ChildrenProps {}
  export interface FormMessageProps extends ComponentProps, ChildrenProps {}

  // Utility types
  export type Signal<T> = {
    get(): T;
    set(value: T): void;
  };

  export type ReadSignal<T> = {
    get(): T;
  };

  export type WriteSignal<T> = {
    set(value: T): void;
  };

  // Component function types
  export type ComponentFunction<T = {}> = (props: T) => any;

  // Export component types
  export const Button: ComponentFunction<ButtonProps>;
  export const Input: ComponentFunction<InputProps>;
  export const Label: ComponentFunction<LabelProps>;
  export const Card: ComponentFunction<CardProps>;
  export const CardHeader: ComponentFunction<CardHeaderProps>;
  export const CardTitle: ComponentFunction<CardTitleProps>;
  export const CardDescription: ComponentFunction<CardDescriptionProps>;
  export const CardContent: ComponentFunction<CardContentProps>;
  export const CardFooter: ComponentFunction<CardFooterProps>;
  export const Dialog: ComponentFunction<DialogProps>;
  export const DialogTrigger: ComponentFunction<DialogTriggerProps>;
  export const DialogContent: ComponentFunction<DialogContentProps>;
  export const DialogHeader: ComponentFunction<DialogHeaderProps>;
  export const DialogTitle: ComponentFunction<DialogTitleProps>;
  export const DialogDescription: ComponentFunction<DialogDescriptionProps>;
  export const DialogFooter: ComponentFunction<DialogFooterProps>;
  export const Tabs: ComponentFunction<TabsProps>;
  export const TabsList: ComponentFunction<TabsListProps>;
  export const TabsTrigger: ComponentFunction<TabsTriggerProps>;
  export const TabsContent: ComponentFunction<TabsContentProps>;
  export const Select: ComponentFunction<SelectProps>;
  export const SelectTrigger: ComponentFunction<SelectTriggerProps>;
  export const SelectValue: ComponentFunction<SelectValueProps>;
  export const SelectContent: ComponentFunction<SelectContentProps>;
  export const SelectItem: ComponentFunction<SelectItemProps>;
  export const Checkbox: ComponentFunction<CheckboxProps>;
  export const RadioGroup: ComponentFunction<RadioGroupProps>;
  export const RadioGroupItem: ComponentFunction<RadioGroupItemProps>;
  export const Alert: ComponentFunction<AlertProps>;
  export const AlertTitle: ComponentFunction<AlertTitleProps>;
  export const AlertDescription: ComponentFunction<AlertDescriptionProps>;
  export const Badge: ComponentFunction<BadgeProps>;
  export const Combobox: ComponentFunction<ComboboxProps>;
  export const ComboboxInput: ComponentFunction<ComboboxInputProps>;
  export const ComboboxContent: ComponentFunction<ComboboxContentProps>;
  export const ComboboxItem: ComponentFunction<ComboboxItemProps>;
  export const Form: ComponentFunction<FormProps>;
  export const FormField: ComponentFunction<FormFieldProps>;
  export const FormItem: ComponentFunction<FormItemProps>;
  export const FormLabel: ComponentFunction<FormLabelProps>;
  export const FormControl: ComponentFunction<FormControlProps>;
  export const FormMessage: ComponentFunction<FormMessageProps>;
}

// Global types for Leptos integration
declare global {
  interface Window {
    leptos?: {
      mount: (component: any, target: string | Element) => void;
      render: (component: any) => any;
    };
  }
}

export {};
