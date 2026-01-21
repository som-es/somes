<script lang="ts" generics="T">
	import ListBoxItem from '../UI/ListBoxItem.svelte';
	import type { FilterInfo } from './types';

	export let filter: FilterInfo<T>;
</script>

<ListBox
	rounded="rounded-container sm:!rounded-base"
	active="preset-filled-secondary-500"
	hover="hover:preset-tonal-secondary"
>
	{#each filter.values as value}
		<ListBoxItem bind:group={filter.filterObj} name={filter.attributeName} value={value.value}
			>{value.title}</ListBoxItem
		>
	{/each}
</ListBox>
