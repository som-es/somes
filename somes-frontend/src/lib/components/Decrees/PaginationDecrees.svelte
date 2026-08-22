<script lang="ts">
	import { t } from '$lib/i18n/i18n.svelte';
	import { errorToNull, get_eurovoc_topics } from '$lib/api/api';
	import { onMount, untrack } from 'svelte';
	import Pagination from '../Pagination.svelte';
	import ExpandablePlaceholder from '../VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import { currentDecreeFilterStore } from '$lib/stores/stores';
	import DecreeBar from '../Delegates/Decrees/DecreeBar.svelte';
	import type { DecreeFilter, DecreesWithMaxPage } from '../Delegates/Decrees/types';
	import { SvelteSet } from 'svelte/reactivity';
	import { cachedAllLegisPeriods } from '$lib/caching/legis_periods';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import SearchBar from '../Filtering/SearchBar.svelte';
	import SortPopover from '../Filtering/SortPopover.svelte';
	import MultiValuesFilter from '../Filtering/MultiValuesFilter.svelte';
	import GenericFilters from '../Filtering/GenericFilters.svelte';
	import type { GenericFilterGroup } from '../Filtering/types';
	import { convertDecreeFilterToUrl } from './urlConversion';
	import DateRangeSnippet from '../Filtering/GenericFilterSnippets/DataRangeSnippet.svelte';
	import TopicFilter from '../Filtering/TopicFilter.svelte';
import { t } from '$lib/i18n/i18n.svelte';
import { localeStore } from '$lib/i18n/i18n.svelte';
	interface Props {
		decrees: DecreesWithMaxPage;
		selectedGp: string | null;
		departmentsPerGp: Record<string, string[]>;
	}

	let { decrees, selectedGp, departmentsPerGp }: Props = $props();

	let currentPage: number | undefined = $state(undefined);

	let legisPeriodFilter = $state({
		title: t('filter.legislaturePeriod'),
		activeValue: 'all',
		hidden: false,
		options: [{ title: t('filterOption.all'), value: 'all' }]
	});

	let searchValue = $state('');
	let sortOrder: 'relevance' | 'Desc' | 'Asc' = $state('relevance');

	let updatedAt = $derived(() => {
		const locale = localeStore.value === 'de' ? 'de-AT' : 'en-AT';
		return decrees.updated_at
			? new Intl.DateTimeFormat(locale, {
					day: '2-digit',
					month: '2-digit',
					year: 'numeric'
				}).format(new Date(decrees.updated_at))
			: t('date.unknown');
	});

	let genericFilters: [GenericFilterGroup<string>] = $state([
		{
			title: t('filter.date'),
			activeValue: undefined,
			hidden: false,
			advanced: true,
			id: 'dateRange',
			data: { dateFrom: '', dateTo: '' },
			options: []
		}
	]);

	let selectedTopics: SvelteSet<string> = $state(new SvelteSet());
	let selectedDepartments: SvelteSet<string> = $state(new SvelteSet());

	let departments = $derived.by(() => {
		if (selectedGp) {
			return departmentsPerGp[selectedGp];
		} else {
			const departments: string[] = [];
			const departmentSet = new Set();
			const keys = Object.keys(departmentsPerGp).sort().reverse();
			keys.forEach((key) => {
				departmentsPerGp[key].forEach((department) => {
					if (!departmentSet.has(department)) {
						departmentSet.add(department);
						departments.push(department);
					}
				});
			});
			return departments;
		}
	});

	onMount(() => {
		const maybeStoredFilter = currentDecreeFilterStore.value;
		if (maybeStoredFilter !== null) {
			if (maybeStoredFilter.legis_period)
				legisPeriodFilter.activeValue = maybeStoredFilter.legis_period;
			if (maybeStoredFilter.topics !== null) {
				selectedTopics = new SvelteSet(maybeStoredFilter.topics);
			}
			if (maybeStoredFilter.departments !== null) {
				selectedDepartments = new SvelteSet(maybeStoredFilter.departments);
			}
			if (maybeStoredFilter.date_from)
				genericFilters[0].data!.dateFrom = maybeStoredFilter.date_from;
			if (maybeStoredFilter.date_to) genericFilters[0].data!.dateTo = maybeStoredFilter.date_to;
			if (maybeStoredFilter.date_to) genericFilters[0].data!.dateTo = maybeStoredFilter.date_to;
			if (maybeStoredFilter.page) currentPage = maybeStoredFilter.page;
		}
	});

	$effect(() => {
		void legisPeriodFilter.activeValue;
		untrack(() => {
			selectedDepartments = new SvelteSet();
		});
	});

	const convertAndStoreFilter = () => {
		let filter: DecreeFilter = {
			gov_officials: null,
			legis_period: legisPeriodFilter.activeValue == 'all' ? null : legisPeriodFilter.activeValue,
			topics: selectedTopics.size > 0 ? [...selectedTopics] : null,
			departments: selectedDepartments.size > 0 ? [...selectedDepartments] : null,
			date_from: genericFilters[0].data?.dateFrom || null,
			date_to: genericFilters[0].data?.dateTo || null,
			page: currentPage ?? null
		};
		currentDecreeFilterStore.value = filter;
		return filter;
	};

	const loadDecrees = async () => {
		if (decrees !== null) {
			decrees.decrees = [];
		}
		const filter = convertAndStoreFilter();
		const nextUrl = convertDecreeFilterToUrl(filter, searchValue, new URL(page.url), sortOrder);

		goto(nextUrl, {
			keepFocus: true,
			replaceState: true,
			noScroll: true
		});
		// filter = null;

		// decrees = errorToNull(await decrees_by_search(page, filter, searchValue));
	};

	const update = () => {
		loadDecrees();
	};

	$effect(() => {
		void searchValue;
		void sortOrder;
		void selectedTopics.size;
		void selectedDepartments.size;
		void legisPeriodFilter.activeValue;
		void genericFilters[0].data?.dateFrom;
		void genericFilters[0].data?.dateTo;
		untrack(update);
	});

	$effect(() => {
		genericFilters[0].activeValue =
			genericFilters[0].data?.dateFrom || genericFilters[0].data?.dateTo ? 'set' : undefined;
	});
	$effect(() => {
		if (currentPage) {
			untrack(convertAndStoreFilter);
		}
	});

	let topics: string[] = $state([]);

	onMount(async () => {
		update();

		// Generic filter - Legislative period
		const fetchedPeriods = await cachedAllLegisPeriods();
		if (fetchedPeriods) {
			legisPeriodFilter.options = [
				{ title: t('filterOption.all'), value: 'all' },
				...fetchedPeriods.map((p) => ({ title: p.gp, value: p.gp }))
			];
		}

		const eurovocTopics = errorToNull(await get_eurovoc_topics());
		if (eurovocTopics) {
			topics = eurovocTopics.map((topic) => topic.topic);
		}
	});
</script>

<span class="mb-2 ml-1 block text-base text-gray-800 sm:mt-1 sm:ml-0 dark:text-gray-300">
	{t('decrees.updatedAt')} {updatedAt}
</span>

<div class="mt-7 md:flex">
	<!-- Search bar with inline sort trigger -->
	<SearchBar bind:searchValue>
		{#snippet rightSlot()}
			{#if searchValue.length > 0}
				<SortPopover bind:sortOrder />
			{/if}
		{/snippet}
	</SearchBar>

	<div class="mt-2 flex h-10 w-full gap-2 text-xs sm:text-base md:mt-0 md:ml-2 md:w-auto">
		<MultiValuesFilter
			title="Ministerien"
			bind:selectedValues={selectedDepartments}
			values={departments}
		/>
		<TopicFilter bind:selectedTopics {topics} />
		{#snippet dateRangeSnippet()}
			<DateRangeSnippet
				bind:dateFrom={genericFilters[0].data!.dateFrom}
				bind:dateTo={genericFilters[0].data!.dateTo}
			/>
		{/snippet}
		<GenericFilters
			bind:genericFilters
			bind:legisPeriodFilter
			snippets={{ dateRange: dateRangeSnippet }}
		/>
	</div>
</div>

<div>
	{#if decrees}
		<!-- <Pagination bind:page maxPage={decrees.max_page} /> -->
		{#if decrees.decrees.length > 0}
			{#each decrees.decrees as decree}
				<DecreeBar
					{decree}
					coloring="bg-primary-300 dark:bg-primary-500 dark:text-white"
					showDelegate
				/>
			{/each}
		{:else if false}
			{#each { length: 9 } as _}
				<ExpandablePlaceholder class="my-4" />
			{/each}
		{:else}
			{t('pagination.noResults')}
		{/if}
		<div class="float-right">
			<Pagination bind:currentPage maxPage={decrees.max_page} />
		</div>
	{:else}
		{#each { length: 9 } as _}
			<ExpandablePlaceholder class="my-4" />
		{/each}
		<!-- <CenterPrograssRadial /> -->
	{/if}
</div>
