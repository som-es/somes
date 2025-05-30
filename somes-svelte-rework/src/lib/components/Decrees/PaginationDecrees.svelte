<script lang="ts">
	import { decrees_per_page, delegate_by_id, errorToNull, gov_proposals_by_search, gov_proposals_per_page } from '$lib/api/api';
	import type { DecreeFilter, DecreesWithMaxPage, Delegate, GovPropFilter, GovProposalsWithMaxPage } from '$lib/types';
	import { onMount } from 'svelte';
	import Pagination from '../Pagination.svelte';
	import ExpandablePlaceholder from '../VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import { currentDecreeFilterStore, currentGovProposalFilterStore } from '$lib/stores/stores';
	import { get } from 'svelte/store';
	import LegisButtons from '../Filtering/LegisButtons.svelte';
	import SButton from '../UI/SButton.svelte';
	import {
		ListBox,
		ListBoxItem,
		popup,
		RadioGroup,
		RadioItem,
		type PopupSettings
	} from '@skeletonlabs/skeleton';
	import DecreeBar from '../Delegates/Decrees/DecreeBar.svelte';
	import { pushState } from '$app/navigation';

	const url = new URL(window.location.href);
	let page = parseInt(url.searchParams.get('page') || '1') || 1;

	let old_page = page;
	let currentlyUpdating = false;
	let decrees: DecreesWithMaxPage | null = null;

	let searchValue = '';
	let selectedPeriod = 'all';
	let govOfficialsFilter: number[] | undefined = undefined;

	const maybeStoredFilter = get(currentDecreeFilterStore);
	if (maybeStoredFilter !== null) {
		if (maybeStoredFilter.gov_officials) govOfficialsFilter = maybeStoredFilter.gov_officials;
		if (maybeStoredFilter.legis_period) selectedPeriod = maybeStoredFilter.legis_period;
	}

	const govOfficials = new Map<number, Delegate>();

	const loadGovProps = async () => {
		currentlyUpdating = true;
		if (decrees !== null) {
			decrees.decrees = [];
		}

		let filter: DecreeFilter | null = {
			gov_officials: null,
			legis_period: selectedPeriod == 'all' ? null : selectedPeriod
		};
		currentDecreeFilterStore.set(filter);
		// filter = null;

		if (searchValue) {
			// const govPropsSearch = errorToNull(await gov_proposals_by_search(page, searchValue, filter));
			// console.log(govPropsSearch);
			// if (govPropsSearch) decrees = govPropsSearch;
		} else {
			decrees = errorToNull(await decrees_per_page(page - 1, filter));
		}

		decrees?.decrees.forEach(decree => {
			const id = decree.gov_official_id;
			if (govOfficials.has(id)) return;
			delegate_by_id(id).then(del => {
				const mayDel = errorToNull(del);
				if (mayDel) {
					govOfficials.set(id, mayDel);
					decrees = decrees;
				}
			});
		});

		currentlyUpdating = false;
	};

	const update = () => {
		loadGovProps();

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

	$: if (page || selectedPeriod || govOfficialsFilter) {
		update();
	}
	
	onMount(update);

	const popupRequiredMajority: PopupSettings = {
		event: 'click',
		target: 'popupRequiresSimpleMajority',
		placement: 'bottom',
		closeQuery: '.listbox-item'
	};

	// const translateGovOfficalsFilterValue = (govOfficials: boolean | undefined) => {
	// 	if (hasVoteResultFilter == undefined) {
	// 		return 'egal';
	// 	}
	// 	return hasVoteResultFilter ? 'mit Abstimmung' : 'ohne Abstimmung';
	// };
</script>

<!-- <div class="lg:hidden flex flex-wrap gap-6">
	<div>
		<h1 class="text-2xl font-bold">notwendige Mehrheit</h1>
		<button
			class="btn variant-filled-secondary w-48 justify-between"
			use:popup={popupRequiredMajority}
		>
			<span class="capitalize">{translateGovOfficalsFilterValue(govOfficialsFilter)}</span>
			<span>↓</span>
		</button>
	</div>
</div> -->

<!-- <div class="card w-48 shadow-xl py-2" data-popup="popupRequiresSimpleMajority">
	<ListBox
		rounded="rounded-container-token sm:!rounded-token"
		active="variant-filled-secondary"
		hover="hover:variant-soft-secondary"
	>
		<ListBoxItem bind:group={govOfficialsFilter} name="hasVoteResult" value={undefined}
			>egal</ListBoxItem
		>
		<ListBoxItem bind:group={govOfficialsFilter} name="hasVoteResult" value={true}
			>mit Abstimmung</ListBoxItem
		>
		<ListBoxItem bind:group={govOfficialsFilter} name="hasVoteResult" value={false}
			>ohne Abstimmung</ListBoxItem
		>
	</ListBox>
</div> -->

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
	{#if decrees}
		<Pagination bind:page maxPage={decrees.max_page} />
		{#if decrees.decrees.length > 0}
			{#each decrees.decrees as decree}
				<DecreeBar {decree} {page} delegate={govOfficials.get(decree.gov_official_id)} coloring="bg-primary-300 dark:bg-primary-500" />
			{/each}
		{:else if currentlyUpdating}
			{#each { length: 9 } as _}
				<ExpandablePlaceholder class="my-4" />
			{/each}
		{:else}
			Keine Verordnungen gefunden
		{/if}
		<div class="float-right">
			<Pagination bind:page maxPage={decrees.max_page} />
		</div>
	{:else}
		{#each { length: 9 } as _}
			<ExpandablePlaceholder class="my-4" />
		{/each}
		<!-- <CenterPrograssRadial /> -->
	{/if}
</div>
