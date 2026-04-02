<script lang="ts">
	import type { VoteResultFilter, VoteResultsWithMaxPage, Party } from '$lib/types';
	import { onMount, untrack } from 'svelte';

	import { cachedAllLegisPeriods } from '$lib/caching/legis_periods';
	import VoteResultExpandableBar from './VoteResultExpandableBar.svelte';
	import { goto } from '$app/navigation';
	import Pagination from '$lib/components/Pagination.svelte';
	import { currentVoteResultFilterStores } from '$lib/stores/stores';
	import ExpandablePlaceholder from './Placeholders/ExpandablePlaceholder.svelte';
	import { Popover } from 'bits-ui';
	import { SvelteSet } from 'svelte/reactivity';
	import { page } from '$app/state';
	import FilterDropdown from '$lib/components/Filtering/FilterDropdown.svelte';
	import type { GenericFilterGroup } from '$lib/components/Filtering/types';
	import GenericFilters from '$lib/components/Filtering/GenericFilters.svelte';
	import SearchBar from '$lib/components/Filtering/SearchBar.svelte';
	import { convertVoteResultFilterToUrl } from './urlConversion';
	import { errorToNull, get_eurovoc_topics } from '$lib/api/api';
	import MultiValuesFilter from '$lib/components/Filtering/MultiValuesFilter.svelte';


	interface Props {
		voteResults: VoteResultsWithMaxPage | null;
		partiesPerGp: Record<string, Party[]>;
		selectedGp: string | null;
		isFinished?: boolean;
		storeIdx?: number;
		showPartyFilter?: boolean;
		showReqMajorityFilter?: boolean;
		showAcceptedFilter?: boolean;
		showNamedVoteFilter?: boolean;
		showIsUrgentFilter?: boolean;
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
		showIsUrgentFilter = false
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
		void legisPeriodFilter.activeValue;
		for (const party of uniqueParties) {
			if (!(party.name in partyFilterState)) {
				untrack(() => {
					partyFilterState[party.name] = 'egal';
				});
			}
		}
	});
	$effect(() => {
		void legisPeriodFilter.activeValue;
		untrack(() => {
			for (const party of uniqueParties) {
				partyFilterState[party.name] = 'egal';
			}
		});
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
		GenericFilterGroup<boolean>,
		GenericFilterGroup<string>
	] = $state([
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
			title: 'namentliche Abstimmung',
			activeValue: undefined,
			hidden: !showNamedVoteFilter,
			options: [
				{ title: 'egal', value: undefined },
				{ title: 'Ja', value: true },
				{ title: 'Nein', value: false }
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
				{ title: 'Abänderung', value: 'Amendment' },
				{ title: 'Bericht', value: 'Report' }
			]
		},
		{
			title: 'Dringlich',
			activeValue: undefined,
			hidden: !showIsUrgentFilter,
			advanced: true,
			options: [
				{ title: 'egal', value: undefined },
				{ title: 'Ja', value: true },
				{ title: 'Nein', value: false }
			]
		},
		{
			title: 'Datum',
			activeValue: undefined,
			hidden: false,
			advanced: true,
			id: 'dateRange',
			data: { dateFrom: '', dateTo: '' },
			options: []
		}
	]);

	let legisPeriodFilter = $state({
		title: 'Legislaturperiode',
		activeValue: 'XXVIII',
		hidden: false,
		options: [{ title: 'Alle', value: 'all' }]
	});

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

	const maybeStoredFilter = $derived(currentVoteResultFilterStore.value);
	onMount(() => {
		if (maybeStoredFilter !== null) {
			if (maybeStoredFilter.simple_majority !== null)
				genericFilters[0].activeValue = maybeStoredFilter.simple_majority;
			if (maybeStoredFilter.gps !== null) {
				if (maybeStoredFilter.gps.length > 0) {
					legisPeriodFilter.activeValue = maybeStoredFilter.gps[0];
				} else {
					legisPeriodFilter.activeValue = 'all';
				}
			}
			if (maybeStoredFilter.accepted !== null)
				genericFilters[1].activeValue = maybeStoredFilter.accepted;
			if (maybeStoredFilter.is_named_vote !== null)
				genericFilters[2].activeValue = maybeStoredFilter.is_named_vote;
			if (maybeStoredFilter.vote_type !== null && maybeStoredFilter.vote_type.length > 0)
				genericFilters[3].activeValue = maybeStoredFilter.vote_type[0];
			if (maybeStoredFilter.topics !== null) {
				selectedTopics = new SvelteSet(maybeStoredFilter.topics);
			}
			if (maybeStoredFilter.party_votes !== null) {
				maybeStoredFilter.party_votes.forEach((party) => {
					partyFilterState[party.party] = party.infavor ? 'pro' : 'contra';
				});
			}
			if (maybeStoredFilter.is_urgent !== null) {
				genericFilters[4].activeValue = maybeStoredFilter.is_urgent;
			}
			if (maybeStoredFilter.date_from) genericFilters[5].data!.dateFrom = maybeStoredFilter.date_from;
			if (maybeStoredFilter.date_to) genericFilters[5].data!.dateTo = maybeStoredFilter.date_to;
		}
	});

	const loadVoteResults = async () => {
		currentlyUpdating = true;
		if (voteResults !== null) {
			voteResults.vote_results = [];
		}

		// accepted: 'a' (accepted), 'd' (declined), 'p' (pre-declined)
		// null "egal"
		let filter: VoteResultFilter = {
			is_finished: isFinished,
			is_named_vote:
				genericFilters[2].activeValue == undefined ? null : genericFilters[2].activeValue,
			accepted: genericFilters[1].activeValue == undefined ? null : genericFilters[1].activeValue,
			simple_majority:
				genericFilters[0].activeValue == undefined ? null : genericFilters[0].activeValue,
			gps:
				legisPeriodFilter.activeValue == 'all' || legisPeriodFilter.activeValue === undefined
					? []
					: [legisPeriodFilter.activeValue],
			vote_type: genericFilters[3].activeValue === undefined ? [] : [genericFilters[3].activeValue],
			topics: selectedTopics.size > 0 ? [...selectedTopics] : null,
			is_urgent: genericFilters[4].activeValue === undefined ? null : genericFilters[4].activeValue,
			party_votes: partyVotesFilter.length > 0 ? partyVotesFilter : null,
			date_from: genericFilters[5].data?.dateFrom || null,
			date_to: genericFilters[5].data?.dateTo || null
		};

		const nextUrl = convertVoteResultFilterToUrl(
			filter,
			searchValue,
			new URL(page.url),
			isFinished
		);

		goto(nextUrl, {
			keepFocus: true,
			replaceState: true,
			noScroll: true
		});

		currentVoteResultFilterStore.value = filter;

		currentlyUpdating = false;
	};

	let topics: string[] = $state([]);

	onMount(async () => {
		update();

		// Generic filter - Legislative period
		const fetchedPeriods = await cachedAllLegisPeriods();
		if (fetchedPeriods) {
			legisPeriodFilter.options = [
				{ title: 'Alle', value: 'all' },
				...fetchedPeriods.map((p) => ({ title: p.gp, value: p.gp }))
			];
		}

		const eurovocTopics = errorToNull(await get_eurovoc_topics());
		if (eurovocTopics) {
			topics = eurovocTopics.map((topic) => topic.topic);
		}
	});

	const update = () => {
		loadVoteResults();
	};

	$effect(() => {
		void searchValue;
		void partyVotesFilter;
		void selectedTopics.size;
		for (let i = 0; i < genericFilters.length; i++) {
			void genericFilters[i].activeValue;
		}
		void legisPeriodFilter.activeValue;
		void genericFilters[5].data?.dateFrom;
		void genericFilters[5].data?.dateTo;
		untrack(update);
	});

	// used to set generic filter count when dateRange filter is active
	$effect(() => {
		genericFilters[5].activeValue = (genericFilters[5].data?.dateFrom || genericFilters[5].data?.dateTo) ? 'set' : undefined;
	});

	let searchValue = $state('');
</script>

<!-- HERE IS THE HTML -->

<span class="mb-2 ml-1 block text-base text-gray-800 sm:mt-1 sm:ml-0 dark:text-gray-300">
	Abstimmungen aktualisiert am: {updatedAt}
</span>

<div class="mt-7 md:flex">
	<!-- Search bar -->
	<SearchBar bind:searchValue />
	<!-- Filter Buttons -->
	<!-- Parteien Filter -->
	<div class="mt-2 flex h-10 w-full gap-2 text-xs sm:text-base md:mt-0 md:ml-2 md:w-auto">
		{#if showPartyFilter}
			<Popover.Root bind:open={isPartiesFilterOpen}>
				<Popover.Trigger
					class="flex h-full grow touch-manipulation items-center justify-center gap-1 rounded-xl bg-secondary-500 px-2 md:grow-0"
				>
					<FilterDropdown
						title="Parteien"
						activefilterCount={activePartyFiltersCount}
						isOpen={isPartiesFilterOpen}
					/>
				</Popover.Trigger>
				<Popover.Content sideOffset={8}>
					<div
						class="z-10 w-72 touch-manipulation rounded-xl border border-gray-300 bg-surface-50 px-6 py-4 shadow-lg dark:bg-surface-600"
						data-popup="popupParties"
					>
						<div class="flex flex-col gap-2">
							{#each uniqueParties as party (party.name)}
								<div class="flex items-center gap-2">
									<!-- Party Name and Color -->
									<div class="flex items-center gap-2">
										<div
											class="h-3 w-3 rounded-full"
											style="background-color: {party.color ?? '#ccc'};"
										></div>
										<span class="text-base font-semibold text-gray-800 dark:text-gray-200"
											>{party.name}</span
										>
									</div>
									<!-- Party Checkbox -->
									<div class="ml-auto flex items-center gap-1">
										<button
											class="cursor-pointer rounded-lg px-2 py-1 text-sm {partyFilterState[
												party.name
											] === 'pro'
												? 'bg-primary-300 dark:bg-primary-400'
												: ''}"
											onclick={() =>
												(partyFilterState[party.name] =
													partyFilterState[party.name] === 'pro' ? 'egal' : 'pro')}
										>
											Pro
										</button>
										<button
											class="cursor-pointer rounded-lg px-2 py-1 text-sm {partyFilterState[
												party.name
											] === 'egal'
												? 'bg-primary-300 dark:bg-primary-400'
												: ''}"
											onclick={() => (partyFilterState[party.name] = 'egal')}
										>
											Egal
										</button>
										<button
											class="cursor-pointer rounded-lg px-2 py-1 text-sm {partyFilterState[
												party.name
											] === 'contra'
												? 'bg-primary-300 dark:bg-primary-400'
												: ''}"
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
		<MultiValuesFilter title="Themen" bind:selectedValues={selectedTopics} values={topics} />
		<!-- Generic Filter -->
		{#snippet dateRangeSnippet()}
			<div class="mt-1 flex flex-col gap-3">
			<div class="flex gap-2 w-full justify-between">
				<div class="flex-1">
					<label for="date-from" class="block w-full text-sm font-semibold text-gray-600 dark:text-gray-50">Von</label>
					<input
						id="date-from"
						type="date"
						class="mt-1 w-full appearance-none rounded-lg border border-primary-300 dark:border-primary-400 bg-transparent px-2 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-primary-400"
						bind:value={genericFilters[5].data!.dateFrom}
					/>
				</div>
				<div class="flex-1">
					<label for="date-to" class="block w-full text-sm font-semibold text-gray-600 dark:text-gray-50">Bis</label>
					<input
						id="date-to"
						type="date"
						class="mt-1 w-full appearance-none rounded-lg border border-primary-300 dark:border-primary-400 bg-transparent px-2 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-primary-400"
						bind:value={genericFilters[5].data!.dateTo}
					/>
				</div>
			</div>
				{#if genericFilters[5].data?.dateFrom || genericFilters[5].data?.dateTo}
					<button
						class="cursor-pointer rounded-lg border border-primary-300 dark:border-primary-400 px-2 py-1 text-xs hover:bg-primary-300 dark:hover:bg-primary-400 md:text-sm"
						onclick={() => { genericFilters[5].data!.dateFrom = ''; genericFilters[5].data!.dateTo = ''; }}
					>
						Zurücksetzen
					</button>
				{/if}
			</div>
		{/snippet}
		<GenericFilters bind:genericFilters bind:legisPeriodFilter snippets={{ dateRange: dateRangeSnippet }} />
	</div>
</div>

<div>
	{#if voteResults}
		{#if voteResults.vote_results.length > 0}
			{#each voteResults.vote_results as voteResult (voteResult.id)}
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
		<!-- Fixes bug of not showing anything if no vote results are found -->
		{#if currentlyUpdating}
			{#each { length: 9 } as _}
				<ExpandablePlaceholder class="my-4" />
			{/each}
		{:else}
			Keine Abstimmungsergebnisse gefunden
		{/if}
		<!-- <CenterPrograssRadial /> -->
	{/if}
</div>
