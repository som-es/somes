<script lang="ts">
	import type { Delegate } from '$lib/types';
	import { url } from '$lib/api/api';
	import { partyColors } from '$lib/partyColor';

	interface Props {
		delegate: Delegate;
		onclick?: () => void;
		class?: string;
		size?: 'sm' | 'md';
		children?: import('svelte').Snippet;
	}

	let { delegate, onclick, class: className = '', size = 'sm', children }: Props = $props();
</script>

<button
	class="flex items-center justify-between gap-2 md:gap-3 rounded-2xl bg-primary-200 px-2.5 py-1.5 md:px-3 md:py-2 text-left shadow-sm transition-colors hover:bg-primary-400 dark:bg-primary-400 dark:hover:bg-primary-500 {className}"
	{onclick}
>
	<div class="flex items-center gap-2 md:gap-3 overflow-hidden">
		<img
			src={`${url}assets/${delegate.id}.jpg`}
			alt={delegate.name}
			class="{size == 'sm' ? 'h-8 w-8' : 'h-10 w-10'} shrink-0 rounded-full object-cover"
		/>
		<div class="flex flex-col overflow-hidden">
			<span class="{size == 'sm' ? 'text-sm' : 'text-md'} truncate font-medium leading-tight">{delegate.name}</span>
			<div class="mt-0.5 flex items-center gap-1 md:gap-1.5">
				<div
					class="h-1.5 w-1.5 shrink-0 rounded-full md:h-2 md:w-2"
					style="background-color: {partyColors.get(delegate.party) ?? '#ccc'};"
				></div>
				<span class="truncate text-xs text-gray-700 dark:text-gray-300">{delegate.party}</span>
			</div>
		</div>
	</div>
	
	{#if children}
		<div class="shrink-0 flex items-center">
			{@render children()}
		</div>
	{/if}
</button>
