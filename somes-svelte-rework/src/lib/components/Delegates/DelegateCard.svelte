<script lang="ts">
	import { partyToColor } from '$lib/partyColor';
	import type { Delegate } from '$lib/types';

	export let delegate: Delegate;
</script>

<div class="!z-0 card card-hover mx-4 drop-shadow-lg hover:cursor-pointer">
	<header class="flex justify-center">
		<img
			src={delegate.image_url}
			class=""
			style="width: 200px;"
			alt="Image of politician {delegate.name}"
		/>
	</header>
	<section class="p-4">
		<h4>
			{delegate.name}
		</h4>
		<h5 style="color: {partyToColor(delegate.party)}">
			{#if delegate.party == 'OK'}
				Ohne Klub
			{:else}
				<span>{delegate.party}</span>
			{/if}
		</h5>
		<h6>{delegate.divisions?.join(', ')}</h6>
	</section>
	<hr class="!border-t-2 my-4" />
	<footer class="card-footer flex justify-between items-center">
		<span>Lorem Ipsum</span>
	</footer>
</div>
