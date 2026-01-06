<script lang="ts" generics="Value = unknown, Meta = unknown">
	import { slide } from 'svelte/transition';
	import { partyToColor } from '$lib/partyColor.js';
	import type { AutocompleteOption } from './types';

	type Option = AutocompleteOption<Value, Meta>;

	// Props
	let {
		input = $bindable(''),
		options = [],
		limit = undefined,
		allowlist = [],
		denylist = [],
		emptyState = 'No Results Found.',
		// Logic Prop: Defaults to the internal 'defaultFilter' function below
		filter = undefined, 
		// Classes
		class: className = '',
		regionNav = '',
		regionList = '',
		regionItem = '',
		regionButton = 'w-full',
		regionEmpty = 'text-center',
		// Transitions
		transitions = true,
		transitionInParams = { duration: 200 },
		transitionOutParams = { duration: 200 },
		// Events
		onselection
	}: {
		input?: any;
		options?: Option[];
		limit?: number;
		allowlist?: Value[];
		denylist?: Value[];
		emptyState?: string;
		filter?: () => Option[]; // <--- The filter prop
		class?: string;
		regionNav?: string;
		regionList?: string;
		regionItem?: string;
		regionButton?: string;
		regionEmpty?: string;
		transitions?: boolean;
		transitionInParams?: any;
		transitionOutParams?: any;
		onselection?: (option: Option) => void;
	} = $props();

	// Internal logic: Filter by Allow/Deny List (Base list)
	const listedOptions = $derived.by(() => {
		if (allowlist.length === 0 && denylist.length === 0) return options;
		let _options = [...options];
		if (allowlist.length > 0) _options = _options.filter((o) => allowlist.includes(o.value));
		if (denylist.length > 0) _options = _options.filter((o) => !denylist.includes(o.value));
		return _options;
	});

	// The default filtering logic
	const defaultFilter = () => {
		const search = String(input ?? '').toLowerCase().trim();
		return listedOptions.filter((option) => {
			const optionFormatted = JSON.stringify([
				option.label,
				option.value,
				option.keywords
			]).toLowerCase();
			return optionFormatted.includes(search);
		});
	};

	// Final list used in the UI
	const optionsFiltered = $derived.by(() => {
		if (!input) return listedOptions;
		// Use the custom filter prop if provided, otherwise fallback to default
		return filter ? filter() : defaultFilter();
	});

	const sliceLimit = $derived(limit ?? optionsFiltered.length);
	// console.log(optionsFiltered);
</script>

<div class="autocomplete {className} z-40">
	{#if optionsFiltered.length > 0}
		<nav class="autocomplete-nav {regionNav} z-40">
			<ul class="autocomplete-list {regionList}">
				{#each optionsFiltered.slice(0, sliceLimit) as option, i (i)}
					<li
						class="autocomplete-item {regionItem} z-40"
						in:slide={transitions ? transitionInParams : { duration: 0 }}
						out:slide={transitions ? transitionOutParams : { duration: 0 }}
					>
						<button
							class="z-40 flex justify-between p-3 hover:bg-secondary-400 dark:hover:bg-secondary-600 rounded-3xl autocomplete-button {regionButton}"
							type="button"
							onclick={() => onselection?.(option)}
						>
							<div>{@html option.label}</div>
							{#if option.right_label}
								<div style="color: {partyToColor(option.right_label)};">
									{@html option.right_label}
								</div>
							{/if}
						</button>
					</li>
				{/each}
			</ul>
		</nav>
	{:else}
		<div class="autocomplete-empty {regionEmpty}">{@html emptyState}</div>
	{/if}
</div>