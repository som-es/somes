<script lang="ts">
	import { filteredDelegates } from '$lib/caching/delegates';
	import type { AutocompleteOption } from '$lib/components/Autocompletion/types';
	import DelegateCard from '$lib/components/Delegates/DelegateCard.svelte';
	import Autocomplete from '$lib/components/Autocompletion/Autocomplete.svelte';
	import type { Delegate, InterestShare, LegisPeriod } from '$lib/types';
	import { popup, type PopupSettings } from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';
	import { delegate_interests, delegates_at } from '$lib/api';
	import InterestTiles from '$lib/components/Delegates/InterestTiles.svelte';
	import { get } from 'svelte/store';
	import { currentDelegateStore, hasGoBackStore } from '$lib/stores/stores';
	import SButton from '$lib/components/UI/SButton.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import {
		convertDelegatesToAutocompleteOptions,
		delegateFilterOptions
	} from '$lib/components/Autocompletion/filtering';
	import LegisButtons from '$lib/components/Filtering/LegisButtons.svelte';
	import VoteParliament from '$lib/components/Parliaments/VoteParliament.svelte';
	import AllBadges from '$lib/components/VoteResults/SimpleYesNo/AllBadges.svelte';
	import { replaceState } from '$app/navigation';
	import { cachedAllLegisPeriods } from '$lib/caching/legis_periods';

	let delegates: Delegate[] | null;
	let delsAtDate: Delegate[] = [];
	let delegate: Delegate | null;

	let selectedPeriod = 'XXVII';
	let periods: LegisPeriod[] = [];

	let popupSettings: PopupSettings = {
		event: 'focus-click',
		target: 'popupAutocomplete',
		placement: 'bottom-start'
	};

	let autocompleteOptions: AutocompleteOption<string>[] = [];
	let interests: InterestShare[] | null;

	let finishedMounting = false;

	onMount(async () => {

		const url = new URL(window.location.href);
		selectedPeriod = url.searchParams.get('gp') || 'XXVII';

		delegates = await filteredDelegates();
		if (delegates !== null) {
			delegate = delegates[Math.floor(Math.random() * delegates.length)];
			autocompleteOptions = convertDelegatesToAutocompleteOptions(delegates);
		}

		const cachedPeriods = await cachedAllLegisPeriods();
		if (cachedPeriods) periods = cachedPeriods.reverse();

		await updateDelsToDisplay();
		const maybeStoredDelegate = get(currentDelegateStore);
		if (maybeStoredDelegate) {
			delegate = maybeStoredDelegate;
			const foundDel = delsAtDate.find(del => del.id === maybeStoredDelegate.id);
			if (foundDel) delegate = foundDel;
		}
		finishedMounting = true;
	});

	let inputValue = '';

	function delegateFilter(): AutocompleteOption<string>[] {
		let _options = [...autocompleteOptions];
		let _inputValue = `${String(inputValue).toLowerCase().trim()} `;
		return delegateFilterOptions(_options, _inputValue);
	}

	function onDelegateSelection(event: CustomEvent<AutocompleteOption<string>>): void {
		// @ts-ignore
		delegate = event.detail.meta;
		inputValue = event.detail.label;
	}


	const updateDelsToDisplay = async () => {
		if (!periods || periods.length == 0) {
			return;
		}
		if (periods[periods.length - 1].gp == selectedPeriod) {
			return;
		}
		const firstIdx = periods.findIndex((x) => x.gp == selectedPeriod);
		if (firstIdx == -1) return;
		// delegate = null;
		const endDate = new Date(periods[firstIdx + 1].start_date);
		endDate.setDate(endDate.getDate() - 5);
		// console.log(endDate);

		const fetchedDelsAtDate = await delegates_at(
			endDate.toISOString().split('T')[0] as unknown as Date
		);
		if (fetchedDelsAtDate) {
			delsAtDate = fetchedDelsAtDate;
			autocompleteOptions = convertDelegatesToAutocompleteOptions(delsAtDate);
		}
	};

	$: if (selectedPeriod) {
		// if (window !== null) {
		// 	const url = new URL(window.location.href);
		// 	url.searchParams.set('gp', selectedPeriod);
		// 	replaceState(url, history.state);
		// }

		delsAtDate = [];
		updateDelsToDisplay();
	}

	$: if (delegate) {
		// interests = null;
		if (finishedMounting) currentDelegateStore.set(delegate);
		delegate_interests(delegate.id).then((res) => {
			if (res != null) res.sort((a, b) => b.self_share - a.self_share);
			interests = res;
		});
	}

</script>

<!-- <div class="mx-auto px-10"> -->
<Container>
	{#if get(hasGoBackStore)}
		<SButton class="bg-primary-500 my-3" on:click={() => history.back()}>Zurück</SButton>
	{/if}
	<br />
	<div class="entry bg-primary-200 dark:bg-primary-400 gap-3 flex flex-wrap">
		<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 p-3">
			<h1 class="font-bold text-3xl">Abgeordnete des Nationalrats</h1>
		</div>
		<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 p-3">
			<LegisButtons bind:periods bind:selectedPeriod={selectedPeriod} showAllButton={false}></LegisButtons>
		</div>
		<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 p-3">
			<input type="range" min="0" max="100" step="1" list="steplist" />
			<datalist id="steplist">
				<option>0</option>
				<option>25</option>
				<option>50</option>
				<option>75</option>
				<option>100</option>
			</datalist>
		</div>
		<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 p-3">
				{#if periods.length > 0 && delegates}
					<AllBadges delsAtDate={(periods[periods.length - 1].gp != selectedPeriod) ? delsAtDate : structuredClone(delegates)} />
				{/if}
			</div>
		{#if delegates}
			
			<div class="text-token w-full space-y-2">
				<input
					class="input w-full h-12 px-2"
					type="search"
					name="ac-demo"
					bind:value={inputValue}
					placeholder="Suchen..."
					use:popup={popupSettings}
				/>

				{#if autocompleteOptions}
					<div
						class="card w-full max-w-sm max-h-64 p-4 overflow-y-auto"
						data-popup="popupAutocomplete"
					>
						<Autocomplete
							bind:input={inputValue}
							options={autocompleteOptions}
							on:selection={onDelegateSelection}
							emptyState={'Keine Person gefunden'}
							filter={delegateFilter}
						/>
					</div>
				{/if}
			</div>
			<div class="flex flex-wrap min-w-full justify-between">
				<div class="rounded-xl w-full parliament-item bg-primary-300 dark:bg-primary-500">
					<div class="px-5">
						<VoteParliament
							againstOpacity={1}
							voteResult={null}
							bind:delegate
							dels={delegates}
							delsAtDate={delsAtDate}
							gp={selectedPeriod}
							orderingFactor={-1}
						/>
					</div>
				</div>
				<div class="rounded-xl delegate-item bg-primary-300 dark:bg-primary-500">
					{#if delegate}
						<DelegateCard {delegate} />
					{/if}
				</div>
			</div>
			{#if interests}
				<InterestTiles interests={interests.slice(0, 4)}></InterestTiles>
			{:else}
				<ExpandablePlaceholder class={'my-3'} />
			{/if}
			<!-- <div class="activity-item bg-primary-300">
                    Activity
                </div> -->
		{/if}
	</div>
</Container>

<!-- </div> -->

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 20px;
		gap: 10px;
	}

	.grid-container {
		box-sizing: border-box;
		display: grid;
		min-width: 0;
		min-height: 0;
		grid-template-columns: 3fr 1fr;
		grid-template-rows: 2fr auto;
		grid-template-areas:
			'p d'
			'i .';
		/* "i i i a"; */
		padding: 10px;
	}

	.grid-container > div {
		padding: 20px 0;
	}

	/* :global(.parliament-item) {
		grid-area: p;
	} */

	@media (min-width: 768px) {
		.parliament-item {
			grid-area: p;
			flex-basis: 66%;
		}
	}

	@media (min-width: 768px) {
		.delegate-item {
			grid-area: d;
			flex-basis: 33%;
		}
	}

	.title-item {
		flex-basis: 100%;
	}

	:global(.interests-item) {
		grid-area: i;
		/* overflow: hidden; */
		/* min-width: 0; */
	}

	.activity-item {
		grid-area: activity;
	}

	.grid-tile-content {
		display: flex;
		justify-content: center;
		flex-direction: column;
		align-items: center;
	}
</style>
