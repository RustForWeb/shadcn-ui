import { test, expect } from '@playwright/test';

test.describe('Dynamic Loading System - Comprehensive E2E Testing', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the enhanced lazy loading demo
    await page.goto('http://localhost:8082');
    // Wait for the app to be fully loaded
    await page.waitForLoadState('networkidle');
    // Wait for WASM to initialize
    await page.waitForFunction(() => (window as any).wasmBindings !== undefined);
  });

  test.describe('Page Structure & Navigation', () => {
    test('should display main header and title', async ({ page }) => {
      const header = page.locator('h1');
      await expect(header).toBeVisible();
      await expect(header).toContainText('ShadCN UI - Leptos Bundle Optimization Demo');
      
      const subtitle = page.locator('h2');
      await expect(subtitle).toBeVisible();
      await expect(subtitle).toContainText('Dynamic Lazy Loading with Essential Components');
    });

    test('should display all main sections', async ({ page }) => {
      // Essential Components section
      await expect(page.locator('h3:has-text("Essential Components")')).toBeVisible();
      
      // Lazy Loaded Components section
      await expect(page.locator('h3:has-text("Lazy Loaded Components")')).toBeVisible();
      
      // Dynamic WASM Components section
      await expect(page.locator('h3:has-text("Dynamic WASM Components")')).toBeVisible();
      
      // Bundle Analysis panel
      await expect(page.locator('.panel.bundle-analysis')).toBeVisible();
      
      // Dynamic WASM Loader Status panel
      await expect(page.locator('.panel.bundle-status')).toBeVisible();
    });
  });

  test.describe('Bundle Analysis Panel', () => {
    test('should display bundle metrics correctly', async ({ page }) => {
      const bundlePanel = page.locator('.panel.bundle-analysis');
      await expect(bundlePanel).toBeVisible();
      
      // Check for bundle size information
      await expect(bundlePanel.locator('text=Bundle Size:')).toBeVisible();
      await expect(bundlePanel.locator('text=Components:')).toBeVisible();
      await expect(bundlePanel.locator('text=Optimization:')).toBeVisible();
    });

    test('should show accurate bundle statistics', async ({ page }) => {
      const bundlePanel = page.locator('.panel.bundle-analysis');
      
      // Bundle size should be displayed
      const sizeText = await bundlePanel.locator('text=/Bundle Size:.*/').textContent();
      expect(sizeText).toMatch(/Bundle Size:.*\d+\.\d+MB/);
      
      // Component count should be accurate
      const componentText = await bundlePanel.locator('text=/Components:.*/').textContent();
      expect(componentText).toMatch(/Components:.*\d+/);
    });
  });

  test.describe('Dynamic WASM Loader Status Panel', () => {
    test('should display loader status information', async ({ page }) => {
      const loaderPanel = page.locator('.panel.bundle-status');
      await expect(loaderPanel).toBeVisible();
      
      // Check for loader header
      await expect(loaderPanel.locator('.loader-header')).toBeVisible();
      await expect(loaderPanel.locator('text=Dynamic WASM Loader Status')).toBeVisible();
    });

    test('should show initial loading state', async ({ page }) => {
      const loaderPanel = page.locator('.panel.bundle-status');
      
      // Initial state should show 0 loaded components
      await expect(loaderPanel.locator('text=Loaded: 0')).toBeVisible();
      await expect(loaderPanel.locator('text=Total Size: 0KB')).toBeVisible();
    });

    test('should have functional load test button', async ({ page }) => {
      const loadButton = page.locator('.load-btn');
      await expect(loadButton).toBeVisible();
      await expect(loadButton).toHaveText('Load Test Component');
      
      // Button should be clickable
      await expect(loadButton).toBeEnabled();
    });

    test('should toggle details visibility', async ({ page }) => {
      const toggleBtn = page.locator('.toggle-btn');
      const detailsContent = page.locator('.details-content');
      
      // Initially details should be hidden
      await expect(detailsContent).toHaveClass(/hidden/);
      
      // Click toggle button
      await toggleBtn.click();
      
      // Details should now be visible
      await expect(detailsContent).not.toHaveClass(/hidden/);
      
      // Click again to hide
      await toggleBtn.click();
      await expect(detailsContent).toHaveClass(/hidden/);
    });
  });

  test.describe('Essential Components Section', () => {
    test('should display all 5 essential components', async ({ page }) => {
      const essentialSection = page.locator('h3:has-text("Essential Components")').locator('..');
      
      // Should have 5 essential components
      const components = essentialSection.locator('.component-item');
      await expect(components).toHaveCount(5);
      
      // Check component names
      const componentNames = ['Button', 'Input', 'Card', 'Badge', 'Label'];
      for (const name of componentNames) {
        await expect(essentialSection.locator(`text=${name}`)).toBeVisible();
      }
    });

    test('essential components should be immediately visible', async ({ page }) => {
      const essentialSection = page.locator('h3:has-text("Essential Components")').locator('..');
      
      // All essential components should be visible without loading
      const components = essentialSection.locator('.component-item');
      for (let i = 0; i < await components.count(); i++) {
        const component = components.nth(i);
        await expect(component).toBeVisible();
        await expect(component).not.toHaveClass(/loading/);
      }
    });
  });

  test.describe('Lazy Loaded Components Section', () => {
    test('should display all component categories', async ({ page }) => {
      const lazySection = page.locator('h3:has-text("Lazy Loaded Components")').locator('..');
      
      // Check for all 4 categories
      const categories = ['Form & Input', 'Layout & Navigation', 'Overlay & Feedback', 'Data & Media'];
      for (const category of categories) {
        await expect(lazySection.locator(`text=${category}`)).toBeVisible();
      }
    });

    test('should show correct component counts per category', async ({ page }) => {
      const lazySection = page.locator('h3:has-text("Lazy Loaded Components")').locator('..');
      
      // Form & Input: 12 components
      const formSection = lazySection.locator('h4:has-text("Form & Input")').locator('..');
      const formComponents = formSection.locator('.lazy-component-wrapper');
      await expect(formComponents).toHaveCount(12);
      
      // Layout & Navigation: 8 components
      const layoutSection = lazySection.locator('h4:has-text("Layout & Navigation")').locator('..');
      const layoutComponents = layoutSection.locator('.lazy-component-wrapper');
      await expect(layoutComponents).toHaveCount(8);
      
      // Overlay & Feedback: 10 components
      const overlaySection = lazySection.locator('h4:has-text("Overlay & Feedback")').locator('..');
      const overlayComponents = overlaySection.locator('.lazy-component-wrapper');
      await expect(overlayComponents).toHaveCount(10);
      
      // Data & Media: 9 components
      const dataSection = lazySection.locator('h4:has-text("Data & Media")').locator('..');
      const dataComponents = dataSection.locator('.lazy-component-wrapper');
      await expect(dataComponents).toHaveCount(9);
    });

    test('lazy components should start in placeholder state', async ({ page }) => {
      const lazySection = page.locator('h3:has-text("Lazy Loaded Components")').locator('..');
      const lazyComponents = lazySection.locator('.lazy-component-wrapper');
      
      // All lazy components should start in placeholder state
      for (let i = 0; i < await lazyComponents.count(); i++) {
        const component = lazyComponents.nth(i);
        await expect(component.locator('.component-placeholder')).toBeVisible();
        await expect(component.locator('.component-content')).not.toBeVisible();
      }
    });
  });

  test.describe('Dynamic WASM Components Section', () => {
    test('should display all 5 dynamic components', async ({ page }) => {
      const dynamicSection = page.locator('h3:has-text("Dynamic WASM Components")').locator('..');
      
      // Should have 5 dynamic components
      const components = dynamicSection.locator('.dynamic-component-wrapper');
      await expect(components).toHaveCount(5);
      
      // Check component names
      const componentNames = ['Button', 'Input', 'Card', 'Modal', 'Table'];
      for (const name of componentNames) {
        await expect(dynamicSection.locator(`text=${name}`)).toBeVisible();
      }
    });

    test('dynamic components should show correct metadata', async ({ page }) => {
      const dynamicSection = page.locator('h3:has-text("Dynamic WASM Components")').locator('..');
      const components = dynamicSection.locator('.dynamic-component-wrapper');
      
      // Check first component (Button)
      const buttonComponent = components.first();
      await expect(buttonComponent.locator('.component-category')).toContainText('Form & Input');
      await expect(buttonComponent.locator('.component-size')).toContainText('15KB');
      await expect(buttonComponent.locator('.component-description')).toContainText('Interactive button component');
    });

    test('dynamic components should start in placeholder state', async ({ page }) => {
      const dynamicSection = page.locator('h3:has-text("Dynamic WASM Components")').locator('..');
      const components = dynamicSection.locator('.dynamic-component-wrapper');
      
      // All dynamic components should start in placeholder state
      for (let i = 0; i < await components.count(); i++) {
        const component = components.nth(i);
        await expect(component.locator('.component-placeholder')).toBeVisible();
        await expect(component.locator('.component-content')).not.toBeVisible();
      }
    });
  });

  test.describe('Component Loading Functionality', () => {
    test('should load lazy components on demand', async ({ page }) => {
      const lazySection = page.locator('h3:has-text("Lazy Loaded Components")').locator('..');
      const firstComponent = lazySection.locator('.lazy-component-wrapper').first();
      
      // Click load button
      const loadBtn = firstComponent.locator('.load-component-btn');
      await loadBtn.click();
      
      // Should show loading state
      await expect(firstComponent.locator('.component-loading')).toBeVisible();
      
      // Wait for loading to complete
      await expect(firstComponent.locator('.component-success')).toBeVisible({ timeout: 10000 });
      
      // Should show component content
      await expect(firstComponent.locator('.component-content')).toBeVisible();
    });

    test('should load dynamic components on demand', async ({ page }) => {
      const dynamicSection = page.locator('h3:has-text("Dynamic WASM Components")').locator('..');
      const firstComponent = dynamicSection.locator('.dynamic-component-wrapper').first();
      
      // Click load button
      const loadBtn = firstComponent.locator('.load-component-btn');
      await loadBtn.click();
      
      // Should show loading state
      await expect(firstComponent.locator('.component-loading')).toBeVisible();
      
      // Wait for loading to complete
      await expect(firstComponent.locator('.component-success')).toBeVisible({ timeout: 10000 });
      
      // Should show component content
      await expect(firstComponent.locator('.component-content')).toBeVisible();
    });

    test('should handle multiple component loads simultaneously', async ({ page }) => {
      const dynamicSection = page.locator('h3:has-text("Dynamic WASM Components")').locator('..');
      const components = dynamicSection.locator('.dynamic-component-wrapper');
      
      // Load first 3 components simultaneously
      for (let i = 0; i < 3; i++) {
        const component = components.nth(i);
        const loadBtn = component.locator('.load-component-btn');
        await loadBtn.click();
      }
      
      // All should show loading state
      for (let i = 0; i < 3; i++) {
        const component = components.nth(i);
        await expect(component.locator('.component-loading')).toBeVisible();
      }
      
      // Wait for all to complete
      for (let i = 0; i < 3; i++) {
        const component = components.nth(i);
        await expect(component.locator('.component-success')).toBeVisible({ timeout: 15000 });
      }
    });
  });

  test.describe('Search and Filter Functionality', () => {
    test('should display search input and category filter', async ({ page }) => {
      const searchSection = page.locator('.search-filters');
      await expect(searchSection).toBeVisible();
      
      // Search input
      await expect(searchSection.locator('input[placeholder*="search"]')).toBeVisible();
      
      // Category filter
      await expect(searchSection.locator('select')).toBeVisible();
    });

    test('should filter components by category', async ({ page }) => {
      const categorySelect = page.locator('select');
      
      // Select "Form & Input" category
      await categorySelect.selectOption('form-input');
      
      // Should show only form components
      const visibleComponents = page.locator('.lazy-component-wrapper:visible');
      await expect(visibleComponents).toHaveCount(12);
      
      // Should hide other categories
      await expect(page.locator('h4:has-text("Layout & Navigation")')).not.toBeVisible();
    });

    test('should search components by name', async ({ page }) => {
      const searchInput = page.locator('input[placeholder*="search"]');
      
      // Search for "button"
      await searchInput.fill('button');
      
      // Should show only button-related components
      const visibleComponents = page.locator('.lazy-component-wrapper:visible');
      await expect(visibleComponents.count()).toBeLessThan(39); // Less than total
      
      // Should show button components
      await expect(page.locator('text=Button')).toBeVisible();
    });
  });

  test.describe('Favorites System', () => {
    test('should allow marking components as favorites', async ({ page }) => {
      const firstComponent = page.locator('.lazy-component-wrapper').first();
      const favoriteBtn = firstComponent.locator('.favorite-btn');
      
      // Initially not favorited
      await expect(favoriteBtn).not.toHaveClass(/favorited/);
      
      // Click to favorite
      await favoriteBtn.click();
      
      // Should now be favorited
      await expect(favoriteBtn).toHaveClass(/favorited/);
    });

    test('should filter by favorites', async ({ page }) => {
      // Mark a few components as favorites
      const components = page.locator('.lazy-component-wrapper');
      for (let i = 0; i < 3; i++) {
        const component = components.nth(i);
        const favoriteBtn = component.locator('.favorite-btn');
        await favoriteBtn.click();
      }
      
      // Click favorites filter
      const favoritesFilter = page.locator('.favorites-filter');
      await favoritesFilter.click();
      
      // Should show only favorited components
      const visibleComponents = page.locator('.lazy-component-wrapper:visible');
      await expect(visibleComponents).toHaveCount(3);
    });
  });

  test.describe('Error Handling', () => {
    test('should handle component loading errors gracefully', async ({ page }) => {
      // This test would require mocking a failed component load
      // For now, we'll test that error states are properly styled
      const errorComponent = page.locator('.component-error');
      
      // Error states should be properly styled when they occur
      // (This will be empty initially, but ensures error styling is available)
      await expect(errorComponent).toBeAttached();
    });

    test('should provide retry functionality for failed loads', async ({ page }) => {
      // Test retry button functionality
      const retryBtn = page.locator('.retry-btn');
      
      // Retry button should be available (though initially hidden)
      await expect(retryBtn).toBeAttached();
    });
  });

  test.describe('Performance and Responsiveness', () => {
    test('should maintain performance with many components', async ({ page }) => {
      // Load several components to test performance
      const components = page.locator('.lazy-component-wrapper');
      const loadButtons = components.locator('.load-component-btn');
      
      // Load first 5 components
      for (let i = 0; i < 5; i++) {
        const loadBtn = loadButtons.nth(i);
        await loadBtn.click();
      }
      
      // Wait for all to complete
      for (let i = 0; i < 5; i++) {
        const component = components.nth(i);
        await expect(component.locator('.component-success')).toBeVisible({ timeout: 20000 });
      }
      
      // Page should remain responsive
      await expect(page.locator('h1')).toBeVisible();
    });

    test('should be responsive on mobile devices', async ({ page }) => {
      // Set mobile viewport
      await page.setViewportSize({ width: 375, height: 667 });
      
      // All sections should still be visible
      await expect(page.locator('h3:has-text("Essential Components")')).toBeVisible();
      await expect(page.locator('h3:has-text("Lazy Loaded Components")')).toBeVisible();
      await expect(page.locator('h3:has-text("Dynamic WASM Components")')).toBeVisible();
      
      // Components should be properly stacked
      const components = page.locator('.lazy-component-wrapper');
      await expect(components.first()).toBeVisible();
    });
  });

  test.describe('Accessibility', () => {
    test('should have proper ARIA labels and roles', async ({ page }) => {
      // Check for proper button labels
      const loadButtons = page.locator('.load-component-btn');
      for (let i = 0; i < await loadButtons.count(); i++) {
        const button = loadButtons.nth(i);
        await expect(button).toHaveAttribute('aria-label', /load.*component/i);
      }
      
      // Check for proper form labels
      const searchInput = page.locator('input[placeholder*="search"]');
      await expect(searchInput).toHaveAttribute('aria-label', /search/i);
    });

    test('should support keyboard navigation', async ({ page }) => {
      // Tab through interactive elements
      await page.keyboard.press('Tab');
      
      // Should focus on first interactive element
      const focusedElement = page.locator(':focus');
      await expect(focusedElement).toBeVisible();
      
      // Should be able to navigate with arrow keys
      await page.keyboard.press('ArrowDown');
    });
  });

  test.describe('Integration with WASM', () => {
    test('should properly initialize WASM bindings', async ({ page }) => {
      // Check that WASM bindings are available
      const wasmBindings = await page.evaluate(() => (window as any).wasmBindings);
      expect(wasmBindings).toBeDefined();
      
      // Check that the app is properly mounted
      await expect(page.locator('h1')).toBeVisible();
    });

    test('should handle WASM loading states correctly', async ({ page }) => {
      // The app should be fully loaded and interactive
      await expect(page.locator('.load-component-btn').first()).toBeEnabled();
      
      // No loading spinners should be visible initially
      const loadingSpinners = page.locator('.loading-spinner:visible');
      await expect(loadingSpinners).toHaveCount(0);
    });
  });
});
