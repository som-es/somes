<script lang="ts">
	import type {
		VoteResultFilter,
		VoteResultsWithMaxPage,
		Party,
		PartyStates,
		UniqueTopic
	} from '$lib/types';
	import { onMount, untrack } from 'svelte';

	import { cachedAllLegisPeriods } from '$lib/caching/legis_periods';
	import VoteResultExpandableBar from './VoteResultExpandableBar.svelte';
	import { goto } from '$app/navigation';
	import Pagination from '$lib/components/Pagination.svelte';
	import { currentVoteResultFilterStores } from '$lib/stores/stores';
	import ExpandablePlaceholder from './Placeholders/ExpandablePlaceholder.svelte';
	import { Popover, Select, ToggleGroup } from 'bits-ui';
	import upDownArrowIcon from '$lib/assets/misc_icons/up-down-arrow.svg?raw';
	import checkmark_small from '$lib/assets/misc_icons/checkmark_small.svg?raw';
	import { SvelteSet } from 'svelte/reactivity';
	import { page } from '$app/state';
	import FilterDropdown from '$lib/components/Filtering/FilterDropdown.svelte';
	import { type GenericFilterGroup } from '$lib/components/Filtering/types';
	import GenericFilters from '$lib/components/Filtering/GenericFilters.svelte';
	import SearchBar from '$lib/components/Filtering/SearchBar.svelte';
	import { convertVoteResultFilterToUrl } from './urlConversion';
	import { errorToNull, get_eurovoc_topics } from '$lib/api/api';
	import { t } from '$lib/i18n/i18n.svelte';
	import { localeStore } from '$lib/i18n/i18n.svelte';
	import DateRangeSnippet from '$lib/components/Filtering/GenericFilterSnippets/DataRangeSnippet.svelte';
	import FilterGroup from '$lib/components/Filtering/FilterGroup.svelte';
	import { cachedUserTopics } from '$lib/caching/user_topics_cache.svelte';
	import TopicFilter from '$lib/components/Filtering/TopicFilter.svelte';
	import SortPopover from '$lib/components/Filtering/SortPopover.svelte';
	import { getParliament } from '$lib/api/parliament';
	import { createFilterGroup } from '$lib/components/Filtering/filterGroup.svelte';

	interface Props {
		voteResults: VoteResultsWithMaxPage | null;
		partiesPerGp: Record<string, Party[]>;
		coalitionPartiesPerGp: Record<string, PartyStates>;
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
		coalitionPartiesPerGp,
		selectedGp,
		storeIdx = 0,
		showPartyFilter = false,
		showReqMajorityFilter = false,
		showAcceptedFilter = false,
		showNamedVoteFilter = false,
		showIsUrgentFilter = false
	}: Props = $props();

	let currentVoteResultFilterStore = $derived(currentVoteResultFilterStores[storeIdx]);

	const isEu = getParliament() === 'eu';

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

	let selectedIssuerPartiesObjects = $derived(
		uniqueParties.filter((p) => selectedIssuerParties.includes(p.name))
	);

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

	let currentPage: number | undefined = $state(undefined);

	let issuerAssociation: GenericFilterGroup<boolean> = $state({
		title: t('filter.camp'),
		activeValue: undefined,
		hidden: isEu,
		disabledText: t('filter.camp.disabledText'),
		options: [
			{ title: t('filterOption.any'), value: undefined },
			{ title: t('filter.gov'), value: true },
			{ title: t('filter.opp'), value: false }
		]
	});

	// ISSUER PARTIES FILTER
	let selectedIssuerParties: string[] = $state([]);
	$effect(() => {
		if (!selectedGp) {
			selectedIssuerParties = [];
			return;
		}

		if (issuerAssociation.activeValue !== undefined) {
			const partyStates = coalitionPartiesPerGp[selectedGp];
			const activeParties = issuerAssociation.activeValue
				? partyStates.coalition_parties
				: partyStates.opposition_parties;

			selectedIssuerParties = activeParties.map((party) => party.name);
		}
	});

	function matchesSelection(selection: string[], matchWith: string[]) {
		if (selection.length != matchWith.length) {
			return false;
		}
		for (let idx = 0; idx < selection.length; idx++) {
			if (!matchWith.find((val) => val == selection[idx])) return false;
		}
		return true;
	}

	$effect(() => {
		if (!selectedGp) {
			issuerAssociation.disabled = true;
			return;
		} else {
			issuerAssociation.disabled = false;
		}

		if (selectedIssuerParties.length == 0) {
			return;
		}

		const partyStates = coalitionPartiesPerGp[selectedGp];
		const coalitionParties = partyStates.coalition_parties.map((party) => party.name);
		const oppositionParties = partyStates.opposition_parties.map((party) => party.name);
		if (matchesSelection(coalitionParties, selectedIssuerParties)) {
			issuerAssociation.activeValue = true;
		} else if (matchesSelection(oppositionParties, selectedIssuerParties)) {
			issuerAssociation.activeValue = false;
		} else {
			issuerAssociation.activeValue = undefined;
		}
	});

	// GENERIC FILTER - storage and render format
	let genericFilters: [
		GenericFilterGroup<boolean>,
		GenericFilterGroup<string>,
		GenericFilterGroup<boolean>,
		GenericFilterGroup<string>,
		GenericFilterGroup<boolean>,
		GenericFilterGroup<string>,
		GenericFilterGroup<string>,
		GenericFilterGroup<boolean>
	] = $state([
		createFilterGroup<boolean>({
			title: () => t('filter.necessaryMajority'),
			hidden: () => !showReqMajorityFilter || isEu,
			options: () => [
				{ title: t('filterOption.any'), value: undefined },
				{ title: t('filterOption.simpleMajority'), value: true },
				{ title: t('filterOption.twoThirdsMajority'), value: false }
			]
		}),
		createFilterGroup<string>({
			title: () => t('filter.accepted'),
			hidden: () => !showAcceptedFilter || isEu,
			options: () => [
				{ title: t('filterOption.any'), value: undefined },
				{ title: t('filterOption.acceptedYes'), value: 'a' },
				{ title: t('filterOption.acceptedNo'), value: 'd' },
				{ title: t('filterOption.acceptedEarlyRejected'), value: 'p' }
			]
		}),
		createFilterGroup<boolean>({
			title: () => t('filter.namedVote'),
			hidden: () => !showNamedVoteFilter || isEu,
			options: () => [
				{ title: t('filterOption.any'), value: undefined },
				{ title: t('filterOption.yes'), value: true },
				{ title: t('filterOption.no'), value: false }
			]
		}),
		createFilterGroup<string>({
			title: () => t('filter.motionType'),
			hidden: () => false,
			options: () => [
				{ title: t('filterOption.any'), value: undefined },
				{ title: t('filterOption.motionLaw'), value: 'Law' },
				{ title: t('filterOption.motionResolution'), value: 'Resolution' },
				...(isEu ? [] : [{ title: t('filterOption.motionAmendment'), value: 'Amendment' }]),
				{ title: t('filterOption.motionReport'), value: 'Report' }
			]
		}),
		createFilterGroup<boolean>({
			title: () => t('filter.urgent'),
			hidden: () => !showIsUrgentFilter || isEu,
			advanced: true,
			options: () => [
				{ title: t('filterOption.any'), value: undefined },
				{ title: t('filterOption.yes'), value: true },
				{ title: t('filterOption.no'), value: false }
			]
		}),
		createFilterGroup<string>({
			title: () => t('filter.date'),
			hidden: () => false,
			advanced: true,
			id: 'dateRange',
			data: { dateFrom: '', dateTo: '' },
			options: () => []
		}),
		createFilterGroup<string>({
			title: () => t('filter.issuedBy'),
			hidden: () => isEu,
			advanced: false,
			id: 'issuerParties',
			options: () => []
		}),
		createFilterGroup<boolean>({
			title: () => t('filter.fromGovernment'),
			hidden: () => isEu,
			advanced: true,
			options: () => [
				{ title: t('filterOption.any'), value: undefined },
				{ title: t('filterOption.yes'), value: true },
				{ title: t('filterOption.no'), value: false }
			]
		})
	]);

	let legisPeriodFilter = $state({
		title: t('filter.legislaturePeriod'),
		activeValue: 'all',
		hidden: false,
		options: [{ title: t('filterOption.all'), value: 'all' }]
	});

	// Variables to count active filters
	let activePartyFiltersCount = $derived(
		Object.values(partyFilterState).filter((v) => v !== 'egal').length
	);

	// PARTY, TOPIC, GENERIC filters - used for managing state of popup filter

	let isPartiesFilterOpen = $state(false);

	// Get and format updated_at date
	let updatedAt = $derived.by(() => {
		const locale = localeStore.value === 'de' ? 'de-AT' : 'en-AT';
		return voteResults?.updated_at
			? new Intl.DateTimeFormat(locale, {
					day: '2-digit',
					month: '2-digit',
					year: 'numeric'
				}).format(new Date(voteResults.updated_at))
			: t('date.unknown');
	});

	// keep filters up to date
	let currentlyUpdating = $state(false);

	const maybeStoredFilter = $derived(currentVoteResultFilterStore.value);
	onMount(() => {
		if (maybeStoredFilter !== null) {
			if (maybeStoredFilter.simple_majority !== null && !genericFilters[0].hidden)
				genericFilters[0].activeValue = maybeStoredFilter.simple_majority;
			if (maybeStoredFilter.gps !== null) {
				if (maybeStoredFilter.gps.length > 0) {
					legisPeriodFilter.activeValue = maybeStoredFilter.gps[0];
				} else {
					legisPeriodFilter.activeValue = 'all';
				}
			}
			if (maybeStoredFilter.accepted !== null && !genericFilters[1].hidden)
				genericFilters[1].activeValue = maybeStoredFilter.accepted;
			if (maybeStoredFilter.is_named_vote !== null && !genericFilters[2].hidden)
				genericFilters[2].activeValue = maybeStoredFilter.is_named_vote;
			if (
				maybeStoredFilter.vote_type !== null &&
				maybeStoredFilter.vote_type.length > 0 &&
				genericFilters[3].options.some((o) => o.value === maybeStoredFilter.vote_type[0])
			)
				genericFilters[3].activeValue = maybeStoredFilter.vote_type[0];
			if (maybeStoredFilter.topics !== null) {
				selectedTopics = new SvelteSet(maybeStoredFilter.topics);
			}
			if (maybeStoredFilter.party_votes !== null) {
				maybeStoredFilter.party_votes.forEach((party) => {
					partyFilterState[party.party] = party.infavor ? 'pro' : 'contra';
				});
			}
			if (maybeStoredFilter.is_urgent !== null && !genericFilters[4].hidden) {
				genericFilters[4].activeValue = maybeStoredFilter.is_urgent;
			}
			if (maybeStoredFilter.date_from)
				genericFilters[5].data!.dateFrom = maybeStoredFilter.date_from;
			if (maybeStoredFilter.date_to) genericFilters[5].data!.dateTo = maybeStoredFilter.date_to;
			if (maybeStoredFilter.issuer_parties !== null && !genericFilters[6].hidden) {
				selectedIssuerParties = [...maybeStoredFilter.issuer_parties];
			}
			if (maybeStoredFilter.issuer_association !== null && !issuerAssociation.hidden) {
				issuerAssociation.activeValue = maybeStoredFilter.issuer_association;
			}
			if (maybeStoredFilter.is_from_governemnt !== null && !genericFilters[7].hidden) {
				genericFilters[7].activeValue = maybeStoredFilter.is_from_governemnt;
			}
			if (maybeStoredFilter.page !== null) {
				currentPage = maybeStoredFilter.page;
			}
		}
	});

	const convertAndStoreFilter = () => {
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
			date_to: genericFilters[5].data?.dateTo || null,
			issuer_parties: selectedIssuerParties.length > 0 ? selectedIssuerParties : null,
			is_from_governemnt:
				genericFilters[7].activeValue === undefined ? null : genericFilters[7].activeValue,
			issuer_association:
				issuerAssociation.activeValue === undefined ? null : issuerAssociation.activeValue,
			page: currentPage ?? null
		};

		currentVoteResultFilterStore.value = filter;
		return filter;
	};

	const loadVoteResults = async () => {
		currentlyUpdating = true;
		if (voteResults !== null) {
			voteResults.vote_results = [];
		}

		// accepted: 'a' (accepted), 'd' (declined), 'p' (pre-declined)
		// null "egal"

		const filter = convertAndStoreFilter();

		const nextUrl = convertVoteResultFilterToUrl(
			filter,
			searchValue,
			new URL(page.url),
			isFinished,
			sortOrder
		);

		goto(nextUrl, {
			keepFocus: true,
			replaceState: true,
			noScroll: true
		});

		currentlyUpdating = false;
	};

	let topics: string[] = $state([]);
	let userTopics: UniqueTopic[] | null = $state(null);
	onMount(async () => {
		update();

		userTopics = await cachedUserTopics();
		// Generic filter - Legislative period
		const fetchedPeriods = await cachedAllLegisPeriods();
		if (fetchedPeriods) {
			legisPeriodFilter.options = [
				{ title: t('filterOption.all'), value: 'all' },
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
		void sortOrder;
		void partyVotesFilter;
		void selectedTopics.size;
		void selectedIssuerParties.length;
		for (let i = 0; i < genericFilters.length; i++) {
			void genericFilters[i].activeValue;
		}
		void legisPeriodFilter.activeValue;
		void genericFilters[5].data?.dateFrom;
		void genericFilters[5].data?.dateTo;
		untrack(update);
	});

	$effect(() => {
		if (currentPage) {
			untrack(convertAndStoreFilter);
		}
	});

	// used to set generic filter count when dateRange filter is active
	$effect(() => {
		genericFilters[5].activeValue =
			genericFilters[5].data?.dateFrom || genericFilters[5].data?.dateTo ? 'set' : undefined;
	});

	// used to set generic filter count when issuer parties filter is active
	$effect(() => {
		genericFilters[6].activeValue = selectedIssuerParties.length > 0 ? 'set' : undefined;
	});

	let searchValue = $state('');
	let sortOrder: 'relevance' | 'Desc' | 'Asc' = $state('relevance');
</script>

<!-- HERE IS THE HTML -->

<span class="mb-2 ml-1 block text-base text-gray-800 sm:mt-1 sm:ml-0 dark:text-gray-300">
	{t('pagination.votesUpdated')}
	{updatedAt}
</span>

<div class="mt-7 md:flex">
	<!-- Search bar with inline sort trigger -->
	<SearchBar bind:searchValue>
		{#snippet rightSlot()}
			{#if searchValue.length > 0}
				<SortPopover bind:sortOrder />
			{/if}
		{/snippet}
	</SearchBar>

	<!-- Filter Buttons -->
	<!-- Parteien Filter -->
	<div class="mt-2 flex h-10 w-full gap-2 text-xs sm:text-base md:mt-0 md:ml-2 md:w-auto">
		{#if showPartyFilter}
			<Popover.Root bind:open={isPartiesFilterOpen}>
				<Popover.Trigger
					class="flex h-full grow touch-manipulation items-center justify-center gap-1 rounded-xl bg-secondary-500 px-2 md:grow-0"
				>
					<FilterDropdown
						title={t('filter.parties')}
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
											{t('voteHistory.filter.pro')}
										</button>
										<button
											class="cursor-pointer rounded-lg px-2 py-1 text-sm {partyFilterState[
												party.name
											] === 'egal'
												? 'bg-primary-300 dark:bg-primary-400'
												: ''}"
											onclick={() => (partyFilterState[party.name] = 'egal')}
										>
											{t('voteHistory.filter.any')}
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
											{t('voteHistory.filter.contra')}
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
		<TopicFilter bind:selectedTopics {topics} />
		<!-- Generic Filter -->
		{#snippet dateRangeSnippet()}
			<DateRangeSnippet
				bind:dateFrom={genericFilters[5].data!.dateFrom}
				bind:dateTo={genericFilters[5].data!.dateTo}
			/>
		{/snippet}
		{#snippet issuerPartiesSnippet()}
			<div class="mt-4 first:mt-0">
				<div class="flex flex-row items-center justify-center gap-3">
					<!-- 3-State Switch for Government / Any / Opposition -->
					<FilterGroup bind:group={issuerAssociation} />

					<div class="flex w-full flex-col">
						<span class="text-base font-semibold text-gray-800 dark:text-gray-50"
							>{t('voteHistory.filter.submittedBy')}</span
						>
						<Select.Root
							type="multiple"
							bind:value={selectedIssuerParties}
							onValueChange={() => {
								issuerAssociation.activeValue = undefined;
							}}
							items={uniqueParties.map((p) => ({ value: p.name, label: p.name }))}
						>
							<Select.Trigger
								class="mt-1 flex w-full touch-manipulation items-center justify-between gap-1 rounded-xl border-2 border-primary-300 px-2 py-1 text-sm dark:border-primary-400"
							>
								<div class="flex items-center gap-2">
									{#each selectedIssuerPartiesObjects.slice(0, 1) as party}
										<div
											class="h-3 w-3 rounded-full"
											style="background-color: {party.color};"
										></div>
										<span class="truncate">{party.name}</span>
									{/each}
									{#if selectedIssuerParties.length > 1}
										<span class="truncate"
											>+{selectedIssuerParties.length - 1}
											{t('delegates.morePeriods').replace('+{count} weitere', '')}</span
										>
									{/if}
									{#if selectedIssuerParties.length === 0}
										<span>{t('vote_result.allParties')}</span>
									{/if}
								</div>
								<span class="mr-2 block w-4 [&>svg]:fill-primary-400 [&>svg]:stroke-primary-400"
									>{@html upDownArrowIcon}</span
								>
							</Select.Trigger>
							<Select.Portal>
								<Select.Content
									class="z-900 max-h-60 w-[200px] overflow-hidden rounded-xl border border-gray-200 bg-surface-100 shadow-lg dark:bg-surface-500"
									align="start"
								>
									<Select.Viewport class="p-1">
										{#each uniqueParties as party}
											<Select.Item
												class="flex h-10 w-full cursor-pointer justify-between rounded-lg py-3 pr-1.5 pl-3 text-sm transition-all duration-75 outline-none select-none data-highlighted:bg-gray-100 dark:data-highlighted:bg-gray-400"
												value={party.name}
												label={party.name}
											>
												{#snippet children({ selected })}
													<div class="flex items-center gap-2">
														<div
															class="h-3 w-3 rounded-full"
															style="background-color: {party.color};"
														></div>
														{party.name}
													</div>
													{#if selected}
														<div class="ml-auto h-4 stroke-black dark:stroke-white">
															{@html checkmark_small}
														</div>
													{/if}
												{/snippet}
											</Select.Item>
										{/each}
									</Select.Viewport>
								</Select.Content>
							</Select.Portal>
						</Select.Root>
					</div>
				</div>
			</div>
		{/snippet}
		<GenericFilters
			bind:genericFilters
			bind:legisPeriodFilter
			snippets={{ dateRange: dateRangeSnippet, issuerParties: issuerPartiesSnippet }}
		/>
	</div>
</div>

<div class="mt-5 flex flex-col gap-5">
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
			{t('pagination.noResults')}
		{/if}
		<div class="flex justify-between">
			<div></div>
			<Pagination bind:currentPage maxPage={voteResults.max_page} />
		</div>
	{:else}
		<!-- Fixes bug of not showing anything if no vote results are found -->
		{#if currentlyUpdating}
			{#each { length: 9 } as _}
				<ExpandablePlaceholder class="my-4" />
			{/each}
		{:else}
			{t('pagination.noResults')}
		{/if}
		<!-- <CenterPrograssRadial /> -->
	{/if}
</div>
