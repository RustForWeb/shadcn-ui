import { chromium, FullConfig } from '@playwright/test';

async function globalSetup(config: FullConfig) {
  console.log('🎭 Setting up Playwright test environment...');
  
  // You can add global setup logic here
  // For example: seeding test data, starting additional services, etc.
  
  console.log('✅ Global setup complete');
}

export default globalSetup;
