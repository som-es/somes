<script lang="ts" generics="Value = unknown, Meta = unknown">
	import { slide } from 'svelte/transition';
	import type { AutocompleteOptionMultiselect } from './types';
	import { Popover } from 'bits-ui';

	type Option = AutocompleteOptionMultiselect<Value, Meta>;

	interface Props {
		input?: string;
		options?: Option[];
		limit?: number;
		emptyState?: string;
		filter?: () => Option[];
		class?: string;
		onselection?: (option: Option) => void;
	}

	let {
		input = '',
		options = [],
		limit = undefined,
		emptyState = 'No Results Found.',
		filter = undefined,
		class: className = '',
		onselection
	}: Props = $props();

	// The default filtering logic
	const defaultFilter = () => {
		const search = String(input ?? '')
			.toLowerCase()
			.trim();
		return options.filter((option) => {
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
		if (!input) return options;
		return filter ? filter() : defaultFilter();
	});

	const sliceLimit = $derived(limit ?? optionsFiltered.length);
</script>

<div class="autocomplete {className} z-40">
	{#if optionsFiltered.length > 0}
		<nav class="autocomplete-nav z-40">
			<ul class="autocomplete-list">
				{#each optionsFiltered.slice(0, sliceLimit) as option, i (i)}
					<li
						class="autocomplete-item z-40"
						in:slide={{ duration: 200 }}
						out:slide={{ duration: 200 }}
					>
						<button
							class="autocomplete-button z-40 flex w-full items-center justify-between rounded-3xl p-3 hover:bg-secondary-400 dark:hover:bg-secondary-600"
							type="button"
							onclick={() => onselection?.(option)}
						>
							<div class="flex items-center gap-2">
								{#if option.isSelected}
									<span class="font-bold text-green-500">✓</span>
								{:else}
									<span class="w-4 text-gray-300"></span>
								{/if}
								<span>{@html option.label}</span>
							</div>
						</button>
					</li>
				{/each}
			</ul>
		</nav>
	{:else}
		<div class="autocomplete-empty text-center">{@html emptyState}</div>
	{/if}
</div>
