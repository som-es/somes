<script lang="ts">
	import { filteredDelegates } from '$lib/caching/delegates';
	import type { AutocompleteOption } from '$lib/components/Autocompletion/types';
	import DelegateCard from '$lib/components/Delegates/DelegateCard.svelte';
	import Autocomplete from '$lib/components/Autocompletion/Autocomplete.svelte';
	import DelegatesParliament from '$lib/components/Parliaments/DelegatesParliament.svelte';
	import type { Delegate, InterestShare } from '$lib/types';
	import { popup, type PopupSettings } from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';
	import { delegate_interests } from '$lib/api';
	import InterestTiles from '$lib/components/Delegates/InterestTiles.svelte';
	import CenterPrograssRadial from '$lib/components/ProgressInfos/CenterPrograssRadial.svelte';
	import { get } from 'svelte/store';
	import { currentDelegateStore, hasGoBackStore } from '$lib/stores/stores';
	import SButton from '$lib/components/UI/SButton.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import {
		convertDelegatesToAutocompleteOptions,
		delegateFilterOptions
	} from '$lib/components/Autocompletion/filtering';

	let delegates: Delegate[] | null;
	let delegate: Delegate | null;

	let popupSettings: PopupSettings = {
		event: 'focus-click',
		target: 'popupAutocomplete',
		placement: 'bottom'
	};

	let autocompleteOptions: AutocompleteOption<string>[] = [];
	let interests: InterestShare[] | null;

	onMount(async () => {
		delegates = await filteredDelegates();
		if (delegates !== null) {
			delegate = delegates[Math.floor(Math.random() * delegates.length)];
			autocompleteOptions = convertDelegatesToAutocompleteOptions(delegates);
		}
		const maybeStoredDelegate = get(currentDelegateStore);
		if (maybeStoredDelegate) {
			delegate = maybeStoredDelegate;
		}
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

	$: if (delegate) {
		// interests = null;
		currentDelegateStore.set(delegate);
		delegate_interests(delegate.id).then((res) => {
			if (res != null) res.sort((a, b) => b.self_share - a.self_share);
			interests = res;
		});
	}
</script>

<!-- <div class="mx-auto px-10"> -->
<Container>
	<div>
		{#if get(hasGoBackStore)}
			<SButton class="bg-primary-500 my-3" on:click={() => history.back()}>Zurück</SButton>
		{/if}
		<br />
		<div class="font-bold text-2xl mb-3">Abgeordnete des Nationalrats</div>
		{#if delegates}
			<div class="text-token w-full max-w-sm space-y-2">
				<input
					class="input h-8 px-2"
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
			<div class="grid-container gap-3">
				<div class="rounded-xl parliament-item bg-primary-300 dark:bg-primary-500">
					{#if delegate?.is_active}
						<div class="px-5">
							<DelegatesParliament bind:delegate dels={delegates} />
						</div>
					{/if}
				</div>
				<div class="rounded-xl delegate-item bg-primary-300 dark:bg-primary-500">
					{#if delegate}
						<DelegateCard {delegate} />
					{/if}
				</div>
				{#if interests}
					<InterestTiles interests={interests.slice(0, 4)}></InterestTiles>
				{:else}
					<ExpandablePlaceholder class={'my-3'} />
				{/if}
				<!-- <div class="activity-item bg-primary-300">
                    Activity
                </div> -->
			</div>
		{/if}
	</div>
</Container>

<!-- </div> -->

<style>
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

	:global(.parliament-item) {
		grid-area: p;
		/* overflow: hidden; */
		/* min-width: 0; */
	}

	.delegate-item {
		grid-area: d;
		/* overflow: hidden; */
		/* min-width: 0; */
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
