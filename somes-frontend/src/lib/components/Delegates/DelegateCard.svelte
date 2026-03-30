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
	import type { SvelteMap, SvelteSet } from 'svelte/reactivity';

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

	let delegateFavos: SvelteMap<number, DelegateFavo> | null = $state(null);
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

<div class="flex h-[calc(100%-1rem)] h-full flex-col card bg-primary-200 p-5 dark:bg-primary-400">
	<!-- Top Row: Fav button & External Link -->
	<div class="flex w-full items-center justify-end gap-2">
		<!-- Favorite Button -->
		{#if delegateFavos}
			{#if delegateFavos.has(delegate.id)}
				<button
					onclick={async () => {
						if (
							(await removeDelegateFavo({ delegate_id: delegate.id, user_info_days: 0 })) == null
						) {
							delegateFavos?.delete(delegate.id);
							delegateFavos = delegateFavos;
						}
					}}
					class="h-6 w-6 text-yellow-500"
				>
					{@html starFilled}
				</button>
			{:else}
				<button
					onclick={async () => {
						if ((await addDelegateFavo({ delegate_id: delegate.id, user_info_days: 7 })) == null) {
							delegateFavos?.set(delegate.id, { delegate_id: delegate.id, user_info_days: 7 });
							delegateFavos = delegateFavos;
						}
					}}
					class="h-6 w-6 text-gray-500 hover:text-yellow-500"
				>
					{@html star}
				</button>
			{/if}
		{/if}

		<!-- Parlament.at link to person -->
		<div class="h-4 w-4 text-gray-500 dark:text-gray-200">
			<a href={personUrl} target="_blank">
				{@html externalLink}
			</a>
		</div>
	</div>

	<!-- Show image if avaiable -->
	{#if showImg}
		<div class="relative flex justify-center pb-6">
			<img
				src={`${url}assets/${delegate.id}.jpg`}
				class="w-24 rounded-full sm:w-32 md:w-46"
				alt="Image of politician {delegate.name}"
			/>
			<span class="absolute bottom-0 rounded px-1 text-[10px]">
				{#if delegate.image_copyright}
					&copy {delegate.image_copyright}
				{:else}
					&copy Parlamentsdirektion
				{/if}
			</span>
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
			<div
				class="mx-2 h-2 w-2 rounded-full"
				style="background-color: {partyToColor(delegate.party)}"
			></div>
			<p class="text-base text-gray-800 dark:text-gray-50">
				{#if delegate.party == null || delegate.party == 'OK'}
					Ohne Klub
				{:else}
					<span>{delegate.party}</span>
				{/if}
			</p>
		</div>
	</div>

	<!-- Mandate if so -->
	<div class="mt-4">
		{#each delegate.mandates_at_time ?? [] as mandate}
			<div class="mt-1 flex w-full items-center">
				<h6 class="text-sm text-wrap md:text-base xl:leading-tight">
					{mandate.name}
				</h6>
			</div>
		{/each}
	</div>

	{@render top?.()}
	{@render info?.()}
	{#if !onlyTop}
		<div>
			<hr class="my-1 !border-t-2 border-gray-500" />
			{#if delegate.constituency != null}
				<h3>{delegate.constituency}</h3>
			{/if}
			<hr class="my-1 !border-t-2 border-gray-500" />
			<h3>{delegate.divisions?.join(', ')}</h3>
		</div>

		<br />
		{#if showDelegate == 'true'}
			ID: {delegate.id}
		{/if}
	{/if}

	<!-- Buttons -->
	<div class="mt-auto flex w-full items-end justify-between gap-1 pt-6">
		{@render footerButtons?.()}
		{#if showMoreDetailsBtn}
			<div></div>
			<button class="rounded-xl bg-primary-600 p-2 px-3 text-white" onclick={onShowDetails}>
				<h4>Details</h4>
			</button>
		{/if}

		{#if !onlyTop}
			{#if showAI}
				<Dialog.Root>
					<Dialog.Trigger>
						<button class="rounded-xl bg-primary-600 p-2 px-3 text-white">
							<h4>AI Chat</h4>
						</button>
					</Dialog.Trigger>
					<Dialog.Portal>
						<Dialog.Overlay
							class="fixed inset-0 z-50 bg-black/80 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:animate-in data-[state=open]:fade-in-0"
						/>

						<Dialog.Content
							class="
							shadow-popover z -50 fixed top-[50%] left-[50%] z-50 h-[90vh] w-full max-w-4xl translate-x-[-50%] translate-y-[-50%] overflow-hidden rounded-lg bg-primary-100 shadow-lg outline-hidden data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 dark:bg-gray-800"
						>
							<AIChatModal {delegate} />
						</Dialog.Content>
					</Dialog.Portal>
				</Dialog.Root>
			{/if}

			{#if showQA && questions.length > 0}
				<Dialog.Root>
					<Dialog.Trigger>
						<button class="rounded-xl bg-primary-600 p-2 px-3 text-white">
							<h4>Vorstellung</h4>
						</button>
					</Dialog.Trigger>
					<Dialog.Portal>
						<Dialog.Overlay
							class="fixed inset-0 z-30 bg-black/80 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:animate-in data-[state=open]:fade-in-0"
						/>
						<Dialog.Content
							class="
							    fixed
								top-[50%] left-[50%] z-30
								h-[90vh] w-full max-w-7xl
								translate-x-[-50%] translate-y-[-50%] overflow-hidden overflow-y-scroll rounded-lg bg-primary-100 shadow-lg outline-hidden data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 dark:bg-gray-800"
						>
							<DelegateQAModal {questions} />
						</Dialog.Content>
					</Dialog.Portal>
				</Dialog.Root>
			{/if}
		{/if}
	</div>
</div>
