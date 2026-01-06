<script lang="ts" >
	import type { AutocompleteOption } from './types';
    import Autocomplete from './Autocomplete.svelte';
	import { Popover } from 'bits-ui';

    interface Props {
        inputValue?: string;
        autocompleteOptions: AutocompleteOption<string>[];
        onDelegateSelection: (option: AutocompleteOption<string>) => void;
        delegateFilter: () => AutocompleteOption<string>[];
    }

    let { inputValue = $bindable(''), autocompleteOptions, onDelegateSelection, delegateFilter }: Props = $props();
</script>

<Popover.Root>
    <Popover.Trigger>
        {#snippet child({ props })}
            <input
                {...props}
                id="autocomplete-input" 
                class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5"
                type="search"
                name="ac-demo"
                bind:value={inputValue}
                placeholder="Suchen..."
                autocomplete="off"
            />
        {/snippet}
    </Popover.Trigger>
    <Popover.Portal>
        <Popover.Content class="w-full max-w-sm p-4 text-sm z-50 bg-white border border-gray-200 rounded-lg shadow-xl">
            {#if autocompleteOptions.length > 0}
                <div class="max-h-64 overflow-y-auto">
                    <Autocomplete
                        bind:input={inputValue}
                        options={autocompleteOptions}
                        onselection={onDelegateSelection}
                        emptyState={'Keine Person gefunden'}
                        filter={delegateFilter}
                    />
                </div>
            {/if}
        </Popover.Content>
    </Popover.Portal>
</Popover.Root>

<!-- <Popover 
    triggeredBy="#autocomplete-input" 
    trigger="click" 
    placement="bottom-start"
    class="w-full max-w-sm p-4 text-sm z-50 bg-white border border-gray-200 rounded-lg shadow-xl"
>
    {#if autocompleteOptions.length > 0}
        <div class="max-h-64 overflow-y-auto">
            <Autocomplete
                bind:input={inputValue}
                options={autocompleteOptions}
                onselection={onDelegateSelection}
                emptyState={'Keine Person gefunden'}
                filter={delegateFilter}
            />
        </div>
    {/if}
</Popover> -->
