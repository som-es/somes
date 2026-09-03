<script lang="ts">
	import { t } from '$lib/i18n/i18n.svelte';
	import type { Delegate } from '$lib/types';
	import { url } from '$lib/api/api';
	import { partyColors } from '$lib/partyColor';
	import { getParliament, type Parliament } from '$lib/api/parliament';

	interface Props {
		delegate: Delegate;
		// For crawler
		href?: string;
		onclick?: () => void;
		class?: string;
		size?: 'sm' | 'md';
		children?: import('svelte').Snippet;
		parliament?: Parliament;
	}

	let {
		delegate,
		href,
		onclick,
		class: className = '',
		size = 'sm',
		children,
		parliament = getParliament()
	}: Props = $props();
	let imgSrc = $derived(
		parliament == 'at' ? `${url}assets/${delegate.id}.jpg` : delegate.image_url
	);

	function handleClick(e: MouseEvent) {
		if (!onclick) return;
		e.preventDefault();
		onclick();
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions link or button -->
<svelte:element
	this={href ? 'a' : 'button'}
	{href}
	class="flex items-center justify-between gap-2 rounded-2xl bg-primary-200 px-2.5 py-1.5 text-left shadow-sm transition-colors hover:bg-primary-400 md:gap-3 md:px-3 md:py-2 dark:bg-primary-400 dark:hover:bg-primary-500 {className}"
	onclick={handleClick}
>
	<div class="flex items-center gap-2 overflow-hidden md:gap-3">
		<img
			src={imgSrc}
			alt={delegate.name}
			loading="lazy"
			class="{size == 'sm' ? 'h-8 w-8' : 'h-10 w-10'} shrink-0 rounded-full object-cover text-[1px]"
		/>
		<div class="flex flex-col overflow-hidden">
			<span class="{size == 'sm' ? 'text-sm' : 'text-md'} truncate leading-tight font-medium"
				>{delegate.name}</span
			>
			<div class="mt-0.5 flex items-center gap-1 md:gap-1.5">
				<div
					class="h-1.5 w-1.5 shrink-0 rounded-full md:h-2 md:w-2"
					style="background-color: {partyColors.get(
						delegate.party?.trim() ? delegate.party : t('delegate.withoutParty')
					) ?? '#ccc'};"
				></div>
				<span class="truncate text-xs text-gray-700 dark:text-gray-300"
					>{delegate.party?.trim() ? delegate.party : t('delegate.withoutParty')}</span
				>
			</div>
		</div>
	</div>

	{#if children}
		<div class="flex shrink-0 items-center">
			{@render children()}
		</div>
	{/if}
</svelte:element>
