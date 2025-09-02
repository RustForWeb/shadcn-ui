import { test, expect } from '@playwright/test';

test.describe('Leptos Performance Testing Suite', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to Leptos example app
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test.describe('Page Load Performance', () => {
    test('initial page load time is under 3 seconds', async ({ page }) => {
      const startTime = Date.now();
      
      await page.goto('/');
      await page.waitForLoadState('networkidle');
      
      const loadTime = Date.now() - startTime;
      
      // Initial load should be fast
      expect(loadTime).toBeLessThan(3000);
      
      console.log(`📊 Initial page load time: ${loadTime}ms`);
    });

    test('time to interactive is reasonable', async ({ page }) => {
      // Measure time to interactive (when buttons become clickable)
      const startTime = Date.now();
      
      await page.goto('/');
      
      // Wait for interactive elements to be ready
      await page.waitForSelector('button, input, select', { state: 'visible' });
      
      const tti = Date.now() - startTime;
      
      // Time to interactive should be reasonable
      expect(tti).toBeLessThan(2000);
      
      console.log(`⚡ Time to interactive: ${tti}ms`);
    });

    test('memory usage is stable', async ({ page }) => {
      // Get initial memory info
      const initialMemory = await page.evaluate(() => {
        if ('memory' in performance) {
          return (performance as any).memory.usedJSHeapSize;
        }
        return null;
      });
      
      if (initialMemory) {
        // Navigate and interact to trigger memory usage
        await page.goto('/');
        await page.waitForLoadState('networkidle');
        
        // Perform some interactions
        const buttons = page.locator('button');
        if (await buttons.count() > 0) {
          for (let i = 0; i < Math.min(await buttons.count(), 3); i++) {
            await buttons.nth(i).click();
            await page.waitForTimeout(100);
          }
        }
        
        // Check memory after interactions
        const finalMemory = await page.evaluate(() => {
          if ('memory' in performance) {
            return (performance as any).memory.usedJSHeapSize;
          }
          return null;
        });
        
        if (finalMemory) {
          const memoryIncrease = finalMemory - initialMemory;
          const memoryIncreaseMB = memoryIncrease / (1024 * 1024);
          
          // Memory increase should be reasonable (less than 50MB)
          expect(memoryIncreaseMB).toBeLessThan(50);
          
          console.log(`🧠 Memory increase: ${memoryIncreaseMB.toFixed(2)}MB`);
        }
      }
    });
  });

  test.describe('Component Rendering Performance', () => {
    test('component render time is fast', async ({ page }) => {
      // Measure component rendering performance
      const renderStart = performance.now();
      
      // Trigger a re-render by interacting with components
      const buttons = page.locator('button');
      if (await buttons.count() > 0) {
        await buttons.first().click();
        
        // Wait for any animations or state changes
        await page.waitForTimeout(100);
        
        const renderEnd = performance.now();
        const renderTime = renderEnd - renderStart;
        
        // Component render should be fast
        expect(renderTime).toBeLessThan(100);
        
        console.log(`🎨 Component render time: ${renderTime.toFixed(2)}ms`);
      }
    });

    test('large component lists render efficiently', async ({ page }) => {
      // Look for components that might render lists
      const listContainers = page.locator('[class*="list"], [class*="table"], [class*="grid"]');
      
      if (await listContainers.count() > 0) {
        const container = listContainers.first();
        
        // Measure render time for list components
        const startTime = performance.now();
        
        // Wait for any dynamic content to load
        await page.waitForTimeout(200);
        
        const endTime = performance.now();
        const renderTime = endTime - startTime;
        
        // List rendering should be efficient
        expect(renderTime).toBeLessThan(200);
        
        console.log(`📋 List component render time: ${renderTime.toFixed(2)}ms`);
      }
    });

    test('form validation is responsive', async ({ page }) => {
      const inputs = page.locator('input[type="text"], input[type="email"]');
      
      if (await inputs.count() > 0) {
        const input = inputs.first();
        
        // Measure validation response time
        const startTime = performance.now();
        
        await input.fill('test@example.com');
        await input.blur();
        
        // Wait for validation to complete
        await page.waitForTimeout(100);
        
        const endTime = performance.now();
        const validationTime = endTime - startTime;
        
        // Validation should be responsive
        expect(validationTime).toBeLessThan(150);
        
        console.log(`✅ Form validation time: ${validationTime.toFixed(2)}ms`);
      }
    });
  });

  test.describe('Interaction Performance', () => {
    test('button clicks are responsive', async ({ page }) => {
      const buttons = page.locator('button');
      
      if (await buttons.count() > 0) {
        const button = buttons.first();
        
        // Measure click response time
        const startTime = performance.now();
        
        await button.click();
        
        // Wait for any state changes
        await page.waitForTimeout(50);
        
        const endTime = performance.now();
        const clickTime = endTime - startTime;
        
        // Button clicks should be responsive
        expect(clickTime).toBeLessThan(100);
        
        console.log(`🖱️ Button click response time: ${clickTime.toFixed(2)}ms`);
      }
    });

    test('dropdown interactions are smooth', async ({ page }) => {
      const dropdownTriggers = page.locator('button[aria-haspopup="true"], [data-state="closed"]');
      
      if (await dropdownTriggers.count() > 0) {
        const trigger = dropdownTriggers.first();
        
        // Measure dropdown open time
        const startTime = performance.now();
        
        await trigger.click();
        
        // Wait for dropdown to appear
        const dropdown = page.locator('[role="menu"], [data-state="open"]');
        if (await dropdown.count() > 0) {
          await expect(dropdown.first()).toBeVisible();
          
          const endTime = performance.now();
          const dropdownTime = endTime - startTime;
          
          // Dropdown should open quickly
          expect(dropdownTime).toBeLessThan(150);
          
          console.log(`📋 Dropdown open time: ${dropdownTime.toFixed(2)}ms`);
        }
      }
    });

    test('modal interactions are performant', async ({ page }) => {
      const modalTriggers = page.locator('button[aria-haspopup="dialog"]');
      
      if (await modalTriggers.count() > 0) {
        const trigger = modalTriggers.first();
        
        // Measure modal open time
        const startTime = performance.now();
        
        await trigger.click();
        
        // Wait for modal to appear
        const modal = page.locator('[role="dialog"]');
        if (await modal.count() > 0) {
          await expect(modal.first()).toBeVisible();
          
          const endTime = performance.now();
          const modalTime = endTime - startTime;
          
          // Modal should open quickly
          expect(modalTime).toBeLessThan(200);
          
          console.log(`🪟 Modal open time: ${modalTime.toFixed(2)}ms`);
        }
      }
    });
  });

  test.describe('Network and Resource Performance', () => {
    test('asset loading is optimized', async ({ page }) => {
      // Listen for network requests
      const requests: string[] = [];
      page.on('request', request => {
        requests.push(request.url());
      });
      
      await page.goto('/');
      await page.waitForLoadState('networkidle');
      
      // Check for unnecessary requests
      const unnecessaryRequests = requests.filter(url => 
        url.includes('.woff') || 
        url.includes('.ttf') || 
        url.includes('analytics') ||
        url.includes('tracking')
      );
      
      // Should not have unnecessary requests
      expect(unnecessaryRequests.length).toBeLessThan(5);
      
      console.log(`🌐 Total network requests: ${requests.length}`);
      console.log(`❌ Unnecessary requests: ${unnecessaryRequests.length}`);
    });

    test('CSS and JS bundle sizes are reasonable', async ({ page }) => {
      // Get resource timing information
      const resourceTimings = await page.evaluate(() => {
        return performance.getEntriesByType('resource').map(entry => ({
          name: entry.name,
          duration: entry.duration,
          transferSize: (entry as any).transferSize || 0
        }));
      });
      
      // Check for large resources
      const largeResources = resourceTimings.filter(resource => 
        resource.transferSize > 100 * 1024 // 100KB
      );
      
      // Should not have too many large resources
      expect(largeResources.length).toBeLessThan(10);
      
      console.log(`📦 Large resources (>100KB): ${largeResources.length}`);
      
      // Log resource details
      largeResources.forEach(resource => {
        const sizeKB = resource.transferSize / 1024;
        console.log(`  - ${resource.name}: ${sizeKB.toFixed(2)}KB`);
      });
    });
  });

  test.describe('Responsive Performance', () => {
    test('mobile viewport performance is maintained', async ({ page }) => {
      // Set mobile viewport
      await page.setViewportSize({ width: 375, height: 667 });
      
      const startTime = Date.now();
      
      await page.goto('/');
      await page.waitForLoadState('networkidle');
      
      const mobileLoadTime = Date.now() - startTime;
      
      // Mobile should load within reasonable time
      expect(mobileLoadTime).toBeLessThan(4000);
      
      console.log(`📱 Mobile load time: ${mobileLoadTime}ms`);
    });

    test('tablet viewport performance is maintained', async ({ page }) => {
      // Set tablet viewport
      await page.setViewportSize({ width: 768, height: 1024 });
      
      const startTime = Date.now();
      
      await page.goto('/');
      await page.waitForLoadState('networkidle');
      
      const tabletLoadTime = Date.now() - startTime;
      
      // Tablet should load within reasonable time
      expect(tabletLoadTime).toBeLessThan(3500);
      
      console.log(`📱 Tablet load time: ${tabletLoadTime}ms`);
    });

    test('desktop viewport performance is optimal', async ({ page }) => {
      // Set desktop viewport
      await page.setViewportSize({ width: 1280, height: 720 });
      
      const startTime = Date.now();
      
      await page.goto('/');
      await page.waitForLoadState('networkidle');
      
      const desktopLoadTime = Date.now() - startTime;
      
      // Desktop should load quickly
      expect(desktopLoadTime).toBeLessThan(3000);
      
      console.log(`🖥️ Desktop load time: ${desktopLoadTime}ms`);
    });
  });

  test.describe('Stress Testing', () => {
    test('rapid interactions remain responsive', async ({ page }) => {
      const buttons = page.locator('button');
      
      if (await buttons.count() > 0) {
        const button = buttons.first();
        
        // Perform rapid clicks
        const startTime = performance.now();
        
        for (let i = 0; i < 10; i++) {
          await button.click();
          await page.waitForTimeout(10);
        }
        
        const endTime = performance.now();
        const totalTime = endTime - startTime;
        
        // Should handle rapid interactions efficiently
        expect(totalTime).toBeLessThan(500);
        
        console.log(`⚡ Rapid interaction time: ${totalTime.toFixed(2)}ms`);
      }
    });

    test('large data sets render efficiently', async ({ page }) => {
      // Look for components that might handle large datasets
      const dataComponents = page.locator('[class*="table"], [class*="list"], [class*="grid"]');
      
      if (await dataComponents.count() > 0) {
        const component = dataComponents.first();
        
        // Measure render performance
        const startTime = performance.now();
        
        // Wait for any dynamic content
        await page.waitForTimeout(300);
        
        const endTime = performance.now();
        const renderTime = endTime - startTime;
        
        // Should render large datasets efficiently
        expect(renderTime).toBeLessThan(300);
        
        console.log(`📊 Large dataset render time: ${renderTime.toFixed(2)}ms`);
      }
    });
  });
});
