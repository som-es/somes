<script lang="ts">
	import { partyToColor } from '$lib/partyColor';
	import type { Delegate, DelegateFavo, DelegateQA, Mandate } from '$lib/types';
	import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
	import SButton from '../UI/SButton.svelte';
	import { currentDelegateStore } from '$lib/stores/stores';
	import { gotoHistory } from '$lib/goto';
	import star from '$lib/assets/misc_icons/star.svg?raw';
	import { onMount } from 'svelte';
	import { cachedDelegateFavos } from '$lib/caching/delegate_favos';

	export let delegate: Delegate;
	export let onlyTop: boolean = false;
	export let showQA: boolean = false;
	export let questions: DelegateQA[] = [];
	export let showMoreDetailsBtn = false;
	export let showImg = true;

	const onShowDetails = () => {
		currentDelegateStore.set(delegate);
		gotoHistory(`/delegates`, true);
	};

	let delegateFavos: DelegateFavo[] | null = null;
	onMount(async () => {
		delegateFavos = await cachedDelegateFavos();
	});

	// console.log(delegate)

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

<div class="!z-0 card {onlyTop ? "" : "min-h-full"}  mx-4 drop-shadow-lg flex flex-col">
	<header class="relative">
		<button on:click={() => {
			if (delegateFavos) {
				delegateFavos.push({delegate_id: 1000})
			}
		}} class="absolute top-0 right-0 w-14 p-2">
			{@html star}
		</button>
		<div class="flex justify-center items-center h-full">
			{#if showImg}
			<img
				src={delegate.image_url}
				style="width: 200px;"
				class="rounded-full"
				alt="Image of politician {delegate.name}"
			/>
			{/if}
		</div>
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

		<h6 class="text-lg">{delegate.active_mandates?.join("\n")}</h6>

		{#if !onlyTop}
		<hr class="!border-t-2 my-1" />
		{#if delegate.constituency != null}
			<h3>{delegate.constituency}</h3>
		{/if}
		<hr class="!border-t-2 my-1" />
		<h3>{delegate.divisions?.join(', ')}</h3>
		{/if}
	</section>


	<hr class="!border-t-2 my-1" />
	<!-- <footer class="card-footer flex justify-end items-end mt-3"> -->
	<footer class="card-footer flex justify-between mt-1">
		{#if showMoreDetailsBtn}
			<div></div>
			<SButton class="bg-tertiary-500 text-black" on:click={onShowDetails}>Zur Person</SButton>
		{/if}

		{#if !onlyTop}
			<button class="btn btn-lg variant-filled" on:click={() => modalStore.trigger(aiChatModal)}
				>AI Chat</button
			>
			{#if showQA && questions.length > 0}
				<button class="btn btn-lg variant-filled" on:click={() => modalStore.trigger(delegateQAModal)}
					>Vorstellung</button
				>
			{/if}
		{/if}
	</footer>
</div>
