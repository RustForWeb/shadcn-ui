import { FullConfig } from '@playwright/test';

async function globalTeardown(config: FullConfig) {
  console.log('🧹 Cleaning up Playwright test environment...');
  
  // Add cleanup logic here if needed
  
  console.log('✅ Global teardown complete');
}

export default globalTeardown;
