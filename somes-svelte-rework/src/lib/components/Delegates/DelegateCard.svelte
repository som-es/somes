<script lang="ts">
	import { partyToColor } from '$lib/partyColor';
	import type { Delegate, DelegateFavo, DelegateQA, Mandate } from '$lib/types';
	import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
	import SButton from '../UI/SButton.svelte';
	import { currentDelegateStore } from '$lib/stores/stores';
	import { gotoHistory } from '$lib/goto';
	import star from '$lib/assets/misc_icons/star.svg?raw';
	import starFilled from '$lib/assets/misc_icons/starFilled.svg?raw';
	import { onMount } from 'svelte';
	import { cachedDelegateFavos } from '$lib/caching/favos';
	import { addDelegateFavo, removeDelegateFavo } from '$lib/api/authed';

	export let delegate: Delegate;
	export let onlyTop: boolean = false;
	export let showQA: boolean = false;
	export let showAI: boolean = true;
	export let questions: DelegateQA[] = [];
	export let showMoreDetailsBtn = false;
	export let showImg = true;
	export let title: string | null = null;

	const onShowDetails = () => {
		currentDelegateStore.set(delegate);
		gotoHistory(`/delegates`, true);
	};

	let delegateFavos: Set<number> | null = null;
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
		{#if delegateFavos}
			{#if delegateFavos.has(delegate.id)}
				<button on:click={async () => {
					if (await removeDelegateFavo({delegate_id: delegate.id}) == null) {
						delegateFavos?.delete(delegate.id);
						delegateFavos = delegateFavos;
					}

				}} class="absolute top-0 right-0 w-14 p-2">
					{@html starFilled}
				</button>
			{:else}
				<button on:click={async () => {
					if (await addDelegateFavo({delegate_id: delegate.id}) == null) {
						delegateFavos?.add(delegate.id);
						delegateFavos = delegateFavos;
					}

				}} class="absolute top-0 right-0 w-14 p-2">
					{@html star}
				</button>
			{/if}
		{/if}
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

		{#if delegate.active_mandates?.length == 0}
			<h6 class="text-lg">{delegate.primary_mandate}</h6>
		{:else}
			<h6 class="text-lg">{delegate.active_mandates?.join("\n")}</h6>
		{/if}

		{#if !onlyTop}
		<hr class="!border-t-2 my-1" />
		{#if delegate.constituency != null}
			<h3>{delegate.constituency}</h3>
		{/if}
		<hr class="!border-t-2 my-1" />
		<h3>{delegate.divisions?.join(', ')}</h3>
		{/if}
		<slot name="title">
		</slot>
		<slot name="info">
		</slot>

	</section>


	<hr class="!border-t-2 my-1" />
	<!-- <footer class="card-footer flex justify-end items-end mt-3"> -->
	<footer class="card-footer flex justify-between mt-1">
		<slot name="footerButtons">
		</slot>
		{#if showMoreDetailsBtn}
			<div></div>
			<SButton class="bg-tertiary-500 text-black" on:click={onShowDetails}>Zur Person</SButton>
		{/if}
		

		{#if !onlyTop}
			{#if showAI}
				<button class="btn btn-lg variant-filled" on:click={() => modalStore.trigger(aiChatModal)}
					>AI Chat</button
				>
			{/if}
			{#if showQA && questions.length > 0}
				<button class="btn btn-lg variant-filled" on:click={() => modalStore.trigger(delegateQAModal)}
					>Vorstellung</button
				>
			{/if}
		{/if}
	</footer>
</div>
