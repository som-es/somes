<script lang="ts">
	import { url } from '$lib/api/api';
	import { getParliament, type Parliament } from '$lib/api/parliament';
	import { getPartyColors, partyToColor } from '$lib/partyColor';
	import type { Delegate } from '$lib/types';

	interface Props {
		delegate: Delegate;
		partyColors?: Map<string, string>;
		onNavigate?: () => void;
		parliament?: Parliament;
	}

	let {
		delegate,
		partyColors = getPartyColors(),
		onNavigate,
		parliament = getParliament()
	}: Props = $props();
	let imgSrc = $derived(
		parliament == 'at' ? `${url}assets/${delegate.id}.jpg` : delegate.image_url
	);
</script>

<div class="flex min-w-0 items-center gap-2">
	<img
		src={imgSrc}
		alt={delegate.name}
		class="h-8 w-8 shrink-0 rounded-full object-cover text-[1px]"
	/>
	<div class="flex min-w-0 flex-col">
		{#if onNavigate}
			<button
				class="truncate text-left text-sm leading-tight font-semibold hover:underline lg:text-base"
				onclick={(e) => {
					e.stopPropagation();
					onNavigate();
				}}
				onkeypress={(e) => e.stopPropagation()}
			>
				{delegate.name}
			</button>
		{:else}
			<span class="truncate text-sm leading-tight font-semibold lg:text-base">{delegate.name}</span>
		{/if}
		<div class="mt-0.5 flex items-center gap-1.5">
			<div
				class="h-2 w-2 shrink-0 rounded-full"
				style="background-color: {partyToColor(delegate.party, partyColors)};"
			></div>
			<span class="truncate text-xs text-gray-700">
				{#if delegate.party == null || delegate.party == 'OK'}
					Ohne Klub
				{:else}
					{delegate.party}
				{/if}
			</span>
		</div>
	</div>
</div>
