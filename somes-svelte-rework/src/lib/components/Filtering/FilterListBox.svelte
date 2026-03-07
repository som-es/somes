<script lang="ts" generics="T">
	import ListBoxItem from '../UI/ListBoxItem.svelte';

	import { ListBox } from '@skeletonlabs/skeleton';
	import type { FilterInfo } from './types';

	export let filter: FilterInfo<T>;
</script>

<ListBox
	rounded="rounded-container-token sm:!rounded-token"
	active="variant-filled-secondary"
	hover="hover:variant-soft-secondary"
>
	{#each filter.values as value}
		<ListBoxItem bind:group={filter.filterObj} name={filter.attributeName} value={value.value}
			>{value.title}</ListBoxItem
		>
	{/each}
</ListBox>
