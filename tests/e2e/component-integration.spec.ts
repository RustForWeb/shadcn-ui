import { test, expect } from '@playwright/test';

test.describe('Leptos Component Integration Testing Suite', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to Leptos example app
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test.describe('Form Component Integration', () => {
    test('complete form workflow with validation', async ({ page }) => {
      // Look for forms
      const forms = page.locator('form');
      
      if (await forms.count() > 0) {
        const form = forms.first();
        
        // Test form inputs
        const inputs = form.locator('input[type="text"], input[type="email"], input[type="password"]');
        if (await inputs.count() > 0) {
          // Fill out form
          for (let i = 0; i < await inputs.count(); i++) {
            const input = inputs.nth(i);
            const type = await input.getAttribute('type');
            
            if (type === 'email') {
              await input.fill('test@example.com');
            } else if (type === 'password') {
              await input.fill('password123');
            } else {
              await input.fill(`Test input ${i + 1}`);
            }
          }
          
          // Test form submission
          const submitButton = form.locator('button[type="submit"], input[type="submit"]');
          if (await submitButton.count() > 0) {
            await expect(submitButton.first()).toBeVisible();
            await expect(submitButton.first()).toBeEnabled();
          }
        }
      }
    });

    test('form validation integration', async ({ page }) => {
      const inputs = page.locator('input[type="text"], input[type="email"]');
      
      if (await inputs.count() > 0) {
        const input = inputs.first();
        
        // Test invalid input
        await input.fill('invalid-email');
        await input.blur();
        
        // Look for validation messages
        const validationMessages = page.locator('[role="alert"], .error, [class*="error"], [aria-invalid="true"]');
        if (await validationMessages.count() > 0) {
          await expect(validationMessages.first()).toBeVisible();
        }
        
        // Test valid input
        await input.clear();
        await input.fill('valid@email.com');
        await input.blur();
        
        // Validation messages should be cleared or hidden
        const remainingErrors = page.locator('[role="alert"], .error, [class*="error"]');
        if (await remainingErrors.count() > 0) {
          // Some validation might remain for other fields
          console.log(`Remaining validation messages: ${await remainingErrors.count()}`);
        }
      }
    });
  });

  test.describe('Navigation Component Integration', () => {
    test('navigation menu with dropdown integration', async ({ page }) => {
      // Test navigation structure
      const navs = page.locator('nav, [role="navigation"]');
      
      if (await navs.count() > 0) {
        const nav = navs.first();
        
        // Test navigation items
        const navItems = nav.locator('a, button, [role="menuitem"]');
        if (await navItems.count() > 0) {
          for (let i = 0; i < Math.min(await navItems.count(), 3); i++) {
            const item = navItems.nth(i);
            await expect(item).toBeVisible();
            
            // Test if item has dropdown
            const hasDropdown = await item.getAttribute('aria-haspopup');
            if (hasDropdown === 'true') {
              await item.click();
              
              // Look for dropdown content
              const dropdown = page.locator('[role="menu"], [data-state="open"]');
              if (await dropdown.count() > 0) {
                await expect(dropdown.first()).toBeVisible();
                
                // Test dropdown items
                const dropdownItems = dropdown.locator('[role="menuitem"], a, button');
                if (await dropdownItems.count() > 0) {
                  await expect(dropdownItems.first()).toBeVisible();
                }
              }
            }
          }
        }
      }
    });

    test('breadcrumb navigation integration', async ({ page }) => {
      const breadcrumbs = page.locator('[class*="breadcrumb"], .breadcrumb, nav[aria-label*="breadcrumb"]');
      
      if (await breadcrumbs.count() > 0) {
        const breadcrumb = breadcrumbs.first();
        
        // Test breadcrumb structure
        const items = breadcrumb.locator('a, span, [class*="breadcrumb-item"]');
        if (await items.count() > 0) {
          // First item should be visible
          await expect(items.first()).toBeVisible();
          
          // Test navigation through breadcrumbs
          const links = breadcrumb.locator('a');
          if (await links.count() > 0) {
            for (let i = 0; i < Math.min(await links.count(), 2); i++) {
              const link = links.nth(i);
              await expect(link).toBeVisible();
              
              // Test link functionality
              const href = await link.getAttribute('href');
              if (href && href !== '#') {
                // This would navigate away, so we'll just verify the link exists
                expect(href).toBeTruthy();
              }
            }
          }
        }
      }
    });
  });

  test.describe('Modal and Dialog Integration', () => {
    test('modal with form integration', async ({ page }) => {
      const modalTriggers = page.locator('button[aria-haspopup="dialog"]');
      
      if (await modalTriggers.count() > 0) {
        const trigger = modalTriggers.first();
        await trigger.click();
        
        // Look for modal
        const modal = page.locator('[role="dialog"]');
        if (await modal.count() > 0) {
          await expect(modal.first()).toBeVisible();
          
          // Test modal content
          const modalContent = modal.first();
          
          // Look for forms in modal
          const modalForms = modalContent.locator('form');
          if (await modalForms.count() > 0) {
            const form = modalForms.first();
            
            // Test form inputs in modal
            const inputs = form.locator('input, select, textarea');
            if (await inputs.count() > 0) {
              for (let i = 0; i < Math.min(await inputs.count(), 2); i++) {
                const input = inputs.nth(i);
                await expect(input).toBeVisible();
                
                // Test input interaction
                if (await input.getAttribute('type') === 'text') {
                  await input.fill('Modal test input');
                  await expect(input).toHaveValue('Modal test input');
                }
              }
            }
          }
          
          // Test modal close
          const closeButton = modalContent.locator('button[aria-label*="close"], button[aria-label*="Close"], [data-state="closed"]');
          if (await closeButton.count() > 0) {
            await closeButton.first().click();
            
            // Modal should be closed
            const isVisible = await modal.first().isVisible();
            expect(isVisible).toBeFalsy();
          }
        }
      }
    });

    test('dropdown with search integration', async ({ page }) => {
      const dropdownTriggers = page.locator('button[aria-haspopup="true"]');
      
      if (await dropdownTriggers.count() > 0) {
        const trigger = dropdownTriggers.first();
        await trigger.click();
        
        // Look for dropdown
        const dropdown = page.locator('[role="menu"], [data-state="open"]');
        if (await dropdown.count() > 0) {
          await expect(dropdown.first()).toBeVisible();
          
          // Test search functionality in dropdown
          const searchInput = dropdown.first().locator('input[type="search"], input[placeholder*="search"], input[placeholder*="Search"]');
          if (await searchInput.count() > 0) {
            const search = searchInput.first();
            await expect(search).toBeVisible();
            
            // Test search input
            await search.fill('test search');
            await expect(search).toHaveValue('test search');
          }
          
          // Test dropdown items
          const dropdownItems = dropdown.first().locator('[role="menuitem"], a, button');
          if (await dropdownItems.count() > 0) {
            await expect(dropdownItems.first()).toBeVisible();
            
            // Test item selection
            await dropdownItems.first().click();
            
            // Dropdown should close after selection
            const isVisible = await dropdown.first().isVisible();
            expect(isVisible).toBeFalsy();
          }
        }
      }
    });
  });

  test.describe('Data Display Integration', () => {
    test('table with pagination integration', async ({ page }) => {
      const tables = page.locator('table, [class*="table"]');
      
      if (await tables.count() > 0) {
        const table = tables.first();
        
        // Test table structure
        const rows = table.locator('tr');
        if (await rows.count() > 0) {
          await expect(rows.first()).toBeVisible();
          
          // Look for pagination
          const pagination = page.locator('[class*="pagination"], .pagination, nav[aria-label*="pagination"]');
          if (await pagination.count() > 0) {
            await expect(pagination.first()).toBeVisible();
            
            // Test pagination controls
            const paginationControls = pagination.first().locator('button, a, [class*="page"]');
            if (await paginationControls.count() > 0) {
              await expect(paginationControls.first()).toBeVisible();
              
              // Test pagination navigation
              const nextButton = pagination.first().locator('button[aria-label*="next"], button[aria-label*="Next"]');
              if (await nextButton.count() > 0) {
                await expect(nextButton.first()).toBeVisible();
                await expect(nextButton.first()).toBeEnabled();
              }
            }
          }
        }
      }
    });

    test('calendar with date picker integration', async ({ page }) => {
      const datePickers = page.locator('[class*="date-picker"], [class*="calendar"]');
      
      if (await datePickers.count() > 0) {
        const datePicker = datePickers.first();
        
        // Test date picker trigger
        const trigger = datePicker.locator('button, input[type="text"]');
        if (await trigger.count() > 0) {
          await trigger.first().click();
          
          // Look for calendar
          const calendar = page.locator('[class*="calendar"], [role="grid"]');
          if (await calendar.count() > 0) {
            await expect(calendar.first()).toBeVisible();
            
            // Test calendar navigation
            const prevButton = calendar.first().locator('button[aria-label*="previous"], button[aria-label*="Previous"]');
            const nextButton = calendar.first().locator('button[aria-label*="next"], button[aria-label*="Next"]');
            
            if (await prevButton.count() > 0) {
              await expect(prevButton.first()).toBeVisible();
            }
            
            if (await nextButton.count() > 0) {
              await expect(nextButton.first()).toBeVisible();
            }
            
            // Test date selection
            const dateCells = calendar.first().locator('[class*="day"], [class*="date"], td[role="gridcell"]');
            if (await dateCells.count() > 0) {
              // Find a selectable date (not disabled)
              for (let i = 0; i < await dateCells.count(); i++) {
                const cell = dateCells.nth(i);
                const isDisabled = await cell.getAttribute('aria-disabled') === 'true' || 
                                 await cell.getAttribute('disabled') !== null;
                
                if (!isDisabled) {
                  await expect(cell).toBeVisible();
                  await cell.click();
                  break;
                }
              }
            }
          }
        }
      }
    });
  });

  test.describe('Interactive Component Integration', () => {
    test('accordion with content integration', async ({ page }) => {
      const accordions = page.locator('[class*="accordion"], [role="region"]');
      
      if (await accordions.count() > 0) {
        const accordion = accordions.first();
        
        // Test accordion triggers
        const triggers = accordion.locator('button[aria-expanded], [data-state="closed"]');
        if (await triggers.count() > 0) {
          const trigger = triggers.first();
          await expect(trigger).toBeVisible();
          
          // Test accordion expansion
          await trigger.click();
          
          // Check if content is expanded
          const isExpanded = await trigger.getAttribute('aria-expanded') === 'true' ||
                           await trigger.getAttribute('data-state') === 'open';
          expect(isExpanded).toBeTruthy();
          
          // Test accordion content
          const content = accordion.locator('[data-state="open"], [aria-hidden="false"]');
          if (await content.count() > 0) {
            await expect(content.first()).toBeVisible();
            
            // Test content interaction
            const contentButtons = content.first().locator('button, input, select');
            if (await contentButtons.count() > 0) {
              await expect(contentButtons.first()).toBeVisible();
            }
          }
        }
      }
    });

    test('tabs with content integration', async ({ page }) => {
      const tabContainers = page.locator('[role="tablist"], [class*="tabs"]');
      
      if (await tabContainers.count() > 0) {
        const tabContainer = tabContainers.first();
        
        // Test tab triggers
        const tabs = tabContainer.locator('[role="tab"], button[aria-selected]');
        if (await tabs.count() > 0) {
          // Test first tab
          const firstTab = tabs.first();
          await expect(firstTab).toBeVisible();
          
          // Test tab selection
          await firstTab.click();
          
          // Check if tab is selected
          const isSelected = await firstTab.getAttribute('aria-selected') === 'true';
          expect(isSelected).toBeTruthy();
          
          // Test tab content
          const tabPanels = page.locator('[role="tabpanel"], [aria-labelledby]');
          if (await tabPanels.count() > 0) {
            await expect(tabPanels.first()).toBeVisible();
            
            // Test content interaction
            const contentElements = tabPanels.first().locator('button, input, select, textarea');
            if (await contentElements.count() > 0) {
              await expect(contentElements.first()).toBeVisible();
            }
          }
        }
      }
    });
  });

  test.describe('State Management Integration', () => {
    test('component state persistence across interactions', async ({ page }) => {
      // Test form state persistence
      const inputs = page.locator('input[type="text"], input[type="email"]');
      
      if (await inputs.count() > 0) {
        const input = inputs.first();
        
        // Fill input
        await input.fill('Persistent test value');
        await expect(input).toHaveValue('Persistent test value');
        
        // Interact with other components
        const buttons = page.locator('button');
        if (await buttons.count() > 0) {
          await buttons.first().click();
          await page.waitForTimeout(100);
        }
        
        // Check if input value is maintained
        await expect(input).toHaveValue('Persistent test value');
      }
    });

    test('component communication and updates', async ({ page }) => {
      // Test if components can communicate state changes
      const formInputs = page.locator('input[type="text"], input[type="email"]');
      const buttons = page.locator('button');
      
      if (await formInputs.count() > 0 && await buttons.count() > 0) {
        const input = formInputs.first();
        const button = buttons.first();
        
        // Fill input and click button
        await input.fill('Communication test');
        await button.click();
        
        // Wait for potential state updates
        await page.waitForTimeout(100);
        
        // Check if input value is maintained
        await expect(input).toHaveValue('Communication test');
      }
    });
  });

  test.describe('Responsive Integration', () => {
    test('components adapt to different viewports', async ({ page }) => {
      const viewports = [
        { width: 375, height: 667, name: 'Mobile' },
        { width: 768, height: 1024, name: 'Tablet' },
        { width: 1280, height: 720, name: 'Desktop' }
      ];
      
      for (const viewport of viewports) {
        await page.setViewportSize(viewport);
        
        // Test that main components are still accessible
        const mainComponents = page.locator('button, input, select, textarea, nav, form');
        if (await mainComponents.count() > 0) {
          await expect(mainComponents.first()).toBeVisible();
        }
        
        // Test navigation adaptation
        const navs = page.locator('nav, [role="navigation"]');
        if (await navs.count() > 0) {
          await expect(navs.first()).toBeVisible();
          
          // On mobile, look for mobile menu patterns
          if (viewport.width <= 768) {
            const mobileMenu = page.locator('[class*="mobile"], [class*="hamburger"], [aria-label*="menu"]');
            if (await mobileMenu.count() > 0) {
              await expect(mobileMenu.first()).toBeVisible();
            }
          }
        }
      }
    });

    test('touch interactions work on mobile', async ({ page }) => {
      // Set mobile viewport
      await page.setViewportSize({ width: 375, height: 667 });
      
      // Test touch-friendly interactions
      const touchElements = page.locator('button, input, select, textarea, a');
      if (await touchElements.count() > 0) {
        for (let i = 0; i < Math.min(await touchElements.count(), 3); i++) {
          const element = touchElements.nth(i);
          
          // Check element size for touch
          const boundingBox = await element.boundingBox();
          if (boundingBox) {
            // Touch targets should be appropriately sized
            expect(boundingBox.width).toBeGreaterThanOrEqual(44);
            expect(boundingBox.height).toBeGreaterThanOrEqual(44);
          }
          
          // Test element interaction
          await expect(element).toBeVisible();
        }
      }
    });
  });
});
