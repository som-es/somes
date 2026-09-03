<script lang="ts">
	import { latestAnswer, questionSlug, type DelegateQuestionView } from './types';
	import QuestionAnswer from './QuestionAnswer.svelte';
	import QuestionPending from './QuestionPending.svelte';
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

<a
	href={detailLink}
	class="group block rounded-xl bg-primary-300 p-4 shadow-sm sm:p-5 dark:bg-primary-500 {className}"
>
	<div class="flex items-start justify-between gap-2">
		<p class="text-lg font-bold group-hover:underline sm:text-xl">
			{question.subject}
		</p>
		<span class="shrink-0 text-xs text-gray-700 dark:text-gray-300">
			{formatDate(question.created_at)}
		</span>
	</div>

	<!-- Latest answer -->
	{#if answer}
		<QuestionAnswer
			{answer}
			{delegate}
			class="mt-3 bg-surface-50 dark:bg-surface-600 [&>p]:line-clamp-3"
		/>
	{:else}
		<QuestionPending
			createdAt={question.created_at}
			{delegate}
			class="mt-3 bg-surface-50 dark:bg-surface-600"
		/>
	{/if}

	<!-- Answer count -->
	{#if question.answers.length > 1}
		<div class="mt-3 text-sm text-gray-700 dark:text-gray-300">
			{t('qa.answerCount', { count: `${question.answers.length}` })}
		</div>
	{/if}
</a>
