<script lang="ts">
	import { cachedDelegates, filteredDelegates } from '$lib/caching/delegates';
	import type { AutocompleteOption } from '$lib/components/Autocompletion/types';
	import DelegateCard from '$lib/components/Delegates/DelegateCard.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import Autocomplete from '$lib/components/Autocompletion/Autocomplete.svelte';
	import DelegatesParliament from '$lib/components/Parliaments/DelegatesParliament.svelte';
	import type { Delegate, InterestShare } from '$lib/types';
	import { popup, ProgressRadial, type PopupSettings } from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';
	import { delegate_interests } from '$lib/api';
	import { topicColors } from '$lib/interestColors';
	import InterestTiles from '$lib/components/Delegates/InterestTiles.svelte';

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
	});

	let inputValue = '';

	function convertDelegatesToAutocompleteOptions(
		delegates: Delegate[]
	): AutocompleteOption<string>[] {
		return delegates.map((delegate) => {
			let genderIdents: string[] = [];
			if (delegate.gender == 'm') {
				genderIdents = ['männlich', 'mann', 'abgeordneter'];
			} else if (delegate.gender == 'f') {
				genderIdents = ['weiblich', 'frau', 'abgeordnete'];
			}

			let genderIdentsString = genderIdents.join(', ');
			let divisionsString = delegate.divisions?.join(', ');

			return {
				right_label: delegate.party,
				label: delegate.name,
				value: delegate.name,
				keywords: `${delegate.id}, ${delegate.party}, ${delegate.constituency}, ${genderIdentsString}, ${delegate.birthdate}, ${delegate.active_since}, ${divisionsString}`,
				meta: delegate
			};
		});
	}

	function delegateFilter(): AutocompleteOption<string>[] {
		let _options = [...autocompleteOptions];
		let _inputValue = `${String(inputValue).toLowerCase().trim()} `;
		return _options.filter((option) => {
			let values = _inputValue.split(' ');
			const optionFormatted = `${option.value.toLowerCase().trim()} ${option.keywords?.toLowerCase().trim()}`;
			for (let idx = 0; idx < values.length; idx++) {
				if (!optionFormatted.includes(values[idx])) {
					return null;
				}
			}
			return option;
		});
	}

	function onDelegateSelection(event: CustomEvent<AutocompleteOption<string>>): void {
		// @ts-ignore
		delegate = event.detail.meta;
		inputValue = event.detail.label;
	}

	$: if (delegate) {
        // interests = null;
		delegate_interests(delegate.id).then((res) => {
			if (res != null) res.sort((a, b) => b.self_share - a.self_share);
			interests = res;
		});
	}
</script>

<div class="mx-auto px-10">
	<div>
		delegates
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
				<div class="parliament-item bg-primary-300 dark:bg-primary-500">
					<!-- <div class="px-5"> -->
						<DelegatesParliament bind:delegate dels={delegates} />
					<!-- </div> -->
				</div>
				<div class="delegate-item grid-tile bg-primary-300 dark:bg-primary-500">
					{#if delegate}
						<div class="grid-tile-content w-96">
							<DelegateCard {delegate} />
						</div>
					{/if}
				</div>
				{#if interests}
					<InterestTiles interests={interests.slice(0, 4)}></InterestTiles>
					
                {:else}
                    <ProgressRadial />
				{/if}
                <!-- <div class="activity-item bg-primary-300">
                    Activity
                </div> -->
			</div>
		{/if}
	</div>
</div>

<style>
	.grid-tile {
		box-sizing: border-box;
		border-radius: 25px;
	}
	.grid-container {
		box-sizing: border-box;
		display: grid;
        min-width: 0;
        min-height: 0;
		grid-template-areas:
			'p p p p d d'
			'i i i i . .';
		/* "i i i a"; */
		padding: 10px;
	}

	.grid-container > div {
		padding: 20px 0;
	}

	.parliament-item {
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

</style>
