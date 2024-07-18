<script lang="ts">
	import { cachedDelegates, filteredDelegates } from '$lib/caching/delegates';
	import DelegateCard from '$lib/components/Delegates/DelegateCard.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import DelegatesParliament from '$lib/components/Parliaments/DelegatesParliament.svelte';
	import type { Delegate } from '$lib/types';
	import {
		Autocomplete,
		type AutocompleteOption,
		type PopupSettings
	} from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';

	let delegates: Delegate[] | null;
	let delegate: Delegate | null;

	let popupSettings: PopupSettings = {
		event: 'focus-click',
		target: 'popupAutocomplete',
		placement: 'bottom'
	};

	onMount(async () => {
		delegates = await filteredDelegates();
		if (delegates !== null) delegate = delegates[Math.floor(Math.random() * delegates.length)];
	});

	let inputDemo = '';

	const flavorOptions: AutocompleteOption<string>[] = [
		{ label: 'Vanilla', value: 'vanilla', keywords: 'plain, basic', meta: { healthy: false } },
		{ label: 'Chocolate', value: 'chocolate', keywords: 'dark, white', meta: { healthy: false } },
		{ label: 'Strawberry', value: 'strawberry', keywords: 'fruit', meta: { healthy: true } },
		{
			label: 'Neapolitan',
			value: 'neapolitan',
			keywords: 'mix, strawberry, chocolate, vanilla',
			meta: { healthy: false }
		},
		{ label: 'Pineapple', value: 'pineapple', keywords: 'fruit', meta: { healthy: true } },
		{ label: 'Peach', value: 'peach', keywords: 'fruit', meta: { healthy: true } }
	];

	function onFlavorSelection(event: CustomEvent<AutocompleteOption<string>>): void {
		inputDemo = event.detail.label;
	}
</script>

<Container>
	<div class="text-token w-full max-w-sm space-y-2">
		<input
			class="input h-8 px-2"
			type="search"
			name="ac-demo"
			bind:value={inputDemo}
			placeholder="Suchen..."
			use:popup={popupSettings}
		/>
		<div data-popup="popupAutocomplete">
			<Autocomplete
				bind:input={inputDemo}
				options={flavorOptions}
				on:selection={onFlavorSelection}
			/>
		</div>
		<div class="card w-full max-w-sm max-h-48 p-4 overflow-y-auto" tabindex="-1">
			<Autocomplete
				bind:input={inputDemo}
				options={flavorOptions}
				on:selection={onFlavorSelection}
			/>
		</div>
	</div>

	<div>
		delegates
		{#if delegates}
			<div class="grid-container">
				<div class="parliament-item">
					<DelegatesParliament bind:delegate dels={delegates} />
				</div>
				<div class="delegate-item">
					{#if delegate}
						<DelegateCard {delegate} />
					{/if}
				</div>
			</div>
		{/if}
	</div>
</Container>

<style>
	.grid-container {
		display: grid;
		gap: 20px;
		grid-template-areas:
			'p p p p p d d'
			'p p p p p d d'
			'p p p p p d d';
		/* "i1 i2 i3 activity . . ."; */
		/* "i i i a"; */
		padding: 10px;
	}

	.grid-container > div {
		padding: 20px 0;
	}

	.parliament-item {
		grid-area: p;
	}

	.delegate-item {
		grid-area: d;
	}

	.activity-item {
		grid-area: activity;
	}
</style>
