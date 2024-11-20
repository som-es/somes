<script lang="ts">
	import { partyToColor } from '$lib/partyColor';
	import type { Delegate, DelegateQA } from '$lib/types';
	import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';

	export let delegate: Delegate;
	export let showQA: boolean = false;
	export let questions: DelegateQA[] = [];

	$: modal = {
		type: "component",
		component: "delegateQA",
		meta: { questions: questions } 
	} as ModalSettings;

	const modalStore = getModalStore();
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
		<h5 style="color: {partyToColor(delegate.party)}">
			{#if delegate.party == 'OK'}
				Ohne Klub
			{:else}
				<span>{delegate.party}</span>
			{/if}
		</h5>
		<h6>{delegate.constituency}</h6>
		<hr class="!border-t-2 my-1" />
		<h6>{delegate.divisions?.join(', ')}</h6>
	</section>
	<hr class="!border-t-2 my-1" />
	<footer class="card-footer flex justify-end items-end mt-3">
		{#if showQA}
			<button class="btn btn-lg variant-filled" on:click={() => modalStore.trigger(modal)}>Vorstellung</button>
		{/if}
	</footer>
</div>
