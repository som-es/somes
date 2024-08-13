<script lang="ts">
	import type { Delegate, VoteResultFilter, VoteResult, VoteResultsWithMaxPage } from '$lib/types';
	import { onMount } from 'svelte';
	import { vote_results_per_page } from '$lib/api';
	import VoteResultExpandableBar from './VoteResultExpandableBar.svelte';
	import { pushState } from '$app/navigation';
	import Pagination from '$lib/components/Pagination.svelte';
	import { RadioGroup, RadioItem, SlideToggle } from '@skeletonlabs/skeleton';
	import LegisButtons from '$lib/components/Filtering/LegisButtons.svelte';
	import CenterPrograssRadial from '$lib/components/ProgressInfos/CenterPrograssRadial.svelte';
	import { currentVoteResultFilterStore } from '$lib/stores/stores';
	import { get } from 'svelte/store';
	import ExpandablePlaceholder from './Placeholders/ExpandablePlaceholder.svelte';

	export let dels: Delegate[];

	let voteResults: VoteResultsWithMaxPage | null = null;

	// get page number from query params
	const url = new URL(window.location.href);
	let page = parseInt(url.searchParams.get('page') || '1') || 1;

	let currentlyUpdating = false;

	let selectedPeriod = 'all';
	let simpleMajorityFilter: boolean | undefined = undefined;
	let acceptedFilter: string | undefined = undefined;
	let namedVoteFilter: boolean | undefined = undefined;

	const maybeStoredFilter = get(currentVoteResultFilterStore);
	if (maybeStoredFilter !== null) {
		if (maybeStoredFilter.simple_majority) simpleMajorityFilter = maybeStoredFilter.simple_majority;
		if (maybeStoredFilter.legis_period) selectedPeriod = maybeStoredFilter.legis_period;
		if (maybeStoredFilter.accepted) acceptedFilter = maybeStoredFilter.accepted;
		if (maybeStoredFilter.is_named_vote) namedVoteFilter = maybeStoredFilter.is_named_vote;
	}

	const loadVoteResults = async () => {
		currentlyUpdating = true;
		if (voteResults !== null) {
			voteResults.vote_results = [];
		}

		let accepted = acceptedFilter == null ? null : acceptedFilter;
		// switch (acceptedFilter) {
		// 	case 'a':
		// 		accepted = 'a';
		// 		break;
		// 	case 'declined':
		// 		accepted = 'd';
		// 		break;
		// 	case 'invisibly':
		// 		accepted = 'p';
		// 		break;
		// }

		let filter: VoteResultFilter | null = {
			is_named_vote: namedVoteFilter == undefined ? null : namedVoteFilter,
			accepted,
			simple_majority: simpleMajorityFilter == undefined ? null : simpleMajorityFilter,
			legis_period: selectedPeriod == 'all' ? null : selectedPeriod
		};
		currentVoteResultFilterStore.set(filter);
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

	$: if (page || selectedPeriod || simpleMajorityFilter || acceptedFilter || namedVoteFilter) {
		update();
	}
</script>

<!-- <br /> -->
<!-- <SlideToggle name="slider-large" active="bg-secondary-400" size="md">
	<span class="text-lg"> Test </span>
</SlideToggle> -->
<div class="mt-5">
	<h1 class="text-2xl font-bold">notwendige Mehrheit</h1>
	<RadioGroup
		rounded="rounded-container-token sm:!rounded-token"
		active="variant-filled-secondary"
		hover="hover:variant-soft-secondary"
		flexDirection="flex-col sm:flex-row"
	>
		<RadioItem bind:group={simpleMajorityFilter} name="simpleMajority" value={undefined}
			>egal</RadioItem
		>
		<RadioItem bind:group={simpleMajorityFilter} name="simpleMajority" value={true}
			>einfache Mehrheit</RadioItem
		>
		<RadioItem bind:group={simpleMajorityFilter} name="simpleMajority" value={false}
			>2/3 Mehrheit</RadioItem
		>
	</RadioGroup>
</div>
<div class="mt-5">
	<h1 class="text-2xl font-bold">Angenommen</h1>
	<RadioGroup
		rounded="rounded-container-token sm:!rounded-token"
		active="variant-filled-secondary"
		hover="hover:variant-soft-secondary"
		flexDirection="flex-col sm:flex-row"
	>
		<RadioItem bind:group={acceptedFilter} name="accepted" value={undefined}>egal</RadioItem>
		<RadioItem bind:group={acceptedFilter} name="accepted" value={'a'}>angenommen</RadioItem>
		<RadioItem bind:group={acceptedFilter} name="accepted" value={'d'}>abgelehnt</RadioItem>
		<RadioItem
			bind:group={acceptedFilter}
			name="accepted"
			value={'p'}
			title="frühzeitig abgelehnt - vor der 3. Lesung">frühzeitig abgelehnt</RadioItem
		>
	</RadioGroup>
</div>
<div class="mt-5">
	<h1 class="text-2xl font-bold">Abstimmung</h1>
	<RadioGroup active="variant-filled-secondary" hover="hover:variant-soft-secondary">
		<RadioItem bind:group={namedVoteFilter} name="namedVote" value={undefined}>egal</RadioItem>
		<RadioItem bind:group={namedVoteFilter} name="namedVote" value={true}
			>namentliche Abstimmung</RadioItem
		>
	</RadioGroup>
</div>
<div class="mt-5">
	<h2 class="font-bold text-2xl">Legislaturperioden</h2>
	<LegisButtons bind:selectedPeriod />
</div>
<div>
	{#if voteResults}
		<Pagination bind:page maxPage={voteResults.max_page} />
		{#if voteResults.vote_results.length > 0}
			{#each voteResults.vote_results as voteResult}
				<VoteResultExpandableBar {dels} {voteResult} class="" />
			{/each}
		{:else if currentlyUpdating}
			{#each { length: 9 } as _}
				<ExpandablePlaceholder class="my-3" />
			{/each}
		{:else}
			Keine Abstimmungsergebnisse gefunden
		{/if}
		<div class="float-right">
			<Pagination bind:page maxPage={voteResults.max_page} />
		</div>
	{:else}
		{#each { length: 9 } as _}
			<ExpandablePlaceholder class="my-3" />
		{/each}
		<!-- <CenterPrograssRadial /> -->
	{/if}
</div>

<style>
</style>
