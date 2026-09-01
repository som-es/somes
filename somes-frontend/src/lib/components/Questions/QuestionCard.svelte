<script lang="ts">
	import { latestAnswer, questionSlug, type DelegateQuestionView } from './types';
	import { t } from '$lib/i18n/i18n.svelte';
	import { formatDate } from '$lib/date';
	import { partyToColor } from '$lib/partyColor';
	import { plink } from '$lib/api/parliament';

	interface Props {
		entry: DelegateQuestionView;
		class?: string;
	}

	let { entry, class: className = '' }: Props = $props();

	const question = $derived(entry.question);
	const delegate = $derived(entry.delegate);
	const answer = $derived(latestAnswer(question));
	const detailLink = $derived(plink(`/questions/${questionSlug(question)}`));

	const party = $derived(delegate?.party ?? null);
	const initials = $derived(
		delegate?.name
			.split(' ')
			.map((part) => part[0])
			.slice(0, 2)
			.join('') ?? ''
	);
</script>

<div class="rounded-xl bg-primary-300 p-4 shadow-sm sm:p-5 dark:bg-primary-500 {className}">
	<!-- Question header -->
	<div class="flex items-baseline justify-between gap-2 text-sm text-gray-700 dark:text-gray-300">
		<span>{t('qa.questionTo', { name: delegate?.name ?? '' })}</span>
		<span class="shrink-0">{formatDate(question.created_at)}</span>
	</div>
	<a href={detailLink} class="mt-1 block text-lg font-bold hover:underline sm:text-xl">
		{question.subject}
	</a>

	<!-- Latest answer -->
	{#if answer}
		<div class="mt-3 rounded-xl bg-surface-50 p-3 sm:p-4 dark:bg-surface-600">
			<div class="flex items-center gap-2.5">
				<div
					class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full text-sm font-semibold text-white"
					style="background-color: {partyToColor(party)};"
				>
					{initials}
				</div>
				<div class="flex flex-col overflow-hidden">
					<span class="truncate leading-tight font-medium">{delegate?.name ?? ''}</span>
					{#if party}
						<div class="mt-0.5 flex items-center gap-1.5">
							<div
								class="h-2 w-2 shrink-0 rounded-full"
								style="background-color: {partyToColor(party)};"
							></div>
							<span class="truncate text-xs text-gray-700 dark:text-gray-300">{party}</span>
						</div>
					{/if}
				</div>
			</div>
			<p class="mt-2 line-clamp-3 text-sm sm:text-base">
				{answer.body}
			</p>
		</div>
	{/if}

	<!-- Read more / unanswered -->
	<div class="mt-3 flex flex-wrap items-center gap-2">
		{#if question.answers.length > 1}
			<span class="text-sm text-gray-700 dark:text-gray-300">
				{t('qa.answerCount', { count: `${question.answers.length}` })}
			</span>
		{/if}
		{#if answer}
			<a href={detailLink} class="ml-auto shrink-0 text-sm hover:underline">
				{t('qa.readMore')} &rarr;
			</a>
		{:else}
			<span class="ml-auto shrink-0 text-sm text-gray-700 italic dark:text-gray-300">
				{t('qa.unansweredHint')}
			</span>
		{/if}
	</div>
</div>
