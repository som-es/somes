<script lang="ts">
	import Container from '$lib/components/Layout/Container.svelte';
	import DelegateCard from '$lib/components/Delegates/DelegateCard.svelte';
	import QuestionAnswer from '$lib/components/Questions/QuestionAnswer.svelte';
	import { plink } from '$lib/api/parliament';
	import { t } from '$lib/i18n/i18n.svelte';
	import { formatDate } from '$lib/date';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	const question = $derived(data.entry?.question ?? null);
	const delegate = $derived(data.entry?.delegate ?? null);
</script>

<svelte:head>
	<title>{question ? question.subject : t('qa.title')}</title>
	<meta name="description" content={t('qa.meta.description')} />
</svelte:head>

<Container>
	{#if question}
		<div class="flex flex-col gap-4 pt-2 sm:pt-0 lg:flex-row lg:items-start">
			<div class="flex min-w-0 flex-1 flex-col gap-4">
				<!-- Question -->
				<div class="rounded-xl bg-primary-300 p-4 sm:p-6 dark:bg-primary-500">
					<div
						class="flex flex-wrap items-baseline justify-between gap-2 text-sm text-gray-700 dark:text-gray-300"
					>
						<span>{t('qa.questionTo', { name: delegate?.name ?? '' })}</span>
						<span class="shrink-0">{formatDate(question.created_at)}</span>
					</div>
					<h1
						class="mt-2 text-xl leading-tight font-bold lg:text-2xl"
						style="hyphens: auto; overflow-wrap: break-word;"
					>
						{question.subject}
					</h1>
					<p class="mt-8 whitespace-pre-line">{question.body}</p>
				</div>

				<!-- Answers (a question can receive several reply mails) -->
				{#each question.answers as answer (answer.received_at)}
					<QuestionAnswer {answer} {delegate} class="bg-primary-300 dark:bg-primary-500" />
				{:else}
					<div
						class="rounded-xl bg-primary-300 p-5 text-center text-gray-700 italic dark:bg-primary-500 dark:text-gray-300"
					>
						{t('qa.unansweredHint')}
					</div>
				{/each}
			</div>

			<!-- Call to action -->
			<div
				class="w-full shrink-0 rounded-xl bg-primary-300 p-5 lg:sticky lg:top-4 lg:w-80 dark:bg-primary-500"
			>
				{#if data.fullDelegate}
					<span class="block text-lg font-bold">{t('qa.askFromTitle')}</span>
					<div class="mt-3">
						<DelegateCard delegate={data.fullDelegate} onlyTop />
					</div>
					<a
						href={plink(`/questions/ask/${data.fullDelegate.id}`)}
						class="mt-4 block w-full rounded-xl bg-secondary-500 px-3 py-2 text-center text-white hover:cursor-pointer hover:bg-secondary-600"
					>
						{t('qa.askButton')}
					</a>
				{/if}
			</div>
		</div>
	{:else}
		<div class="rounded-xl bg-primary-300 p-5 text-center dark:bg-primary-500">
			{t('qa.notFound')}
		</div>
	{/if}
</Container>
