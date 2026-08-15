<script lang="ts">
	import VoteResults from '$lib/components/VoteResults/VoteResults.svelte';
	import type { Delegate, GovProposalDelegate, Topic, UniqueTopic, VoteResult } from '$lib/types';
	import { onMount } from 'svelte';
	import { cachedLatestVoteResults } from '$lib/caching/vote_results';
	import Container from '$lib/components/Layout/Container.svelte';
	import { cachedLatestGovProposals } from '$lib/caching/gov_proposals';
	import { cachedUserTopics } from '$lib/caching/user_topics_cache.svelte';
	import NextSessionInfo from '$lib/components/PlenarySessions/NextSessionInfo.svelte';
	import { dashDateToDotDate } from '$lib/date';
	import { filteredDelegates } from '$lib/caching/delegates.svelte';
	import LatestProposals from '$lib/components/Proposals/Latest/LatestProposals.svelte';
	import SessionActivityOverview from '$lib/components/PlenarySessions/SessionActivityOverview.svelte';
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
	import { t } from '$lib/i18n/i18n.svelte';

	let { data }: PageProps = $props();

	let dels: Delegate[] | null = $derived(data.delegates);
	let voteResults: VoteResult[] | null = $derived(errorToNull(data.latestVotes));
	let govProposals: GovProposalDelegate[] | null = $derived(
		errorToNull(data.latestMinisterialProposals)
	);

	let decrees: DecreeDelegate[] | null = $derived(errorToNull(data.latestDelegateDecrees));
	let latestSessionActivity = $derived(errorToNull(data.latestSessionActivity));

	let userVoteResults: VoteResult[] | null = $state(null);
	let userGovProposals: GovProposalDelegate[] | null = $state(null);
	let userDecrees: DecreeDelegate[] | null = $state(null);

	function hasFavorites(favoriteTopics: UniqueTopic[], entryTopics: Topic[]): boolean {
		for (let i = 0; i < entryTopics.length; i++) {
			for (let j = 0; j < favoriteTopics.length; j++) {
				if (entryTopics[i].topic == favoriteTopics[j].topic) {
					return true;
				}
			}
		}
		return false;
	}

	onMount(async function () {
		const userTopics = await cachedUserTopics();

		const tempVoteResults = structuredClone($state.snapshot(voteResults));
		const tempGovProposals = structuredClone($state.snapshot(govProposals));
		const tempDecrees = structuredClone($state.snapshot(decrees));

		if (userTopics && tempVoteResults && tempGovProposals && tempDecrees) {
			voteResults = [];
			userVoteResults = tempVoteResults.filter((voteResult) => {
				if (hasFavorites(userTopics, voteResult.eurovoc_topics)) {
					return true;
				}
				voteResults?.push(voteResult);
				return false;
			});

			voteResults = voteResults;

			govProposals = [];
			userGovProposals = tempGovProposals.filter((govProp) => {
				if (
					hasFavorites(
						userTopics,
						govProp.gov_proposal.eurovoc_topics.length > 0
							? govProp.gov_proposal.eurovoc_topics
							: (govProp.gov_proposal.ai_summary?.full_summary.topics?.map((topic) => {
									return { topic };
								}) ?? [])
					)
				) {
					return true;
				}
				govProposals?.push(govProp);
				return false;
			});

			govProposals = govProposals;

			decrees = [];
			userDecrees = tempDecrees.filter((decree) => {
				if (
					hasFavorites(
						userTopics,
						decree.decree.ai_summary?.full_summary.topics.map((topic) => {
							return { topic };
						}) ?? []
					)
				) {
					return true;
				}
				decrees?.push(decree);
				return false;
			});

			decrees = decrees;
		} else {
			voteResults = tempVoteResults;
			govProposals = tempGovProposals;
			decrees = tempDecrees;
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
	<title>{t('home.meta.title')}</title>
	<meta name="description" content={t('home.meta.description')} />
</svelte:head>

<Container>
	<NextSessionInfo {nextPlenarySessionDateStr} />
	<SessionActivityOverview overview={latestSessionActivity} />
	<h2 class="mt-6 px-1 pt-2 text-3xl font-bold sm:p-0 sm:text-4xl">{t('home.latestVotes')}</h2>
	<span class="mb-2 ml-1 block text-base text-gray-800 sm:mt-1 sm:ml-0 dark:text-gray-200">
		{#if voteDate}
			{t('home.votedOn')} {dashDateToDotDate(voteDate)}
		{/if}
	</span>
	<!-- User Interests -->
	{#if userVoteResults && dels}
		<h2 class="text-xl font-semibold sm:text-2xl">{t('home.byInterest')}</h2>

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
				{t('home.moreVotes')}
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
		{t('home.ministerialDrafts')}
	</h2>
	{#if govProposals}
		{#if userGovProposals}
			<h2 class="text-xl font-semibold sm:text-2xl">{t('home.byInterest')}</h2>

			<LatestProposals govProposals={userGovProposals} />
		{/if}
		{#if userGovProposals}
			<h2 class="mt-2 text-xl font-semibold sm:text-2xl">{t('home.other')}</h2>
		{/if}
		{#if govProposals.length == 0}
			<div class="w-full rounded-lg bg-surface-100-900 p-20 text-center">{t('home.none')}</div>
		{:else}
			<LatestProposals {govProposals} />
		{/if}
		<div class="mt-3">
			<a
				href={ministerialHistoryUrl.href}
				class="group flex w-fit items-center gap-1 text-base text-gray-800 hover:text-black dark:text-gray-300 dark:hover:text-white"
			>
				{t('home.moreMinisterialDrafts')}
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
		{t('home.decrees')}
	</h2>
	{#if decrees}
		{#if userDecrees}
			<h2 class="text-xl font-semibold sm:text-2xl">{t('home.byInterest')}</h2>
			{#each userDecrees as decree (decree.decree.ris_id)}
				<DecreeBar
					{decree}
					showDelegate
					coloring="bg-primary-300 dark:bg-primary-500 dark:text-white"
				/>
			{/each}
		{/if}
		{#if userDecrees}
			<h2 class="mt-2 text-xl font-semibold sm:text-2xl">{t('home.other')}</h2>
		{/if}
		{#if decrees.length == 0}
			<div class="w-full rounded-lg bg-surface-100-900 p-20 text-center">{t('home.none')}</div>
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
				{t('home.moreDecrees')}
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
