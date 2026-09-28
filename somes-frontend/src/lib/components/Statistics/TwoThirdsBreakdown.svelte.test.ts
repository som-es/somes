import '../../../routes/layout.css';
import { expect, it, vi, beforeEach } from 'vitest';
import { render } from 'vitest-browser-svelte';
import { userEvent } from 'vitest/browser';
import TwoThirdsBreakdown from './TwoThirdsBreakdown.svelte';
import CustomBarChart from './charts/CustomBarChart.svelte';
import Page from '../../../routes/[parliament=parliament]/statistics/two_thirds_majority/+page.svelte';
import { justPostStatistics } from '$lib/api/api';

vi.mock('$lib/caching/legis_periods', () => ({
	cachedAllLegisPeriods: async () => [{ gp: 'XXVIII' }]
}));
vi.mock('$lib/api/parliament', () => ({ getParliament: () => 'at' }));
vi.mock('$lib/api/api', () => ({ justPostStatistics: vi.fn() }));
vi.mock('$lib/i18n/i18n.svelte', () => ({ t: (key: string) => key }));
vi.mock('$lib/i18n', () => ({ localeStore: { value: 'de' } }));
const rows = [{ category: 'Party A', positive: 3, negative: 1, other: 0, unknown: 0, total: 4 }];
beforeEach(() => {
	vi.mocked(justPostStatistics).mockReset().mockResolvedValue(rows);
});

async function openFilters(container: HTMLElement) {
	const trigger = [...container.querySelectorAll('button')].find((button) =>
		button.textContent?.includes('filter.title')
	)!;
	trigger.click();
	await expect.poll(() => document.querySelector('[data-popover-content]')).not.toBeNull();
}

function filterOption(label: string) {
	return [...document.querySelectorAll('button')].find(
		(button) => button.textContent?.trim() === label
	)!;
}

it('renders LayerChart bars and switches between percentage and count without refetching', async () => {
	vi.mocked(justPostStatistics).mockResolvedValue([
		...rows,
		{ category: 'Party B', positive: 8, negative: 0, other: 0, unknown: 0, total: 8 }
	]);
	const { container } = render(TwoThirdsBreakdown, { mode: 'party', periods: ['XXVIII'] });
	await expect.poll(() => container.querySelectorAll('.lc-bars').length).toBe(8);
	expect(justPostStatistics).toHaveBeenCalledWith('two_thirds_by_party', {
		legis_period: 'XXVIII'
	});
	expect(container.querySelector('[aria-label*="Party A:"]')?.getAttribute('aria-label')).toContain(
		'75%'
	);
	const before = container.querySelector('[aria-label*="Party A:"] svg')?.innerHTML;
	await openFilters(container);
	filterOption('statistics.absolute').click();
	await expect
		.poll(() => container.querySelector('[aria-label*="Party A:"] svg')?.innerHTML === before)
		.toBe(false);
	expect(justPostStatistics).toHaveBeenCalledTimes(1);
	expect(container.innerHTML).not.toContain('NaN');
});

it('defaults to the newest period and allows selecting all periods', async () => {
	const { container } = render(TwoThirdsBreakdown, { mode: 'topic', periods: ['XXVII', 'XXVIII'] });
	await expect.poll(() => container.querySelectorAll('.lc-bars').length).toBe(4);
	expect(justPostStatistics).toHaveBeenLastCalledWith('two_thirds_by_topic', {
		legis_period: 'XXVIII'
	});
	vi.mocked(justPostStatistics).mockResolvedValue([]);
	await openFilters(container);
	filterOption('statistics.all').click();
	await expect.poll(() => container.textContent).toContain('statistics.chartControl.noData');
	expect(justPostStatistics).toHaveBeenLastCalledWith('two_thirds_by_topic', {
		legis_period: null
	});
});

it('recovers from a failed request using retry', async () => {
	vi.mocked(justPostStatistics).mockResolvedValueOnce({
		error: 'unavailable',
		error_type: 'test',
		field: '',
		meta: null
	});
	const { container } = render(TwoThirdsBreakdown, { mode: 'party', periods: [] });
	await expect.poll(() => container.querySelector('[role="alert"]')).not.toBeNull();
	const retry = [...container.querySelectorAll('button')].find((b) =>
		b.textContent?.includes('statistics.twoThirds.retry')
	)!;
	retry.click();
	await expect.poll(() => container.querySelectorAll('.lc-bars').length).toBe(4);
	expect(container.querySelector('[role="alert"]')).toBeNull();
});

it('sorts and searches categories locally with the shared controls', async () => {
	vi.mocked(justPostStatistics).mockResolvedValue([
		...rows,
		{ category: 'Party B', positive: 8, negative: 0, other: 0, unknown: 0, total: 8 }
	]);
	const { container } = render(TwoThirdsBreakdown, { mode: 'party', periods: ['XXVIII'] });
	await expect.poll(() => container.querySelectorAll('.lc-bars').length).toBe(8);
	expect(container.querySelector('[role="img"]')?.getAttribute('aria-label')).toContain('Party B');
	await openFilters(container);
	filterOption('statistics.ascending').click();
	await expect
		.poll(() => container.querySelector('[role="img"]')?.getAttribute('aria-label'))
		.toContain('Party A');
	const search = container.querySelector('input[type="search"]') as HTMLInputElement;
	search.value = 'Party B';
	search.dispatchEvent(new Event('input', { bubbles: true }));
	await expect.poll(() => container.querySelectorAll('[role="img"]').length).toBe(1);
	expect(container.querySelector('[role="img"]')?.getAttribute('aria-label')).toContain('Party B');
	expect(justPostStatistics).toHaveBeenCalledTimes(1);
});

it('places category search between analysis and filters and uses standard normalization labels', async () => {
	vi.mocked(justPostStatistics).mockImplementation(async (route) =>
		route === 'legislative_initiative_outcomes_by_period'
			? [{ gp: 'XXVIII', accepted: 'a', total_initiatives: 4 }]
			: rows
	);
	const { container } = render(Page);
	const partyButton = [...container.querySelectorAll('button')].find((button) =>
		button.textContent?.includes('statistics.twoThirds.byParty')
	)!;
	partyButton.click();
	await expect.poll(() => container.querySelectorAll('.lc-bars').length).toBe(4);
	const selectedButton = [...container.querySelectorAll('button')].find((button) =>
		button.textContent?.includes('statistics.twoThirds.byParty')
	)!;
	const filterButton = [...container.querySelectorAll('button')].find((button) =>
		button.textContent?.includes('filter.title')
	)!;
	const search = container.querySelector('input[type="search"]')!;
	expect(search.closest('section')).toBe(selectedButton.closest('section'));
	expect(filterButton.closest('section')).toBe(selectedButton.closest('section'));
	expect(
		selectedButton.compareDocumentPosition(search) & Node.DOCUMENT_POSITION_FOLLOWING
	).toBeTruthy();
	expect(
		search.compareDocumentPosition(filterButton) & Node.DOCUMENT_POSITION_FOLLOWING
	).toBeTruthy();
	await openFilters(container);
	expect(filterOption('statistics.normalized')).toBeDefined();
	expect(filterOption('statistics.absolute')).toBeDefined();
});

it('uses the shared chart layout with stacked colours and keyboard details', async () => {
	const { container } = render(TwoThirdsBreakdown, { mode: 'party', periods: ['XXVIII'] });
	await expect.poll(() => container.querySelectorAll('.lc-bars').length).toBe(4);
	expect(container.querySelector('.chart-scrollbar')).not.toBeNull();
	expect(container.querySelector('.chart-axis')?.textContent).toContain('100%');
	const marks = [...container.querySelectorAll('.lc-bars path, .lc-bars rect')];
	expect(marks.some((mark) => getComputedStyle(mark).fill === 'rgb(34, 197, 94)')).toBe(true);
	expect(marks.some((mark) => getComputedStyle(mark).fill === 'rgb(239, 68, 68)')).toBe(true);
	const row = container.querySelector('[role="listitem"]') as HTMLElement;
	row.focus();
	expect(container.textContent).toContain('75%');
	expect(container.textContent).toContain('25%');
	expect(container.textContent).toContain('statistics.twoThirds.number: 4');
});

it('keeps the existing single-series bar chart working', async () => {
	const { container } = render(CustomBarChart, {
		data: [
			{ category: 'Example', value: 1.25, party: '', color: '#123456', valueLabel: 'Complexity' }
		],
		metricLabel: 'Complexity',
		selectedCategory: 'party',
		chartDescription: 'Description',
		valuePrecision: 2
	});
	await expect.poll(() => container.querySelectorAll('.lc-bars').length).toBe(1);
	expect(container.querySelector('[role="img"]')?.getAttribute('aria-label')).toContain('1,25');
	expect(container.querySelector('.chart-axis')?.textContent).not.toContain('%');
	expect(container.innerHTML).not.toContain('NaN');
	const marks = [...container.querySelectorAll('.lc-bars path, .lc-bars rect')];
	expect(marks.some((mark) => getComputedStyle(mark).fill === 'rgb(18, 52, 86)')).toBe(true);
});

it('uses the shared chart for period outcomes and preserves undefined acceptance rates', async () => {
	vi.mocked(justPostStatistics).mockResolvedValue([
		{ gp: 'XXVII', accepted: 'a', total_initiatives: 3 },
		{ gp: 'XXVII', accepted: 'd', total_initiatives: 1 }
	]);
	const { container } = render(Page);
	await expect.poll(() => container.querySelectorAll('.lc-bars').length).toBe(8);
	expect(container.querySelector('.chart-scrollbar')).not.toBeNull();
	expect(container.querySelector('.chart-axis')).not.toBeNull();
	const rate = [...container.querySelectorAll('button')].find((button) =>
		button.textContent?.includes('statistics.twoThirds.acceptanceRate')
	)!;
	rate.click();
	await expect.poll(() => container.querySelectorAll('.lc-bars').length).toBe(1);
	expect(container.querySelector('[aria-label^="XXVII:"]')?.getAttribute('aria-label')).toContain(
		'75%'
	);
	expect(container.querySelector('[aria-label^="XXVIII:"]')?.getAttribute('aria-label')).toContain(
		'–'
	);
	expect(container.querySelector('[aria-label^="XXVIII:"] svg')).toBeNull();
	expect(container.querySelector('.chart-axis')?.textContent).toContain('100%');
});

it('left-aligns long category labels while allowing them to wrap', async () => {
	const name = 'A very long category name that wraps across multiple lines';
	const { container } = render(CustomBarChart, {
		data: [{ category: name, value: 5, party: '', color: '#123456', valueLabel: 'Count' }],
		metricLabel: 'Count',
		selectedCategory: 'topic',
		chartDescription: 'Description'
	});
	await expect.poll(() => container.querySelectorAll('.lc-bars').length).toBe(1);
	const label = container.querySelector(`[title="${name}"]`)!;
	expect(getComputedStyle(label).textAlign).toBe('left');
	expect(getComputedStyle(label).whiteSpace).toBe('normal');
});

it('keeps trend search, outcome and generic filters in the top analysis toolbar', async () => {
	vi.mocked(justPostStatistics).mockResolvedValue([
		{ gp: 'XXVIII', accepted: 'a', total_initiatives: 3 },
		{ gp: 'XXVIII', accepted: 'd', total_initiatives: 1 }
	]);
	const { container } = render(Page);
	await expect.poll(() => container.querySelectorAll('.lc-bars').length).toBe(4);
	const trend = [...container.querySelectorAll('button')].find((button) =>
		button.textContent?.includes('statistics.twoThirds.countTrend')
	)!;
	trend.click();
	await expect.poll(() => container.querySelector('input[type="search"]')).not.toBeNull();
	const toolbar = container.querySelector('input[type="search"]')!.closest('section')!;
	expect(toolbar.textContent).toContain('statistics.twoThirds.countTrend');
	expect(toolbar.textContent).toContain('statistics.twoThirds.outcomeFilter');
	expect(toolbar.textContent).toContain('filter.title');
	expect(
		[...container.querySelectorAll('button')].some(
			(button) => button.textContent?.trim() === 'statistics.legislature'
		)
	).toBe(false);
	await expect.poll(() => container.querySelector('dl dd')?.textContent).toBe('4');
	const outcome = [...toolbar.querySelectorAll('button')].find((button) =>
		button.textContent?.includes('statistics.twoThirds.outcomeFilter')
	)!;
	await userEvent.click(outcome);
	await expect.poll(() => document.querySelectorAll('[role="option"]').length).toBe(4);
	const accepted = [...document.querySelectorAll('[role="option"]')].find((option) =>
		option.textContent?.includes('filterOption.acceptedYes')
	) as HTMLElement;
	await userEvent.click(accepted);
	await expect.poll(() => container.querySelector('dl dd')?.textContent).toBe('3');
	const search = container.querySelector('input[type="search"]') as HTMLInputElement;
	search.value = 'no matching period';
	search.dispatchEvent(new Event('input', { bubbles: true }));
	await expect.poll(() => container.textContent).toContain('statistics.chartControl.noData');
});
