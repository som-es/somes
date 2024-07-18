<script lang="ts">
	import { cachedDelegates, filteredDelegates } from '$lib/caching/delegates';
	import type { AutocompleteOption } from '$lib/components/Autocompletion/types';
	import DelegateCard from '$lib/components/Delegates/DelegateCard.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import Autocomplete from '$lib/components/Autocompletion/Autocomplete.svelte';
	import DelegatesParliament from '$lib/components/Parliaments/DelegatesParliament.svelte';
	import type { Delegate } from '$lib/types';
	import {
		popup,
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

	let autocompleteOptions: AutocompleteOption<string>[] = [];
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

			return {
                right_label: delegate.party, 
				label: delegate.name,
				value: delegate.name,
				keywords: `${delegate.id}, ${delegate.party}, ${delegate.constituency}, ${genderIdentsString}, ${delegate.birthdate}, ${delegate.active_since}`,
                meta: delegate,
			};
		});
	}

    function delegateFilter(): AutocompleteOption<string>[] {
        let _options = [...autocompleteOptions];
        let _inputValue = `${String(inputValue).toLowerCase().trim()} `
        return _options.filter((option) => {
            let values = _inputValue.split(" ");
            const optionFormatted = `${option.value.toLowerCase().trim()} ${option.keywords?.toLowerCase().trim()}`;
            for (let idx = 0; idx < values.length; idx++) {
                if (!optionFormatted.includes(values[idx])) {
                    return null
                }
            }
            return option

        });
    }


	function onDelegateSelection(event: CustomEvent<AutocompleteOption<string>>): void {
        // @ts-ignore
        delegate = event.detail.meta;
		inputValue = event.detail.label;
	}
</script>

<Container>
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
                        class="card w-full max-w-sm max-h-48 p-4 overflow-y-auto"
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
