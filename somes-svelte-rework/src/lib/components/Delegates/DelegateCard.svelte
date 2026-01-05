<script lang="ts">
	import { partyToColor } from '$lib/partyColor';
	import type { Delegate, DelegateFavo, DelegateQA, Mandate } from '$lib/types';
	import { type ModalSettings } from '@skeletonlabs/skeleton-svelte';
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

<div
	class="z-0! card bg-primary-200 {onlyTop ? '' : 'min-h-full'}  mx-4 drop-shadow-lg flex flex-col"
>
	<header class="relative">
		{#if delegateFavos}
			{#if delegateFavos.has(delegate.id)}
				<button
					on:click={async () => {
						if ((await removeDelegateFavo({ delegate_id: delegate.id })) == null) {
							delegateFavos?.delete(delegate.id);
							delegateFavos = delegateFavos;
						}
					}}
					class="absolute top-0 right-0 w-14 p-2 z-10"
				>
					{@html starFilled}
				</button>
			{:else}
				<button
					on:click={async () => {
						if ((await addDelegateFavo({ delegate_id: delegate.id })) == null) {
							delegateFavos?.add(delegate.id);
							delegateFavos = delegateFavos;
						}
					}}
					class="absolute top-0 right-0 w-14 p-2 z-10"
				>
					{@html star}
				</button>
			{/if}
		{/if}
		<a
			class="absolute {delegateFavos ? 'top-10' : 'top-0'} right-0 mt-2 mr-3 z-10"
			href={personUrl}
			target="_blank"
		>
			<img
				class="w-12"
				alt="parlament.gv.at favicon"
				src="https://www.parlament.gv.at/static/img/favicon/favicon.svg"
			/>
		</a>
		<div class="relative flex justify-center items-center h-full">
			{#if showImg}
				<img
					src={`${address}/assets/${delegate.id}.jpg`}
					class="rounded-full w-32 sm:w-44 md:w-52"
					alt="Image of politician {delegate.name}"
				/>
			{/if}
		</div>
	</header>

	<section class="p-4 grow">
		<h4 class="font-bold md:text-xl">
			{delegate.name}
			{#if delegate.is_active && showAge}
				- {Math.floor(dateDiffInDays(new Date(delegate.birthdate), new Date()) / 365)}
			{/if}
		</h4>
		{#if new Date().toString() == new Date(delegate.birthdate).toString()}
			<hr />
			Alles Gute zum Geburtstag!
		{/if}

		<h5 class="text-sm" style="color: {partyToColor(delegate.party)}">
			{#if delegate.party == 'OK'}
				Ohne Klub
			{:else if delegate.party != null}
				<span>{delegate.party}</span>
			{/if}
		</h5>
		{#each delegate.mandates_at_time ?? [] as mandate}
			<h6 class="text-sm md:text-lg">
				{mandate.name}
			</h6>
		{/each}

		{#if !onlyTop}
			<hr class="border-t-2! my-1" />
			{#if delegate.constituency != null}
				<h3>{delegate.constituency}</h3>
			{/if}
			<hr class="border-t-2! my-1" />
			<h3>{delegate.divisions?.join(', ')}</h3>
		{/if}

		<slot name="title"></slot>
		<slot name="info"></slot>

		<br />
		{#if showDelegate == 'true'}
			ID: {delegate.id}
		{/if}
		<!-- </span> -->
	</section>

	<hr class="border-t-2! my-1" />
	<!-- <footer class="card-footer flex justify-end items-end mt-3"> -->
	<footer class="card-footer flex justify-between mt-1">
		<slot name="footerButtons"></slot>
		{#if showMoreDetailsBtn}
			<div></div>
			<SButton class="bg-tertiary-500 text-black" on:click={onShowDetails}>Zur Person</SButton>
		{/if}

		{#if !onlyTop}
			{#if showAI}
				<button
					class="btn sm:btn-lg preset-filled"
					on:click={() => modalStore.trigger(aiChatModal)}>AI Chat</button
				>
			{/if}
			{#if showQA && questions.length > 0}
				<button
					class="btn sm:btn-lg preset-filled"
					on:click={() => modalStore.trigger(delegateQAModal)}>Vorstellung</button
				>
			{/if}
		{/if}
	</footer>
</div>
