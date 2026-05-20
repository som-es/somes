import AxeBuilder from '@axe-core/playwright';
import { expect, test } from '@playwright/test';

test.describe('accessibility', () => {
	test('landing page has no automatically detectable WCAG A/AA violations', async ({ page }) => {
		await page.goto('/');
		await expect(page.locator('h1')).toBeVisible();

		const results = await new AxeBuilder({ page })
			.withTags(['wcag2a', 'wcag2aa', 'wcag21a', 'wcag21aa'])
			.analyze();

		expect(results.violations).toEqual([]);
	});

	test('document language and skip link are present', async ({ page }) => {
		await page.goto('/');

		await expect(page.locator('html')).toHaveAttribute('lang', 'de-AT');
		await page.keyboard.press('Tab');
		await expect(page.getByRole('link', { name: 'Zum Inhalt springen' })).toBeFocused();
	});
});
