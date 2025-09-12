<script lang="ts">
	import VoteResults from '$lib/components/VoteResults/VoteResults.svelte';
	import { filteredDelegates } from '$lib/caching/delegates';
	import type { Delegate, GovProposalDelegate, VoteResult } from '$lib/types';
	import { onMount } from 'svelte';
	import { cachedLatestVoteResults } from '$lib/caching/vote_results';
	import Container from '$lib/components/Layout/Container.svelte';
	import { cachedLatestGovProposals } from '$lib/caching/gov_proposals';
	import LatestProposals from '$lib/components/Proposals/Latest/LatestProposals.svelte';
	import { cachedUserTopics } from '$lib/caching/user_topics_cache';
	import Calendar from '$lib/components/Calendar/Calendar.svelte';
	import PlenarCalendar from '$lib/components/PlenarySessions/PlenarCalendar.svelte';
	import NextSessionInfo from '$lib/components/PlenarySessions/NextSessionInfo.svelte';

	let dels: Delegate[] | null = null;
	let voteResults: VoteResult[] | null = null;
	let govProposals: GovProposalDelegate[] | null = null;
	let userVoteResults: VoteResult[] | null = null;
	onMount(async function () {
		// await updateColorStorage();
		dels = (await filteredDelegates())?.nr ?? null;
		const userTopics = await cachedUserTopics();
		const tempVoteResults = await cachedLatestVoteResults(false);
		govProposals = await cachedLatestGovProposals();

		if (userTopics && tempVoteResults) {
			voteResults = [];
			userVoteResults = tempVoteResults.filter((voteResult) => {
				for (let i = 0; i < voteResult.eurovoc_topics.length; i++) {
					for (let j = 0; j < userTopics.length; j++) {
						if (voteResult.eurovoc_topics[i].topic == userTopics[j].topic) {
							return true;
						}
					}
				}
				voteResults?.push(voteResult);
				return false;
			});

			voteResults = voteResults;
		} else {
			voteResults = tempVoteResults;
		}
	});

</script>

<Container>	
	<h1 class="mt-2 text-3xl sm:text-5xl font-bold">
		Neuigkeiten
	</h1>
	<NextSessionInfo />	
	<h2 class="text-xl sm:text-3xl font-bold">Letzte Abstimmungen</h2>
	{#if userVoteResults && dels}
		<h2 class="text-xl sm:text-3xl font-bold">nach Interesse</h2>

		<VoteResults {dels} voteResults={userVoteResults} />
		<!-- {:else if use} -->
	{/if}
	{#if voteResults && dels}
		<VoteResults {dels} {voteResults} showHistory />
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

	<h2 class="mt-2 text-xl sm:text-3xl font-bold">Ministerialentwürfe der letzten 30 Tage</h2>

	{#if govProposals}
		{#if govProposals.length == 0}
			<div class="w-full p-20 text-center bg-surface-100-800-token rounded-lg">Keine</div>
		{:else}
			<LatestProposals {govProposals} />
		{/if}
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
			</div>
		</section>
		<section class="card mt-1 w-full animate-pulse">
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
			</div>
		</section>
	{/if}	
</Container>
