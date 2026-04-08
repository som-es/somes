<script lang="ts">
	import { Popover } from 'bits-ui';
	import type { Snippet } from 'svelte';
	import type { GenericFilterGroup } from './types';
	import FilterDropdown from './FilterDropdown.svelte';
	import FilterGroup from './FilterGroup.svelte';
	import downArrowIcon from '$lib/assets/misc_icons/down-arrow.svg?raw';

	interface Props {
		genericFilters: (GenericFilterGroup<string> | GenericFilterGroup<boolean>)[];
		legisPeriodFilter?: GenericFilterGroup<string>;
		snippets?: Record<string, Snippet>;
		extraActiveCount?: number;
	}

	let { genericFilters = $bindable(), legisPeriodFilter = $bindable(), snippets = {}, extraActiveCount = 0 }: Props = $props();

	let activeGenericFiltersCount = $derived(
		genericFilters.filter((f) => f.activeValue !== undefined && f.activeValue !== 'all').length +
			(legisPeriodFilter ? +(legisPeriodFilter?.activeValue !== 'all') : 0) +
			extraActiveCount
	);
	let isGenericFilterOpen = $state(false);
	let isExpanded = $state(false);

	let hasAdvancedFilters = $derived(genericFilters.some((f) => f.advanced && !f.hidden));
</script>

<Popover.Root bind:open={isGenericFilterOpen}>
	<Popover.Trigger
		class="flex h-full grow touch-manipulation items-center justify-center gap-1 rounded-xl bg-secondary-500 px-2 md:grow-0"
	>
		<FilterDropdown
			title="Filter"
			activefilterCount={activeGenericFiltersCount}
			isOpen={isGenericFilterOpen}
		/>
	</Popover.Trigger>
	<Popover.Content sideOffset={8}>
		<div
			class="z-10 w-auto touch-manipulation rounded-xl border border-gray-300 bg-surface-50 px-5 pt-4 pb-5 text-black shadow-lg md:px-6 dark:bg-surface-600 dark:text-white"
		>
			{#each genericFilters as group, i}
			{#if !group.advanced && !group.hidden}
				{#if group.id && snippets[group.id]}
					<div class="mt-4 first:mt-0">
						<span class="text-base font-semibold text-gray-800 dark:text-gray-50">{group.title}</span>
						{@render snippets[group.id]()}
					</div>
				{:else}
					<FilterGroup bind:group={genericFilters[i]} />
				{/if}
			{/if}
		{/each}
			{#if legisPeriodFilter}
			<div class="mt-4 first:mt-0">
				<span class="text-base font-semibold text-gray-800 dark:text-gray-50"
					>{legisPeriodFilter.title}</span
				>
				<div class="flex w-72 flex-wrap gap-1 text-sm">
					{#each legisPeriodFilter.options as option}
						<button
							class="cursor-pointer rounded-lg border border-primary-300 {legisPeriodFilter.activeValue ===
							option.value
								? 'bg-primary-300 dark:bg-primary-400'
								: ''} px-2 py-1 text-xs md:text-sm"
							onclick={() => {
								legisPeriodFilter.activeValue = option.value;
							}}
						>
							<span class="text-nowrap">{option.title}</span>
						</button>
					{/each}
				</div>
			</div>
		{/if}
		{#if hasAdvancedFilters}
			<button
				class="mt-4 flex items-center gap-1 text-xs text-gray-600 hover:text-gray-800 dark:text-gray-400 dark:hover:text-gray-100"
				onclick={() => (isExpanded = !isExpanded)}
			>
				<span class="block w-3 transition-transform duration-200" class:rotate-180={isExpanded}>
					{@html downArrowIcon}
				</span>
				{isExpanded ? 'Weniger Filter' : 'Mehr Filter'}
			</button>
		{/if}
		{#if isExpanded}
			{#each genericFilters as group, i}
				{#if group.advanced && !group.hidden}
					{#if group.id && snippets[group.id]}
						<div class="mt-4 first:mt-0">
							<span class="text-base font-semibold text-gray-800 dark:text-gray-50">{group.title}</span>
							{@render snippets[group.id]()}
						</div>
					{:else}
						<FilterGroup bind:group={genericFilters[i]} />
					{/if}
				{/if}
			{/each}
		{/if}
			<Popover.Arrow class="rounded-sm fill-current stroke-gray-300 text-gray-300" />
		</div>
	</Popover.Content>
</Popover.Root>
