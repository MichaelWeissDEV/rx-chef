import { defineConfig } from '@playwright/test';
export default defineConfig({
  testDir: './e2e', workers: 1, timeout: 30000,
  use: { baseURL: 'http://127.0.0.1:18787', viewport: { width: 1600, height: 1000 } },
  webServer: { command: 'node scripts/test-server.mjs', url: 'http://127.0.0.1:18787/api/v1/health', reuseExistingServer: false, timeout: 30000 },
});
