<script lang="ts">
	import { partyToColor } from '$lib/partyColor';
	import type { Delegate, DelegateQA } from '$lib/types';
	import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';

	export let delegate: Delegate;
	export let showQA: boolean = false;
	export let questions: DelegateQA[] = [];

	$: delegateQAModal = {
		type: 'component',
		component: 'delegateQA',
		meta: { questions: questions }
	} as ModalSettings;

	$: aiChatModal = {
		type: 'component',
		component: 'aiChat',
		meta: { delegate: delegate }
	} as ModalSettings;

	const modalStore = getModalStore();
</script>

<div class="!z-0 card min-h-full mx-4 drop-shadow-lg flex flex-col">
	<header class="flex justify-center">
		<img
			src={delegate.image_url}
			style="width: 200px;"
			class="rounded-full"
			alt="Image of politician {delegate.name}"
		/>
	</header>
	<section class="p-4 flex-grow">
		<h4 class="font-bold text-xl">
			{delegate.name}
		</h4>
		<h5 style="color: {partyToColor(delegate.party)}">
			{#if delegate.party == 'OK'}
				Ohne Klub
			{:else if delegate.party != null}
				<span>{delegate.party}</span>
			{/if}
		</h5>

		<h6 class=" text-lg">{delegate.primary_mandate}</h6>
		<hr class="!border-t-2 my-1" />
		{#if delegate.constituency != null}
			<h3>{delegate.constituency}</h3>
		{/if}
		<hr class="!border-t-2 my-1" />
		<h3>{delegate.divisions?.join(', ')}</h3>
	</section>
	<hr class="!border-t-2 my-1" />
	<!-- <footer class="card-footer flex justify-end items-end mt-3"> -->
	<footer class="card-footer flex justify-between mt-3">
		<button class="btn btn-lg variant-filled" on:click={() => modalStore.trigger(aiChatModal)}
			>AI Chat</button
		>
		{#if showQA && questions.length > 0}
			<button class="btn btn-lg variant-filled" on:click={() => modalStore.trigger(delegateQAModal)}
				>Vorstellung</button
			>
		{/if}
	</footer>
</div>
