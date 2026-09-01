<script lang="ts">
	import { latestAnswer, questionSlug, type DelegateQuestionView } from './types';
	import QuestionAnswer from './QuestionAnswer.svelte';
	import { t } from '$lib/i18n/i18n.svelte';
	import { formatDate } from '$lib/date';
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
		<QuestionAnswer
			{answer}
			{delegate}
			class="mt-3 bg-surface-50 dark:bg-surface-600 [&>p]:line-clamp-3"
		/>
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
