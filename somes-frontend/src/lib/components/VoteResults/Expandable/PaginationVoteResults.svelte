<script lang="ts">
	import type {
		VoteResultFilter,
		VoteResultsWithMaxPage,
		Party
	} from '$lib/types';
	import { onMount, untrack } from 'svelte';
	
	import { cachedAllLegisPeriods } from '$lib/caching/legis_periods';
	import VoteResultExpandableBar from './VoteResultExpandableBar.svelte';
	import { goto, pushState } from '$app/navigation';
	import Pagination from '$lib/components/Pagination.svelte';
	import { currentVoteResultFilterStores } from '$lib/stores/stores';
	import ExpandablePlaceholder from './Placeholders/ExpandablePlaceholder.svelte';
	import searchIcon from '$lib/assets/misc_icons/search-glass.svg?raw';
	import { Popover } from 'bits-ui';
	import { SvelteSet } from 'svelte/reactivity';
	import { page } from '$app/state';
	import FilterDropdown from '$lib/components/Filtering/FilterDropdown.svelte';
	import type { GenericFilterGroup } from '$lib/components/Filtering/types';
	import TopicsFilter from '$lib/components/Filtering/TopicsFilter.svelte';
	import GenericFilters from '$lib/components/Filtering/GenericFilters.svelte';

	interface Props {
		voteResults: VoteResultsWithMaxPage;
		partiesPerGp: Record<string, Party[]>;
		selectedGp: string | null;
		isFinished?: boolean;
		storeIdx?: number;
		showPartyFilter?: boolean;
		showReqMajorityFilter?: boolean;
		showAcceptedFilter?: boolean;
		showNamedVoteFilter?: boolean;
	}

	let {
		isFinished = true,
		voteResults,
		partiesPerGp,
		selectedGp,
		storeIdx = 0,
		showPartyFilter = false,
		showReqMajorityFilter = false,
		showAcceptedFilter = false,
		showNamedVoteFilter = false,
	}: Props = $props();

	let currentVoteResultFilterStore = $derived(currentVoteResultFilterStores[storeIdx]);

	// TOPIC FILTER
	let selectedTopics: SvelteSet<string> = $state(new SvelteSet());

	// PARTY FILTER - get all parties available in the request
	// let uniqueParties = $derived([...new Set(dels.map((d) => d.party))].sort());
	let uniqueParties = $derived.by(() => {
		if (selectedGp) {
			return partiesPerGp[selectedGp].sort((a, b) => {
				return b.fraction - a.fraction;
			});
		} else {
			const parties: Party[] = [];
			const namedParties = new Set();
			const keys = Object.keys(partiesPerGp).sort().reverse();
			keys.forEach((key) => {
				partiesPerGp[key].forEach((party) => {
					if (!namedParties.has(party.code)) {
						namedParties.add(party.code);
						parties.push(party);
					}
				});
			});
			return parties.sort((a, b) => {
				return b.fraction - a.fraction;
			});
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
				});
			}
		}
	});

	// Convert State to API format
	let partyVotesFilter = $derived(
		Object.entries(partyFilterState)
			.filter(([_, filterOption]) => filterOption !== 'egal')
			.map(([party, filterOption]) => ({
				party: party,
				infavor: filterOption === 'pro'
			}))
	);

	// GENERIC FILTER - storage and render format
	let genericFilters: [
		GenericFilterGroup<boolean>,
		GenericFilterGroup<string>,
		GenericFilterGroup<boolean>,
		GenericFilterGroup<string>,
		GenericFilterGroup<string>
	] = $derived([
		{
			title: 'notwendige Mehrheit',
			activeValue: undefined,
			hidden: !showReqMajorityFilter,
			options: [
				{ title: 'egal', value: undefined },
				{ title: 'einfache Mehrheit', value: true },
				{ title: '2/3 Mehrheit', value: false }
			]
		},
		{
			title: 'Angenommen',
			activeValue: undefined,
			hidden: !showAcceptedFilter,
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
			hidden: !showNamedVoteFilter,
			options: [
				{ title: 'egal', value: undefined },
				{ title: 'namentliche Abstimmung', value: true }
			]
		},
		{
			title: 'Antragstyp',
			activeValue: undefined,
			hidden: false,
			options: [
				{ title: 'egal', value: undefined },
				{ title: 'Gesetz', value: 'Law' },
				{ title: 'Entschließung', value: 'Resolution' },
				{ title: 'Abänderung', value: 'Amendment' }
			]
		},
		{
			title: 'Legislaturperiode',
			activeValue: 'XXVIII',
			hidden: false,
			options: [{ title: 'Alle', value: 'all' }]
		}
	]);

	// Variables to count active filters
	let activePartyFiltersCount = $derived(
		Object.values(partyFilterState).filter((v) => v !== 'egal').length
	);

	// PARTY, TOPIC, GENERIC filters - used for managing state of popup filter

	let isPartiesFilterOpen = $state(false);

	// Get and format updated_at date
	let updatedAt = $derived(
		voteResults?.updated_at
			? new Intl.DateTimeFormat('de-AT', {
					day: '2-digit',
					month: '2-digit',
					year: 'numeric'
				}).format(new Date(voteResults.updated_at))
			: 'Unbekannt'
	);

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
		let paramPage = nextUrl.searchParams.get('page');
		if (paramPage == null) {
			paramPage = '1';
		}

		nextUrl.search = '';
		if (paramPage) nextUrl.searchParams.set('page', paramPage);
		if (filter.is_named_vote !== null) {
			nextUrl.searchParams.set(
				'legislative_initiative[voted_by_name][eq]',
				filter.is_named_vote.toString()
			);
		}
		if (filter.accepted !== null) {
			nextUrl.searchParams.set('legislative_initiative[accepted][eq]', filter.accepted);
		}
		if (filter.vote_type.length > 0) {
			nextUrl.searchParams.set('legislative_initiative[voting][in][0]', filter.vote_type[0]);
		}

		if (filter.gps.length > 0) {
			nextUrl.searchParams.set('legislative_initiative[gp][in][0]', filter.gps[0]);
		}

		if (filter.simple_majority !== null) {
			nextUrl.searchParams.set(
				'legislative_initiative[requires_simple_majority][eq]',
				filter.simple_majority.toString()
			);
		}

		filter.party_votes?.forEach((partyVotes, i) => {
			nextUrl.searchParams.set(`party_votes[${i}][infavor]`, partyVotes.infavor.toString());
			nextUrl.searchParams.set(`party_votes[${i}][party]`, partyVotes.party);
		});

		nextUrl.searchParams.set('search', searchValue);

		filter.topics?.forEach((topic, i) => {
			nextUrl.searchParams.set(`eurovoc_topics[${i}][topic][cn]`, topic);
		});

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
		void selectedPeriod;
		void searchValue;
		void partyVotesFilter;
		void selectedTopics.size;
		void genericFilters[0].activeValue;
		void genericFilters[1].activeValue;
		void genericFilters[2].activeValue;
		void genericFilters[3].activeValue;
		void genericFilters[4].activeValue;
		untrack(update);
	});

	let visibleFilters = $derived(genericFilters.slice(0, 4));

	let searchValue = $state('');
</script>

<!-- HERE IS THE HTML -->

<span class="mb-2 ml-1 block text-base text-gray-800 dark:text-gray-300 sm:mt-1 sm:ml-0">
	Abstimmungen aktualisiert am: {updatedAt}
</span>

<div class="mt-12 md:flex">
	<!-- Search bar -->
	<div class="flex h-10 flex-grow rounded-xl border-[2px] border-gray-400">
		<div class="flex h-9 w-10 items-center justify-center text-gray-600">
			{@html searchIcon}
		</div>
		<input
			type="search"
			class="block w-full bg-transparent py-2 placeholder:text-gray-600 focus:outline-none"
			placeholder="Suche..."
			bind:value={searchValue}
		/>
	</div>
	<!-- Filter Buttons -->
	<!-- Parteien Filter -->
	<div class="mt-2 flex h-10 w-full md:mt-0 md:w-auto">
		{#if showPartyFilter}
			<Popover.Root bind:open={isPartiesFilterOpen}>
				<Popover.Trigger>
					<FilterDropdown title="Parteien" activefilterCount={activePartyFiltersCount} isOpen={isPartiesFilterOpen} />
				</Popover.Trigger>
				<Popover.Content sideOffset={8}>
					<div
						class="z-10 w-72 rounded-xl border border-gray-300 bg-surface-50 px-6 py-4 shadow-lg"
						data-popup="popupParties"
					>
						<div class="flex flex-col gap-2">
							{#each uniqueParties as party}
								<div class="flex items-center gap-2">
									<!-- Party Name and Color -->
									<div class="flex items-center gap-2">
										<div
											class="h-3 w-3 rounded-full"
											style="background-color: {party.color ?? '#ccc'};"
										></div>
										<span class="text-base font-semibold text-gray-800">{party.name}</span>
									</div>
									<!-- Party Checkbox -->
									<div class="ml-auto flex items-center gap-1">
										<button
											class="cursor-pointer rounded-lg px-2 py-1 text-sm"
											class:bg-primary-300={partyFilterState[party.name] === 'pro'}
											onclick={() =>
												(partyFilterState[party.name] =
													partyFilterState[party.name] === 'pro' ? 'egal' : 'pro')}
										>
											Pro
										</button>
										<button
											class="cursor-pointer rounded-lg px-2 py-1 text-sm"
											class:bg-primary-300={partyFilterState[party.name] === 'egal'}
											onclick={() => (partyFilterState[party.name] = 'egal')}
										>
											Egal
										</button>
										<button
											class="cursor-pointer rounded-lg px-2 py-1 text-sm"
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
						<Popover.Arrow class="rounded-sm fill-current stroke-gray-300 text-gray-300" />
					</div>
				</Popover.Content>
			</Popover.Root>
		{/if}
		<!-- Themen Filter -->
		<TopicsFilter bind:selectedTopics />
		<!-- Generic Filter -->
		<GenericFilters 
			genericFilters={visibleFilters} 	
			bind:legisPeriodFilter={genericFilters[4]} 
		/>
	</div>
</div>

<div>
	{#if voteResults}
		{#if voteResults.vote_results.length > 0}
			{#each voteResults.vote_results as voteResult}
				<VoteResultExpandableBar {voteResult} class="" />
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
