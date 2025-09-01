# Testing Strategy for Rust shadcn/ui

## Current State Analysis

### **✅ Existing Infrastructure**
- **No current test files** found in the project
- **WASM-capable dependencies** already configured (web-sys, wasm-bindgen)
- **Component architecture** well-structured for testing
- **Workspace configuration** supports test dependencies

### **🔍 Testing Challenges Identified**
- **Cross-Framework Testing**: Leptos vs Yew implementations need identical behavior
- **WASM Testing Environment**: Browser-based component testing complexity
- **Theme Consistency**: Visual regression testing across Default/New York themes
- **Component Parity**: Ensuring feature equivalence between frameworks
- **CLI Integration**: Testing component generation and installation

## 🎯 **Multi-Layer Testing Strategy**

### **Layer 1: Unit Tests (Rust Native)**
```rust
// Component logic testing
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn button_class_generation() {
        let class = ButtonClass {
            variant: ButtonVariant::Primary,
            size: ButtonSize::Default,
        };
        assert!(class.to_string().contains("bg-primary"));
    }
    
    #[test]
    fn props_validation() {
        let props = ButtonProps {
            disabled: true,
            variant: ButtonVariant::Destructive,
            ..Default::default()
        };
        // Test prop combinations and validation
    }
}
```

### **Layer 2: Component Integration Tests (WASM)**
```rust
// WASM-based component rendering tests
#[cfg(test)]
mod integration_tests {
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn button_renders_correctly() {
        // Test component rendering in browser environment
    }
    
    #[wasm_bindgen_test]
    fn event_handlers_work() {
        // Test user interactions and callbacks
    }
}
```

### **Layer 3: Cross-Framework Parity Tests**
```rust
// Ensure Leptos and Yew components behave identically
#[cfg(test)]
mod parity_tests {
    #[test]
    fn leptos_yew_props_equivalence() {
        // Compare component APIs
    }
    
    #[test]
    fn theme_output_consistency() {
        // Verify same theme generates same CSS classes
    }
}
```

### **Layer 4: Visual Regression Tests (Browser)**
```javascript
// Playwright-based visual testing
describe('Button Component', () => {
  test('visual consistency across themes', async ({ page }) => {
    await page.goto('/components/button');
    await expect(page.locator('.button-default')).toHaveScreenshot();
    await expect(page.locator('.button-new-york')).toHaveScreenshot();
  });
});
```

### **Layer 5: End-to-End CLI Tests**
```bash
# CLI functionality testing
cargo test --bin rust-shadcn -- --test-threads=1
```

## 🛠 **Component Testing Framework**

### **Test Structure per Component**
```
packages/{framework}/{component}/
├── src/
│   ├── lib.rs
│   ├── default.rs
│   ├── new_york.rs
│   └── tests/           # New testing module
│       ├── mod.rs
│       ├── unit.rs      # Logic tests
│       ├── integration.rs # WASM tests
│       └── parity.rs    # Cross-framework tests
└── tests/
    ├── visual/          # Visual regression assets
    └── e2e/            # End-to-end test scenarios
```

### **Shared Test Utilities**
```rust
// packages/test-utils/src/lib.rs
pub mod component_tester;
pub mod theme_validator;
pub mod parity_checker;
pub mod visual_regression;

pub struct ComponentTester<T> {
    component: T,
    theme: Theme,
    framework: Framework,
}

impl<T> ComponentTester<T> {
    pub fn new(component: T) -> Self { /* */ }
    pub fn with_theme(self, theme: Theme) -> Self { /* */ }
    pub fn test_rendering(&self) -> TestResult { /* */ }
    pub fn test_interactions(&self) -> TestResult { /* */ }
    pub fn compare_with_framework<U>(&self, other: U) -> ParityResult { /* */ }
}
```

## 📊 **Quality Gates & Success Criteria**

### **Component Release Criteria**
```yaml
unit_tests:
  coverage: >=90%
  status: REQUIRED
  
integration_tests:
  browser_compatibility: [Chrome, Firefox, Safari, Edge]
  status: REQUIRED
  
parity_tests:
  framework_consistency: 100%
  theme_consistency: 100%
  status: REQUIRED
  
visual_regression:
  pixel_perfect_threshold: 99.5%
  status: REQUIRED
  
performance_tests:
  wasm_bundle_size: <50KB per component
  render_time: <16ms (60fps)
  status: REQUIRED
```

### **CI/CD Pipeline Integration**

#### **GitHub Actions Workflow**
```yaml
# .github/workflows/test.yml
name: Test Suite

on: [push, pull_request]

jobs:
  unit_tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --workspace
      
  wasm_tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
      - run: wasm-pack test --headless --firefox
      
  visual_regression:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
      - run: npm ci
      - run: npx playwright test
      
  parity_validation:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo test parity_tests --workspace
```

### **Test Automation Tools**

#### **Custom Test Runner**
```rust
// scripts/src/test_runner.rs
use std::process::{Command, ExitStatus};

pub struct TestSuite {
    components: Vec<String>,
    frameworks: Vec<Framework>,
}

impl TestSuite {
    pub fn run_unit_tests(&self) -> TestResults {
        // Execute Rust unit tests
    }
    
    pub fn run_wasm_tests(&self) -> TestResults {
        // Execute WASM-based integration tests
    }
    
    pub fn run_visual_tests(&self) -> TestResults {
        // Execute Playwright visual regression tests
    }
    
    pub fn run_parity_tests(&self) -> TestResults {
        // Execute cross-framework parity validation
    }
    
    pub fn generate_report(&self) -> TestReport {
        // Comprehensive test reporting
    }
}
```

#### **Visual Regression Test Generator**
```rust
// test-utils/src/visual_regression.rs
pub struct VisualRegressionSuite {
    components: Vec<ComponentSpec>,
    themes: Vec<Theme>,
    viewports: Vec<Viewport>,
}

impl VisualRegressionSuite {
    pub fn generate_test_cases(&self) -> Vec<TestCase> {
        // Generate comprehensive visual test matrix
    }
    
    pub fn capture_baselines(&self) -> Result<(), Error> {
        // Capture reference screenshots
    }
    
    pub fn run_comparisons(&self) -> Vec<VisualDiff> {
        // Compare current vs baseline
    }
}
```

## 📋 **Implementation Timeline**

### **Week 1-2: Test Infrastructure Setup**
```yaml
deliverables:
  - test-utils package creation
  - WASM test environment configuration
  - GitHub Actions CI/CD pipeline
  - Visual regression baseline capture system
  
tools:
  - wasm-bindgen-test for WASM testing
  - Playwright for visual regression
  - Custom test utilities for parity checking
```

### **Week 3-4: Component Test Implementation**
```yaml
scope:
  - Unit tests for existing 20 components (Leptos + Yew)
  - Integration tests for browser compatibility
  - Parity tests between framework implementations
  
coverage_target: 90% for existing components
automation: Full CI integration
```

### **Week 5-11: Progressive Test Development**
```yaml
approach: Test-Driven Development (TDD)
strategy: Write tests BEFORE implementing new components
coverage: Each new component must pass all 5 test layers
validation: Automatic quality gate enforcement
```

## 🔧 **Testing Tools & Dependencies**

### **Cargo.toml Dependencies**
```toml
[workspace.dependencies]
# Testing dependencies
wasm-bindgen-test = "0.3"
web-sys = { version = "0.3", features = ["console", "Document", "Element", "HtmlElement"] }
js-sys = "0.3"
console_error_panic_hook = "0.1"

# Visual testing
playwright = "0.1"
image = "0.24"
imageproc = "0.23"

# Test utilities
tokio-test = "0.4"
pretty_assertions = "1.4"
```

### **Component Testing Dependencies per Package**
```toml
[dev-dependencies]
wasm-bindgen-test = { workspace = true }
web-sys = { workspace = true }
test-utils = { path = "../../../test-utils" }
```

### **Browser Test Configuration**
```javascript
// playwright.config.js
module.exports = {
  testDir: './packages',
  timeout: 30000,
  use: {
    headless: true,
    viewport: { width: 1280, height: 720 },
  },
  projects: [
    { name: 'chromium', use: { ...devices['Desktop Chrome'] } },
    { name: 'firefox', use: { ...devices['Desktop Firefox'] } },
    { name: 'webkit', use: { ...devices['Desktop Safari'] } },
  ],
};
```

## 🎯 **Success Metrics**

### **Quantitative Metrics**
- **Test Coverage**: ≥90% unit test coverage across all components
- **Cross-Browser Support**: 100% compatibility across Chrome, Firefox, Safari, Edge
- **Performance**: Component render time <16ms for 60fps
- **Bundle Size**: Individual component bundles <50KB
- **Visual Accuracy**: 99.5% pixel-perfect theme consistency

### **Qualitative Metrics**
- **Developer Experience**: <5min to run full test suite locally
- **CI/CD Speed**: <10min total pipeline execution time
- **Test Reliability**: <1% flaky test rate
- **Framework Parity**: 100% API and behavior consistency

### **Continuous Monitoring**
```rust
// Quality metrics tracking
pub struct QualityMetrics {
    pub test_coverage: f64,
    pub performance_score: f64,
    pub parity_score: f64,
    pub visual_accuracy: f64,
    pub bundle_sizes: HashMap<String, u64>,
}

impl QualityMetrics {
    pub fn meets_release_criteria(&self) -> bool {
        self.test_coverage >= 0.90
            && self.performance_score >= 0.95
            && self.parity_score >= 1.0
            && self.visual_accuracy >= 0.995
    }
}
```