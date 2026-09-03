<script lang="ts">
	import { t } from '$lib/i18n/i18n.svelte';
	import type { Delegate } from '$lib/types';
	import SpeechDelegateHeader from '../Speeches/SpeechDelegateHeader.svelte';

	interface Props {
		delegate: Delegate | null;
		text: string | null;
		onclick?: () => void;
		onNavigate?: () => void;
	}

	let { delegate, text, onclick, onNavigate }: Props = $props();
</script>

<!-- svelte-ignore a11y_no_static_element_interactions button or plain div -->
<svelte:element
	this={onclick ? 'button' : 'div'}
	class="flex w-full flex-col gap-1.5 rounded-lg bg-primary-200 px-3 py-2 text-left text-black sm:flex-row sm:items-center sm:gap-4 dark:bg-primary-600 dark:text-white {onclick
		? 'cursor-pointer transition-colors hover:bg-primary-400 dark:hover:bg-primary-700'
		: ''}"
	title={onclick ? t('interjections.openSpeech') : undefined}
	{onclick}
>
	<div class="flex w-full shrink-0 flex-col sm:w-56">
		{#if delegate}
			<SpeechDelegateHeader {delegate} {onNavigate} />
		{:else}
			<span class="text-sm text-gray-700 dark:text-gray-300">
				{t('interjections.loadingSpeaker')}
			</span>
		{/if}
	</div>
	<p class="min-w-0 flex-1 text-sm italic">
		{#if text}
			&ldquo;{text}&rdquo;
		{:else}
			<span class="text-tertiary-200 not-italic">{t('interjections.noText')}</span>
		{/if}
	</p>
</svelte:element>
