<script lang="ts">
	import { pushState } from '$app/navigation';
	import { errorToNull, gov_proposals_by_search, gov_proposals_per_page } from '$lib/api/api';
	import type { GovPropFilter, GovProposalsWithMaxPage } from '$lib/types';
	import { onMount } from 'svelte';
	import Pagination from '../Pagination.svelte';
	import ExpandablePlaceholder from '../VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import { currentGovProposalFilterStore } from '$lib/stores/stores';
	import { get } from 'svelte/store';
	import LegisButtons from '../Filtering/LegisButtons.svelte';
	import GovProposalExpandableBar from './Latest/GovProposalExpandableBar.svelte';
	import SButton from '../UI/SButton.svelte';
	import {
		ListBox,
		ListBoxItem,
		popup,
		RadioGroup,
		RadioItem,
		type PopupSettings
	} from '@skeletonlabs/skeleton';

	const url = new URL(window.location.href);
	let page = parseInt(url.searchParams.get('page') || '1') || 1;

	let old_page = page;
	let currentlyUpdating = false;
	let govProposals: GovProposalsWithMaxPage | null = null;

	let searchValue = '';
	let selectedPeriod = 'all';
	let hasVoteResultFilter: boolean | undefined = undefined;

	const maybeStoredFilter = get(currentGovProposalFilterStore);
	if (maybeStoredFilter !== null) {
		if (maybeStoredFilter.has_vote_result) hasVoteResultFilter = maybeStoredFilter.has_vote_result;
		if (maybeStoredFilter.legis_period) selectedPeriod = maybeStoredFilter.legis_period;
	}

	const loadGovProps = async () => {
		currentlyUpdating = true;
		if (govProposals !== null) {
			govProposals.gov_proposals = [];
		}

		let filter: GovPropFilter | null = {
			has_vote_result: hasVoteResultFilter == undefined ? null : hasVoteResultFilter,
			legis_period: selectedPeriod == 'all' ? null : selectedPeriod
		};
		currentGovProposalFilterStore.set(filter);
		// filter = null;

		if (searchValue) {
			const govPropsSearch = errorToNull(await gov_proposals_by_search(page, searchValue, filter));
			console.log(govPropsSearch);
			if (govPropsSearch) govProposals = govPropsSearch;
		} else {
			govProposals = errorToNull(await gov_proposals_per_page(page - 1, filter));
		}
		currentlyUpdating = false;
	};

	const update = () => {
		loadGovProps();

		// update query params
		const url = new URL(window.location.href);
		url.searchParams.set('page', page.toString());
		try {
			// pushState(url.toString(), { replaceState: true });
		} catch (e) {
			page = old_page;
		}

		old_page = page;
	};

	$: if (page || selectedPeriod || hasVoteResultFilter) {
		update();
	}

	onMount(update);

	const popupRequiredMajority: PopupSettings = {
		event: 'click',
		target: 'popupRequiresSimpleMajority',
		placement: 'bottom',
		closeQuery: '.listbox-item'
	};

	const translateHasVoteResultFilterValue = (hasVoteResultFilter: boolean | undefined) => {
		if (hasVoteResultFilter == undefined) {
			return 'egal';
		}
		return hasVoteResultFilter ? 'mit Abstimmung' : 'ohne Abstimmung';
	};
</script>

<div class="lg:hidden flex flex-wrap gap-6">
	<div>
		<h1 class="text-2xl font-bold">notwendige Mehrheit</h1>
		<button
			class="btn variant-filled-secondary w-48 justify-between"
			use:popup={popupRequiredMajority}
		>
			<span class="capitalize">{translateHasVoteResultFilterValue(hasVoteResultFilter)}</span>
			<span>↓</span>
		</button>
	</div>
</div>

<div class="card w-48 shadow-xl py-2" data-popup="popupRequiresSimpleMajority">
	<ListBox
		rounded="rounded-container-token sm:!rounded-token"
		active="variant-filled-secondary"
		hover="hover:variant-soft-secondary"
	>
		<ListBoxItem bind:group={hasVoteResultFilter} name="hasVoteResult" value={undefined}
			>egal</ListBoxItem
		>
		<ListBoxItem bind:group={hasVoteResultFilter} name="hasVoteResult" value={true}
			>mit Abstimmung</ListBoxItem
		>
		<ListBoxItem bind:group={hasVoteResultFilter} name="hasVoteResult" value={false}
			>ohne Abstimmung</ListBoxItem
		>
	</ListBox>
</div>
<div class="max-lg:hidden flex gap-4 flex-wrap">
	<div class="mt-5">
		<h1 class="text-2xl font-bold">Abstimmungsstatus</h1>
		<RadioGroup
			rounded="rounded-container-token sm:!rounded-token"
			active="variant-filled-secondary"
			hover="hover:variant-soft-secondary"
			flexDirection="flex-col sm:flex-row"
		>
			<RadioItem bind:group={hasVoteResultFilter} name="hasVoteResult" value={undefined}
				>egal</RadioItem
			>
			<RadioItem bind:group={hasVoteResultFilter} name="hasVoteResult" value={true}
				>mit Abstimmung</RadioItem
			>
			<RadioItem bind:group={hasVoteResultFilter} name="hasVoteResult" value={false}
				>ohne Abstimmung</RadioItem
			>
		</RadioGroup>
	</div>
</div>

<div class="mt-5">
	<h2 class="font-bold text-2xl">Legislaturperioden</h2>
	<LegisButtons bind:selectedPeriod />
</div>
<div class="mt-5">
	<h2 class="font-bold text-2xl">Suche</h2>
	<div class="flex flex-row gap-4">
		<input
			class="input w-full h-12 px-2"
			type="search"
			name="ac-demo"
			bind:value={searchValue}
			on:change={update}
			placeholder="Suchen..."
		/>
		<SButton class="bg-secondary-500 text-black" on:click={update}>Suchen</SButton>
	</div>
</div>
<div>
	{#if govProposals}
		<Pagination bind:page maxPage={govProposals.max_page} />
		{#if govProposals.gov_proposals.length > 0}
			{#each govProposals.gov_proposals as govProposal}
				<GovProposalExpandableBar {govProposal} class="" />
			{/each}
		{:else if currentlyUpdating}
			{#each { length: 9 } as _}
				<ExpandablePlaceholder class="my-4" />
			{/each}
		{:else}
			Keine Ministerialentwürfe gefunden
		{/if}
		<div class="float-right">
			<Pagination bind:page maxPage={govProposals.max_page} />
		</div>
	{:else}
		{#each { length: 9 } as _}
			<ExpandablePlaceholder class="my-4" />
		{/each}
		<!-- <CenterPrograssRadial /> -->
	{/if}
</div>
