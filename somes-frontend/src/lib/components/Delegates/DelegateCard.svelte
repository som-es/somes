<script lang="ts">
	import { t } from '$lib/i18n/i18n.svelte';
	import { partyToColor, partyColors as globalPartyColors } from '$lib/partyColor';
	import type { Delegate, DelegateFavo, DelegateQA } from '$lib/types';
	import { currentDelegateStore } from '$lib/stores/stores';
	import { gotoHistory } from '$lib/goto';
	import star from '$lib/assets/misc_icons/star.svg?raw';
	import starFilled from '$lib/assets/misc_icons/starFilled.svg?raw';
	import externalLink from '$lib/assets/misc_icons/external-link.svg?raw';
	import { onMount } from 'svelte';
	import { cachedDelegateFavos } from '$lib/caching/favos';
	import { addDelegateFavo, removeDelegateFavo } from '$lib/api/authed';
	import { url } from '$lib/api/api';
	import AIChatModal from './AIChat/AIChatModal.svelte';
	import DelegateQuestionModal from './Questions/DelegateQuestionModal.svelte';
	import { Dialog } from 'bits-ui';
	import DelegateQAModal from './QA/DelegateQAModal.svelte';
	import { getParliament, plink, type Parliament } from '$lib/api/parliament';
	import type { SvelteMap } from 'svelte/reactivity';

	const onShowDetailsDefault = () => {
		currentDelegateStore.value = delegate;
		gotoHistory(plink('/delegates'), true);
	};

	interface Props {
		delegate: Delegate;
		onlyTop?: boolean;
		showQA?: boolean;
		showAI?: boolean;
		showMandates?: boolean;
		questions?: DelegateQA[];
		showMoreDetailsBtn?: boolean;
		showImg?: boolean;
		showAge?: boolean;
		title?: string | null;
		date?: string;
		top?: import('svelte').Snippet;
		info?: import('svelte').Snippet;
		footerButtons?: import('svelte').Snippet;
		partyColors?: Map<string, string>;
		parliament?: Parliament;
		onShowDetails?: () => void;
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
		showMandates = true,
		title = null,
		date,
		top,
		info,
		footerButtons,
		partyColors = globalPartyColors,
		parliament = getParliament(),
		onShowDetails = onShowDetailsDefault
	}: Props = $props();

	const showDelegate = import.meta.env.VITE_SHOW_DELEGATE_ID;

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
	let personUrl = $derived.by(() => {
		switch (parliament) {
			case 'at':
				return `https://parlament.gv.at/person/${delegate.id}?utm_source=somes.at`;
			case 'eu':
				return `https://www.europarl.europa.eu/meps/en/${delegate.id}?utm_source=somes.at`;
		}
	});

	const mandatesToDisplay = $derived.by(() => {
		if (date) {
			const cmpDate = new Date(date);
			return delegate.mandates?.filter((mandate) => {
				if (!mandate.start_date) {
					return false;
				}
				let startDate = new Date(mandate.start_date);
				let endDate = mandate.end_date ? new Date(mandate.end_date) : new Date();
				return cmpDate >= startDate && cmpDate <= endDate;
			});
		} else {
			return delegate.mandates_at_time;
		}
	});
	let imgSrc = $derived(
		parliament == 'at' ? `${url}assets/${delegate.id}.jpg` : delegate.image_url
	);

	function handleImgError() {
		imgSrc = delegate.image_url ?? '';
	}
</script>

<div class="flex h-[calc(100%-1rem)] h-full flex-col card bg-primary-200 p-5 dark:bg-primary-400">
	<!-- Top Row: Fav button & External Link -->
	<div class="flex w-full items-center justify-between">
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
					class="h-5 w-5 text-yellow-500"
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
					class="h-5 w-5 text-gray-400 hover:text-yellow-500"
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
				src={imgSrc}
				onerror={handleImgError}
				class="w-42 rounded-full md:w-46"
				alt="Image of politician {delegate.name}"
			/>
			<span class="absolute bottom-0 rounded px-1 text-[10px]">
				{#if delegate.image_copyright}
					&copy {delegate.image_copyright}
				{/if}
			</span>
		</div>
	{/if}

	<!-- Delegate name and party-->
	<div>
		<!-- Name and Age -->
		<h4 class="text-xl font-bold">
			{delegate.name}
			{#if delegate.is_active && showAge && delegate.birthdate}
				- {Math.floor(dateDiffInDays(new Date(delegate.birthdate), new Date()) / 365)}
			{/if}
		</h4>
		<!-- Birthday Check -->
		{#if delegate.birthdate && new Date().toString() == new Date(delegate.birthdate).toString()}
			<hr />
			{t('delegate.birthday')}
		{/if}

		<!-- Party -->
		<div class="flex items-center">
			<div
				class="mx-2 h-2 w-2 rounded-full"
				style="background-color: {partyToColor(delegate.party, partyColors)}"
			></div>
			<p class="text-base text-gray-800 dark:text-gray-50">
				{#if delegate.party == null || delegate.party == 'OK'}
					{t('delegate.withoutParty')}
				{:else}
					<span>{delegate.party}</span>
				{/if}
			</p>
		</div>
	</div>

	{#if showMandates}
		<!-- Mandate if so -->
		<div class="mt-4">
			{#each mandatesToDisplay ?? [] as mandate}
				<div class="mt-1 flex w-full items-center">
					<h6 class="text-base text-wrap xl:leading-tight">
						{mandate.name}
					</h6>
				</div>
			{/each}
		</div>
	{/if}

	{@render top?.()}
	{@render info?.()}
	{#if !onlyTop}
		<div>
			{#if delegate.constituency != null}
				<hr class="my-1 !border-t-2 border-gray-500" />
				<h3>{delegate.constituency}</h3>
			{/if}
			{#if delegate.divisions != null && delegate.divisions.length > 0}
				<hr class="my-1 !border-t-2 border-gray-500" />
				<h3>{delegate.divisions.join(', ')}</h3>
			{/if}
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
			{#if !footerButtons && !showAI}
				<div></div>
			{/if}
			<button class="rounded-xl bg-primary-600 p-2 px-3 text-white" onclick={onShowDetails}>
				<h4>{t('ui.details')}</h4>
			</button>
		{/if}

		{#if !onlyTop}
			<Dialog.Root>
				<Dialog.Trigger>
					<div class="rounded-xl bg-secondary-500 p-2 px-3 text-white">
						<h4>Frage stellen</h4>
					</div>
				</Dialog.Trigger>
				<Dialog.Portal>
					<Dialog.Overlay
						class="fixed inset-0 z-50 bg-black/80 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:animate-in data-[state=open]:fade-in-0"
					/>
					<Dialog.Content
						class="fixed top-[50%] left-[50%] z-50 w-full max-w-xl translate-x-[-50%] translate-y-[-50%] overflow-hidden rounded-lg bg-primary-100 shadow-lg outline-hidden data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 dark:bg-gray-800"
					>
						<DelegateQuestionModal {delegate} />
					</Dialog.Content>
				</Dialog.Portal>
			</Dialog.Root>

			{#if showAI}
				<Dialog.Root>
					<Dialog.Trigger>
						<div class="rounded-xl bg-primary-600 p-2 px-3 text-white">
							<h4>AI Chat</h4>
						</div>
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
						<div class="rounded-xl bg-primary-600 p-2 px-3 text-white">
							<h4>Vorstellung</h4>
						</div>
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
