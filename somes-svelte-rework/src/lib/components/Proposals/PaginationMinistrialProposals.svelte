<script lang="ts">
	import { pushState } from "$app/navigation";
	import { errorToNull, gov_proposals_by_search, gov_proposals_per_page } from "$lib/api/api";
	import type { GovPropFilter, GovProposalsWithMaxPage } from "$lib/types";
	import { onMount } from "svelte";
	import Pagination from "../Pagination.svelte";
	import ExpandablePlaceholder from "../VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte";
	import { currentGovProposalFilterStore  } from "$lib/stores/stores";
	import { get } from "svelte/store";
	import LegisButtons from "../Filtering/LegisButtons.svelte";
	import GovProposalExpandableBar from "./Latest/GovProposalExpandableBar.svelte";
	import SButton from "../UI/SButton.svelte";

    
    const url = new URL(window.location.href);
	let page = parseInt(url.searchParams.get('page') || '1') || 1;

	let old_page = page;
    let currentlyUpdating = false;
    let govProposals: GovProposalsWithMaxPage | null = null;

    let searchValue = "";
	let selectedPeriod = 'all';
	let hasVoteResultFilter: boolean | undefined = undefined;

    const maybeStoredFilter = get(currentGovProposalFilterStore);
	if (maybeStoredFilter !== null) {
		if (maybeStoredFilter.has_vote_result) hasVoteResultFilter = maybeStoredFilter.has_vote_result;
		if (maybeStoredFilter.legis_period) selectedPeriod = maybeStoredFilter.legis_period;
	}

	const loadGovProps = async () => {
		currentlyUpdating = true;
		if (govProposals !== null) {
			govProposals.gov_proposals = [];
		}

		
		let filter: GovPropFilter | null = {
			has_vote_result: hasVoteResultFilter == undefined ? null : hasVoteResultFilter,
			legis_period: selectedPeriod == 'all' ? null : selectedPeriod
		};
		currentGovProposalFilterStore.set(filter);
		// filter = null;

		if (searchValue) {
			const govPropsSearch = errorToNull(
				await gov_proposals_by_search(page, searchValue, filter)
			);
            console.log(govPropsSearch);
			if (govPropsSearch) govProposals = govPropsSearch;
		} else {
			govProposals = errorToNull(await gov_proposals_per_page(page - 1, filter));
		}
		currentlyUpdating = false;
	};

    const update = () => {
		loadGovProps();

		// update query params
		const url = new URL(window.location.href);
		url.searchParams.set('page', page.toString());
		try {
			// pushState(url.toString(), { replaceState: true });
		} catch (e) {
			page = old_page;
		}

		old_page = page;
	};

	$: if (page || selectedPeriod || hasVoteResultFilter) {
		update();
	}

    onMount(update);
</script>


<div class="mt-5">
	<h2 class="font-bold text-2xl">Legislaturperioden</h2>
	<LegisButtons bind:selectedPeriod />
</div>
<div class="mt-5">
	<h2 class="font-bold text-2xl">Suche</h2>
	<div class="flex flex-row gap-4">
		<input
			class="input w-full h-12 px-2"
			type="search"
			name="ac-demo"
			bind:value={searchValue}
			on:change={update}
			placeholder="Suchen..."
		/>
		<SButton class="bg-secondary-500 text-black" on:click={update}>Suchen</SButton>
	</div>
</div>
<div>
	{#if govProposals}
		<Pagination bind:page maxPage={govProposals.max_page} />
		{#if govProposals.gov_proposals.length > 0}
			{#each govProposals.gov_proposals as govProposal}
				<GovProposalExpandableBar {govProposal} class="" />
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
