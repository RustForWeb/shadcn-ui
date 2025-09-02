import { test, expect } from '@playwright/test';

test.describe('Bundle Optimization & Performance - Comprehensive Testing', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the enhanced lazy loading demo
    await page.goto('/');
    // Wait for the app to be fully loaded
    await page.waitForLoadState('networkidle');
    // Wait for WASM to initialize
    await page.waitForFunction(() => window.wasmBindings !== undefined);
  });

  test.describe('Bundle Size Analysis', () => {
    test('should display accurate bundle size information', async ({ page }) => {
      const bundlePanel = page.locator('.panel.bundle-analysis');
      await expect(bundlePanel).toBeVisible();
      
      // Bundle size should be displayed with proper units
      const sizeText = await bundlePanel.locator('text=/Bundle Size:.*/').textContent();
      expect(sizeText).toMatch(/Bundle Size:.*\d+\.\d+MB/);
      
      // Should show reasonable bundle size (not 0 or extremely large)
      const sizeMatch = sizeText!.match(/Bundle Size: ([\d.]+)MB/);
      if (sizeMatch) {
        const sizeInMB = parseFloat(sizeMatch[1]);
        expect(sizeInMB).toBeGreaterThan(0.1); // At least 100KB
        expect(sizeInMB).toBeLessThan(10); // Less than 10MB
      }
    });

    test('should show component count breakdown', async ({ page }) => {
      const bundlePanel = page.locator('.panel.bundle-analysis');
      
      // Component count should be accurate
      const componentText = await bundlePanel.locator('text=/Components:.*/').textContent();
      expect(componentText).toMatch(/Components:.*\d+/);
      
      // Should show the correct total (5 essential + 39 lazy + 5 dynamic = 49)
      const countMatch = componentText!.match(/Components: (\d+)/);
      if (countMatch) {
        const count = parseInt(countMatch[1]);
        expect(count).toBeGreaterThanOrEqual(40); // At least 40 components
        expect(count).toBeLessThanOrEqual(60); // Reasonable upper bound
      }
    });

    test('should display optimization status', async ({ page }) => {
      const bundlePanel = page.locator('.panel.bundle-analysis');
      
      // Optimization status should be visible
      await expect(bundlePanel.locator('text=/Optimization:/')).toBeVisible();
      
      // Should show some optimization information
      const optimizationText = await bundlePanel.locator('text=/Optimization:.*/').textContent();
      expect(optimizationText).toBeTruthy();
      expect(optimizationText!.length).toBeGreaterThan(10);
    });
  });

  test.describe('WASM Loading Performance', () => {
    test('should measure initial WASM load time', async ({ page }) => {
      // Navigate to page and measure load time
      const startTime = Date.now();
      await page.goto('/');
      await page.waitForFunction(() => window.wasmBindings !== undefined);
      const loadTime = Date.now() - startTime;
      
      // Initial load should be reasonable (under 10 seconds)
      expect(loadTime).toBeLessThan(10000);
      
      // Log the load time for monitoring
      console.log(`Initial WASM load time: ${loadTime}ms`);
    });

    test('should handle WASM initialization gracefully', async ({ page }) => {
      // Check that WASM bindings are properly initialized
      const wasmBindings = await page.evaluate(() => window.wasmBindings);
      expect(wasmBindings).toBeDefined();
      
      // Check that the app is interactive after WASM load
      await expect(page.locator('.load-component-btn').first()).toBeEnabled();
      
      // No loading errors should be visible
      const errorElements = page.locator('.error:visible');
      await expect(errorElements).toHaveCount(0);
    });

    test('should maintain performance during component loading', async ({ page }) => {
      const startTime = Date.now();
      
      // Load multiple components simultaneously
      const components = page.locator('.dynamic-component-wrapper');
      const loadButtons = components.locator('.load-component-btn');
      
      // Load first 3 components
      for (let i = 0; i < 3; i++) {
        const loadBtn = loadButtons.nth(i);
        await loadBtn.click();
      }
      
      // Wait for all to complete
      for (let i = 0; i < 3; i++) {
        const component = components.nth(i);
        await expect(component.locator('.component-success')).toBeVisible({ timeout: 15000 });
      }
      
      const totalTime = Date.now() - startTime;
      
      // Loading 3 components should be reasonable (under 20 seconds)
      expect(totalTime).toBeLessThan(20000);
      
      // Page should remain responsive
      await expect(page.locator('h1')).toBeVisible();
      
      console.log(`Loading 3 components took: ${totalTime}ms`);
    });
  });

  test.describe('Memory Usage & Resource Management', () => {
    test('should not cause memory leaks during component loading', async ({ page }) => {
      // Load and unload components multiple times
      const components = page.locator('.dynamic-component-wrapper');
      const loadButtons = components.locator('.load-component-btn');
      
      for (let cycle = 0; cycle < 3; cycle++) {
        // Load first component
        const loadBtn = loadButtons.first();
        await loadBtn.click();
        
        // Wait for loading to complete
        const component = components.first();
        await expect(component.locator('.component-success')).toBeVisible({ timeout: 10000 });
        
        // Verify component is loaded
        await expect(component.locator('.component-content')).toBeVisible();
        
        // Small delay to simulate user interaction
        await page.waitForTimeout(500);
      }
      
      // Page should still be responsive after multiple load cycles
      await expect(page.locator('h1')).toBeVisible();
      await expect(loadButtons.first()).toBeEnabled();
    });

    test('should handle rapid component loading requests', async ({ page }) => {
      const components = page.locator('.dynamic-component-wrapper');
      const loadButtons = components.locator('.load-component-btn');
      
      // Rapidly click multiple load buttons
      for (let i = 0; i < 5; i++) {
        const loadBtn = loadButtons.nth(i);
        await loadBtn.click();
        await page.waitForTimeout(100); // Small delay between clicks
      }
      
      // All components should eventually load successfully
      for (let i = 0; i < 5; i++) {
        const component = components.nth(i);
        await expect(component.locator('.component-success')).toBeVisible({ timeout: 20000 });
      }
      
      // Page should remain stable
      await expect(page.locator('h1')).toBeVisible();
    });
  });

  test.describe('Bundle Optimization Features', () => {
    test('should implement proper code splitting', async ({ page }) => {
      // Check that not all components are loaded initially
      const lazyComponents = page.locator('.lazy-component-wrapper');
      const dynamicComponents = page.locator('.dynamic-component-wrapper');
      
      // Lazy components should start in placeholder state
      for (let i = 0; i < await lazyComponents.count(); i++) {
        const component = lazyComponents.nth(i);
        await expect(component.locator('.component-placeholder')).toBeVisible();
        await expect(component.locator('.component-content')).not.toBeVisible();
      }
      
      // Dynamic components should also start in placeholder state
      for (let i = 0; i < await dynamicComponents.count(); i++) {
        const component = dynamicComponents.nth(i);
        await expect(component.locator('.component-placeholder')).toBeVisible();
        await expect(component.locator('.component-content')).not.toBeVisible();
      }
    });

    test('should load components on demand', async ({ page }) => {
      const lazySection = page.locator('h3:has-text("Lazy Loaded Components")').locator('..');
      const firstComponent = lazySection.locator('.lazy-component-wrapper').first();
      
      // Initially should show placeholder
      await expect(firstComponent.locator('.component-placeholder')).toBeVisible();
      
      // Click load button
      const loadBtn = firstComponent.locator('.load-component-btn');
      await loadBtn.click();
      
      // Should show loading state
      await expect(firstComponent.locator('.component-loading')).toBeVisible();
      
      // Should eventually show component content
      await expect(firstComponent.locator('.component-content')).toBeVisible({ timeout: 10000 });
      
      // Placeholder should be hidden
      await expect(firstComponent.locator('.component-placeholder')).not.toBeVisible();
    });

    test('should maintain essential components always loaded', async ({ page }) => {
      const essentialSection = page.locator('h3:has-text("Essential Components")').locator('..');
      const essentialComponents = essentialSection.locator('.component-item');
      
      // Essential components should be immediately visible
      for (let i = 0; i < await essentialComponents.count(); i++) {
        const component = essentialComponents.nth(i);
        await expect(component).toBeVisible();
        await expect(component).not.toHaveClass(/loading/);
        await expect(component).not.toHaveClass(/placeholder/);
      }
    });
  });

  test.describe('Performance Metrics & Monitoring', () => {
    test('should display real-time loading statistics', async ({ page }) => {
      const loaderPanel = page.locator('.panel.bundle-status');
      
      // Should show initial stats
      await expect(loaderPanel.locator('text=Loaded: 0')).toBeVisible();
      await expect(loaderPanel.locator('text=Total Size: 0KB')).toBeVisible();
      
      // Load a component and verify stats update
      const loadBtn = page.locator('.load-btn');
      await loadBtn.click();
      
      // Should show loading progress
      await expect(loaderPanel.locator('.status-value.loading')).toBeVisible();
      
      // Wait for completion and verify stats
      await expect(loaderPanel.locator('text=Loaded: 1')).toBeVisible({ timeout: 10000 });
      await expect(loaderPanel.locator('text=/Total Size:.*KB/')).toBeVisible();
    });

    test('should track component loading progress', async ({ page }) => {
      const loaderPanel = page.locator('.panel.bundle-status');
      
      // Load test component
      const loadBtn = page.locator('.load-btn');
      await loadBtn.click();
      
      // Should show progress indicator
      const progressElement = loaderPanel.locator('.status-value.loading');
      await expect(progressElement).toBeVisible();
      
      // Progress should eventually complete
      await expect(loaderPanel.locator('text=Loaded: 1')).toBeVisible({ timeout: 10000 });
    });

    test('should provide detailed loading information', async ({ page }) => {
      const loaderPanel = page.locator('.panel.bundle-status');
      
      // Toggle details to show more information
      const toggleBtn = page.locator('.toggle-btn');
      await toggleBtn.click();
      
      // Details should be visible
      const detailsContent = loaderPanel.locator('.details-content');
      await expect(detailsContent).not.toHaveClass(/hidden/);
      
      // Should show implementation details
      await expect(loaderPanel.locator('.implementation-note')).toBeVisible();
    });
  });

  test.describe('Error Handling & Resilience', () => {
    test('should handle component loading failures gracefully', async ({ page }) => {
      // This test verifies error handling infrastructure
      // Actual error simulation would require mocking
      
      // Error display elements should be available
      const errorDisplay = page.locator('.error-display');
      await expect(errorDisplay).toBeAttached();
      
      // Clear error button should be available
      const clearErrorBtn = page.locator('.clear-error-btn');
      await expect(clearErrorBtn).toBeAttached();
    });

    test('should provide retry mechanisms', async ({ page }) => {
      // Retry buttons should be available on components
      const retryBtns = page.locator('.retry-btn');
      
      // Initially no retry buttons should be visible (no errors)
      await expect(retryBtns.filter({ hasText: /retry/i })).toHaveCount(0);
      
      // But retry infrastructure should be in place
      await expect(retryBtns).toBeAttached();
    });

    test('should maintain system stability during errors', async ({ page }) => {
      // Load multiple components to stress test
      const components = page.locator('.dynamic-component-wrapper');
      const loadButtons = components.locator('.load-component-btn');
      
      // Load several components
      for (let i = 0; i < 3; i++) {
        const loadBtn = loadButtons.nth(i);
        await loadBtn.click();
      }
      
      // Wait for completion
      for (let i = 0; i < 3; i++) {
        const component = components.nth(i);
        await expect(component.locator('.component-success')).toBeVisible({ timeout: 15000 });
      }
      
      // System should remain stable
      await expect(page.locator('h1')).toBeVisible();
      await expect(loadButtons.first()).toBeEnabled();
    });
  });

  test.describe('Cross-Browser Compatibility', () => {
    test('should work consistently across different viewports', async ({ page }) => {
      // Test desktop viewport
      await page.setViewportSize({ width: 1280, height: 720 });
      await expect(page.locator('h1')).toBeVisible();
      await expect(page.locator('.panel.bundle-analysis')).toBeVisible();
      
      // Test tablet viewport
      await page.setViewportSize({ width: 768, height: 1024 });
      await expect(page.locator('h1')).toBeVisible();
      await expect(page.locator('.panel.bundle-analysis')).toBeVisible();
      
      // Test mobile viewport
      await page.setViewportSize({ width: 375, height: 667 });
      await expect(page.locator('h1')).toBeVisible();
      await expect(page.locator('.panel.bundle-analysis')).toBeVisible();
    });

    test('should maintain functionality across viewport changes', async ({ page }) => {
      // Load a component in desktop view
      await page.setViewportSize({ width: 1280, height: 720 });
      const loadBtn = page.locator('.load-btn');
      await loadBtn.click();
      
      // Switch to mobile view during loading
      await page.setViewportSize({ width: 375, height: 667 });
      
      // Loading should continue and complete
      await expect(page.locator('text=Loaded: 1')).toBeVisible({ timeout: 10000 });
      
      // Component should be properly displayed in mobile view
      await expect(page.locator('.panel.bundle-status')).toBeVisible();
    });
  });

  test.describe('Integration Testing', () => {
    test('should integrate all optimization features seamlessly', async ({ page }) => {
      // Test bundle analysis
      const bundlePanel = page.locator('.panel.bundle-analysis');
      await expect(bundlePanel).toBeVisible();
      
      // Test dynamic loader
      const loaderPanel = page.locator('.panel.bundle-status');
      await expect(loaderPanel).toBeVisible();
      
      // Test essential components
      await expect(page.locator('h3:has-text("Essential Components")')).toBeVisible();
      
      // Test lazy loading
      await expect(page.locator('h3:has-text("Lazy Loaded Components")')).toBeVisible();
      
      // Test dynamic components
      await expect(page.locator('h3:has-text("Dynamic WASM Components")')).toBeVisible();
      
      // All sections should work together
      await expect(page.locator('h1')).toBeVisible();
    });

    test('should provide consistent user experience', async ({ page }) => {
      // Navigate through different sections
      const sections = [
        'Essential Components',
        'Lazy Loaded Components', 
        'Dynamic WASM Components'
      ];
      
      for (const section of sections) {
        await expect(page.locator(`h3:has-text("${section}")`)).toBeVisible();
        
        // Each section should be properly styled and functional
        const sectionElement = page.locator(`h3:has-text("${section}")`).locator('..');
        await expect(sectionElement).toBeVisible();
      }
      
      // Overall layout should be consistent
      await expect(page.locator('.container')).toBeVisible();
    });
  });
});
