<script lang="ts">
	import searchIcon from '$lib/assets/misc_icons/search-glass.svg?raw';
	import type { HTMLInputAttributes } from 'svelte/elements';

	interface Props extends HTMLInputAttributes {
		searchValue: string;
		placeholder?: string;
	}

	let {
		searchValue = $bindable(),
		placeholder = 'Suche...',
		'aria-label': ariaLabel,
		...rest
	}: Props = $props();
	const accessibleName = $derived(ariaLabel ?? placeholder);
</script>

<div class="flex h-10 flex-grow touch-manipulation rounded-xl border-[2px] border-gray-400">
	<div class="flex h-9 w-10 items-center justify-center text-gray-600 dark:text-gray-300" aria-hidden="true">
		{@html searchIcon}
	</div>
	<input
		type="search"
		class="block w-full bg-transparent py-2 placeholder:text-gray-600 focus:outline-none dark:placeholder:text-gray-300"
		{placeholder}
		aria-label={accessibleName}
		bind:value={searchValue}
		{...rest}
	/>
</div>
