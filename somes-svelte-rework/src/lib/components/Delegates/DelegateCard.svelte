<script lang="ts">
	import { partyToColor } from '$lib/partyColor';
	import type { Delegate } from '$lib/types';
	import SButton from '../UI/SButton.svelte';

	export let delegate: Delegate;
</script>

<div class="!z-0 card card-hover min-h-full mx-4 drop-shadow-lg flex flex-col">
	<header class="flex justify-center">
		<img
			src={delegate.image_url}
			style="width: 200px;"
			class="rounded-full"
			alt="Image of politician {delegate.name}"
		/>
	</header>
	<section class="p-4 flex-grow">
		<h4 class="font-bold text-lg">
			{delegate.name}
		</h4>
		<h5 style="color: {partyToColor(delegate.current_party)}">
			{#if delegate.party == 'OK'}
				Ohne Klub
			{:else}
				<span>{delegate.current_party}</span>
			{/if}
		</h5>
		<h6>{delegate.constituency}</h6>
		<hr class="!border-t-2 my-1" />
		<h6>{delegate.divisions?.join(', ')}</h6>
	</section>
	<hr class="!border-t-2 my-1" />
	<footer class="card-footer flex justify-end items-end mt-3"></footer>
</div>
