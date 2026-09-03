import { expect, test } from '@playwright/test';

// Small smoke test: default locale per site (SSR) and manual locale switching.
//
// Only always-rendered chrome (hero, footer) is asserted, because the data
// sections depend on the backend API which may not be reachable in CI.

test.describe('locale smoke', () => {
	test('AT landing renders in German by default (SSR)', async ({ page }) => {
		const response = await page.goto('/');
		// The SSR output must already be German (no client-side flash)
		const html = await response.text();
		expect(html).toContain('Demokratie');

		await expect(page.getByRole('heading', { level: 1 }).first()).toContainText('Demokratie');
	});

	test('EU pages render in English by default (SSR)', async ({ page }) => {
		const response = await page.goto('/eu/home');
		const html = await response.text();

		expect(html).toContain('Association for Democracy and Political Transparency');
		expect(html).not.toContain('Verein für Demokratie und politische Transparenz');
	});

	test('switching locale updates rendered text and persists the choice', async ({ page }) => {
		await page.goto('/');
		// Scope to the footer: the landing page also renders the same string
		// in its body (landing.join.org), which would break strict mode.
		await expect(
			page.locator('footer').getByText('Verein für Demokratie und politische Transparenz')
		).toBeVisible();

		// Use the footer switcher (always present in the root layout)
		await page.locator('footer').getByRole('button', { name: 'EN', exact: true }).click();

		await expect(
			page.locator('footer').getByText('Association for Democracy and Political Transparency')
		).toBeVisible();

		const stored = await page.evaluate(() => localStorage.getItem('locale'));
		expect(JSON.parse(stored ?? 'null')).toBe('en');
	});
});
