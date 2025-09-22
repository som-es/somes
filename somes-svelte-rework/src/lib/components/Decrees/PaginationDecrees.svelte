<script lang="ts">
	import { delegate_by_id, errorToNull } from '$lib/api/api';
	import { onMount } from 'svelte';
	import Pagination from '../Pagination.svelte';
	import ExpandablePlaceholder from '../VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import { currentDecreeFilterStore } from '$lib/stores/stores';
	import { get } from 'svelte/store';
	import DecreeBar from '../Delegates/Decrees/DecreeBar.svelte';
	import { pushState } from '$app/navigation';
	import FiltersAny from '../Filtering/FiltersAny.svelte';
	import type { FilterInfo } from '../Filtering/types';
	import type { DecreeFilter, DecreesWithMaxPage } from '../Delegates/Decrees/types';
	import type { Delegate } from '$lib/types';
	import { decrees_by_search, decrees_per_page } from '../Delegates/Decrees/api';

	const url = new URL(window.location.href);
	let page = parseInt(url.searchParams.get('page') || '1') || 1;

	let old_page = page;
	let currentlyUpdating = false;
	let decrees: DecreesWithMaxPage | null = null;

	let searchValue = '';
	let selectedPeriod = 'all';
	let govOfficialsFilter: number[] | undefined = undefined;

	const maybeStoredFilter = get(currentDecreeFilterStore);
	if (maybeStoredFilter !== null) {
		if (maybeStoredFilter.gov_officials) govOfficialsFilter = maybeStoredFilter.gov_officials;
		if (maybeStoredFilter.legis_period) selectedPeriod = maybeStoredFilter.legis_period;
	}

	const govOfficials = new Map<number, Delegate>();

	const loadGovProps = async () => {
		currentlyUpdating = true;
		if (decrees !== null) {
			decrees.decrees = [];
		}

		let filter: DecreeFilter | null = {
			gov_officials: null,
			legis_period: selectedPeriod == 'all' ? null : selectedPeriod
		};
		currentDecreeFilterStore.set(filter);
		// filter = null;

		decrees = errorToNull(await decrees_by_search(page, filter, searchValue));

		decrees?.decrees.forEach((decree) => {
			const id = decree.gov_official_id;
			if (govOfficials.has(id)) return;
			delegate_by_id(id).then((del) => {
				const mayDel = errorToNull(del);
				if (mayDel) {
					govOfficials.set(id, mayDel);
					decrees = decrees;
				}
			});
		});

		currentlyUpdating = false;
	};

	const update = () => {
		loadGovProps();

		// update query params
		const url = new URL(window.location.href);
		url.searchParams.set('page', page.toString());
		try {
			pushState(url.toString(), { replaceState: true });
		} catch (e) {
			page = old_page;
		}

		old_page = page;
	};

	$: if (page || selectedPeriod || govOfficialsFilter) {
		update();
	}

	onMount(update);

	let filters: FilterInfo<boolean | undefined>[] = [];
</script>

<FiltersAny bind:filters bind:searchValue bind:selectedPeriod {update} />

<div>
	{#if decrees}
		<!-- <Pagination bind:page maxPage={decrees.max_page} /> -->
		{#if decrees.decrees.length > 0}
			{#each decrees.decrees as decree}
				<DecreeBar
					{decree}
					{page}
					delegate={govOfficials.get(decree.gov_official_id)}
					coloring="bg-primary-300 dark:bg-primary-500"
				/>
			{/each}
		{:else if currentlyUpdating}
			{#each { length: 9 } as _}
				<ExpandablePlaceholder class="my-4" />
			{/each}
		{:else}
			Keine Verordnungen gefunden
		{/if}
		<div class="float-right">
			<Pagination bind:page maxPage={decrees.max_page} />
		</div>
	{:else}
		{#each { length: 9 } as _}
			<ExpandablePlaceholder class="my-4" />
		{/each}
		<!-- <CenterPrograssRadial /> -->
	{/if}
</div>
