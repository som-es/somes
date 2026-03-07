<script lang="ts" generics="T">
	import { RadioGroup, RadioItem } from '@skeletonlabs/skeleton';
	import type { FilterInfo } from './types';

	export let filter: FilterInfo<T>;
</script>

{#if !filter.hidden}
	<div class="mt-5 mr-5">
		<h1 class="sm:text-2xl font-bold">{filter.title}</h1>
		<RadioGroup
			rounded="rounded-container-token sm:!rounded-token"
			active="variant-filled-secondary"
			hover="hover:variant-soft-secondary"
			flexDirection="flex-col sm:flex-row"
		>
			{#each filter.values as value}
				<RadioItem bind:group={filter.filterObj} name={filter.attributeName} value={value.value}
					>{value.title}</RadioItem
				>
			{/each}
		</RadioGroup>
	</div>
{/if}
