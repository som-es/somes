<script lang="ts">
	import type { Delegate, LegisInitFilter, VoteResult, VoteResultsWithMaxPage } from '$lib/types';
	import { onMount } from 'svelte';
	import { vote_results_per_page } from '$lib/api';
	import SButton from '../../UI/SButton.svelte';
	import VoteResultExpandableBar from './VoteResultExpandableBar.svelte';
	import { pushState } from '$app/navigation';
	import Pagination from '$lib/components/Pagination.svelte';
	import { ProgressRadial } from '@skeletonlabs/skeleton';
	import LegisButtons from '$lib/components/Filtering/LegisButtons.svelte';

	export let dels: Delegate[];

	let voteResults: VoteResultsWithMaxPage | null = null;
	let selectedPeriod = "all";

	// get page number from query params
	const url = new URL(window.location.href);
	let page = parseInt(url.searchParams.get('page') || '1') || 1;

	let currentlyUpdating = false;

	const loadVoteResults = async () => {
		currentlyUpdating = true;
		if (voteResults !== null) {
			voteResults.vote_results = []
		}
		
		let filter: LegisInitFilter | null = {
			invisibly_declined: false,
			accepted: null,
			simple_majority: null,
			legis_period: selectedPeriod == "all" ? null : selectedPeriod,
		};
		// filter = null;
		voteResults = await vote_results_per_page(page - 1, filter);
		currentlyUpdating = false;
	};
	onMount(async () => {
		update();
	});

	let old_page = page;

	const update = () => {
		loadVoteResults();

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

	$: if (page || selectedPeriod) {
		update();
	}

</script>

<LegisButtons bind:selectedPeriod={selectedPeriod} />
<div>
	{#if voteResults}
		<Pagination bind:page={page} maxPage={voteResults.max_page} />
		{#if voteResults.vote_results.length > 0}
			{#each voteResults.vote_results as voteResult}
				<VoteResultExpandableBar {dels} {voteResult} class="" />
			{/each}
		{:else if currentlyUpdating}
			<ProgressRadial />
		{:else}
			Keine Abstimmungsergebnisse gefunden
		{/if}
		<div class="float-right">
			<Pagination bind:page={page} maxPage={voteResults.max_page} />
		</div>
	{:else}
		<ProgressRadial />
	{/if}
</div>

<style>
</style>
