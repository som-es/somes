<script lang="ts">
	import type {
		Delegate,
		VoteResultFilter,
		VoteResultsWithMaxPage,
		HasError,
		UniqueTopic
	} from '$lib/types';
	import { onMount } from 'svelte';
	import {
		errorToNull,
		vote_results_by_search,
		vote_results_per_page,
		get_eurovoc_topics
	} from '$lib/api/api';
	import VoteResultExpandableBar from './VoteResultExpandableBar.svelte';
	import { pushState } from '$app/navigation';
	import Pagination from '$lib/components/Pagination.svelte';
	import { type PopupSettings, popup } from '@skeletonlabs/skeleton';
	import { currentVoteResultFilterStores } from '$lib/stores/stores';
	import { get } from 'svelte/store';
	import ExpandablePlaceholder from './Placeholders/ExpandablePlaceholder.svelte';
	import searchIcon from '$lib/assets/misc_icons/search-glass.svg?raw';
	import downArrowIcon from '$lib/assets/misc_icons/down-arrow.svg?raw';
	import { partyColors } from '$lib/partyColor';

	export let dels: Delegate[];
	export let isFinished = true;
	export let voteResultsPostFn: (
		page: number,
		voteResultFilter: VoteResultFilter
	) => Promise<VoteResultsWithMaxPage | HasError> = vote_results_per_page;
	export let voteResultsSearchPostFn: (
		page: number,
		search: string,
		voteResultFilter: VoteResultFilter
	) => Promise<VoteResultsWithMaxPage | HasError> = vote_results_by_search;
	export let storeIdx: number = 0;

	let currentVoteResultFilterStore = currentVoteResultFilterStores[storeIdx];
	$: currentVoteResultFilterStore = currentVoteResultFilterStores[storeIdx];

	let voteResults: VoteResultsWithMaxPage | null = null;
	let topics: UniqueTopic[] = [];
	let selectedTopics: Set<string> = new Set();
	let topicSearchValue = '';

	$: uniqueParties = [...new Set(dels.map((d) => d.party))].sort();

	// Track each party's filter preference: 'egal' = no filter, 'pro' = voted in favor, 'contra' = voted against
	type PartyFilterOption = 'egal' | 'pro' | 'contra';
	let partyFilterState: Record<string, PartyFilterOption> = {};

	// Initialize new parties with 'egal' (no filter)
	$: {
		for (const party of uniqueParties) {
			if (!(party in partyFilterState)) {
				partyFilterState[party] = 'egal';
			}
		}
	}

	// Convert filter state to API format (only include parties with active filters)
	$: partyVotesFilter = Object.entries(partyFilterState)
		.filter(([party, filterOption]) => filterOption !== 'egal')
		.map(([party, filterOption]) => ({
			party: party,
			infavor: filterOption === 'pro'
		}));


	// General Filter
	type GenericFilterGroup<T extends string | boolean> = {
        title: string;
        activeValue: T | undefined;
        options: { title: string; value: T | undefined }[];
    };
	let genericFilters: [
		GenericFilterGroup<boolean>,
		GenericFilterGroup<string>,
		GenericFilterGroup<boolean>,
		GenericFilterGroup<string>
	] = [
        {
            title: 'notwendige Mehrheit',
            activeValue: undefined,
            options: [
                { title: 'egal', value: undefined },
                { title: 'einfache Mehrheit', value: true },
                { title: '2/3 Mehrheit', value: false }
            ]
        },
        {
            title: 'Angenommen',
            activeValue: undefined,
            options: [
                { title: 'egal', value: undefined },
                { title: 'angenommen', value: 'a' },
                { title: 'abgelehnt', value: 'd' },
                { title: 'frühzeitig abgelehnt', value: 'p' }
            ]
        },
        {
            title: 'Abstimmung',
            activeValue: undefined,
            options: [
                { title: 'egal', value: undefined },
                { title: 'namentliche Abstimmung', value: true }
            ]
        },
        {
            title: 'Antragstyp',
            activeValue: undefined,
            options: [
                { title: 'egal', value: undefined },
                { title: 'Gesetz', value: 'Law' },
                { title: 'Entschließung', value: 'Resolution' },
                { title: 'Abänderung', value: 'Amendment' }
            ]
        }
    ];
	// CHRISTOPH REWORK END

	// get page number from query params
	const url = new URL(window.location.href);
	let page = parseInt(url.searchParams.get('page') || '1') || 1;

	const popupNamedVote: PopupSettings = {
		event: 'click',
		target: 'popupNamedVote',
		placement: 'bottom',
		closeQuery: '.listbox-item'
	};

	// CHRISTOPH REWORK
	// used for managing state of popup filter
	let isPartiesFilterOpen = false;
	const popupParties: PopupSettings = {
		event: 'click',
		target: 'popupParties',
		placement: 'bottom',
		state: (e) => {
			isPartiesFilterOpen = e.state;
		}
	};

	let isTopicFilterOpen = false;
	const popupTopics: PopupSettings = {
		event: 'click',
		target: 'popupTopics',
		placement: 'bottom',
		state: (e) => {
			isTopicFilterOpen = e.state;
		}
	};

	let isGenericFilterOpen = false;
	const popupGenericFilter: PopupSettings = {
		event: 'click',
		target: 'popupGenericFilter',
		placement: 'bottom',
		state: (e) => {
			isGenericFilterOpen = e.state;
		}
	};
	// CHRISTOPH REWORK END

	let currentlyUpdating = false;
	let selectedPeriod = 'all';

	const maybeStoredFilter = get(currentVoteResultFilterStore);
	if (maybeStoredFilter !== null) {
		if (maybeStoredFilter.simple_majority !== null)
			genericFilters[0].activeValue = maybeStoredFilter.simple_majority;
		// if (maybeStoredFilter.legis_period !== null) selectedPeriod = maybeStoredFilter.legis_period;
		if (maybeStoredFilter.accepted !== null) genericFilters[1].activeValue = maybeStoredFilter.accepted;
		if (maybeStoredFilter.is_named_vote !== null)
			genericFilters[2].activeValue = maybeStoredFilter.is_named_vote;
		if (maybeStoredFilter.vote_type !== null) genericFilters[3].activeValue = maybeStoredFilter.vote_type;
	}

	const loadVoteResults = async () => {
		currentlyUpdating = true;
		if (voteResults !== null) {
			voteResults.vote_results = [];
		}

		// accepted: 'a' (accepted), 'd' (declined), 'p' (pre-declined)
		// null "egal"
		let filter: VoteResultFilter | null = {
			is_finished: isFinished,
			is_named_vote: genericFilters[2].activeValue == undefined ? null : genericFilters[2].activeValue,
			accepted: genericFilters[1].activeValue == undefined ? null : genericFilters[1].activeValue,
			is_law: null,
			simple_majority:
				genericFilters[0].activeValue == undefined ? null : genericFilters[0].activeValue,
			legis_period: selectedPeriod == 'all' ? null : selectedPeriod,
			vote_type: genericFilters[3].activeValue === undefined ? null : genericFilters[3].activeValue,
			topics: selectedTopics.size > 0 ? [...selectedTopics] : null,
			is_urgent: null,
			party_votes: partyVotesFilter.length > 0 ? partyVotesFilter : null
		};

		currentVoteResultFilterStore.set(filter);

		// filter = null;

		if (searchValue) {
			const voteResultsSearch = errorToNull(
				await voteResultsSearchPostFn(page, searchValue, filter)
			);
			if (voteResultsSearch) voteResults = voteResultsSearch;
		} else {
			voteResults = errorToNull(await voteResultsSearchPostFn(page, searchValue, filter));
			// voteResults = errorToNull(await voteResultsPostFn(page - 1, filter));
		}
		currentlyUpdating = false;
	};
	onMount(async () => {
		update();
		// CHRISTOPH REWORK - Fetch available topics
		const fetchedTopics = errorToNull(await get_eurovoc_topics());
		if (fetchedTopics) {
			topics = fetchedTopics;
		}
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

	let searchValue = '';

	$: if (page || selectedPeriod || searchValue || partyVotesFilter || selectedTopics) {
		update();
	}

	$: if (searchValue) {
		page = 1;
	}
</script>

<!-- HERE IS THE HTML -->

<!-- TODO: Fetch API from Backend (waiting for backend)
 <span class="block text-base text-gray-800 mt-1 mb-2"
	>Abstimmungen aktualisiert am: {latestVoteDate}</span
> -->

<div class="flex mt-12">
	<!-- Search bar -->
	<div class="flex flex-grow h-10 rounded-xl border-[2px] border-gray-400">
		<div class="w-10 h-9 flex items-center justify-center text-gray-600">
			{@html searchIcon}
		</div>
		<input
			type="search"
			class="block w-full py-2 focus:outline-none bg-transparent placeholder:text-gray-600"
			placeholder="Suche..."
			bind:value={searchValue}
		/>
	</div>
	<!-- Filters -->
	<!-- Parteien Filter -->
	<div
		class="px-2 ml-2 rounded-xl bg-secondary-500 flex items-center justify-center gap-1"
		use:popup={popupParties}
	>
		<span class="text-white ml-1">Parteien</span>
		<div
			class="block text-white w-4 transition-transform duration-200"
			class:rotate-180={isPartiesFilterOpen}
		>
			{@html downArrowIcon}
		</div>
	</div>
	<!-- Parteien Filter PopUp-->
	<div
		class="bg-surface-50 border border-gray-300 px-6 py-4 z-10 shadow-lg rounded-xl w-72"
		data-popup="popupParties"
	>
		<div class="flex flex-col gap-2">
			{#each uniqueParties as party}
				<div class="flex items-center gap-2">
					<!-- Party Name and Color -->
					<div class="flex items-center gap-2">
						<div
							class="w-3 h-3 rounded-full"
							style="background-color: {partyColors.get(party) ?? '#ccc'};"
						></div>
						<span class="text-base text-gray-800 font-semibold">{party}</span>
					</div>
					<!-- Party Checkbox -->
					<div class="ml-auto flex items-center gap-1">
						<button
							class="px-2 py-1 rounded-lg text-sm cursor-pointer"
							class:bg-primary-300={partyFilterState[party] === 'pro'}
							on:click={() =>
								(partyFilterState[party] = partyFilterState[party] === 'pro' ? 'egal' : 'pro')}
						>
							Pro
						</button>
						<button
							class="px-2 py-1 rounded-lg text-sm cursor-pointer"
							class:bg-primary-300={partyFilterState[party] === 'egal'}
							on:click={() => (partyFilterState[party] = 'egal')}
						>
							Egal
						</button>
						<button
							class="px-2 py-1 rounded-lg text-sm cursor-pointer"
							class:bg-primary-300={partyFilterState[party] === 'contra'}
							on:click={() =>
								(partyFilterState[party] =
									partyFilterState[party] === 'contra' ? 'egal' : 'contra')}
						>
							Contra
						</button>
					</div>
				</div>
			{/each}
		</div>
		<div class="arrow bg-surface-50 border border-gray-300" />
	</div>
	<!-- Themen Filter -->
	<div
		class="px-2 ml-2 rounded-xl bg-secondary-500 flex items-center justify-center gap-1"
		use:popup={popupTopics}
	>
		<span class="text-white ml-1">Themen</span>
		<div
			class="block text-white w-4 transition-transform duration-200"
			class:rotate-180={isTopicFilterOpen}
		>
			{@html downArrowIcon}
		</div>
	</div>
	<!-- Themen Filter PopUp -->
	<div
		class="bg-surface-50 border border-gray-300 z-10 shadow-lg rounded-xl w-72"
		data-popup="popupTopics"
	>
		<!-- Search bar -->
		<div class="flex items-center gap-2 border-b border-gray-400 px-2 py-1">
			<div class="w-10 h-9 flex items-center justify-center text-gray-600">
				{@html searchIcon}
			</div>
			<input
				type="search"
				class="block w-full py-2 focus:outline-none bg-transparent placeholder:text-gray-600"
				placeholder="Suche nach Themen..."
				bind:value={topicSearchValue}
			/>
		</div>
		<div class="flex flex-col gap-1 max-h-72 overflow-y-auto px-3 py-2">
			<!-- Selected topics first -->
			{#each topics.filter((t) => selectedTopics.has(t.topic) && t.topic
						.toLowerCase()
						.includes(topicSearchValue.toLowerCase())) as topic}
				<button
					class="flex items-center gap-2 cursor-pointer"
					on:click={() => {
						selectedTopics.delete(topic.topic);
						selectedTopics = selectedTopics;
					}}
				>
					<div class="w-4 h-4 bg-primary-500 rounded-md"></div>
					<span class="text-gray-800 text-left text-sm font-semibold">{topic.topic}</span>
				</button>
			{/each}
			<!-- Unselected topics -->
			{#each topics.filter((t) => !selectedTopics.has(t.topic) && t.topic
						.toLowerCase()
						.includes(topicSearchValue.toLowerCase())) as topic}
				<button
					class="flex items-center gap-2 cursor-pointer"
					on:click={() => {
						selectedTopics.add(topic.topic);
						selectedTopics = selectedTopics;
					}}
				>
					<div class="w-4 h-4 border-[2px] border-primary-500 rounded-md"></div>
					<span class="text-gray-800 text-left text-sm">{topic.topic}</span>
				</button>
			{/each}
		</div>
		<div class="arrow bg-surface-50 border border-gray-300" />
	</div>
	<!-- Generic Filter -->
	<div
		class="px-2 ml-2 rounded-xl bg-secondary-500 flex items-center justify-center gap-1"
		use:popup={popupGenericFilter}
	>
		<span class="text-white ml-1">Filter</span>
		<div
			class="block text-white w-4 transition-transform duration-200"
			class:rotate-180={isGenericFilterOpen}
		>
			{@html downArrowIcon}
		</div>
	</div>
	<!-- Generic Filter PopUp -->
	<div
        class="bg-surface-50 border border-gray-300 px-6 pt-4 pb-5 z-10 shadow-lg rounded-xl w-82"
        data-popup="popupGenericFilter"
    >
        {#each genericFilters as group}
            <div class="mt-4 first:mt-0">
                <span class="text-gray-800 text-base font-semibold">{group.title}</span>
                <div class="flex w-fit text-sm border border-primary-300 rounded-lg gap-1">
                    {#each group.options as option}
                        <button
                            class="px-2 py-1 text-sm cursor-pointer rounded-lg"
                            class:bg-primary-300={group.activeValue === option.value}
                            on:click={() => { group.activeValue = option.value; update(); }}
                        >
                            <span class="text-nowrap">{option.title}</span>
                        </button>
                    {/each}
                </div>
            </div>
        {/each}
        <div class="arrow bg-surface-50 border border-gray-300" />
    </div>
</div>

<div>
	{#if voteResults}
		{#if voteResults.vote_results.length > 0}
			{#each voteResults.vote_results as voteResult}
				<VoteResultExpandableBar {dels} {voteResult} class="" />
			{/each}
		{:else if currentlyUpdating}
			{#each { length: 9 } as _}
				<ExpandablePlaceholder class="my-4" />
			{/each}
		{:else}
			Keine Abstimmungsergebnisse gefunden
		{/if}
		<div class="float-right">
			<Pagination bind:page maxPage={voteResults.max_page} />
		</div>
	{:else}
		{#each { length: 9 } as _}
			<ExpandablePlaceholder class="my-4" />
		{/each}
		<!-- <CenterPrograssRadial /> -->
	{/if}
</div>

<style>
</style>
