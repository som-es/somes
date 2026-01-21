<script lang="ts" generics="T">
	import { Segment } from '@skeletonlabs/skeleton-svelte';
	import type { FilterInfo } from './types';

	export let filter: FilterInfo<T>;
</script>

{#if !filter.hidden}
	<div class="mt-5 mr-5">
		<h1 class="sm:text-2xl font-bold">{filter.title}</h1>
		<Segment
			rounded="rounded-container sm:!rounded-base"
			active="preset-filled-secondary-500"
			hover="hover:preset-tonal-secondary"
			flexDirection="flex-col sm:flex-row"
		>
			{#each filter.values as value}
				<Segment.Item bind:group={filter.filterObj} name={filter.attributeName} value={value.value}
					>{value.title}</Segment.Item
				>
			{/each}
		</Segment>
	</div>
{/if}
