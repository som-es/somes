<script lang="ts">
	import { t } from '$lib/i18n/i18n.svelte';
	import { delegate_by_id, errorToNull } from '$lib/api/api';
	import { partyToColor } from '$lib/partyColor';
	import type { Delegate, Interjection } from '$lib/types';
	import SpeechModal from '../Speeches/SpeechModal.svelte';
	import SpeechDelegateHeader from '../Speeches/SpeechDelegateHeader.svelte';

	interface Props {
		interjection: Interjection;
		ty: 'issued' | 'received';
	}

	let { interjection, ty }: Props = $props();

	let delegate = $state<Delegate | null>(null);
	let loading = $state(true);
	let modalOpen = $state(false);

	let fetchedSpeaker = $state<Delegate | null>(null);
	let speaker = $derived(ty === 'issued' ? delegate : fetchedSpeaker);

	$effect(() => {
		loading = true;
		delegate_by_id(
			ty === 'issued' ? interjection.speaker_delegate_id : interjection.interjector_delegate_id
		).then((res) => {
			delegate = errorToNull(res);
			loading = false;
		});
	});

	$effect(() => {
		if (!modalOpen || ty === 'issued' || fetchedSpeaker) return;
		delegate_by_id(interjection.speaker_delegate_id).then((res) => {
			fetchedSpeaker = errorToNull(res);
		});
	});
</script>

{#if !loading && delegate}
	<div
		class="flex w-full cursor-pointer items-center gap-4 rounded-lg bg-primary-200 p-3 shadow-md transition-colors hover:bg-primary-400 dark:bg-primary-600 dark:hover:bg-primary-700"
		role="button"
		tabindex="0"
		title={t('interjections.openSpeech')}
		onclick={() => (modalOpen = true)}
		onkeypress={(e) => (e.key === 'Enter' || e.key === ' ') && (modalOpen = true)}
	>
		<div
			class="flex min-w-28 flex-col items-center justify-center border-r border-primary-500/50 pr-4"
		>
			<span
				class="text-primary-950 text-[10px] font-bold tracking-widest uppercase dark:text-primary-100"
			>
				{ty === 'issued' ? t('interjections.labelIssued') : t('interjections.labelReceived')}
			</span>
			<div class="mt-1 flex items-center gap-2">
				<div
					class="h-2 w-2 rounded-full ring-1 ring-white/20"
					style="background-color: {partyToColor(delegate.party)}"
				></div>
				<span class="text-sm font-bold">
					{delegate.name}
				</span>
			</div>
		</div>

		<div class="flex-1 overflow-hidden">
			{#if interjection.interjection_text}
				<p class="text-sm leading-relaxed italic">
					&ldquo;{interjection.interjection_text}&rdquo;
				</p>
			{:else}
				<p class="text-sm font-medium text-tertiary-200">
					{t('interjections.noText')}
				</p>
			{/if}
		</div>
	</div>

	<SpeechModal speech={interjection.plenar_speech_id} bind:open={modalOpen}>
		{#snippet header()}
			{#if speaker}
				<div class="mb-1.5">
					<SpeechDelegateHeader delegate={speaker} />
				</div>
			{/if}
		{/snippet}
	</SpeechModal>
{/if}
