<script lang="ts">
	import { formatDate } from '$lib/date';
	import { partyToColor } from '$lib/partyColor';
	import type { PublicDelegateQuestionAnswer } from '$lib/types';
	import type { QuestionDelegate } from './types';

	interface Props {
		answer: PublicDelegateQuestionAnswer;
		delegate: QuestionDelegate | null;
		class?: string;
	}

	let { answer, delegate, class: className = '' }: Props = $props();

	const initials = $derived(
		delegate?.name
			.split(' ')
			.map((part) => part[0])
			.slice(0, 2)
			.join('') ?? ''
	);
</script>

<!-- The background comes from the caller (class prop), since it depends on what
     the answer sits on: gray on the overview cards, primary on the detail page. -->
<div class="rounded-xl p-3 sm:p-4 {className}">
	<div class="flex justify-between text-sm text-gray-700 dark:text-gray-300">
		<div class="flex items-center gap-2.5">
			<div
				class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full text-sm font-semibold text-white"
				style="background-color: {partyToColor(delegate?.party ?? null)};"
			>
				{initials}
			</div>
			<div class="flex min-w-0 flex-col">
				<span class="truncate leading-tight font-medium">{delegate?.name ?? ''}</span>
				{#if delegate?.party}
					<div class="mt-0.5 flex items-center gap-1.5">
						<div
							class="h-2 w-2 shrink-0 rounded-full"
							style="background-color: {partyToColor(delegate.party)};"
						></div>
						<span class="truncate text-xs text-gray-700 dark:text-gray-300">{delegate.party}</span>
					</div>
				{/if}
			</div>
		</div>
		<span class="shrink-0">{formatDate(answer.received_at)}</span>
	</div>

	<p class="mt-2 text-sm whitespace-pre-line sm:text-base">{answer.body}</p>
</div>
