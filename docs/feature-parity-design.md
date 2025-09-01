# Feature Parity Design: Rust shadcn/ui

## Current State Analysis

### ✅ **Existing Architecture**
- **Project Structure**: Monorepo with framework-specific packages (Leptos, Yew)
- **Registry System**: Central registry for component metadata and CLI integration
- **Theme Support**: Default & New York style variants for each component
- **CLI Tool**: `rust-shadcn` for component installation and management

### 📊 **Current Component Coverage**

**Leptos Framework** (44 components - 86% complete):
- accordion, alert, alert-dialog, aspect-ratio, badge, breadcrumb, button, calendar, card, carousel, checkbox, collapsible, combobox, command, context-menu, date-picker, dialog, drawer, dropdown-menu, form, hover-card, input, input-otp, label, menubar, navigation-menu, pagination, popover, progress, radio-group, scroll-area, select, separator, sheet, skeleton, slider, switch, table, tabs, textarea, toast, toggle, tooltip, utils

**Yew Framework** (20 components - 39% complete):
- alert, aspect-ratio, avatar, badge, breadcrumb, button, card, checkbox, dialog, input, label, pagination, radio-group, select, separator, skeleton, switch, table, textarea, tooltip

**Missing from both frameworks** (7 components):
- avatar (Leptos only), data-table, chart, resizable, sidebar, sonner, typography

**Missing from Yew** (25 components):
- accordion, alert-dialog, calendar, carousel, collapsible, combobox, command, context-menu, date-picker, drawer, dropdown-menu, form, hover-card, input-otp, menubar, navigation-menu, popover, progress, scroll-area, sheet, slider, tabs, toast, toggle, utils

## 🎯 **Feature Parity Architecture Design**

### **Phase 1: Foundation Enhancement**

#### **Registry System Optimization**
```rust
// Enhanced registry structure
pub struct ComponentRegistry {
    leptos: FrameworkRegistry,
    yew: FrameworkRegistry,
    dioxus: FrameworkRegistry,  // Future framework
}

pub struct FrameworkRegistry {
    components: HashMap<String, ComponentDef>,
    dependencies: DependencyGraph,
    theme_variants: ThemeRegistry,
}
```

#### **Component Generation Pipeline**
```
Source Definition → Framework Adapter → Theme Variants → Output Files
```

### **Phase 2: Systematic Component Implementation**

#### **Priority Matrix**
```yaml
tier_1_critical: [dialog, dropdown-menu, select, checkbox, radio-group]
tier_2_forms: [form, combobox, date-picker, textarea, input-otp] 
tier_3_navigation: [navigation-menu, menubar, tabs, breadcrumb]
tier_4_feedback: [toast, alert-dialog, progress, skeleton]
tier_5_layout: [sheet, resizable, scroll-area, collapsible]
tier_6_advanced: [data-table, chart, carousel, command]
```

#### **Framework Parity Strategy**
```
Yew (15) → Leptos (5) = 10 component gap
Target: Leptos reaches Yew parity + Both frameworks implement remaining 36 components
```

### **Phase 3: Advanced Features**

#### **Cross-Framework Compatibility Layer**
```rust
pub trait ShadcnComponent {
    type Props;
    fn render(props: Self::Props) -> RenderedComponent;
    fn theme_variants() -> Vec<ThemeVariant>;
    fn dependencies() -> Vec<Dependency>;
}
```

#### **Enhanced CLI Integration**
```bash
# Enhanced command structure
rust-shadcn add <component> --framework <leptos|yew|dioxus>
rust-shadcn init --framework <framework> --theme <default|new-york>
rust-shadcn diff --component <name> --between <version1> <version2>
rust-shadcn sync --target-framework <framework>
```

## 🚀 **Implementation Roadmap**

### **Phase 1: Infrastructure (Weeks 1-2)**
1. **Registry Enhancement**
   - Populate `registry_ui.rs` with component definitions
   - Implement dependency resolution system
   - Add theme variant management

2. **Code Generation Pipeline** 
   - Template system for consistent component structure
   - Framework-specific adapters
   - Automated testing integration

### **Phase 2: Component Implementation (Weeks 3-8)**
1. **Leptos Parity** (Week 3)
   - Port missing 10 components from Yew to Leptos
   - Ensure API consistency

2. **Tier 1 Critical Components** (Weeks 4-5)
   - dialog, dropdown-menu, select, checkbox, radio-group
   - Both frameworks simultaneously

3. **Tier 2-6 Progressive Implementation** (Weeks 6-8)
   - Form components → Navigation → Feedback → Layout → Advanced
   - Maintain framework parity throughout

### **Phase 3: Advanced Features (Weeks 9-10)**
1. **Cross-Framework Compatibility**
   - Shared trait definitions
   - Component interoperability tests

2. **Enhanced Tooling**
   - CLI feature completion
   - Documentation generation
   - Migration utilities

## 📋 **Technical Specifications**

### **Component Structure Standard**
```rust
// Each component package structure
src/
├── lib.rs          // Public API and framework integration
├── default.rs      // Default theme implementation  
├── new_york.rs     // New York theme implementation
└── types.rs        // Shared types and props
```

### **Dependency Management**
```toml
# Standardized dependency patterns
[dependencies]
framework-core = { version = "0.7", features = ["web"] }
framework-dom = "0.7"
web-sys = "0.3"
wasm-bindgen = "0.2"

[dev-dependencies] 
wasm-bindgen-test = "0.3"
```

### **Quality Assurance Framework**
- **Component Testing**: Unit tests for each variant
- **Cross-Browser Testing**: WASM compatibility validation  
- **Theme Consistency**: Automated style verification
- **API Compatibility**: Framework-agnostic interface validation

## 🎨 **Design Principles**

### **Consistency**
- Identical API surface across frameworks
- Matching visual output between themes
- Unified documentation and examples

### **Performance**
- Minimal WASM bundle sizes
- Efficient DOM updates
- Lazy loading capabilities

### **Developer Experience**
- Clear migration paths between frameworks
- Comprehensive documentation
- Interactive examples and demos