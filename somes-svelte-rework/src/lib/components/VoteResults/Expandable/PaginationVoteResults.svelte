<script lang="ts">
	import type {
		Delegate,
		VoteResultFilter,
		VoteResultsWithMaxPage,
		HasError
	} from '$lib/types';
	import { onMount } from 'svelte';
	import { errorToNull, vote_results_by_search, vote_results_per_page } from '$lib/api/api';
	import VoteResultExpandableBar from './VoteResultExpandableBar.svelte';
	import { pushState } from '$app/navigation';
	import Pagination from '$lib/components/Pagination.svelte';
	import {
		ListBox,
		ListBoxItem,
		popup,
		RadioGroup,
		RadioItem,
		type PopupSettings
	} from '@skeletonlabs/skeleton';
	import LegisButtons from '$lib/components/Filtering/LegisButtons.svelte';
	import { currentVoteResultFilterStores } from '$lib/stores/stores';
	import { get } from 'svelte/store';
	import ExpandablePlaceholder from './Placeholders/ExpandablePlaceholder.svelte';
	import SButton from '$lib/components/UI/SButton.svelte';
	export let showAcceptedFilter = true;
	export let showVoteTypeFilter = true;

	export let dels: Delegate[];
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
	let currentVoteResultFilterStore = currentVoteResultFilterStores[storeIdx]
	$: currentVoteResultFilterStore = currentVoteResultFilterStores[storeIdx];


	let voteResults: VoteResultsWithMaxPage | null = null;

	// get page number from query params
	const url = new URL(window.location.href);
	let page = parseInt(url.searchParams.get('page') || '1') || 1;

	let currentlyUpdating = false;

	let selectedPeriod = 'all';
	let simpleMajorityFilter: boolean | undefined = undefined;
	let acceptedFilter: string | undefined = undefined;
	let namedVoteFilter: boolean | undefined = undefined;
	let isLawFilter: boolean | undefined = undefined;
	let votingFilter: string | undefined = undefined;

	const maybeStoredFilter = get(currentVoteResultFilterStore);
	if (maybeStoredFilter !== null) {
		if (maybeStoredFilter.simple_majority !== null) simpleMajorityFilter = maybeStoredFilter.simple_majority;
		if (maybeStoredFilter.legis_period !== null) selectedPeriod = maybeStoredFilter.legis_period;
		if (maybeStoredFilter.accepted !== null) acceptedFilter = maybeStoredFilter.accepted;
		if (maybeStoredFilter.is_named_vote !== null) namedVoteFilter = maybeStoredFilter.is_named_vote;
		if (maybeStoredFilter.is_law !== null) isLawFilter = maybeStoredFilter.is_law;
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

		// accepted: 'a' (accepted), 'd' (declined), 'p' (pre-declined)
		// null "egal"
		let filter: VoteResultFilter | null = {
			is_named_vote: namedVoteFilter == undefined ? null : namedVoteFilter,
			accepted,
			is_law: isLawFilter == undefined ? null : isLawFilter,
			simple_majority: simpleMajorityFilter == undefined ? null : simpleMajorityFilter,
			legis_period: selectedPeriod == 'all' ? null : selectedPeriod,
			vote_type: votingFilter == undefined ? null : votingFilter,
			topics: null,
			is_urgent: null,
			party_votes: null,
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
		console.log(voteResults);
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

	$: if (
		page ||
		selectedPeriod ||
		simpleMajorityFilter ||
		acceptedFilter ||
		namedVoteFilter ||
		votingFilter ||
		isLawFilter // ||
		// searchValue
	) {
		update();
	}

	const popupRequiredMajority: PopupSettings = {
		event: 'click',
		target: 'popupRequiresSimpleMajority',
		placement: 'bottom',
		closeQuery: '.listbox-item'
	};

	const translateSimpleMajorityFilterValue = (simpleMajorityFilter: boolean | undefined) => {
		if (simpleMajorityFilter == undefined) {
			return 'egal';
		}
		return simpleMajorityFilter ? 'einfache Mehrheit' : '2/3 Mehrheit';
	};

	const popupAccepted: PopupSettings = {
		event: 'click',
		target: 'popupAccepted',
		placement: 'bottom',
		closeQuery: '.listbox-item'
	};

	const translateAcceptedValue = (acceptedFilter: string | undefined) => {
		if (acceptedFilter == undefined) {
			return 'egal';
		}
		switch (acceptedFilter) {
			case 'a':
				return 'angenommen';
			case 'd':
				return 'abgelehnt';
			case 'p':
				return 'frühzeitig abgelehnt';
		}
	};

	const popupNamedVote: PopupSettings = {
		event: 'click',
		target: 'popupNamedVote',
		placement: 'bottom',
		closeQuery: '.listbox-item'
	};

	const translateNamedVoteValue = (namedVoteFilter: boolean | undefined) => {
		if (namedVoteFilter == undefined) {
			return 'egal';
		}
		return namedVoteFilter ? 'namentliche' : 'nicht namentliche';
	};

	const popupIsLaw: PopupSettings = {
		event: 'click',
		target: 'popupIsLaw',
		placement: 'bottom',
		closeQuery: '.listbox-item'
	};

	const translateIsLawValue = (isLawFilter: boolean | undefined) => {
		if (isLawFilter == undefined) {
			return 'egal';
		}
		return isLawFilter ? 'ja' : 'nein';
	};
</script>

<!-- <br /> -->
<!-- <SlideToggle name="slider-large" active="bg-secondary-400" size="md">
	<span class="text-lg"> Test </span>
</SlideToggle> -->

<!-- FILTER OPTIONS 

-->

<!-- Small Screen PopUps (keep them out of <div>...</div>) -->
<div class="card w-48 shadow-xl py-2" data-popup="popupRequiresSimpleMajority">
	<ListBox
		rounded="rounded-container-token sm:!rounded-token"
		active="variant-filled-secondary"
		hover="hover:variant-soft-secondary"
	>
		<ListBoxItem bind:group={simpleMajorityFilter} name="simpleMajority" value={undefined}
			>egal</ListBoxItem
		>
		<ListBoxItem bind:group={simpleMajorityFilter} name="simpleMajority" value={true}
			>einfache Mehrheit</ListBoxItem
		>
		<ListBoxItem bind:group={simpleMajorityFilter} name="simpleMajority" value={false}
			>2/3 Mehrheit</ListBoxItem
		>
	</ListBox>
</div>

<div class="card w-48 shadow-xl py-2" data-popup="popupAccepted">
	<ListBox
		rounded="rounded-container-token sm:!rounded-token"
		active="variant-filled-secondary"
		hover="hover:variant-soft-secondary"
	>
		<ListBoxItem bind:group={acceptedFilter} name="accepted" value={undefined}>egal</ListBoxItem>
		<ListBoxItem bind:group={acceptedFilter} name="accepted" value={'a'}>angenommen</ListBoxItem>
		<ListBoxItem bind:group={acceptedFilter} name="accepted" value={'d'}>abgelehnt</ListBoxItem>
		<ListBoxItem bind:group={acceptedFilter} name="accepted" value={'p'}
			>frühzeitig abgelehnt</ListBoxItem
		>
	</ListBox>
</div>

<div class="card w-52 shadow-xl py-2" data-popup="popupNamedVote">
	<ListBox
		rounded="rounded-container-token sm:!rounded-token"
		active="variant-filled-secondary"
		hover="hover:variant-soft-secondary"
	>
		<ListBoxItem bind:group={namedVoteFilter} name="namedVote" value={undefined}>egal</ListBoxItem>
		<ListBoxItem bind:group={namedVoteFilter} name="namedVote" value={true}
			>namentliche Abstimmung</ListBoxItem
		>
	</ListBox>
</div>

<div class="card w-52 shadow-xl py-2" data-popup="popupIsLaw">
	<ListBox
		rounded="rounded-container-token sm:!rounded-token"
		active="variant-filled-secondary"
		hover="hover:variant-soft-secondary"
	>
		<ListBoxItem bind:group={isLawFilter} name="isLaw" value={undefined}>egal</ListBoxItem>
		<ListBoxItem bind:group={isLawFilter} name="isLaw" value={true}>ja</ListBoxItem>
		<ListBoxItem bind:group={isLawFilter} name="isLaw" value={false}>nein</ListBoxItem>
	</ListBox>
</div>

<div>
	<!-- FILTER OPTIONS -->
	<!-- Large Screens-->
	<div class="max-lg:hidden flex flex-wrap mt-5">
		<div class="mt-5 mr-5">
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
		{#if showVoteTypeFilter}
		<div class="mt-5 mr-5">
			<h1 class="text-2xl font-bold">Abstimmung</h1>
			<RadioGroup active="variant-filled-secondary" hover="hover:variant-soft-secondary">
				<RadioItem bind:group={namedVoteFilter} name="namedVote" value={undefined}>egal</RadioItem>
				<RadioItem bind:group={namedVoteFilter} name="namedVote" value={true}
					>namentliche Abstimmung</RadioItem
				>
			</RadioGroup>
		</div>
		{/if}
	</div>
	<div class="max-lg:hidden flex flex-wrap mt-5 mr-5">
		{#if showAcceptedFilter}
		<div class="mt-5 mr-5">
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
		{/if}
		<div class="mt-5 mr-5">
			<h1 class="text-2xl font-bold">Typ</h1>
			<RadioGroup active="variant-filled-secondary" hover="hover:variant-soft-secondary">
				<RadioItem bind:group={votingFilter} name="voting" value={undefined}>egal</RadioItem>
				<RadioItem bind:group={votingFilter} name="voting" value={"Law"}>Gesetz</RadioItem>
				<RadioItem bind:group={votingFilter} name="voting" value={"Resolution"}>Entschließung</RadioItem>
				<RadioItem bind:group={votingFilter} name="voting" value={"Amendment"}>Abänderung</RadioItem>
			</RadioGroup>
		</div>
	</div>

	<!-- Small Screens-->
	<div class="lg:hidden flex flex-wrap gap-6">
		<div>
			<h1 class="text-2xl font-bold">notwendige Mehrheit</h1>
			<button
				class="btn variant-filled-secondary w-48 justify-between"
				use:popup={popupRequiredMajority}
			>
				<span class="capitalize">{translateSimpleMajorityFilterValue(simpleMajorityFilter)}</span>
				<span>↓</span>
			</button>
		</div>
	
		{#if showAcceptedFilter}
			<div>
				<h1 class="text-2xl font-bold">Angenommen</h1>
				<button class="btn variant-filled-secondary w-48 justify-between" use:popup={popupAccepted}>
					<span class="capitalize">{translateAcceptedValue(acceptedFilter)}</span>
					<span>↓</span>
				</button>
			</div>
		{/if}
	
		{#if showVoteTypeFilter}
		<div>
			<h1 class="text-2xl font-bold">Abstimmung</h1>
			<button class="btn variant-filled-secondary w-48 justify-between" use:popup={popupNamedVote}>
				<span class="capitalize">{translateNamedVoteValue(namedVoteFilter)}</span>
				<span>↓</span>
			</button>
		</div>
		{/if}
		<div>
			<h1 class="text-2xl font-bold">Gesetz</h1>
			<button class="btn variant-filled-secondary w-48 justify-between" use:popup={popupIsLaw}>
				<span class="capitalize">{translateIsLawValue(isLawFilter)}</span>
				<span>↓</span>
			</button>
		</div>
	</div>

	<!-- LEGIS PERIODS -->
	<div class="mt-10">
		<h2 class="font-bold text-2xl mb-1">Legislaturperioden</h2>
		<LegisButtons bind:selectedPeriod />
	</div>

	<!-- SEARCH OPTION -->
	<div class="mt-10">
		<div class="flex flex-row gap-4">
			<input
				class="rounded-lg !bg-surface-200-700-token w-full h-12 px-2 placeholder-gray-500"
				type="search"
				name="ac-demo"
				bind:value={searchValue}
				on:change={update}
				placeholder="Suche..."
			/>
			<SButton class="bg-secondary-500 text-black" on:click={update}>Suchen</SButton>
		</div>
	</div>

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
