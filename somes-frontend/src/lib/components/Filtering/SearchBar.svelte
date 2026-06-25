<script lang="ts">
	import searchIcon from '$lib/assets/misc_icons/search-glass.svg?raw';
	import type { HTMLInputAttributes } from 'svelte/elements';
	import type { Snippet } from 'svelte';

	interface Props extends HTMLInputAttributes {
		searchValue: string;
		placeholder?: string;
		rightSlot?: Snippet;
	}

	let { searchValue = $bindable(), placeholder = 'Suche...', rightSlot, ...rest }: Props = $props();
</script>

<div class="flex h-10 flex-grow touch-manipulation rounded-xl border-[2px] border-gray-400">
	<div class="flex h-9 w-10 shrink-0 items-center justify-center text-gray-600 dark:text-gray-300">
		{@html searchIcon}
	</div>
	<input
		type="search"
		class="block w-full bg-transparent py-2 placeholder:text-gray-600 focus:outline-none dark:placeholder:text-gray-300"
		{placeholder}
		bind:value={searchValue}
		{...rest}
	/>
	{#if rightSlot}
		{@render rightSlot()}
	{/if}
</div>
