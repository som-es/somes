<script lang="ts" module>
	import type { FullSpeech } from '$lib/speechTypes';
	import type { Delegate } from '$lib/types';

	export interface KeypointSpeaker {
		delegate: Delegate;
		speech: FullSpeech;
		/** Indizes in die Schwerpunkte der Rede, die diesen Antrags-Schwerpunkt betreffen. */
		pointIndexes: number[];
	}

	/**
	 * Über alle Schwerpunkte hinweg geteilt, damit beim Wandern mit der Maus
	 * nicht mehrere Popups gleichzeitig offen bleiben. Der Schlüssel enthält die
	 * Instanz, weil dieselbe Rede zu mehreren Schwerpunkten gehören kann.
	 */
	let openKey: string | null = $state(null);
	let instanceCount = 0;
</script>

<script lang="ts">
	import { Dialog, Popover } from 'bits-ui';
	import { t } from '$lib/i18n/i18n.svelte';
	import { url } from '$lib/api/api';
	import SpeechModal from '$lib/components/Delegates/Speeches/SpeechModal.svelte';
	import SpeechBar from '$lib/components/Delegates/Speeches/SpeechBar.svelte';
	import SpeechDelegateHeader from '$lib/components/Delegates/Speeches/SpeechDelegateHeader.svelte';
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';

	interface Props {
		speakers: KeypointSpeaker[];
		/** Der Schwerpunkt selbst, als Überschrift der vollständigen Liste. */
		pointText?: string;
	}

	let { speakers, pointText }: Props = $props();

	const instanceId = instanceCount++;
	const MAX_VISIBLE = 5;

	const keyOf = (speaker: KeypointSpeaker) => `${instanceId}:${speaker.speech.id}`;

	let visible = $derived(speakers.slice(0, MAX_VISIBLE));
	let restCount = $derived(speakers.length - visible.length);

	let openSpeaker: KeypointSpeaker | null = $state(null);
	let modalOpen = $state(false);
	let listOpen = $state(false);

	// clises popup when modal is open
	$effect(() => {
		if (modalOpen) openKey = null;
	});

	function speechPoints(speaker: KeypointSpeaker): string[] {
		const points = speaker.speech.ai_summary?.full_speech_summary.key_points ?? [];
		return speaker.pointIndexes.map((i) => points[i]?.summarized_point).filter((p) => !!p);
	}
</script>

{#if speakers.length > 0}
	<div class="hidden shrink-0 items-center md:flex">
		{#each visible as speaker (speaker.speech.id)}
			<Popover.Root
				open={openKey === keyOf(speaker)}
				onOpenChange={(isOpen) => {
					if (isOpen) openKey = keyOf(speaker);
					else if (openKey === keyOf(speaker)) openKey = null;
				}}
			>
				<Popover.Trigger
					openOnHover
					openDelay={0}
					title={speaker.delegate.name}
					class="-ml-2 transition-transform first:ml-0 hover:z-10 hover:scale-110"
					onclick={() => {
						openSpeaker = speaker;
						modalOpen = true;
					}}
				>
					<img
						src={`${url}assets/${speaker.delegate.id}.jpg`}
						alt={speaker.delegate.name}
						class="h-6 w-6 rounded-full object-cover text-[1px] ring-2 ring-primary-300 dark:ring-primary-500"
					/>
				</Popover.Trigger>
				<Popover.Content
					align="start"
					collisionPadding={8}
					class="z-50! w-72 max-w-[calc(100vw-2rem)] card bg-primary-300-700 p-3 shadow-xl"
				>
					<div class="font-semibold">{speaker.delegate.name}</div>
					<ul class="mt-1 flex flex-col gap-1">
						{#each speechPoints(speaker) as point, i (i)}
							<li class="text-sm text-gray-800 dark:text-gray-200">{point}</li>
						{/each}
					</ul>
					<div class="mt-2 text-xs text-gray-700 dark:text-gray-300">
						{t('emphasis.clickForFullSpeech')}
					</div>
				</Popover.Content>
			</Popover.Root>
		{/each}

		{#if restCount > 0}
			<button
				title={t('emphasis.showAllSpeeches', { count: speakers.length })}
				class="-ml-2 flex h-6 min-w-6 shrink-0 items-center justify-center rounded-full bg-primary-600 px-1 text-[10px] font-semibold text-white ring-2 ring-primary-300 transition-transform hover:z-10 hover:scale-110 dark:ring-primary-500"
				onclick={() => (listOpen = true)}
			>
				+{restCount}
			</button>
		{/if}
	</div>

	{#if openSpeaker}
		{@const speaker = openSpeaker}
		<SpeechModal speech={speaker.speech} bind:open={modalOpen}>
			{#snippet header()}
				<div class="mb-1.5">
					<SpeechDelegateHeader delegate={speaker.delegate} />
				</div>
			{/snippet}
		</SpeechModal>
	{/if}

	<Dialog.Root bind:open={listOpen}>
		<Dialog.Portal>
			<Dialog.Overlay
				class="fixed inset-0 z-70 bg-black/80 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:animate-in data-[state=open]:fade-in-0"
			/>
			<Dialog.Content
				class="fixed top-[50%] left-[50%] z-70 h-[90vh] w-4xl max-w-[90%] translate-x-[-50%] translate-y-[-50%] overflow-y-auto rounded-lg bg-primary-100 shadow-lg outline-hidden data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:animate-in data-[state=open]:fade-in-0 dark:bg-primary-300"
			>
				<div class="p-5 text-black lg:p-8">
					<div class="flex items-start justify-between gap-3">
						<div class="min-w-0">
							<h1 class="text-xl font-bold lg:text-2xl">{t('emphasis.speechesTitle')}</h1>
							{#if pointText}
								<p class="mt-1 text-base text-gray-800 dark:text-gray-200">{pointText}</p>
							{/if}
							<p class="mt-1 text-sm text-gray-800 dark:text-gray-300">
								{speakers.length}
								{speakers.length === 1 ? t('emphasis.speech.one') : t('emphasis.speech.other')}
							</p>
						</div>
						<Dialog.Close>
							<ModalCloseButton />
						</Dialog.Close>
					</div>

					{#each speakers as speaker (speaker.speech.id)}
						<SpeechBar speech={speaker.speech}>
							{#snippet header()}
								<SpeechDelegateHeader delegate={speaker.delegate} />
							{/snippet}
						</SpeechBar>
					{/each}
				</div>
			</Dialog.Content>
		</Dialog.Portal>
	</Dialog.Root>
{/if}
