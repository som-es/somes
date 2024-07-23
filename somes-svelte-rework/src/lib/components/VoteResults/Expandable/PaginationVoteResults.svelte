<script lang="ts">
	import type { Delegate, VoteResult, VoteResultsWithMaxPage } from '$lib/types';
	import { onMount } from 'svelte';
	import { vote_results_per_page } from '$lib/api';
	import SButton from '../../UI/SButton.svelte';
	import VoteResultExpandableBar from './VoteResultExpandableBar.svelte';
	import { pushState } from '$app/navigation';
	import Pagination from '$lib/components/Pagination.svelte';
	import { ProgressRadial } from '@skeletonlabs/skeleton';

	export let dels: Delegate[];

	let voteResults: VoteResultsWithMaxPage | null = null;

	// get page number from query params
	const url = new URL(window.location.href);
	let page = parseInt(url.searchParams.get('page') || '1') || 1;

	const loadVoteResults = async () => {
		if (voteResults != null) {
			voteResults.vote_results = [];
		}
		voteResults = await vote_results_per_page(page - 1);
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

	$: if (page) {
		update();
	}
</script>

<div>
	{#if voteResults}
		<Pagination bind:page maxPage={voteResults.max_page} />
		{#if voteResults.vote_results.length > 0}
			{#each voteResults.vote_results as voteResult}
				<VoteResultExpandableBar {dels} {voteResult} class="" />
			{/each}
		{:else}
			<ProgressRadial />
		{/if}
		<div class="float-right">
			<Pagination bind:page maxPage={voteResults.max_page} />
		</div>
	{:else}
		<ProgressRadial />
	{/if}
</div>

<style>
</style>
