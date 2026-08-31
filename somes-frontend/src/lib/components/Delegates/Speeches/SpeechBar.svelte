<script lang="ts">
	import { t } from '$lib/i18n/i18n.svelte';
	import { aiViewEnabledStore } from '$lib/stores/stores';
	import clockIcon from '$lib/assets/misc_icons/clock-two.svg?raw';
	import { Opinion, type FullSpeech } from '$lib/speechTypes';
	import SpeechModal from './SpeechModal.svelte';
	import type { Snippet } from 'svelte';

	interface Props {
		speech: FullSpeech;
		header?: Snippet;
	}

	let { speech, header }: Props = $props();

	let modalOpen = $state(false);

	function stanceColor(stance: Opinion | null): string {
		if (stance === Opinion.Pro) return 'bg-green-600';
		if (stance === Opinion.Contra) return 'bg-red-500';
		return 'bg-gray-400';
	}

	let opinion = $derived(
		speech.speech.infavor != null
			? speech.speech.infavor
				? t('speeches.pro')
				: t('speeches.contra')
			: speech.speech.opinion
	);
	let barColor = $derived(
		stanceColor(
			speech.speech.infavor === true
				? Opinion.Pro
				: speech.speech.infavor === false
					? Opinion.Contra
					: null
		)
	);

	let speechDuration = $derived.by(() => {
		if (speech.speech.duration_in_seconds === null || speech.speech.duration_in_seconds === 0)
			return null;
		const mins = Math.floor(speech.speech.duration_in_seconds / 60);
		return { mins, seconds: speech.speech.duration_in_seconds - mins * 60 };
	});

	let aiSummary = $derived(aiViewEnabledStore.value ? speech.ai_summary : null);

	let hasDetails = $derived(aiSummary != null || speech.relations.length > 0);
</script>

<div class="mt-5">
	<!-- svelte-ignore a11y_no_noninteractive_tabindex (tabindex is only set when role="button" is too) -->
	<div
		class="entry flex items-stretch overflow-hidden bg-primary-200 text-black transition-colors dark:bg-primary-300 {hasDetails
			? 'hover:bg-primary-400 dark:hover:bg-primary-400'
			: ''}"
		role={hasDetails ? 'button' : undefined}
		tabindex={hasDetails ? 0 : undefined}
		onclick={() => hasDetails && (modalOpen = true)}
		onkeypress={(e) => hasDetails && (e.key === 'Enter' || e.key === ' ') && (modalOpen = true)}
	>
		<div class="w-1.5 shrink-0 {barColor}"></div>
		<div
			class="min-w-0 flex-1 p-3 after:clear-both after:block after:content-[''] lg:px-5 lg:py-4"
			style="hyphens: auto; word-break: normal; overflow-wrap: break-word;"
		>
			{#if header}
				<div class="mb-1.5 flex min-w-0 items-center gap-2">
					{@render header()}
					<div class="ml-auto flex shrink-0 items-center gap-3 self-start text-gray-700">
						{@render metaIcons()}
					</div>
				</div>
			{:else}
				<div class="float-right ml-3 flex items-center gap-3 text-gray-700">
					{@render metaIcons()}
				</div>
			{/if}
			{#if aiSummary}
				{#if !header}
					<span class="block text-lg leading-snug font-semibold">
						{aiSummary.short_title}
					</span>
				{/if}
				<span class="mt-1 block text-sm text-gray-800 sm:text-base">
					{aiSummary.short_summary}
				</span>
			{:else}
				{#if !header}
					<span class="block text-lg leading-snug font-semibold">{opinion}</span>
				{/if}
				{#if speech.speech.about}
					<span class="mt-1 block text-sm text-gray-800 sm:text-base">
						{speech.speech.about}
					</span>
				{/if}
			{/if}
		</div>
	</div>

	{#snippet metaIcons()}
		{#if speechDuration}
			<span class="flex items-center gap-1 text-sm whitespace-nowrap">
				<span class="h-4 w-4 shrink-0 [&_path]:stroke-current [&>svg]:h-full [&>svg]:w-full">
					{@html clockIcon}
				</span>
				{speechDuration.mins}:{speechDuration.seconds.toString().padStart(2, '0')}
				{t('speeches.min')}
			</span>
		{/if}
		{#each speech.speech.document_urls ?? [] as url}
			<a
				href={url}
				target="_blank"
				aria-label={t('speeches.documentLabel')}
				title={t('speeches.openProtocol')}
				class="transition-transform hover:scale-110"
				onclick={(e) => e.stopPropagation()}
			>
				<svg
					class="h-4 w-4 lg:h-5 lg:w-5"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					><path
						d="M2 4h7a3 3 0 0 1 3 3v13a2 2 0 0 0-2-2H2zM22 4h-7a3 3 0 0 0-3 3v13a2 2 0 0 1 2-2h8z"
					/></svg
				>
			</a>
		{/each}
	{/snippet}

	<SpeechModal {speech} {header} bind:open={modalOpen} />
</div>

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
	}
</style>
