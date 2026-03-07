<script lang="ts">
	import type { Delegate, VoteResultFilter, VoteResultsWithMaxPage, HasError } from '$lib/types';
	import { onMount } from 'svelte';
	import { errorToNull, vote_results_by_search, vote_results_per_page } from '$lib/api/api';
	import VoteResultExpandableBar from './VoteResultExpandableBar.svelte';
	import { pushState } from '$app/navigation';
	import Pagination from '$lib/components/Pagination.svelte';
	import { type PopupSettings } from '@skeletonlabs/skeleton';
	import { currentVoteResultFilterStores } from '$lib/stores/stores';
	import { get } from 'svelte/store';
	import ExpandablePlaceholder from './Placeholders/ExpandablePlaceholder.svelte';
	export let showAcceptedFilter = true;
	export let showVoteTypeFilter = true;
	import { translationFn, type FilterInfo } from '$lib/components/Filtering/types';
	import FiltersAny from '$lib/components/Filtering/FiltersAny.svelte';

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

	// get page number from query params
	const url = new URL(window.location.href);
	let page = parseInt(url.searchParams.get('page') || '1') || 1;

	const popupRequiredMajority: PopupSettings = {
		event: 'click',
		target: 'popupRequiresSimpleMajority',
		placement: 'bottom',
		closeQuery: '.listbox-item'
	};

	const popupAccepted: PopupSettings = {
		event: 'click',
		target: 'popupAccepted',
		placement: 'bottom',
		closeQuery: '.listbox-item'
	};

	const popupNamedVote: PopupSettings = {
		event: 'click',
		target: 'popupNamedVote',
		placement: 'bottom',
		closeQuery: '.listbox-item'
	};

	const popupIsLaw: PopupSettings = {
		event: 'click',
		target: 'popupIsLaw',
		placement: 'bottom',
		closeQuery: '.listbox-item'
	};

	let currentlyUpdating = false;

	let simpleMajorityFilter: FilterInfo<boolean | undefined> = {
		title: 'notwendige Mehrheit',
		popup: popupRequiredMajority,
		attributeName: 'requiredMajority',
		filterObj: undefined,
		translationFn,
		hidden: false,
		values: [
			{ title: 'egal', value: undefined },
			{ title: 'einfache Mehrheit', value: true },
			{ title: '2/3 Mehrheit', value: false }
		]
	};
	let acceptedFilter: FilterInfo<string | undefined> = {
		title: 'Angenommen',
		popup: popupAccepted,
		attributeName: 'accepted',
		filterObj: undefined,
		translationFn,
		hidden: !showAcceptedFilter,
		values: [
			{ title: 'egal', value: undefined },
			{ title: 'angenommen', value: 'a' },
			{ title: 'abgelehnt', value: 'd' },
			{ title: 'frühzeitig abgelehnt', value: 'p' }
		]
	};
	let namedVoteFilter: FilterInfo<boolean | undefined> = {
		title: 'Abstimmung',
		popup: popupNamedVote,
		attributeName: 'namedVote',
		filterObj: undefined,
		translationFn,
		hidden: !showVoteTypeFilter,
		values: [
			{ title: 'egal', value: undefined },
			{ title: 'namentliche Abstimmung', value: true }
		]
	};
	let votingFilter: FilterInfo<string | undefined> = {
		title: 'Antragstyp',
		popup: popupIsLaw,
		attributeName: 'votingFilter',
		filterObj: undefined,
		translationFn,
		hidden: false,
		values: [
			{ title: 'egal', value: undefined },
			{ title: 'Gesetz', value: 'Law' },
			{ title: 'Entschließung', value: 'Resolution' },
			{ title: 'Abänderung', value: 'Amendment' }
		]
	};

	let selectedPeriod = 'all';

	const maybeStoredFilter = get(currentVoteResultFilterStore);
	if (maybeStoredFilter !== null) {
		if (maybeStoredFilter.simple_majority !== null)
			simpleMajorityFilter.filterObj = maybeStoredFilter.simple_majority;
		if (maybeStoredFilter.legis_period !== null) selectedPeriod = maybeStoredFilter.legis_period;
		if (maybeStoredFilter.accepted !== null) acceptedFilter.filterObj = maybeStoredFilter.accepted;
		if (maybeStoredFilter.is_named_vote !== null)
			namedVoteFilter.filterObj = maybeStoredFilter.is_named_vote;
		if (maybeStoredFilter.vote_type !== null) votingFilter.filterObj = maybeStoredFilter.vote_type;
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
			is_named_vote: namedVoteFilter.filterObj == undefined ? null : namedVoteFilter.filterObj,
			accepted: acceptedFilter.filterObj == undefined ? null : acceptedFilter.filterObj,
			is_law: null,
			simple_majority:
				simpleMajorityFilter.filterObj == undefined ? null : simpleMajorityFilter.filterObj,
			legis_period: selectedPeriod == 'all' ? null : selectedPeriod,
			vote_type: votingFilter.filterObj === undefined ? null : votingFilter.filterObj,
			topics: null,
			is_urgent: null,
			party_votes: null
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

	$: if (page || selectedPeriod || filters || searchValue) {
		update();
	}

	$: if (searchValue) {
		page = 1;
	}

	let filters = [simpleMajorityFilter, acceptedFilter, namedVoteFilter, votingFilter];
</script>

<FiltersAny bind:filters bind:searchValue bind:selectedPeriod {update} />

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
