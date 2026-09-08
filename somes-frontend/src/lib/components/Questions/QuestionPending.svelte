<script lang="ts">
	import { goto } from '$app/navigation';
	import { plink } from '$lib/api/parliament';
	import clockIcon from '$lib/assets/misc_icons/clock-two.svg?raw';
	import SpeechDelegateHeader from '$lib/components/Delegates/Speeches/SpeechDelegateHeader.svelte';
	import { t } from '$lib/i18n/i18n.svelte';
	import type { QuestionDelegate } from './types';

	interface Props {
		createdAt: string;
		delegate: QuestionDelegate | null;
		class?: string;
	}

	let { createdAt, delegate, class: className = '' }: Props = $props();

	const days = $derived(
		Math.max(0, Math.floor((Date.now() - new Date(createdAt).getTime()) / 86_400_000))
	);
	const label = $derived(
		days == 0
			? t('qa.pendingSince.today')
			: days == 1
				? t('qa.pendingSince.day')
				: t('qa.pendingSince.days', { days: `${days}` })
	);

	function goToDelegate() {
		if (delegate) goto(plink(`/delegates?delegate=${delegate.id}`));
	}
</script>

<!-- Same layout as QuestionAnswer, but with the pending hint instead of a date and body.
     The background comes from the caller (class prop). -->
<div class="rounded-xl p-3 sm:p-4 {className}">
	<div class="flex items-start justify-between gap-2 text-sm text-gray-700 dark:text-gray-300">
		{#if delegate}
			<SpeechDelegateHeader {delegate} onNavigate={goToDelegate} />
		{/if}
		<span class="inline-flex shrink-0 items-center gap-1.5 text-xs italic">
			<span
				aria-hidden="true"
				class="h-4 w-4 shrink-0 [&_path]:stroke-current [&>svg]:h-full [&>svg]:w-full"
			>
				{@html clockIcon}
			</span>
			{label}
		</span>
	</div>
</div>
