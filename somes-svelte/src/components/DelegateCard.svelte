<script lang="ts">
	import type { Delegate } from "$lib/types";

	export let delegate: Delegate | undefined;
</script>

<div
	class="card card-hover h-[60vh] aspect-[3/5] overflow-hidden grid grid-rows-[max-content_1fr_max-content] drop-shadow-lg hover:cursor-pointer"
>
	{#if delegate}
		<header>
			<img
				src={delegate?.image_url}
				class="bg-black/50 w-full aspect-[1/1]"
				alt="Image of politician {delegate?.name}"
			/>
		</header>
		<section class="p-4">
			<h4>
				{delegate?.name}
			</h4>
			<span>
				{delegate?.party}
			</span>
		</section>
		<hr class="!border-t-2 my-4" />
		<footer class="card-footer flex justify-between items-center">
			<span>Lorem Ipsum</span>
		</footer>
	{:else}
		<!-- TODO: undefined delegate placeholder -->
	{/if}
</div>
