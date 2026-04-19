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
	import { resolve } from '$app/paths';
	import { convertVoteResultFilterToUrl } from '$lib/components/VoteResults/Expandable/urlConversion';
	import {
		currentDecreeFilterStore,
		currentGovProposalFilterStore,
		currentVoteResultFilterStore
	} from '$lib/stores/stores';
	import { convertGovPropFilterToUrl } from '$lib/components/Proposals/urlConversion';
	import type { Decree, DecreeDelegate } from '$lib/components/Delegates/Decrees/types';
	import DecreeBar from '$lib/components/Delegates/Decrees/DecreeBar.svelte';
	import { convertDecreeFilterToUrl } from '$lib/components/Decrees/urlConversion';

	let { data }: PageProps = $props();

	let dels: Delegate[] | null = $derived(data.delegates);
	let voteResults: VoteResult[] | null = $derived(errorToNull(data.latestVotes));
	let govProposals: GovProposalDelegate[] | null = $derived(
		errorToNull(data.latestMinisterialProposals)
	);

	let decrees: DecreeDelegate[] | null = $derived(errorToNull(data.latestDelegateDecrees));

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
		return first.legislative_initiative.nr_plenary_activity_date;
	});

	const nextPlenarySessionDateStr = $derived(
		errorToNull(data.nextPlenarDate)?.date_and_time?.toString()
	);
	const voteResultUrl = $derived(
		convertVoteResultFilterToUrl(currentVoteResultFilterStore.value, '', undefined, true)
	);
	const ministerialHistoryUrl = $derived(
		convertGovPropFilterToUrl(currentGovProposalFilterStore.value, '', undefined)
	);
	const decreeHistoryUrl = $derived(
		convertDecreeFilterToUrl(currentDecreeFilterStore.value, '', undefined)
	);
</script>

<svelte:head>
	<title>Neuigkeiten</title>
	<meta name="description" content="Neue Ereignisse im Nationalrat" />
</svelte:head>

<Container>
	<NextSessionInfo {nextPlenarySessionDateStr} />
	<h2 class="mt-6 px-1 pt-2 text-3xl font-bold sm:p-0 sm:text-4xl">Letzte Abstimmungen</h2>
	<span class="mb-2 ml-1 block text-base text-gray-800 sm:mt-1 sm:ml-0 dark:text-gray-200">
		{#if voteDate}
			Abgestimmt am {dashDateToDotDate(voteDate)}
		{/if}
	</span>
	<!-- User Interests -->
	{#if userVoteResults && dels}
		<h2 class="text-xl font-bold sm:text-3xl">nach Interesse</h2>

		<VoteResults {dels} allSeats={data.allSeats} voteResults={userVoteResults} />
		<!-- {:else if use} -->
	{/if}
	{#if voteResults && dels}
		<div class="mt-5 flex flex-col gap-5">
			{#each voteResults.slice(0, 3) as voteResult}
				<VoteResultExpandableBar {voteResult} class="" />
			{/each}
		</div>
		<div class="mt-3">
			<a
				href={voteResultUrl.href}
				class="group flex w-fit items-center gap-1 text-base text-gray-800 hover:text-black dark:text-gray-300 dark:hover:text-white"
			>
				Weitere Abstimmungen
				<span class="transition-transform group-hover:translate-x-1">→</span>
			</a>
		</div>
	{:else}
		<section class="w-full animate-pulse card">
			<div class="space-y-4 p-4">
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

	<h2 class="mt-12 px-1 pt-2 text-3xl font-bold sm:p-0 sm:text-4xl">
		Ministerialentwürfe der letzten 30 Tage
	</h2>
	{#if govProposals}
		{#if govProposals.length == 0}
			<div class="w-full rounded-lg bg-surface-100-900 p-20 text-center">Keine</div>
		{:else}
			<LatestProposals {govProposals} />
		{/if}
		<div class="mt-3">
			<a
				href={ministerialHistoryUrl.href}
				class="group flex w-fit items-center gap-1 text-base text-gray-800 hover:text-black dark:text-gray-300 dark:hover:text-white"
			>
				Weitere Ministerialentwürfe
				<span class="transition-transform group-hover:translate-x-1">→</span>
			</a>
		</div>
	{:else}
		<section class="w-full animate-pulse card">
			<div class="space-y-4 p-4">
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
		<section class="mt-1 w-full animate-pulse card">
			<div class="space-y-4 p-4">
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
	<h2 class="mt-12 px-1 pt-2 text-3xl font-bold sm:p-0 sm:text-4xl">
		Verordnungen der letzten Woche
	</h2>
	{#if decrees}
		{#if decrees.length == 0}
			<div class="w-full rounded-lg bg-surface-100-900 p-20 text-center">Keine</div>
		{:else}
			{#each decrees as decree (decree.decree.ris_id)}
				<DecreeBar
					{decree}
					showDelegate
					coloring="bg-primary-300 dark:bg-primary-500 dark:text-white"
				/>
			{/each}
		{/if}
		<div class="mt-3">
			<a
				href={decreeHistoryUrl.href}
				class="group flex w-fit items-center gap-1 text-base text-gray-800 hover:text-black dark:text-gray-300 dark:hover:text-white"
			>
				Weitere Verordnungen
				<span class="transition-transform group-hover:translate-x-1">→</span>
			</a>
		</div>
	{:else}
		<section class="w-full animate-pulse card">
			<div class="space-y-4 p-4">
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
		<section class="mt-1 w-full animate-pulse card">
			<div class="space-y-4 p-4">
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
