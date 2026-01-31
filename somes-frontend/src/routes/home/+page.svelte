<script lang="ts">
	import VoteResults from '$lib/components/VoteResults/VoteResults.svelte';
	import type { Delegate, GovProposalDelegate, VoteResult } from '$lib/types';
	import { onMount } from 'svelte';
	import { cachedLatestVoteResults } from '$lib/caching/vote_results';
	import Container from '$lib/components/Layout/Container.svelte';
	import { cachedLatestGovProposals } from '$lib/caching/gov_proposals';
	import { cachedUserTopics } from '$lib/caching/user_topics_cache.svelte';
	import NextSessionInfo from '$lib/components/PlenarySessions/NextSessionInfo.svelte';
	import { dashDateToDotDate } from '$lib/date';
	import { filteredDelegates } from '$lib/caching/delegates.svelte';
	import LatestProposals from '$lib/components/Proposals/Latest/LatestProposals.svelte';
	import type { PageProps } from './$types';
	import { errorToNull } from '$lib/api/api';
	import VoteResultExpandableBar from '$lib/components/VoteResults/Expandable/VoteResultExpandableBar.svelte';
	import RightArrowIcon from '$lib/assets/misc_icons/right-arrow-small.svg';

	let { data }: PageProps = $props();

	let dels: Delegate[] | null = $derived(data.delegates);
	let voteResults: VoteResult[] | null = $derived(errorToNull(data.latestVotes));
	let govProposals: GovProposalDelegate[] | null = $derived(errorToNull(data.latestMinisterialProposals));

	let userVoteResults: VoteResult[] | null = $state(null);

	onMount(async function () {
		const userTopics = await cachedUserTopics();

		const tempVoteResults = structuredClone($state.snapshot(voteResults));

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

	const voteDate: string | null = $derived.by(() => {
		if (voteResults == null) return null;
		const first = voteResults.at(0);
		if (first == null) return null;
		return first.legislative_initiative.nr_plenary_activity_date

	});

	const nextPlenarySessionDateStr = $derived(errorToNull(data.nextPlenarDate)?.date_and_time?.toString());
	
</script>

<svelte:head>
    <title>Neuigkeiten</title>
    <meta name="description" content="Neue Ereignisse im Nationalrat" />
</svelte:head>

<Container>
	<h1 class="text-3xl sm:text-4xl font-bold pt-2 px-1 sm:p-0">Neuigkeiten</h1>
	<NextSessionInfo nextPlenarySessionDateStr={nextPlenarySessionDateStr} />
	<h2 class="text-2xl font-bold mt-8">Letzte Abstimmungen</h2>
	<span class="ml-1 -mb-3 block text-base text-gray-800">
		{#if voteDate}
			am {dashDateToDotDate(voteDate)}
		{/if}
	</span>
	<!-- User Interests -->
	{#if userVoteResults && dels}
		<h2 class="text-xl sm:text-3xl font-bold">nach Interesse</h2>

		<VoteResults {dels} allSeats={data.allSeats} voteResults={userVoteResults} />
		<!-- {:else if use} -->
	{/if}
	{#if voteResults && dels}
		{#each voteResults.slice(0, 3) as voteResult}
			<VoteResultExpandableBar {dels} {voteResult} class="" />
		{/each}
		<div class="mt-3">
			<span class="text-base text-gray-800">Weitere Abstimmungen &#8594;</span>
		</div>
	{:else}
		<section class="card w-full animate-pulse">
			<div class="p-4 space-y-4">
				<div class="placeholder"></div>
				<div class="grid grid-cols-3 gap-8">
					<div class="placeholder"></div>
					<div class="placeholder"></div>
					<div class="placeholder"></div>
				</div>
				<div class="grid grid-cols-4 gap-4">
					<div class="placeholder"></div>
					<div class="placeholder"></div>
					<div class="placeholder"></div>
					<div class="placeholder"></div>
				</div>
				<div class="grid grid-cols-3 gap-8">
					<div class="placeholder"></div>
					<div class="placeholder"></div>
					<div class="placeholder"></div>
				</div>
				<div class="grid grid-cols-2 gap-5">
					<div class="placeholder"></div>
					<div class="placeholder"></div>
				</div>
				<div class="grid grid-cols-3 gap-7">
					<div class="placeholder"></div>
					<div class="placeholder"></div>
					<div class="placeholder"></div>
				</div>
				<div class="grid grid-cols-4 gap-3">
					<div class="placeholder"></div>
					<div class="placeholder"></div>
					<div class="placeholder"></div>
					<div class="placeholder"></div>
				</div>
			</div>
		</section>
	{/if}

	
	<h2 class="text-2xl font-bold mt-8">Ministerialentwürfe der letzten 30 Tage</h2>
	{#if govProposals}
		{#if govProposals.length == 0}
			<div class="w-full p-20 text-center bg-surface-100-900 rounded-lg">Keine</div>
		{:else}
			<LatestProposals {govProposals} />
		{/if}
	{:else}
		<section class="card w-full animate-pulse">
			<div class="p-4 space-y-4">
				<div class="placeholder"></div>
				<div class="grid grid-cols-3 gap-8">
					<div class="placeholder"></div>
					<div class="placeholder"></div>
					<div class="placeholder"></div>
				</div>
				<div class="grid grid-cols-4 gap-4">
					<div class="placeholder"></div>
					<div class="placeholder"></div>
					<div class="placeholder"></div>
					<div class="placeholder"></div>
				</div>
			</div>
		</section>
		<section class="card mt-1 w-full animate-pulse">
			<div class="p-4 space-y-4">
				<div class="placeholder"></div>
				<div class="grid grid-cols-3 gap-8">
					<div class="placeholder"></div>
					<div class="placeholder"></div>
					<div class="placeholder"></div>
				</div>
				<div class="grid grid-cols-4 gap-4">
					<div class="placeholder"></div>
					<div class="placeholder"></div>
					<div class="placeholder"></div>
					<div class="placeholder"></div>
				</div>
			</div>
		</section>
	{/if}
</Container>
