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
	import Calendar from '$lib/components/Calendar.svelte';

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
	const date = new Date();
	const year = date.getFullYear();
	const month = date.getMonth();

	var daysInThisMonth = new Date(year, month+1, 0).getDate();

	const days = []

	for (let i = 0; i < daysInThisMonth; i++) {
		let testDate = new Date(year,month,i+1);
		console.log(testDate);
		console.log(testDate.getDay());
		if (testDate.getDay() > 0 && testDate.getDay() < 6) {
			days.push({name: (i +1).toString(), enabled: true, date: testDate})
		}
	}
	
</script>

<Container>
	<Calendar {days} headers={["Mo", "Di", "Mi", "Do", "Fr"]} />
	{#if userVoteResults && dels}
		<h1 class="text-2xl sm:text-4xl font-bold">Abstimmungsergebnisse nach Interesse</h1>

		<VoteResults {dels} voteResults={userVoteResults} />
		<!-- {:else if use} -->
	{/if}
	<h1 class="text-2xl sm:text-4xl font-bold">Neuste Abstimmungsergebnisse</h1>
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

	<h1 class="mt-2 text-2xl sm:text-4xl font-bold">Ministerialentwürfe der letzten 30 Tage</h1>

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
