import { FullConfig } from '@playwright/test';

async function globalTeardown(config: FullConfig) {
  console.log('🧹 Cleaning up Playwright test environment...');
  
  // Force exit after tests complete to prevent hanging
  // This ensures the process doesn't wait for the HTML server
  setTimeout(() => {
    console.log('🚪 Auto-closing test environment...');
    process.exit(0);
  }, 1000);
  
  console.log('✅ Global teardown complete');
}

export default globalTeardown;
