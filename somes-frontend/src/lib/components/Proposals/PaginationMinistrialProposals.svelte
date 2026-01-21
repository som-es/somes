<script lang="ts">
	import { pushState } from '$app/navigation';
	import { errorToNull, gov_proposals_by_search } from '$lib/api/api';
	import type { GovPropFilter, GovProposalsWithMaxPage } from '$lib/types';
	import { onMount } from 'svelte';
	import Pagination from '../Pagination.svelte';
	import ExpandablePlaceholder from '../VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import { currentGovProposalFilterStore } from '$lib/stores/stores';
	import GovProposalExpandableBar from './Latest/GovProposalExpandableBar.svelte';
	import { translationFn, type FilterInfo } from '../Filtering/types';
	import FiltersAny from '../Filtering/FiltersAny.svelte';

	const url = new URL(window.location.href);
	let page = parseInt(url.searchParams.get('page') || '1') || 1;

	let old_page = page;
	let currentlyUpdating = false;
	let govProposals: GovProposalsWithMaxPage | null = null;

	let searchValue = '';
	let selectedPeriod = 'all';

	let hasVoteResultFilter: FilterInfo<boolean | undefined> = {
		title: 'Abstimmungsstatus',
		attributeName: 'hasVoteFilter',
		filterObj: undefined,
		translationFn,
		hidden: false,
		values: [
			{ title: 'egal', value: undefined },
			{ title: 'mit Abstimmung', value: true },
			{ title: 'ohne Abstimmung', value: false }
		]
	};

	const maybeStoredFilter = currentGovProposalFilterStore.value;
	if (maybeStoredFilter !== null) {
		if (maybeStoredFilter.has_vote_result)
			hasVoteResultFilter.filterObj = maybeStoredFilter.has_vote_result;
		if (maybeStoredFilter.legis_period) selectedPeriod = maybeStoredFilter.legis_period;
	}

	const loadGovProps = async () => {
		currentlyUpdating = true;
		if (govProposals !== null) {
			govProposals.gov_proposals = [];
		}

		let filter: GovPropFilter | null = {
			has_vote_result:
				hasVoteResultFilter.filterObj == undefined ? null : hasVoteResultFilter.filterObj,
			legis_period: selectedPeriod == 'all' ? null : selectedPeriod
		};
		currentGovProposalFilterStore.value = filter;
		// filter = null;

		govProposals = errorToNull(await gov_proposals_by_search(page, searchValue, filter));
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

	let filters = [hasVoteResultFilter];

	$: if (page || selectedPeriod || filters || searchValue) {
		update();
	}

	onMount(update);
</script>

<!-- <FiltersAny bind:filters bind:selectedPeriod bind:searchValue {update} /> -->

<div>
	{#if govProposals}
		<!-- <Pagination bind:page maxPage={govProposals.max_page} /> -->
		{#if govProposals.gov_proposals.length > 0}
			{#each govProposals.gov_proposals as govProposal}
				<GovProposalExpandableBar {govProposal} showDelegate class="" />
			{/each}
		{:else if currentlyUpdating}
			{#each { length: 9 } as _}
				<ExpandablePlaceholder class="my-4" />
			{/each}
		{:else}
			Keine Ministerialentwürfe gefunden
		{/if}
		<div class="float-right">
			<Pagination bind:page maxPage={govProposals.max_page} />
		</div>
	{:else}
		{#each { length: 9 } as _}
			<ExpandablePlaceholder class="my-4" />
		{/each}
		<!-- <CenterPrograssRadial /> -->
	{/if}
</div>
