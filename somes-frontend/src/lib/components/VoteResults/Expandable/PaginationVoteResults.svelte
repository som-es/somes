<script lang="ts">
	import { run } from 'svelte/legacy';

	import type {
		Delegate,
		VoteResultFilter,
		VoteResultsWithMaxPage,
		HasError,
		UniqueTopic,

		Party

	} from '$lib/types';
	import { onMount, untrack } from 'svelte';
	import {
		errorToNull,
		vote_results_by_search,
		vote_results_per_page,
		get_eurovoc_topics
	} from '$lib/api/api';
	import { cachedAllLegisPeriods } from '$lib/caching/legis_periods';
	import VoteResultExpandableBar from './VoteResultExpandableBar.svelte';
	import { goto, pushState } from '$app/navigation';
	import Pagination from '$lib/components/Pagination.svelte';
	import { currentVoteResultFilterStores } from '$lib/stores/stores';
	import ExpandablePlaceholder from './Placeholders/ExpandablePlaceholder.svelte';
	import searchIcon from '$lib/assets/misc_icons/search-glass.svg?raw';
	import downArrowIcon from '$lib/assets/misc_icons/down-arrow.svg?raw';
	import { Popover } from 'bits-ui';
	import { SvelteSet } from 'svelte/reactivity';
	import { page } from '$app/state';


	interface Props {
		voteResults: VoteResultsWithMaxPage;
		partiesPerGp: Record<string, Party[]>;
		selectedGp: string | null;
		dels: Delegate[];
		isFinished?: boolean;
		storeIdx?: number;
	}

	let {
		dels,
		isFinished = true,
		voteResults,
		partiesPerGp,
		selectedGp,
		storeIdx = 0
	}: Props = $props();

	let currentVoteResultFilterStore = $derived(currentVoteResultFilterStores[storeIdx]);

	// TOPIC FILTER
	let topics: UniqueTopic[] = $state([]);
	let selectedTopics: SvelteSet<string> = new SvelteSet();
	let topicSearchValue = $state('');

	// PARTY FILTER - get all parties available in the request
	// let uniqueParties = $derived([...new Set(dels.map((d) => d.party))].sort());
	let uniqueParties = $derived.by(() => {
		if (selectedGp) {
			return partiesPerGp[selectedGp].sort((a, b) => { return b.fraction - a.fraction})
		} else {
			const parties: Party[] = [];
			const namedParties = new Set();
			const keys = Object.keys(partiesPerGp).sort().reverse()
			keys.forEach(key => {
				partiesPerGp[key].forEach(party => {
					if (!namedParties.has(party.code)) {
						namedParties.add(party.code);
						parties.push(party)
					}
				})
			})
			return parties.sort((a, b) => { return b.fraction - a.fraction});
		}
	});

	// Track each party's filter preference: 'egal' = no filter, 'pro' = voted in favor, 'contra' = voted against
	type PartyFilterOption = 'egal' | 'pro' | 'contra';
	let partyFilterState: Record<string, PartyFilterOption> = $state({});

	// Initialize new parties with 'egal' (no filter)
	$effect(() => {
		for (const party of uniqueParties) {
			if (!(party.name in partyFilterState)) {
				untrack(() => {
					partyFilterState[party.name] = 'egal';
				})
			}
		}
	});

	// Convert State to API format
	let partyVotesFilter = $derived(Object.entries(partyFilterState)
		.filter(([_, filterOption]) => filterOption !== 'egal')
		.map(([party, filterOption]) => ({
			party: party,
			infavor: filterOption === 'pro'
		})));

	// GENERIC FILTER - storage and render format
	type GenericFilterGroup<T extends string | boolean> = {
		title: string;
		activeValue: T | undefined;
		options: { title: string; value: T | undefined }[];
	};
	let genericFilters: [
		GenericFilterGroup<boolean>,
		GenericFilterGroup<string>,
		GenericFilterGroup<boolean>,
		GenericFilterGroup<string>,
		GenericFilterGroup<string>
	] = $state([
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
		},
		{
			title: 'Legislaturperiode',
			activeValue: 'all',
			options: [{ title: 'Alle', value: 'all' }]
		}
	]);

	// Variables to count active filters
	let activePartyFiltersCount = $derived(Object.values(partyFilterState).filter((v) => v !== 'egal').length);
	let activeTopicFiltersCount = $derived(selectedTopics.size);
	let activeGenericFiltersCount = $derived(genericFilters.filter(
		(f) => f.activeValue !== undefined && f.activeValue !== 'all'
	).length);

	// PARTY, TOPIC, GENERIC filters - used for managing state of popup filter

	let isPartiesFilterOpen = false;
	let isTopicFilterOpen = false;
	let isGenericFilterOpen = false;

	// get page number from query params
	const url = new URL(window.location.href);

	// Get and format updated_at date
	let updatedAt = $derived(voteResults?.updated_at
		? new Intl.DateTimeFormat('de-AT', {
				day: '2-digit',
				month: '2-digit',
				year: 'numeric'
			}).format(new Date(voteResults.updated_at))
		: 'Unbekannt');

	// keep filters up to date
	let currentlyUpdating = $state(false);
	let selectedPeriod = 'all';

	const maybeStoredFilter = $derived(currentVoteResultFilterStore.value);
	onMount(() => {
		if (maybeStoredFilter !== null) {
			if (maybeStoredFilter.simple_majority !== null)
				genericFilters[0].activeValue = maybeStoredFilter.simple_majority;
			if (maybeStoredFilter.gps !== null && maybeStoredFilter.gps.length > 0)
				genericFilters[4].activeValue = maybeStoredFilter.gps[0];
			if (maybeStoredFilter.accepted !== null)
				genericFilters[1].activeValue = maybeStoredFilter.accepted;
			if (maybeStoredFilter.is_named_vote !== null)
				genericFilters[2].activeValue = maybeStoredFilter.is_named_vote;
			if (maybeStoredFilter.vote_type !== null && maybeStoredFilter.vote_type.length > 0)
				genericFilters[3].activeValue = maybeStoredFilter.vote_type[0];
		}
	});
	

	const loadVoteResults = async () => {
		currentlyUpdating = true;
		if (voteResults !== null) {
			voteResults.vote_results = [];
		}

		// accepted: 'a' (accepted), 'd' (declined), 'p' (pre-declined)
		// null "egal"
		let filter: VoteResultFilter | null = {
			is_finished: isFinished,
			is_named_vote:
				genericFilters[2].activeValue == undefined ? null : genericFilters[2].activeValue,
			accepted: genericFilters[1].activeValue == undefined ? null : genericFilters[1].activeValue,
			simple_majority:
				genericFilters[0].activeValue == undefined ? null : genericFilters[0].activeValue,
			gps:
				genericFilters[4].activeValue == 'all' || genericFilters[4].activeValue === undefined
					? []
					: [genericFilters[4].activeValue],
			vote_type: genericFilters[3].activeValue === undefined ? [] : [genericFilters[3].activeValue],
			topics: selectedTopics.size > 0 ? [...selectedTopics] : null,
			is_urgent: null,
			party_votes: partyVotesFilter.length > 0 ? partyVotesFilter : null
		};

		const nextUrl = new URL(page.url); 
		let paramPage = nextUrl.searchParams.get("page");
		if (paramPage == null) {
			paramPage = "1"
		}

		nextUrl.search = "";
		if (paramPage) nextUrl.searchParams.set("page", paramPage);
		if (filter.is_named_vote !== null) {
			nextUrl.searchParams.set("legislative_initiative[voted_by_name][eq]", filter.is_named_vote.toString());
		}
		if (filter.accepted !== null) {
			nextUrl.searchParams.set("legislative_initiative[accepted][eq]", filter.accepted);
		}
		if (filter.vote_type.length > 0) {
			nextUrl.searchParams.set("legislative_initiative[voting][in][0]", filter.vote_type[0])
		}

		if (filter.gps.length > 0) {
			nextUrl.searchParams.set("legislative_initiative[gp][in][0]", filter.gps[0])
		}

		if (filter.simple_majority !== null) {
			nextUrl.searchParams.set("legislative_initiative[requires_simple_majority][eq]", filter.simple_majority.toString())
		}

		filter.party_votes?.forEach((partyVotes, i) => {
			nextUrl.searchParams.set(`party_votes[${i}][infavor]`, partyVotes.infavor.toString());
			nextUrl.searchParams.set(`party_votes[${i}][party]`, partyVotes.party);
		})

		nextUrl.searchParams.set("search", searchValue);

		filter.topics?.forEach((topic, i) => {
			nextUrl.searchParams.set(`eurovoc_topics[${i}][topic][cn]`, topic)
		})

		goto(nextUrl, { 
			keepFocus: true,
			replaceState: true,
			noScroll: true
		});

		currentVoteResultFilterStore.value = filter;

		currentlyUpdating = false;
	};

	onMount(async () => {
		update();
		// TOPIC FILTER - Fetch available topics
		const fetchedTopics = errorToNull(await get_eurovoc_topics());
		if (fetchedTopics) {
			topics = fetchedTopics;
		}

		// Generic filter - Legislative period
		const fetchedPeriods = await cachedAllLegisPeriods();
		if (fetchedPeriods) {
			genericFilters[4].options = [
				{ title: 'Alle', value: 'all' },
				...fetchedPeriods.map((p) => ({ title: p.gp, value: p.gp }))
			];
		}
	});

	const update = () => {
		loadVoteResults();
	};

	$effect(() => {
		void selectedPeriod
		void searchValue
		void partyVotesFilter
		void selectedTopics.size
		void genericFilters[0].activeValue
		void genericFilters[1].activeValue
		void genericFilters[2].activeValue
		void genericFilters[3].activeValue
		void genericFilters[4].activeValue
		untrack(update)
	});

	let searchValue = $state('');

</script>

<!-- HERE IS THE HTML -->

<span class="block text-base text-gray-800 ml-1 sm:ml-0 sm:mt-1 mb-2">
	Abstimmungen aktualisiert am: {updatedAt}
</span>

<div class="md:flex mt-12">
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
	<!-- Filter Buttons -->
	<!-- Parteien Filter -->
	<div class="flex w-full md:w-auto h-10 mt-2 md:mt-0">
		<Popover.Root>
			<Popover.Trigger>
				<div
					class="grow md:grow-0 px-2 md:ml-2 rounded-xl bg-secondary-500 flex items-center justify-center gap-1"
				>
					{#if activePartyFiltersCount > 0}
						<div
							class="flex items-center justify-center w-5 h-5 text-xs font-semibold text-white border rounded-full"
						>
							{activePartyFiltersCount}
						</div>
					{/if}
					<span class="text-white ml-1">Parteien</span>
					<div
						class="block text-white w-4 transition-transform duration-200"
						class:rotate-180={isPartiesFilterOpen}
					>
						{@html downArrowIcon}
					</div>
				</div>
			</Popover.Trigger>
			<Popover.Portal>
				<Popover.Content>
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
											style="background-color: {party.color ?? '#ccc'};"
										></div>
										<span class="text-base text-gray-800 font-semibold">{party.name}</span>
									</div>
									<!-- Party Checkbox -->
									<div class="ml-auto flex items-center gap-1">
										<button
											class="px-2 py-1 rounded-lg text-sm cursor-pointer"
											class:bg-primary-300={partyFilterState[party.name] === 'pro'}
											onclick={() =>
												(partyFilterState[party.name] = partyFilterState[party.name] === 'pro' ? 'egal' : 'pro')}
										>
											Pro
										</button>
										<button
											class="px-2 py-1 rounded-lg text-sm cursor-pointer"
											class:bg-primary-300={partyFilterState[party.name] === 'egal'}
											onclick={() => (partyFilterState[party.name] = 'egal')}
										>
											Egal
										</button>
										<button
											class="px-2 py-1 rounded-lg text-sm cursor-pointer"
											class:bg-primary-300={partyFilterState[party.name] === 'contra'}
											onclick={() =>
												(partyFilterState[party.name] =
													partyFilterState[party.name] === 'contra' ? 'egal' : 'contra')}
										>
											Contra
										</button>
									</div>
								</div>
							{/each}
						</div>
						<div class="arrow bg-surface-50 border border-gray-300"></div>
					</div>
				</Popover.Content>
			</Popover.Portal>
		</Popover.Root>
		<!-- Themen Filter -->
		<Popover.Root>
			<Popover.Trigger>
				<div
					class="grow md:grow-0 px-2 ml-2 rounded-xl bg-secondary-500 flex items-center justify-center gap-1"
				>
					{#if activeTopicFiltersCount > 0}
						<div
							class="flex items-center justify-center w-5 h-5 text-xs font-semibold text-white border rounded-full"
						>
							{activeTopicFiltersCount}
						</div>
					{/if}
					<span class="text-white ml-1">Themen</span>
					<div
						class="block text-white w-4 transition-transform duration-200"
						class:rotate-180={isTopicFilterOpen}
					>
						{@html downArrowIcon}
					</div>
				</div>
			</Popover.Trigger>
			<Popover.Portal>
				<Popover.Content>
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
									onclick={() => {
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
									onclick={() => {
										selectedTopics.add(topic.topic);
										selectedTopics = selectedTopics;
									}}
								>
									<div class="w-4 h-4 border-[2px] border-primary-500 rounded-md"></div>
									<span class="text-gray-800 text-left text-sm">{topic.topic}</span>
								</button>
							{/each}
						</div>
						<div class="arrow bg-surface-50 border border-gray-300"></div>
					</div>
				</Popover.Content>
			</Popover.Portal>
		 </Popover.Root>
		<!-- Generic Filter -->
		<Popover.Root>
			<Popover.Trigger>
				<div
					class="grow md:grow-0 px-2 ml-2 rounded-xl bg-secondary-500 flex items-center justify-center gap-1"
				>
					{#if activeGenericFiltersCount > 0}
						<div
							class="flex items-center justify-center w-5 h-5 text-xs font-semibold text-white border rounded-full"
						>
							{activeGenericFiltersCount}
						</div>
					{/if}
					<span class="text-white ml-1">Filter</span>
					<div
						class="block text-white w-4 transition-transform duration-200"
						class:rotate-180={isGenericFilterOpen}
					>
						{@html downArrowIcon}
					</div>
				</div>
			</Popover.Trigger>
			<Popover.Portal>
				<Popover.Content>
	<div
		class="bg-surface-50 border border-gray-300 px-5 md:px-6 pt-4 pb-5 z-10 shadow-lg rounded-xl w-auto"
		data-popup="popupGenericFilter"
	>
		{#each genericFilters.slice(0, 4) as group}
			<div class="mt-4 first:mt-0">
				<span class="text-gray-800 text-base font-semibold">{group.title}</span>
				<div class="flex w-fit text-sm border border-primary-300 rounded-lg gap-1">
					{#each group.options as option}
						<button
							class="px-2 py-1 text-xs md:text-sm cursor-pointer rounded-lg"
							class:bg-primary-300={group.activeValue === option.value}
							onclick={() => {
								group.activeValue = option.value;
							}}
						>
							<span class="text-nowrap">{option.title}</span>
						</button>
					{/each}
				</div>
			</div>
		{/each}
		<div class="mt-4 first:mt-0">
			<span class="text-gray-800 text-base font-semibold">{genericFilters[4].title}</span>
			<div class="flex flex-wrap text-sm gap-1 w-72">
				{#each genericFilters[4].options as option}
					<button
						class="px-2 py-1 text-xs md:text-sm cursor-pointer rounded-lg border border-primary-300"
						class:bg-primary-300={genericFilters[4].activeValue === option.value}
						onclick={() => {
							genericFilters[4].activeValue = option.value;
						}}
					>
						<span class="text-nowrap">{option.title}</span>
					</button>
				{/each}
			</div>
		</div>
		<div class="arrow bg-surface-50 border border-gray-300"></div>
	</div>
				</Popover.Content>
			</Popover.Portal>
		</Popover.Root>
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
			<Pagination maxPage={voteResults.max_page} />
		</div>
	{:else}
		{#each { length: 9 } as _}
			<ExpandablePlaceholder class="my-4" />
		{/each}
		<!-- <CenterPrograssRadial /> -->
	{/if}
</div>
