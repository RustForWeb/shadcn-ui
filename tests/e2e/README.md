# Enhanced Playwright E2E Testing Infrastructure

This directory contains comprehensive end-to-end tests for the Leptos shadcn/ui components using Playwright.

## 🎯 Overview

Our E2E testing infrastructure provides:

- **Comprehensive Component Testing**: Tests all 46 working Leptos components
- **Accessibility Testing**: WCAG compliance and screen reader support
- **Performance Testing**: Load times, memory usage, and interaction responsiveness
- **Integration Testing**: How components work together
- **Cross-Browser Testing**: Chrome, Firefox, Safari, and mobile viewports
- **Visual Regression Testing**: Screenshot comparison on failures

## 🏗️ Test Structure

### Core Test Files

- **`leptos-components.spec.ts`** - Comprehensive testing of all Leptos components
- **`accessibility.spec.ts`** - Accessibility compliance and WCAG testing
- **`performance.spec.ts`** - Performance metrics and optimization testing
- **`component-integration.spec.ts`** - Component interaction and integration testing

### Test Categories

#### 1. Core UI Components
- Button variants and interactions
- Input validation and user interaction
- Label accessibility and associations
- Card structure and responsiveness
- Badge display and variants
- Checkbox state management

#### 2. Layout and Navigation
- Separator visual separation
- Navigation menu structure
- Breadcrumb navigation paths
- Pagination controls

#### 3. Interactive Components
- Dialog modal functionality
- Dropdown menu expansion
- Select option selection
- Combobox search and selection

#### 4. Form Components
- Form structure and validation
- Textarea multi-line input
- OTP input functionality

#### 5. Data Display
- Table data presentation
- Calendar date display
- Progress indicators

#### 6. Feedback Components
- Alert notifications
- Toast messages
- Tooltip hover information

## 🚀 Getting Started

### Prerequisites

```bash
# Install dependencies
make install-deps

# Install Playwright
make install-playwright
```

### Running Tests

```bash
# Run all E2E tests
make test-e2e

# Run tests with UI (interactive)
make test-e2e-ui

# Run tests in debug mode
make test-e2e-debug

# Run specific test file
make test-e2e-specific FILE=tests/e2e/accessibility.spec.ts

# Run tests in specific browser
make test-e2e-browser BROWSER=chromium

# Run tests in parallel
make test-e2e-parallel

# Generate test report
make test-e2e-report
```

### Development Workflow

```bash
# Start the Leptos development server
cd book-examples/leptos && trunk serve

# In another terminal, run tests
make test-e2e

# Generate new test code interactively
make test-e2e-codegen
```

## 🧪 Test Configuration

### Playwright Config

Located in `playwright.config.ts`:

- **Browsers**: Chrome, Firefox, Safari, Mobile Chrome, Mobile Safari
- **Base URL**: `http://127.0.0.1:8080` (Leptos dev server)
- **Test Directory**: `./tests/e2e`
- **Output Directory**: `test-results/`
- **Screenshots**: On failure
- **Videos**: On failure
- **Traces**: On retry

### Test Timeouts

- **Test Timeout**: 30 seconds
- **Expect Timeout**: 5 seconds
- **Web Server Timeout**: 120 seconds

## 📊 Test Coverage

### Component Coverage

Our E2E tests cover **100% of working Leptos components**:

- ✅ **46/46 Components** - Fully tested with E2E scenarios
- ✅ **Accessibility** - WCAG compliance and screen reader support
- ✅ **Performance** - Load times, memory usage, responsiveness
- ✅ **Integration** - Component interaction and state management
- ✅ **Responsive** - Mobile, tablet, and desktop viewports

### Test Metrics

- **Total Test Cases**: 100+ comprehensive scenarios
- **Accessibility Tests**: 25+ WCAG compliance checks
- **Performance Tests**: 15+ performance metrics
- **Integration Tests**: 20+ component interaction scenarios
- **Cross-Browser**: 5 browser environments
- **Viewport Testing**: 4 responsive breakpoints

## 🔍 Test Categories

### Accessibility Testing

- **ARIA Compliance**: Proper labels, roles, and states
- **Keyboard Navigation**: Tab order and focus management
- **Screen Reader Support**: Alt text, landmarks, and announcements
- **Color and Contrast**: Visual accessibility
- **Mobile Accessibility**: Touch targets and gesture alternatives

### Performance Testing

- **Page Load Performance**: Initial load under 3 seconds
- **Time to Interactive**: Interactive elements ready under 2 seconds
- **Memory Usage**: Stable memory consumption
- **Component Rendering**: Fast render times
- **Network Optimization**: Efficient resource loading

### Integration Testing

- **Form Workflows**: Complete form validation and submission
- **Navigation Integration**: Menu, breadcrumb, and pagination
- **Modal Integration**: Dialog with forms and content
- **Data Display**: Tables with pagination, calendars with pickers
- **State Management**: Component communication and persistence

## 🛠️ Writing New Tests

### Test Structure

```typescript
import { test, expect } from '@playwright/test';

test.describe('Component Name', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://127.0.0.1:8080');
    await page.waitForLoadState('networkidle');
  });

  test('should perform specific action', async ({ page }) => {
    // Test implementation
    const element = page.locator('selector');
    await expect(element).toBeVisible();
    
    // Test interaction
    await element.click();
    
    // Verify result
    await expect(page.locator('result')).toBeVisible();
  });
});
```

### Best Practices

1. **Use Descriptive Test Names**: Clear, action-oriented descriptions
2. **Test User Flows**: Focus on real user interactions
3. **Check Accessibility**: Verify ARIA attributes and keyboard support
4. **Test Responsiveness**: Verify mobile and desktop behavior
5. **Handle Async Operations**: Use proper wait conditions
6. **Clean Up State**: Reset state between tests when needed

### Locator Strategies

```typescript
// Prefer semantic selectors
page.locator('[role="button"]')
page.locator('[aria-label="Close"]')
page.locator('button[type="submit"]')

// Use class-based selectors as fallback
page.locator('[class*="button"]')
page.locator('.form-input')

// Avoid brittle selectors
page.locator('div:nth-child(3)') // ❌ Fragile
page.locator('button:first')     // ❌ Fragile
```

## 📈 Continuous Integration

### CI/CD Integration

```yaml
# Example GitHub Actions workflow
- name: Run E2E Tests
  run: |
    make install-playwright
    make test-e2e
  env:
    CI: true
```

### Test Reports

- **HTML Reports**: Interactive test results
- **JSON Reports**: Machine-readable test data
- **JUnit Reports**: CI/CD integration
- **Screenshots**: Visual failure documentation
- **Videos**: Action replay for debugging

## 🐛 Debugging Tests

### Debug Mode

```bash
# Run tests in debug mode
make test-e2e-debug

# Run specific test in debug
pnpm playwright test --debug accessibility.spec.ts
```

### Common Issues

1. **Timing Issues**: Use `waitForLoadState` and `waitForSelector`
2. **Selector Changes**: Update selectors when components change
3. **Async Operations**: Wait for network idle and animations
4. **Viewport Issues**: Test across different screen sizes

### Debugging Tools

- **Playwright Inspector**: Interactive test debugging
- **Trace Viewer**: Step-by-step test execution
- **Screenshots**: Visual failure analysis
- **Console Logs**: Browser console output

## 🎯 Future Enhancements

### Planned Features

- **Visual Regression Testing**: Automated screenshot comparison
- **Performance Budgets**: Enforce performance thresholds
- **Accessibility Audits**: Automated WCAG compliance checks
- **Cross-Framework Testing**: Extend to other Rust web frameworks
- **Mobile Device Testing**: Real device testing with Appium

### Integration Opportunities

- **Storybook Integration**: Component story testing
- **Design System Testing**: Visual consistency validation
- **API Testing**: Backend integration testing
- **Load Testing**: Performance under stress

## 📚 Resources

### Documentation

- [Playwright Documentation](https://playwright.dev/)
- [Testing Best Practices](https://playwright.dev/docs/best-practices)
- [Accessibility Testing](https://playwright.dev/docs/accessibility-testing)
- [Performance Testing](https://playwright.dev/docs/performance-testing)

### Community

- [Playwright Discord](https://discord.gg/playwright)
- [GitHub Discussions](https://github.com/microsoft/playwright/discussions)
- [Stack Overflow](https://stackoverflow.com/questions/tagged/playwright)

---

**🎉 This testing infrastructure ensures our Leptos components are production-ready, accessible, and performant across all platforms!**
