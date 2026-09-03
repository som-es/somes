<script lang="ts">
	import { formatDate } from '$lib/date';
	import SpeechDelegateHeader from '$lib/components/Delegates/Speeches/SpeechDelegateHeader.svelte';
	import type { PublicDelegateQuestionAnswer } from '$lib/types';
	import type { QuestionDelegate } from './types';

	interface Props {
		answer: PublicDelegateQuestionAnswer;
		delegate: QuestionDelegate | null;
		class?: string;
	}

	let { answer, delegate, class: className = '' }: Props = $props();
</script>

<!-- The background comes from the caller (class prop), since it depends on what
     the answer sits on: gray on the overview cards, primary on the detail page. -->
<div class="rounded-xl p-3 sm:p-4 {className}">
	<div class="flex items-center justify-between gap-2 text-sm text-gray-700 dark:text-gray-300">
		{#if delegate}
			<SpeechDelegateHeader {delegate} />
		{/if}
		<span class="shrink-0">{formatDate(answer.received_at)}</span>
	</div>

	<p class="mt-2 text-sm whitespace-pre-line sm:text-base">{answer.body}</p>
</div>
