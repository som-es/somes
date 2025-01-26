<script lang="ts">
	import VoteResults from '$lib/components/VoteResults/VoteResults.svelte';
	import { filteredDelegates } from '$lib/caching/delegates';
	import type { Delegate, GovProposalDelegate, VoteResult } from '$lib/types';
	import { onMount } from 'svelte';
	import { cachedLatestVoteResults } from '$lib/caching/vote_results';
	import Container from '$lib/components/Layout/Container.svelte';
	import { cachedLatestGovProposals } from '$lib/caching/gov_proposals';
	import LatestProposals from '$lib/components/Proposals/Latest/LatestProposals.svelte';

	let dels: Delegate[] | null = null;
	let voteResults: VoteResult[] | null = null;
	let govProposals: GovProposalDelegate[] | null = null;
	onMount(async function () {
		// await updateColorStorage();
		dels = (await filteredDelegates())?.nr ?? null;
		voteResults = await cachedLatestVoteResults();
		govProposals = await cachedLatestGovProposals();
	});
</script>

<Container>
	<h1 class="text-2xl sm:text-4xl font-bold">
		Neuste Abstimmungsergebnisse
	</h1>
	{#if voteResults && dels}
		<VoteResults {dels} {voteResults} />
	{:else}
		<section class="card w-full animate-pulse">
			<div class="p-4 space-y-4">
				<div class="placeholder" />
				<div class="grid grid-cols-3 gap-8">
					<div class="placeholder" />
					<div class="placeholder" />
					<div class="placeholder" />
				</div>
				<div class="grid grid-cols-4 gap-4">
					<div class="placeholder" />
					<div class="placeholder" />
					<div class="placeholder" />
					<div class="placeholder" />
				</div>
				<div class="grid grid-cols-3 gap-8">
					<div class="placeholder" />
					<div class="placeholder" />
					<div class="placeholder" />
				</div>
				<div class="grid grid-cols-2 gap-5">
					<div class="placeholder" />
					<div class="placeholder" />
				</div>
				<div class="grid grid-cols-3 gap-7">
					<div class="placeholder" />
					<div class="placeholder" />
					<div class="placeholder" />
				</div>
				<div class="grid grid-cols-4 gap-3">
					<div class="placeholder" />
					<div class="placeholder" />
					<div class="placeholder" />
					<div class="placeholder" />
				</div>
			</div>
		</section>
	{/if}

	<h1 class="mt-2 text-2xl sm:text-4xl font-bold">
		Ministerialentwürfe der letzten 30 Tage
	</h1>

	{#if govProposals}
		<LatestProposals govProposals={govProposals} />
	{/if}
</Container>
