<script lang="ts">
	import { t } from '$lib/i18n/i18n.svelte';
	import {
		delegate_by_id,
		errorToNull,
		interjections_made_by_delegate_per_page,
		interjections_received_by_delegate_per_page
	} from '$lib/api/api';
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';
	import type { Delegate, Interjection, InterjectionsWithMaxPage } from '$lib/types';
	import { Dialog } from 'bits-ui';
	import InterjectionBar from './InterjectionBar.svelte';
	import Pagination from '$lib/components/Pagination.svelte';
	import SpeechModal from '../Speeches/SpeechModal.svelte';
	import SpeechDelegateHeader from '../Speeches/SpeechDelegateHeader.svelte';

	interface Props {
		delegateId: number;
		interjectionsPage0: InterjectionsWithMaxPage;
		ty: 'issued' | 'received';
	}

	let { delegateId, interjectionsPage0, ty }: Props = $props();

	let currentPageInterjections = $derived(interjectionsPage0.interjections);

	const interjectionsPerPage = $derived(
		ty === 'issued'
			? interjections_made_by_delegate_per_page
			: interjections_received_by_delegate_per_page
	);

	let page = $state(1);
	$effect(() => {
		interjectionsPerPage(delegateId, page - 1).then((res) => {
			currentPageInterjections = errorToNull(res)?.interjections ?? [];
		});
	});

	const delegateCache = new Map<number, Promise<Delegate | null>>();
	function loadDelegate(id: number): Promise<Delegate | null> {
		let cached = delegateCache.get(id);
		if (!cached) {
			cached = delegate_by_id(id).then(errorToNull);
			delegateCache.set(id, cached);
		}
		return cached;
	}

	let speechModalId = $state<number | null>(null);
	let speechModalOpen = $state(false);
	let speechModalSpeaker = $state<Delegate | null>(null);

	async function openSpeech(interjection: Interjection) {
		speechModalId = interjection.plenar_speech_id;
		speechModalSpeaker = null;
		speechModalOpen = true;
		const speaker = await loadDelegate(interjection.speaker_delegate_id);
		if (speechModalId === interjection.plenar_speech_id) speechModalSpeaker = speaker;
	}
</script>

<div class="card px-4">
	<div class="flex items-center justify-between p-8">
		<h1 class="text-xl font-bold lg:text-2xl">
			{ty === 'issued'
				? t('interjections.modalTitleIssued')
				: t('interjections.modalTitleReceived')}
		</h1>
		<Dialog.Close>
			<ModalCloseButton />
		</Dialog.Close>
	</div>

	<div class="flex flex-col gap-2">
		{#if currentPageInterjections.length === 0}
			<div class="w-full rounded-lg bg-surface-100-900 p-20 text-center">{t('ui.none')}</div>
		{/if}
		{#each currentPageInterjections as interjection (interjection)}
			{#await loadDelegate(ty === 'issued' ? interjection.speaker_delegate_id : interjection.interjector_delegate_id)}
				<InterjectionBar delegate={null} text={interjection.interjection_text} />
			{:then delegate}
				<InterjectionBar
					{delegate}
					text={interjection.interjection_text}
					onclick={() => openSpeech(interjection)}
				/>
			{/await}
		{/each}
	</div>

	<div class="float-right">
		<Pagination bind:dynPage={page} maxPage={interjectionsPage0.max_page} />
	</div>
</div>

{#if speechModalId !== null}
	{#key speechModalId}
		<SpeechModal speech={speechModalId} bind:open={speechModalOpen}>
			{#snippet header()}
				{#if speechModalSpeaker}
					<div class="mb-1.5">
						<SpeechDelegateHeader delegate={speechModalSpeaker} />
					</div>
				{/if}
			{/snippet}
		</SpeechModal>
	{/key}
{/if}
