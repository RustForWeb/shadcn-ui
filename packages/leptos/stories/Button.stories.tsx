import type { Meta, StoryObj } from '@storybook/react';
import { useState } from 'react';

// Mock Button component for Storybook demonstration
// In a real implementation, this would be the actual Leptos Button component
const Button = ({ 
  variant = 'default', 
  size = 'default', 
  disabled = false, 
  loading = false,
  children,
  onClick,
  className = '',
  ...props 
}: {
  variant?: 'default' | 'destructive' | 'outline' | 'secondary' | 'ghost' | 'link';
  size?: 'default' | 'sm' | 'lg' | 'icon';
  disabled?: boolean;
  loading?: boolean;
  children: React.ReactNode;
  onClick?: () => void;
  className?: string;
  [key: string]: any;
}) => {
  const baseClasses = 'inline-flex items-center justify-center rounded-md font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background';
  
  const variantClasses = {
    default: 'bg-primary text-primary-foreground hover:bg-primary/90',
    destructive: 'bg-destructive text-destructive-foreground hover:bg-destructive/90',
    outline: 'border border-input hover:bg-accent hover:text-accent-foreground',
    secondary: 'bg-secondary text-secondary-foreground hover:bg-secondary/80',
    ghost: 'hover:bg-accent hover:text-accent-foreground',
    link: 'underline-offset-4 hover:underline text-primary'
  };
  
  const sizeClasses = {
    default: 'h-10 py-2 px-4',
    sm: 'h-9 px-3 rounded-md',
    lg: 'h-11 px-8 rounded-md',
    icon: 'h-10 w-10'
  };
  
  const classes = `${baseClasses} ${variantClasses[variant]} ${sizeClasses[size]} ${className}`;
  
  return (
    <button 
      className={classes} 
      disabled={disabled || loading}
      onClick={onClick}
      {...props}
    >
      {loading && (
        <svg className="mr-2 h-4 w-4 animate-spin" fill="none" viewBox="0 0 24 24">
          <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
          <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
        </svg>
      )}
      {children}
    </button>
  );
};

const meta: Meta<typeof Button> = {
  title: 'Components/Button',
  component: Button,
  parameters: {
    layout: 'centered',
    docs: {
      description: {
        component: 'A versatile button component with multiple variants, sizes, and states. Built with Tailwind CSS and designed for accessibility.',
      },
    },
  },
  argTypes: {
    variant: {
      control: { type: 'select' },
      options: ['default', 'destructive', 'outline', 'secondary', 'ghost', 'link'],
      description: 'The visual style variant of the button',
    },
    size: {
      control: { type: 'select' },
      options: ['default', 'sm', 'lg', 'icon'],
      description: 'The size of the button',
    },
    disabled: {
      control: { type: 'boolean' },
      description: 'Whether the button is disabled',
    },
    loading: {
      control: { type: 'boolean' },
      description: 'Whether the button shows a loading spinner',
    },
    children: {
      control: { type: 'text' },
      description: 'The content inside the button',
    },
    onClick: {
      action: 'clicked',
      description: 'Function called when the button is clicked',
    },
  },
  tags: ['autodocs'],
};

export default meta;
type Story = StoryObj<typeof meta>;

// Basic button story
export const Default: Story = {
  args: {
    children: 'Button',
    variant: 'default',
    size: 'default',
  },
};

// All variants showcase
export const Variants: Story = {
  render: () => (
    <div className="component-showcase">
      <h3 className="text-lg font-semibold mb-4">Button Variants</h3>
      <div className="grid grid-cols-2 gap-4">
        <div className="component-variant">
          <h4 className="font-medium mb-2">Default</h4>
          <Button variant="default">Default Button</Button>
        </div>
        <div className="component-variant">
          <h4 className="font-medium mb-2">Destructive</h4>
          <Button variant="destructive">Destructive Button</Button>
        </div>
        <div className="component-variant">
          <h4 className="font-medium mb-2">Outline</h4>
          <Button variant="outline">Outline Button</Button>
        </div>
        <div className="component-variant">
          <h4 className="font-medium mb-2">Secondary</h4>
          <Button variant="secondary">Secondary Button</Button>
        </div>
        <div className="component-variant">
          <h4 className="font-medium mb-2">Ghost</h4>
          <Button variant="ghost">Ghost Button</Button>
        </div>
        <div className="component-variant">
          <h4 className="font-medium mb-2">Link</h4>
          <Button variant="link">Link Button</Button>
        </div>
      </div>
    </div>
  ),
};

// All sizes showcase
export const Sizes: Story = {
  render: () => (
    <div className="component-showcase">
      <h3 className="text-lg font-semibold mb-4">Button Sizes</h3>
      <div className="flex items-center gap-4">
        <div className="component-variant">
          <h4 className="font-medium mb-2">Small</h4>
          <Button size="sm">Small Button</Button>
        </div>
        <div className="component-variant">
          <h4 className="font-medium mb-2">Default</h4>
          <Button size="default">Default Button</Button>
        </div>
        <div className="component-variant">
          <h4 className="font-medium mb-2">Large</h4>
          <Button size="lg">Large Button</Button>
        </div>
        <div className="component-variant">
          <h4 className="font-medium mb-2">Icon</h4>
          <Button size="icon">🚀</Button>
        </div>
      </div>
    </div>
  ),
};

// Interactive playground
export const Playground: Story = {
  render: () => {
    const [props, setProps] = useState({
      variant: 'default' as const,
      size: 'default' as const,
      disabled: false,
      loading: false,
      children: 'Interactive Button',
    });

    return (
      <div className="component-showcase">
        <h3 className="text-lg font-semibold mb-4">Interactive Playground</h3>
        
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {/* Component Demo */}
          <div className="component-demo">
            <h4 className="font-medium mb-4">Live Preview</h4>
            <Button
              variant={props.variant}
              size={props.size}
              disabled={props.disabled}
              loading={props.loading}
              onClick={() => alert('Button clicked!')}
            >
              {props.children}
            </Button>
          </div>

          {/* Props Panel */}
          <div className="props-panel">
            <h4 className="font-medium mb-4">Customize Props</h4>
            
            <div className="props-control">
              <label className="props-label">Variant</label>
              <select
                className="props-select"
                value={props.variant}
                onChange={(e) => setProps(prev => ({ ...prev, variant: e.target.value as any }))}
              >
                <option value="default">Default</option>
                <option value="destructive">Destructive</option>
                <option value="outline">Outline</option>
                <option value="secondary">Secondary</option>
                <option value="ghost">Ghost</option>
                <option value="link">Link</option>
              </select>
            </div>

            <div className="props-control">
              <label className="props-label">Size</label>
              <select
                className="props-select"
                value={props.size}
                onChange={(e) => setProps(prev => ({ ...prev, size: e.target.value as any }))}
              >
                <option value="sm">Small</option>
                <option value="default">Default</option>
                <option value="lg">Large</option>
                <option value="icon">Icon</option>
              </select>
            </div>

            <div className="props-control">
              <label className="props-label">Button Text</label>
              <input
                className="props-input"
                type="text"
                value={props.children}
                onChange={(e) => setProps(prev => ({ ...prev, children: e.target.value }))}
              />
            </div>

            <div className="props-control">
              <label className="flex items-center">
                <input
                  className="props-checkbox mr-2"
                  type="checkbox"
                  checked={props.disabled}
                  onChange={(e) => setProps(prev => ({ ...prev, disabled: e.target.checked }))}
                />
                Disabled
              </label>
            </div>

            <div className="props-control">
              <label className="flex items-center">
                <input
                  className="props-checkbox mr-2"
                  type="checkbox"
                  checked={props.loading}
                  onChange={(e) => setProps(prev => ({ ...prev, loading: e.target.checked }))}
                />
                Loading
              </label>
            </div>
          </div>
        </div>

        {/* Code Example */}
        <div className="mt-6">
          <h4 className="font-medium mb-2">Generated Code</h4>
          <div className="code-example">
            <pre>
              <span className="code-keyword">view!</span> {'{'}
              <br />
              {'  '}<span className="code-keyword">&lt;</span><span className="code-string">Button</span>
              <br />
              {'    '}<span className="code-keyword">variant</span>=<span className="code-string">"{props.variant}"</span>
              <br />
              {'    '}<span className="code-keyword">size</span>=<span className="code-string">"{props.size}"</span>
              <br />
              {'    '}<span className="code-keyword">disabled</span>=<span className="code-string">"{props.disabled}"</span>
              <br />
              {'    '}<span className="code-keyword">loading</span>=<span className="code-string">"{props.loading}"</span>
              <br />
              {'  '}<span className="code-keyword">&gt;</span>
              <br />
              {'    '}<span className="code-string">"{props.children}"</span>
              <br />
              {'  '}<span className="code-keyword">&lt;/</span><span className="code-string">Button</span><span className="code-keyword">&gt;</span>
              <br />
              {'}'}
            </pre>
          </div>
        </div>
      </div>
    );
  },
};

// States showcase
export const States: Story = {
  render: () => (
    <div className="component-showcase">
      <h3 className="text-lg font-semibold mb-4">Button States</h3>
      <div className="grid grid-cols-2 gap-4">
        <div className="component-variant">
          <h4 className="font-medium mb-2">Normal</h4>
          <Button>Normal Button</Button>
        </div>
        <div className="component-variant">
          <h4 className="font-medium mb-2">Hover</h4>
          <Button className="hover:bg-blue-600">Hover Button</Button>
        </div>
        <div className="component-variant">
          <h4 className="font-medium mb-2">Disabled</h4>
          <Button disabled>Disabled Button</Button>
        </div>
        <div className="component-variant">
          <h4 className="font-medium mb-2">Loading</h4>
          <Button loading>Loading Button</Button>
        </div>
      </div>
    </div>
  ),
};
