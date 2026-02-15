<script lang="ts">
	import { partyToColor } from '$lib/partyColor';
	import type { Delegate, DelegateFavo, DelegateQA, Mandate } from '$lib/types';
	import SButton from '../UI/SButton.svelte';
	import { currentDelegateStore } from '$lib/stores/stores';
	import { gotoHistory } from '$lib/goto';
	import star from '$lib/assets/misc_icons/star.svg?raw';
	import starFilled from '$lib/assets/misc_icons/starFilled.svg?raw';
	import externalLink from '$lib/assets/misc_icons/external-link.svg?raw';
	import { onMount } from 'svelte';
	import { cachedDelegateFavos } from '$lib/caching/favos';
	import { addDelegateFavo, removeDelegateFavo } from '$lib/api/authed';
	import { delegatesStore } from '$lib/caching/stores/stores.svelte';
	import { address, url } from '$lib/api/api';
	import AIChatModal from './AIChat/AIChatModal.svelte';
	import { Dialog } from 'bits-ui';
	import DelegateQAModal from './QA/DelegateQAModal.svelte';
	import { resolve } from '$app/paths';

	interface Props {
		delegate: Delegate;
		onlyTop?: boolean;
		showQA?: boolean;
		showAI?: boolean;
		questions?: DelegateQA[];
		showMoreDetailsBtn?: boolean;
		showImg?: boolean;
		showAge?: boolean;
		title?: string | null;
		top?: import('svelte').Snippet;
		info?: import('svelte').Snippet;
		footerButtons?: import('svelte').Snippet;
	}

	let {
		delegate,
		onlyTop = false,
		showQA = false,
		showAI = true,
		questions = [],
		showMoreDetailsBtn = false,
		showImg = true,
		showAge = true,
		title = null,
		top,
		info,
		footerButtons
	}: Props = $props();

	const showDelegate = import.meta.env.VITE_SHOW_DELEGATE_ID;

	const onShowDetails = () => {
		currentDelegateStore.value = delegate;
		gotoHistory(resolve(`/delegates`), true);
	};

	let delegateFavos: Set<number> | null = $state(null);
	onMount(async () => {
		delegateFavos = await cachedDelegateFavos();
	});

	function dateDiffInDays(a: Date, b: Date) {
		const _MS_PER_DAY = 1000 * 60 * 60 * 24;
		const utc1 = Date.UTC(a.getFullYear(), a.getMonth(), a.getDate());
		const utc2 = Date.UTC(b.getFullYear(), b.getMonth(), b.getDate());

		return Math.floor((utc2 - utc1) / _MS_PER_DAY);
	}
	let personUrl = $derived(`https://parlament.gv.at/person/${delegate.id}?utm_source=somes.at`);
</script>

<div class="card bg-primary-200 dark:bg-primary-400 p-5 h-full flex flex-col h-[calc(100%-1rem)]">
	<!-- Top Row: Fav button & External Link -->
	<div class="w-full flex justify-end gap-2 items-center">
		<!-- Favorite Button -->
		{#if delegateFavos}
			{#if delegateFavos.has(delegate.id)}
				<button
					onclick={async () => {
						if ((await removeDelegateFavo({ delegate_id: delegate.id })) == null) {
							delegateFavos?.delete(delegate.id);
							delegateFavos = delegateFavos;
						}
					}}
					class="w-6 h-6 text-yellow-500"
				>
					{@html starFilled}
				</button>
			{:else}
				<button
					onclick={async () => {
						if ((await addDelegateFavo({ delegate_id: delegate.id })) == null) {
							delegateFavos?.add(delegate.id);
							delegateFavos = delegateFavos;
						}
					}}
					class="w-6 h-6 text-gray-500 hover:text-yellow-500"
				>
					{@html star}
				</button>
			{/if}
		{/if}

		<!-- Parlament.at link to person -->
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
				src={`${url}/assets/${delegate.id}.jpg`}
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
		<!-- Birthday Check -->
		{#if new Date().toString() == new Date(delegate.birthdate).toString()}
			<hr />
			Alles Gute zum Geburtstag!
		{/if}

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

	{#if !onlyTop}
		<div>
			<hr class="!border-t-2 my-1 border-gray-500" />
			{#if delegate.constituency != null}
				<h3>{delegate.constituency}</h3>
			{/if}
			<hr class="!border-t-2 my-1 border-gray-500" />
			<h3>{delegate.divisions?.join(', ')}</h3>
		</div>

		{@render top?.()}
		{@render info?.()}

		<br />
		{#if showDelegate == 'true'}
			ID: {delegate.id}
		{/if}
	{/if}

	<!-- Buttons -->
	<div class="w-full flex justify-between mt-auto items-end">
		{@render footerButtons?.()}
		{#if showMoreDetailsBtn}
			<div></div>
			<SButton class="bg-tertiary-500 text-black" on:click={onShowDetails}>Zur Person</SButton>
		{/if}

		{#if !onlyTop}
			{#if showAI}
				<Dialog.Root>
					<Dialog.Trigger>
						<button class="bg-primary-600 p-2 px-3 rounded-xl text-white">
							<h4>AI Chat</h4>
						</button>
					</Dialog.Trigger>
					<Dialog.Portal>
						<Dialog.Overlay
							class="data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 fixed inset-0 z-50 bg-black/80"
						/>

						<Dialog.Content
							class="
							shadow-popover data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 outline-hidden fixed left-[50%] top-[50%] z-50 translate-x-[-50%] translate-y-[-50%] z -50 w-full max-w-4xl h-[90vh] bg-primary-100 dark:bg-gray-800 shadow-lg rounded-lg overflow-hidden"
						>
							<AIChatModal {delegate} />
						</Dialog.Content>
					</Dialog.Portal>
				</Dialog.Root>
			{/if}

			{#if showQA && questions.length > 0}
				<Dialog.Root>
					<Dialog.Trigger>
						<button class="bg-primary-600 p-2 px-3 rounded-xl text-white">
							<h4>Vorstellung</h4>
						</button>
					</Dialog.Trigger>
					<Dialog.Portal>
						<Dialog.Overlay
							class="data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 fixed inset-0 z-30 bg-black/80"
						/>
						<Dialog.Content
							class="
							    overflow-y-scroll
								data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 
								data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 
								outline-hidden fixed left-[50%] top-[50%] z-30 translate-x-[-50%] translate-y-[-50%] w-full max-w-7xl h-[90vh] bg-primary-100 dark:bg-gray-800 shadow-lg rounded-lg overflow-hidden"
						>
							<DelegateQAModal {questions} />
						</Dialog.Content>
					</Dialog.Portal>
				</Dialog.Root>
			{/if}
		{/if}
	</div>
</div>
