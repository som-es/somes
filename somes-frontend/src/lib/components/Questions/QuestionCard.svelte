<script lang="ts">
	import type { PoliticianQuestion } from './types';
	import { t } from '$lib/i18n/i18n.svelte';
	import { dashDateToDotDate } from '$lib/date';
	import { partyToColor } from '$lib/partyColor';
	import { plink } from '$lib/api/parliament';
	import Topics from '$lib/components/Topics/Topics.svelte';

	interface Props {
		question: PoliticianQuestion;
		class?: string;
	}

	let { question, class: className = '' }: Props = $props();

	const detailLink = $derived(plink(`/questions/${question.id}`));

	const initials = $derived(
		question.answer?.delegateName
			.split(' ')
			.map((part) => part[0])
			.slice(0, 2)
			.join('') ?? ''
	);
	const councilLabel = $derived(
		question.parliament === 'at' ? t('nav.nationalCouncil') : t('qa.council.eu')
	);
</script>

<div class="rounded-xl bg-primary-300 p-4 shadow-sm sm:p-5 dark:bg-primary-500 {className}">
	<!-- Question header -->
	<div class="flex items-baseline justify-between gap-2 text-sm text-gray-700 dark:text-gray-300">
		<span>{t('qa.questionFrom', { name: question.askedBy })}</span>
		<span class="shrink-0">{dashDateToDotDate(question.date)}</span>
	</div>
	<a href={detailLink} class="mt-1 block text-lg font-bold hover:underline sm:text-xl">
		{question.question}
	</a>

	<!-- Answer -->
	{#if question.answer}
		<div class="mt-3 rounded-xl bg-surface-50 p-3 sm:p-4 dark:bg-surface-600">
			<div class="flex items-center gap-2.5">
				<div
					class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full text-sm font-semibold text-white"
					style="background-color: {partyToColor(question.answer.party)};"
				>
					{initials}
				</div>
				<div class="flex flex-col overflow-hidden">
					<span class="truncate leading-tight font-medium">{question.answer.delegateName}</span>
					<div class="mt-0.5 flex items-center gap-1.5">
						<div
							class="h-2 w-2 shrink-0 rounded-full"
							style="background-color: {partyToColor(question.answer.party)};"
						></div>
						<span class="truncate text-xs text-gray-700 dark:text-gray-300"
							>{question.answer.party}</span
						>
					</div>
				</div>
			</div>
			<p class="mt-2 line-clamp-3 text-sm sm:text-base">
				{question.answer.text}
			</p>
		</div>
	{/if}

	<!-- Tags & read more -->
	<div class="mt-3 flex flex-wrap items-center gap-2">
		<Topics topics={question.topics.map((topic) => ({ topic }))} />
		{#if question.answer}
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
