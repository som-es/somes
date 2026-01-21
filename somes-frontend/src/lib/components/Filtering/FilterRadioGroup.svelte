<script lang="ts" generics="T">
	import { ToggleGroup } from "bits-ui";
	import type { FilterInfo } from './types';

	// Svelte 5 Props
	let { filter = $bindable() }: { filter: FilterInfo<T> } = $props();

	// Helper for the "active" style (Skeleton's preset-filled-secondary-500)
	const activeClass = "bg-secondary-500 text-white";
	const hoverClass = "hover:bg-secondary-500/20";
</script>

{#if !filter.hidden}
	<div class="mt-5 mr-5">
		<h1 class="sm:text-2xl font-bold mb-2">{filter.title}</h1>
		
		<ToggleGroup.Root
			type="single"
			bind:value={filter.filterObj as string}
			class="flex flex-col sm:flex-row border border-surface-400 overflow-hidden 
                   rounded-container sm:rounded-base w-fit"
		>
			{#each filter.values as value}
				<ToggleGroup.Item
					value={value.value}
					class="px-4 py-2 text-sm font-medium transition-colors
                           border-b sm:border-b-0 sm:border-r last:border-0 border-surface-400
                           data-[state=on]:{activeClass} 
                           data-[state=off]:{hoverClass}
                           data-[state=off]:bg-surface-200"
				>
					{value.title}
				</ToggleGroup.Item>
			{/each}
		</ToggleGroup.Root>
	</div>
{/if}