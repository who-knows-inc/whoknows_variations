import { test, expect } from '@playwright/test';

test('application serves the main page and has correct title', async ({ page }) => {
  await page.goto('/');
  await expect(page).toHaveTitle(/¿Who Knows?/);
});
