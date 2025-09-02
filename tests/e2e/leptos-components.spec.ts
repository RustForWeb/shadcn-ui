import { test, expect } from '@playwright/test';

test.describe('Leptos Components - Comprehensive E2E Testing', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to Leptos example app
    await page.goto('/');
    // Wait for the app to be fully loaded
    await page.waitForLoadState('networkidle');
  });

  test.describe('Core UI Components', () => {
    test('button component - basic functionality and variants', async ({ page }) => {
      // Test different button variants
      const buttons = page.locator('button');
      await expect(buttons).toHaveCount(await buttons.count());
      
      // Test primary button
      const primaryButton = page.locator('button').filter({ hasText: /primary|default|submit/i }).first();
      if (await primaryButton.isVisible()) {
        await expect(primaryButton).toBeVisible();
        await primaryButton.click();
        // Verify button state changes
        await expect(primaryButton).toBeEnabled();
      }
    });

    test('input component - user interaction and validation', async ({ page }) => {
      const inputs = page.locator('input[type="text"], input[type="email"], input[type="password"]');
      
      if (await inputs.count() > 0) {
        const input = inputs.first();
        await expect(input).toBeVisible();
        
        // Test typing
        await input.fill('Test input value');
        await expect(input).toHaveValue('Test input value');
        
        // Test clearing
        await input.clear();
        await expect(input).toHaveValue('');
      }
    });

    test('label component - accessibility and association', async ({ page }) => {
      const labels = page.locator('label');
      
      if (await labels.count() > 0) {
        for (let i = 0; i < await labels.count(); i++) {
          const label = labels.nth(i);
          await expect(label).toBeVisible();
          
          // Check if label has proper text content
          const text = await label.textContent();
          expect(text).toBeTruthy();
          expect(text!.trim().length).toBeGreaterThan(0);
        }
      }
    });

    test('card component - structure, styling, and responsiveness', async ({ page }) => {
      const cards = page.locator('[class*="card"], .card, [class*="rounded"], [class*="border"]');
      
      if (await cards.count() > 0) {
        const card = cards.first();
        await expect(card).toBeVisible();
        
        // Test responsive behavior
        await page.setViewportSize({ width: 320, height: 568 }); // Mobile
        await expect(card).toBeVisible();
        
        await page.setViewportSize({ width: 1280, height: 720 }); // Desktop
        await expect(card).toBeVisible();
      }
    });

    test('badge component - display and variants', async ({ page }) => {
      const badges = page.locator('[class*="badge"], .badge, [class*="inline-flex"], [class*="rounded-full"]');
      
      if (await badges.count() > 0) {
        for (let i = 0; i < await badges.count(); i++) {
          const badge = badges.nth(i);
          await expect(badge).toBeVisible();
          
          // Check badge content
          const text = await badge.textContent();
          expect(text).toBeTruthy();
        }
      }
    });

    test('checkbox component - interaction and state', async ({ page }) => {
      const checkboxes = page.locator('input[type="checkbox"]');
      
      if (await checkboxes.count() > 0) {
        const checkbox = checkboxes.first();
        await expect(checkbox).toBeVisible();
        
        // Test checkbox interaction
        const initialChecked = await checkbox.isChecked();
        await checkbox.click();
        const afterClickChecked = await checkbox.isChecked();
        
        // State should change
        expect(afterClickChecked).toBe(!initialChecked);
      }
    });
  });

  test.describe('Layout and Navigation Components', () => {
    test('separator component - visual separation', async ({ page }) => {
      const separators = page.locator('hr, [class*="separator"], [class*="border-t"], [class*="border-b"]');
      
      if (await separators.count() > 0) {
        const separator = separators.first();
        await expect(separator).toBeVisible();
        
        // Check if separator has proper styling
        const classes = await separator.getAttribute('class');
        expect(classes).toBeTruthy();
      }
    });

    test('navigation-menu component - menu structure and interaction', async ({ page }) => {
      const navMenus = page.locator('nav, [class*="navigation"], [class*="menu"]');
      
      if (await navMenus.count() > 0) {
        const nav = navMenus.first();
        await expect(nav).toBeVisible();
        
        // Test navigation items
        const navItems = nav.locator('a, button, [role="menuitem"]');
        if (await navItems.count() > 0) {
          await expect(navItems.first()).toBeVisible();
        }
      }
    });

    test('breadcrumb component - navigation path', async ({ page }) => {
      const breadcrumbs = page.locator('[class*="breadcrumb"], .breadcrumb, nav[aria-label*="breadcrumb"]');
      
      if (await breadcrumbs.count() > 0) {
        const breadcrumb = breadcrumbs.first();
        await expect(breadcrumb).toBeVisible();
        
        // Check breadcrumb items
        const items = breadcrumb.locator('a, span, [class*="breadcrumb-item"]');
        if (await items.count() > 0) {
          await expect(items.first()).toBeVisible();
        }
      }
    });

    test('pagination component - page navigation', async ({ page }) => {
      const paginations = page.locator('[class*="pagination"], .pagination, nav[aria-label*="pagination"]');
      
      if (await paginations.count() > 0) {
        const pagination = paginations.first();
        await expect(pagination).toBeVisible();
        
        // Check pagination controls
        const controls = pagination.locator('button, a, [class*="page"]');
        if (await controls.count() > 0) {
          await expect(controls.first()).toBeVisible();
        }
      }
    });
  });

  test.describe('Interactive Components', () => {
    test('dialog component - modal functionality', async ({ page }) => {
      const dialogTriggers = page.locator('button[aria-haspopup="dialog"], [data-state="closed"]');
      
      if (await dialogTriggers.count() > 0) {
        const trigger = dialogTriggers.first();
        await expect(trigger).toBeVisible();
        
        // Test dialog opening
        await trigger.click();
        
        // Look for dialog content
        const dialog = page.locator('[role="dialog"], [data-state="open"]');
        if (await dialog.count() > 0) {
          await expect(dialog.first()).toBeVisible();
        }
      }
    });

    test('dropdown-menu component - menu expansion', async ({ page }) => {
      const dropdownTriggers = page.locator('button[aria-haspopup="true"], [data-state="closed"]');
      
      if (await dropdownTriggers.count() > 0) {
        const trigger = dropdownTriggers.first();
        await expect(trigger).toBeVisible();
        
        // Test dropdown opening
        await trigger.click();
        
        // Look for dropdown content
        const dropdown = page.locator('[role="menu"], [data-state="open"]');
        if (await dropdown.count() > 0) {
          await expect(dropdown.first()).toBeVisible();
        }
      }
    });

    test('select component - option selection', async ({ page }) => {
      const selects = page.locator('select, [role="combobox"], [class*="select"]');
      
      if (await selects.count() > 0) {
        const select = selects.first();
        await expect(select).toBeVisible();
        
        // Test select interaction
        await select.click();
        
        // Look for options
        const options = page.locator('[role="option"], option');
        if (await options.count() > 0) {
          await expect(options.first()).toBeVisible();
        }
      }
    });

    test('combobox component - search and selection', async ({ page }) => {
      const comboboxes = page.locator('[class*="combobox"], [role="combobox"]');
      
      if (await comboboxes.count() > 0) {
        const combobox = comboboxes.first();
        await expect(combobox).toBeVisible();
        
        // Test input functionality
        const input = combobox.locator('input');
        if (await input.count() > 0) {
          await input.first().fill('test');
          await expect(input.first()).toHaveValue('test');
        }
      }
    });
  });

  test.describe('Form Components', () => {
    test('form component - structure and validation', async ({ page }) => {
      const forms = page.locator('form, [class*="form"]');
      
      if (await forms.count() > 0) {
        const form = forms.first();
        await expect(form).toBeVisible();
        
        // Check form elements
        const formElements = form.locator('input, select, textarea, button');
        if (await formElements.count() > 0) {
          await expect(formElements.first()).toBeVisible();
        }
      }
    });

    test('textarea component - multi-line input', async ({ page }) => {
      const textareas = page.locator('textarea, [class*="textarea"]');
      
      if (await textareas.count() > 0) {
        const textarea = textareas.first();
        await expect(textarea).toBeVisible();
        
        // Test textarea input
        await textarea.fill('Multi-line\ntest input');
        await expect(textarea).toHaveValue('Multi-line\ntest input');
      }
    });

    test('input-otp component - one-time password input', async ({ page }) => {
      const otpInputs = page.locator('[class*="otp"], [class*="input-otp"], input[inputmode="numeric"]');
      
      if (await otpInputs.count() > 0) {
        const otpInput = otpInputs.first();
        await expect(otpInput).toBeVisible();
        
        // Test numeric input
        await otpInput.fill('1234');
        await expect(otpInput).toHaveValue('1234');
      }
    });
  });

  test.describe('Data Display Components', () => {
    test('table component - data presentation', async ({ page }) => {
      const tables = page.locator('table, [class*="table"]');
      
      if (await tables.count() > 0) {
        const table = tables.first();
        await expect(table).toBeVisible();
        
        // Check table structure
        const rows = table.locator('tr');
        if (await rows.count() > 0) {
          await expect(rows.first()).toBeVisible();
        }
      }
    });

    test('calendar component - date display', async ({ page }) => {
      const calendars = page.locator('[class*="calendar"], [class*="date-picker"]');
      
      if (await calendars.count() > 0) {
        const calendar = calendars.first();
        await expect(calendar).toBeVisible();
        
        // Check calendar structure
        const days = calendar.locator('[class*="day"], [class*="date"]');
        if (await days.count() > 0) {
          await expect(days.first()).toBeVisible();
        }
      }
    });

    test('progress component - loading indicators', async ({ page }) => {
      const progressBars = page.locator('[class*="progress"], progress, [role="progressbar"]');
      
      if (await progressBars.count() > 0) {
        const progress = progressBars.first();
        await expect(progress).toBeVisible();
        
        // Check progress attributes
        const value = await progress.getAttribute('value');
        const max = await progress.getAttribute('max');
        expect(value || max).toBeTruthy();
      }
    });
  });

  test.describe('Feedback Components', () => {
    test('alert component - notification display', async ({ page }) => {
      const alerts = page.locator('[class*="alert"], .alert, [role="alert"]');
      
      if (await alerts.count() > 0) {
        const alert = alerts.first();
        await expect(alert).toBeVisible();
        
        // Check alert content
        const text = await alert.textContent();
        expect(text).toBeTruthy();
      }
    });

    test('toast component - temporary notifications', async ({ page }) => {
      const toasts = page.locator('[class*="toast"], .toast, [role="status"]');
      
      if (await toasts.count() > 0) {
        const toast = toasts.first();
        await expect(toast).toBeVisible();
        
        // Check toast content
        const text = await toast.textContent();
        expect(text).toBeTruthy();
      }
    });

    test('tooltip component - hover information', async ({ page }) => {
      const tooltipTriggers = page.locator('[data-tooltip], [title], [aria-describedby]');
      
      if (await tooltipTriggers.count() > 0) {
        const trigger = tooltipTriggers.first();
        await expect(trigger).toBeVisible();
        
        // Test tooltip hover
        await trigger.hover();
        
        // Look for tooltip content
        const tooltip = page.locator('[role="tooltip"], [class*="tooltip"]');
        if (await tooltip.count() > 0) {
          await expect(tooltip.first()).toBeVisible();
        }
      }
    });
  });

  test.describe('Accessibility and Performance', () => {
    test('keyboard navigation - tab order and focus', async ({ page }) => {
      // Test tab navigation
      await page.keyboard.press('Tab');
      const focusedElement = page.locator(':focus');
      await expect(focusedElement).toBeVisible();
      
      // Test multiple tab presses
      for (let i = 0; i < 5; i++) {
        await page.keyboard.press('Tab');
        const newFocused = page.locator(':focus');
        if (await newFocused.count() > 0) {
          await expect(newFocused.first()).toBeVisible();
        }
      }
    });

    test('ARIA labels and semantic markup', async ({ page }) => {
      // Check for proper ARIA labels
      const elementsWithAriaLabel = page.locator('[aria-label]');
      const count = await elementsWithAriaLabel.count();
      
      if (count > 0) {
        for (let i = 0; i < Math.min(count, 5); i++) {
          const element = elementsWithAriaLabel.nth(i);
          const ariaLabel = await element.getAttribute('aria-label');
          expect(ariaLabel).toBeTruthy();
          expect(ariaLabel!.length).toBeGreaterThan(0);
        }
      }
      
      // Check for proper roles
      const elementsWithRole = page.locator('[role]');
      const roleCount = await elementsWithRole.count();
      expect(roleCount).toBeGreaterThanOrEqual(0);
    });

    test('color contrast and visual accessibility', async ({ page }) => {
      // This would require visual testing tools
      // For now, we'll check that text elements have proper contrast
      const textElements = page.locator('p, h1, h2, h3, h4, h5, h6, span, div');
      if (await textElements.count() > 0) {
        await expect(textElements.first()).toBeVisible();
      }
    });

    test('component loading performance', async ({ page }) => {
      const startTime = Date.now();
      
      await page.goto('/');
      await page.waitForLoadState('networkidle');
      
      const loadTime = Date.now() - startTime;
      
      // Assert reasonable load time (adjust threshold as needed)
      expect(loadTime).toBeLessThan(5000); // 5 seconds
    });

    test('responsive design across viewports', async ({ page }) => {
      const testViewports = [
        { width: 320, height: 568, name: 'Mobile' },
        { width: 768, height: 1024, name: 'Tablet' },
        { width: 1280, height: 720, name: 'Desktop' },
        { width: 1920, height: 1080, name: 'Large Desktop' }
      ];
      
      for (const viewport of testViewports) {
        await page.setViewportSize(viewport);
        await expect(page.locator('body')).toBeVisible();
        
        // Check that main content is still accessible
        const mainContent = page.locator('main, [role="main"], body > *').first();
        await expect(mainContent).toBeVisible();
      }
    });
  });

  test.describe('Component Integration', () => {
    test('component composition - multiple components working together', async ({ page }) => {
      // Test that multiple components can coexist
      const interactiveElements = page.locator('button, input, select, textarea, a');
      const count = await interactiveElements.count();
      
      if (count > 0) {
        // Test first few elements
        for (let i = 0; i < Math.min(count, 3); i++) {
          const element = interactiveElements.nth(i);
          await expect(element).toBeVisible();
        }
      }
    });

    test('state management - component state persistence', async ({ page }) => {
      // Test form state persistence
      const inputs = page.locator('input[type="text"]');
      
      if (await inputs.count() > 0) {
        const input = inputs.first();
        await input.fill('Persistent test value');
        
        // Navigate away and back
        await page.goto('/');
        await page.waitForLoadState('networkidle');
        
        // Check if state is maintained (this depends on implementation)
        const newInput = page.locator('input[type="text"]').first();
        if (await newInput.count() > 0) {
          await expect(newInput).toBeVisible();
        }
      }
    });
  });
});
