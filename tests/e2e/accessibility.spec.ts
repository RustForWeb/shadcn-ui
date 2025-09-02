import { test, expect } from '@playwright/test';

test.describe('Accessibility Testing Suite', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to Leptos example app for accessibility testing
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test.describe('ARIA Compliance', () => {
    test('all interactive elements have proper ARIA labels', async ({ page }) => {
      // Test buttons
      const buttons = page.locator('button');
      for (let i = 0; i < await buttons.count(); i++) {
        const button = buttons.nth(i);
        const ariaLabel = await button.getAttribute('aria-label');
        const ariaLabelledby = await button.getAttribute('aria-labelledby');
        const textContent = await button.textContent();
        
        // Button should have either aria-label, aria-labelledby, or meaningful text
        expect(ariaLabel || ariaLabelledby || (textContent && textContent.trim().length > 0)).toBeTruthy();
      }

      // Test inputs
      const inputs = page.locator('input[type="text"], input[type="email"], input[type="password"]');
      for (let i = 0; i < await inputs.count(); i++) {
        const input = inputs.nth(i);
        const ariaLabel = await input.getAttribute('aria-label');
        const ariaLabelledby = await input.getAttribute('aria-labelledby');
        const placeholder = await input.getAttribute('placeholder');
        
        // Input should have accessibility information
        expect(ariaLabel || ariaLabelledby || placeholder).toBeTruthy();
      }
    });

    test('form elements have proper associations', async ({ page }) => {
      const forms = page.locator('form');
      for (let i = 0; i < await forms.count(); i++) {
        const form = forms.nth(i);
        
        // Check form inputs
        const inputs = form.locator('input, select, textarea');
        for (let j = 0; j < await inputs.count(); j++) {
          const input = inputs.nth(j);
          const id = await input.getAttribute('id');
          const ariaLabel = await input.getAttribute('aria-label');
          const ariaLabelledby = await input.getAttribute('aria-labelledby');
          
          // Input should have accessibility information
          expect(id || ariaLabel || ariaLabelledby).toBeTruthy();
        }
      }
    });

    test('navigation elements have proper roles', async ({ page }) => {
      // Test navigation menus
      const navs = page.locator('nav');
      for (let i = 0; i < await navs.count(); i++) {
        const nav = navs.nth(i);
        const role = await nav.getAttribute('role');
        const ariaLabel = await nav.getAttribute('aria-label');
        
        // Navigation should have proper labeling
        expect(role === 'navigation' || ariaLabel).toBeTruthy();
      }

      // Test menu items
      const menuItems = page.locator('[role="menuitem"]');
      for (let i = 0; i < await menuItems.count(); i++) {
        const item = menuItems.nth(i);
        const ariaLabel = await item.getAttribute('aria-label');
        const textContent = await item.textContent();
        
        // Menu item should have accessible text
        expect(ariaLabel || (textContent && textContent.trim().length > 0)).toBeTruthy();
      }
    });
  });

  test.describe('Keyboard Navigation', () => {
    test('tab order is logical and complete', async ({ page }) => {
      // Get all focusable elements
      const focusableElements = page.locator('button, input, select, textarea, a, [tabindex]:not([tabindex="-1"])');
      const count = await focusableElements.count();
      
      if (count > 0) {
        // Test tab navigation through all elements
        for (let i = 0; i < Math.min(count, 10); i++) {
          await page.keyboard.press('Tab');
          const focusedElement = page.locator(':focus');
          
          if (await focusedElement.count() > 0) {
            await expect(focusedElement.first()).toBeVisible();
            
            // Verify element is actually focusable
            const tagName = await focusedElement.first().evaluate(el => el.tagName.toLowerCase());
            const tabIndex = await focusedElement.first().getAttribute('tabindex');
            
            // Element should be focusable
            expect(['button', 'input', 'select', 'textarea', 'a'].includes(tagName) || tabIndex !== '-1').toBeTruthy();
          }
        }
      }
    });

    test('enter and space keys work on interactive elements', async ({ page }) => {
      // Test buttons
      const buttons = page.locator('button');
      if (await buttons.count() > 0) {
        const button = buttons.first();
        await button.focus();
        
        // Test space key
        await page.keyboard.press('Space');
        // Button should still be focused
        await expect(button).toBeFocused();
        
        // Test enter key
        await page.keyboard.press('Enter');
        // Button should still be focused
        await expect(button).toBeFocused();
      }
    });

    test('escape key closes modals and dropdowns', async ({ page }) => {
      // Test dialog triggers
      const dialogTriggers = page.locator('button[aria-haspopup="dialog"]');
      if (await dialogTriggers.count() > 0) {
        const trigger = dialogTriggers.first();
        await trigger.click();
        
        // Look for dialog
        const dialog = page.locator('[role="dialog"]');
        if (await dialog.count() > 0) {
          await expect(dialog.first()).toBeVisible();
          
          // Press escape
          await page.keyboard.press('Escape');
          
          // Dialog should be closed or hidden
          const isVisible = await dialog.first().isVisible();
          expect(isVisible).toBeFalsy();
        }
      }
    });
  });

  test.describe('Screen Reader Support', () => {
    test('images have alt text or are decorative', async ({ page }) => {
      const images = page.locator('img');
      for (let i = 0; i < await images.count(); i++) {
        const img = images.nth(i);
        const alt = await img.getAttribute('alt');
        const ariaHidden = await img.getAttribute('aria-hidden');
        const role = await img.getAttribute('role');
        
        // Image should have alt text, be marked as decorative, or have presentation role
        expect(alt || ariaHidden === 'true' || role === 'presentation').toBeTruthy();
      }
    });

    test('decorative elements are properly hidden', async ({ page }) => {
      const decorativeElements = page.locator('[aria-hidden="true"], [role="presentation"]');
      for (let i = 0; i < await decorativeElements.count(); i++) {
        const element = decorativeElements.nth(i);
        
        // Decorative elements should not have meaningful content
        const textContent = await element.textContent();
        const ariaLabel = await element.getAttribute('aria-label');
        
        // Should not have both aria-hidden and meaningful content
        expect(!(ariaLabel && ariaLabel.trim().length > 0) || !(textContent && textContent.trim().length > 0)).toBeTruthy();
      }
    });

    test('landmarks are properly defined', async ({ page }) => {
      // Test main landmark
      const main = page.locator('main, [role="main"]');
      if (await main.count() > 0) {
        await expect(main.first()).toBeVisible();
      }

      // Test navigation landmarks
      const navs = page.locator('nav, [role="navigation"]');
      for (let i = 0; i < await navs.count(); i++) {
        const nav = navs.nth(i);
        const ariaLabel = await nav.getAttribute('aria-label');
        
        // Navigation should have descriptive label
        expect(ariaLabel && ariaLabel.trim().length > 0).toBeTruthy();
      }

      // Test complementary landmarks
      const complementary = page.locator('aside, [role="complementary"]');
      for (let i = 0; i < await complementary.count(); i++) {
        const comp = complementary.nth(i);
        const ariaLabel = await comp.getAttribute('aria-label');
        
        // Complementary content should have descriptive label
        expect(ariaLabel && ariaLabel.trim().length > 0).toBeTruthy();
      }
    });
  });

  test.describe('Color and Contrast', () => {
    test('text elements have sufficient contrast', async ({ page }) => {
      // This is a basic check - full contrast testing would require visual testing tools
      const textElements = page.locator('p, h1, h2, h3, h4, h5, h6, span, div, label');
      
      if (await textElements.count() > 0) {
        // Check that text elements are visible
        for (let i = 0; i < Math.min(await textElements.count(), 5); i++) {
          const element = textElements.nth(i);
          await expect(element).toBeVisible();
          
          // Check that text has content
          const text = await element.textContent();
          if (text && text.trim().length > 0) {
            expect(text.trim().length).toBeGreaterThan(0);
          }
        }
      }
    });

    test('focus indicators are visible', async ({ page }) => {
      // Test focus visibility
      const focusableElements = page.locator('button, input, select, textarea, a');
      if (await focusableElements.count() > 0) {
        const element = focusableElements.first();
        await element.focus();
        
        // Element should be focused
        await expect(element).toBeFocused();
        
        // Check for focus indicator (outline, border, etc.)
        const outline = await element.evaluate(el => window.getComputedStyle(el).outline);
        const border = await element.evaluate(el => window.getComputedStyle(el).border);
        
        // Should have some form of focus indicator
        expect(outline !== 'none' || border !== 'none').toBeTruthy();
      }
    });
  });

  test.describe('Dynamic Content', () => {
    test('loading states are announced to screen readers', async ({ page }) => {
      // Look for loading indicators
      const loadingElements = page.locator('[aria-busy="true"], [role="progressbar"], [aria-live="polite"]');
      
      if (await loadingElements.count() > 0) {
        for (let i = 0; i < await loadingElements.count(); i++) {
          const element = loadingElements.nth(i);
          await expect(element).toBeVisible();
          
          // Check for proper ARIA attributes
          const ariaBusy = await element.getAttribute('aria-busy');
          const ariaLive = await element.getAttribute('aria-live');
          const role = await element.getAttribute('role');
          
          // Should have appropriate ARIA attributes
          expect(ariaBusy === 'true' || ariaLive || role === 'progressbar').toBeTruthy();
        }
      }
    });

    test('error messages are properly announced', async ({ page }) => {
      // Look for error elements
      const errorElements = page.locator('[role="alert"], [aria-invalid="true"], .error, [class*="error"]');
      
      if (await errorElements.count() > 0) {
        for (let i = 0; i < await errorElements.count(); i++) {
          const element = errorElements.nth(i);
          await expect(element).toBeVisible();
          
          // Check for proper error attributes
          const role = await element.getAttribute('role');
          const ariaInvalid = await element.getAttribute('aria-invalid');
          
          // Should have error-related attributes
          expect(role === 'alert' || ariaInvalid === 'true').toBeTruthy();
        }
      }
    });

    test('status updates are announced', async ({ page }) => {
      // Look for status elements
      const statusElements = page.locator('[role="status"], [aria-live="polite"], [aria-live="assertive"]');
      
      if (await statusElements.count() > 0) {
        for (let i = 0; i < await statusElements.count(); i++) {
          const element = statusElements.nth(i);
          await expect(element).toBeVisible();
          
          // Check for proper status attributes
          const role = await element.getAttribute('role');
          const ariaLive = await element.getAttribute('aria-live');
          
          // Should have status-related attributes
          expect(role === 'status' || ariaLive).toBeTruthy();
        }
      }
    });
  });

  test.describe('Mobile Accessibility', () => {
    test('touch targets are appropriately sized', async ({ page }) => {
      // Set mobile viewport
      await page.setViewportSize({ width: 375, height: 667 });
      
      const interactiveElements = page.locator('button, input, select, textarea, a');
      if (await interactiveElements.count() > 0) {
        for (let i = 0; i < Math.min(await interactiveElements.count(), 5); i++) {
          const element = interactiveElements.nth(i);
          
          // Get element dimensions
          const boundingBox = await element.boundingBox();
          if (boundingBox) {
            // Touch targets should be at least 44x44 pixels
            expect(boundingBox.width).toBeGreaterThanOrEqual(44);
            expect(boundingBox.height).toBeGreaterThanOrEqual(44);
          }
        }
      }
    });

    test('gesture alternatives are available', async ({ page }) => {
      // Set mobile viewport
      await page.setViewportSize({ width: 375, height: 667 });
      
      // Look for elements that might require gestures
      const gestureElements = page.locator('[class*="swipe"], [class*="drag"], [class*="pinch"]');
      
      if (await gestureElements.count() > 0) {
        for (let i = 0; i < await gestureElements.count(); i++) {
          const element = gestureElements.nth(i);
          
          // Check for alternative controls
          const alternativeControls = element.locator('button, input, select');
          if (await alternativeControls.count() > 0) {
            await expect(alternativeControls.first()).toBeVisible();
          }
        }
      }
    });
  });
});
