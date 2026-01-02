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
	import { delegatesStore } from '$lib/caching/stores/stores';
	import { address } from '$lib/api/api';
	import externalLink from '$lib/assets/misc_icons/external-link.svg?raw';

	export let delegate: Delegate;
	export let onlyTop: boolean = false;
	export let showQA: boolean = false;
	export let showAI: boolean = true;
	export let questions: DelegateQA[] = [];
	export let showMoreDetailsBtn = false;
	export let showImg = true;
	export let showAge = true;
	export let title: string | null = null;

	const showDelegate = import.meta.env.VITE_SHOW_DELEGATE_ID;

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

	function dateDiffInDays(a: Date, b: Date) {
		const _MS_PER_DAY = 1000 * 60 * 60 * 24;
		const utc1 = Date.UTC(a.getFullYear(), a.getMonth(), a.getDate());
		const utc2 = Date.UTC(b.getFullYear(), b.getMonth(), b.getDate());

		return Math.floor((utc2 - utc1) / _MS_PER_DAY);
	}

	const modalStore = getModalStore();

	$: personUrl = `https://parlament.gv.at/person/${delegate.id}?utm_source=somes.at`;
</script>


<div class="card bg-primary-200 p-5 h-full flex flex-col h-[calc(100%-1rem)]">
	<!-- Parlament.at link to person -->
	<div class="w-full flex justify-end">
		<div class="w-4 h-4 text-gray-500">
			<a href={personUrl} target="_blank">
				{@html externalLink}
			</a>
		</div>
	</div>
	<!-- Show image if avaiable -->
	{#if showImg}
		<div class="flex justify-center pb-6">
			<img
				src={`${address}/assets/${delegate.id}.jpg`}
				class="rounded-full w-32 sm:w-44 md:w-52"
				alt="Image of politician {delegate.name}"
			/>
		</div>
	{/if}

	<!-- Delegate name and party-->
	<div>
		<!-- Name and Age -->
		<h4 class="font-bold md:text-xl">
			{delegate.name}
			{#if delegate.is_active && showAge}
				- {Math.floor(dateDiffInDays(new Date(delegate.birthdate), new Date()) / 365)}
			{/if}
		</h4>
		<!-- Party -->
		<div class="flex items-center">
			<div class="w-2 h-2 rounded-full mx-2" style="background-color: {partyToColor(delegate.party)}"></div>
			<p class="text-base text-gray-800">
				{#if delegate.party == 'OK'}
					Ohne Klub
				{:else if delegate.party != null}
					<span>{delegate.party}</span>
				{/if}
			</p>
		</div>
	</div>
	<!-- Mandate if so -->
	<div class="mt-4">
		{#each delegate.mandates_at_time ?? [] as mandate}
		<div class="flex w-full items-center mt-1">
			<h6 class="text-sm text-wrap md:text-base xl:leading-tight">
				{mandate.name}
			</h6>
		</div>
		{/each}
	</div>

	<div>
		<hr class="!border-t-2 my-1" />
		{#if delegate.constituency != null}
			<h3>{delegate.constituency}</h3>
		{/if}
		<hr class="!border-t-2 my-1" />
		<h3>{delegate.divisions?.join(', ')}</h3>
	</div>
	<!-- Buttons -->
	<div class="w-full flex justify-between mt-auto">
		<button 
			class="bg-primary-600 p-2 px-3 rounded-xl text-white"
			on:click={() => modalStore.trigger(aiChatModal)}
		>
			<h4>AI Chat</h4>
		</button>
		<button 
			class="bg-primary-600 p-2 px-3 rounded-xl text-white"
			on:click={() => modalStore.trigger(delegateQAModal)}
		>
			<h4>Vorstellung</h4>
		</button>
	</div>
</div>