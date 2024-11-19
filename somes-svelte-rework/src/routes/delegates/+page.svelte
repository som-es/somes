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
	import AllBadges from '$lib/components/VoteResults/SimpleYesNo/AllBadges.svelte';
	import { replaceState } from '$app/navigation';
	import { cachedAllLegisPeriods } from '$lib/caching/legis_periods';
	import { dashDateToDotDate } from '$lib/date';
	import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';
	import { errorToNull } from '$lib';

	let delegates: Delegate[];
	let delegate: Delegate | null;

	let selectedPeriod = 'XXVIII';
	let prevSelectedPeriod = '';
	let periods: LegisPeriod[] = [];

	let popupSettings: PopupSettings = {
		event: 'focus-click',
		target: 'popupAutocomplete',
		placement: 'bottom-start'
	};

	let autocompleteOptions: AutocompleteOption<string>[] = [];
	let interests: InterestShare[] | null;
	let maxDayOffset = 365 * 5;
	let dayOffset = maxDayOffset;

	let renderStartDate: Date | null;
	let renderEndDate: Date | null;

	let finishedMounting = false;
	let supplyDate: Date | null = null;

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
	
	onMount(async () => {
		const url = new URL(window.location.href);
		selectedPeriod = url.searchParams.get('gp') || 'XXVIII';

		const cachedPeriods = await cachedAllLegisPeriods();
		if (cachedPeriods) periods = cachedPeriods.reverse();

		const firstIdx = periods.findIndex((x) => x.gp == selectedPeriod);
		if (firstIdx == -1) return;
// ).toISOString().split('T')[0] as unknown as Date
		const endDate = periods[firstIdx + 1]?.start_date;
		const newDate = new Date(endDate ? endDate : new Date())
		newDate.setDate(newDate.getDate() - 1)
		supplyDate = newDate.toISOString().split('T')[0] as unknown as Date
		// console.log(supplyDate);
		finishedMounting = true;
	})
	

	const updateDelsToDisplay = async () => {
		if (!periods || periods.length == 0) {
			return;
		}
		const firstIdx = periods.findIndex((x) => x.gp == selectedPeriod);
		if (firstIdx == -1) return;
		// delegate = null;
		// const endDate = new Date(periods[firstIdx + 1].start_date);
		renderStartDate = periods[firstIdx].start_date;
		renderEndDate = periods[firstIdx + 1]?.start_date;
		const startDate = new Date(renderStartDate);
		const endDate = new Date(renderEndDate ? renderEndDate : new Date());

		const diffTime = Math.abs(endDate.getTime() - startDate.getTime());
		maxDayOffset = Math.floor(diffTime / (1000 * 60 * 60 * 24));
		if (prevSelectedPeriod !== selectedPeriod) {
			dayOffset = maxDayOffset;
		}

		startDate.setDate(startDate.getDate() + dayOffset - 2);

		supplyDate = startDate.toISOString().split('T')[0] as unknown as Date
		// console.log(`supply ${supplyDate}`);
		// const fetchedDelsAtDate = await delegates_at(
		// );
		// console.log(fetchedDelsAtDate);

		// if (fetchedDelsAtDate) {
			// delsAtDate = fetchedDelsAtDate;
			// autocompleteOptions = convertDelegatesToAutocompleteOptions(delsAtDate);
		// }
	};

	$: if (selectedPeriod || dayOffset) {
		renderEndDate = null;
		renderStartDate = null;
		// if (window !== null) {
		// 	const url = new URL(window.location.href);
		// 	url.searchParams.set('gp', selectedPeriod);
		// 	replaceState(url, history.state);
		// }

		// delsAtDate = [];
		updateDelsToDisplay();
		if (finishedMounting) prevSelectedPeriod = selectedPeriod;
	}

	$: if (delegates && delegate == null) {
		const maybeStoredDelegate = get(currentDelegateStore);
		if (maybeStoredDelegate) {
			delegate = maybeStoredDelegate;
			const foundDel = delegates.find((del) => del.id === maybeStoredDelegate.id);
			if (foundDel) {
				delegate = foundDel;
			} else {
				delegate = delegates[Math.floor(Math.random() * delegates.length)];
			}
		} else {
			delegate = delegates[Math.floor(Math.random() * delegates.length)];
		}
	}
	$: if (delegates) autocompleteOptions = convertDelegatesToAutocompleteOptions(delegates);

	$: if (delegate) {
		// interests = null;
		if (finishedMounting) currentDelegateStore.set(delegate);
		delegate_interests(delegate.id).then((res) => {
			const input = errorToNull(res);
			if (input != null) input.sort((a, b) => b.self_share - a.self_share);
			interests = input;
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
			<LegisButtons bind:periods bind:selectedPeriod showAllButton={false}></LegisButtons>
		</div>
		<div
			class="flex flex-row flex-wrap title-item rounded-xl bg-primary-300 dark:bg-primary-500 p-3"
		>
			<div class="flex justify-between min-w-full">
				<div>
					Anfang ({renderStartDate == null ? '' : dashDateToDotDate(renderStartDate.toString())})
				</div>
				<div>
					Ende ({renderEndDate == null ? '' : dashDateToDotDate(renderEndDate.toString())})
				</div>
			</div>
			<input
				bind:value={dayOffset}
				type="range"
				min="2"
				max={maxDayOffset + 2}
				step={maxDayOffset / 30}
				list="steplist"
			/>
			<datalist id="steplist">
				<option>0</option>
				<option>25</option>
				<option>50</option>
				<option>75</option>
				<option>365</option>
			</datalist>
		</div>
		<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 p-3">
			{#if periods.length > 0 && delegates}
				<AllBadges
					delsAtDate={structuredClone(delegates)}
				/>
			{/if}
		</div>
		<!-- {#if delegates} -->
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
						{#if supplyDate}
							<VoteParliament2
								againstOpacity={1}
								voteResult={null}
								bind:delegate
								bind:delegates
								gp={selectedPeriod}
								supplyDate={supplyDate}
								orderingFactor={-1}
							/>
						{/if}
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
		<!-- {/if} -->
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
