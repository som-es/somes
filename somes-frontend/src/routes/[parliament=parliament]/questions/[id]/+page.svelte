<script lang="ts">
	import { page } from '$app/state';
	import Container from '$lib/components/Layout/Container.svelte';
	import Topics from '$lib/components/Topics/Topics.svelte';
	import { mockQuestions } from '$lib/components/Questions/mock';
	import { getParliament } from '$lib/api/parliament';
	import { t } from '$lib/i18n/i18n.svelte';
	import { dashDateToDotDate } from '$lib/date';
	import { partyToColor } from '$lib/partyColor';

	const parliament = $derived(getParliament());
	// TODO: replace mock data with the API call once the questions endpoint exists
	const question = $derived(
		mockQuestions(parliament).find((q) => q.id === Number(page.params.id)) ?? null
	);

	const initials = $derived(
		question?.answer?.delegateName
			.split(' ')
			.map((part) => part[0])
			.slice(0, 2)
			.join('') ?? ''
	);
</script>

<svelte:head>
	<title>{question ? question.question : t('qa.title')}</title>
	<meta name="description" content={t('qa.meta.description')} />
</svelte:head>

<Container>
	{#if question}
		<div class="flex flex-col gap-4 lg:flex-row lg:items-start">
			<div class="flex min-w-0 flex-1 flex-col gap-4">
				<!-- Question -->
				<div class="rounded-xl bg-primary-300 p-4 sm:p-6 dark:bg-primary-500">
					<div
						class="flex flex-wrap items-baseline justify-between gap-2 text-sm text-gray-700 dark:text-gray-300"
					>
						<span>{t('qa.questionFrom', { name: question.askedBy })}</span>
						<span class="shrink-0">{dashDateToDotDate(question.date)}</span>
					</div>
					<h1
						class="mt-2 text-xl leading-tight font-bold lg:text-2xl"
						style="hyphens: auto; overflow-wrap: break-word;"
					>
						{question.question}
					</h1>
					<p class="mt-8 whitespace-pre-line">{question.text}</p>
					<div class="mt-4">
						<Topics topics={question.topics.map((topic) => ({ topic }))} />
					</div>
				</div>

				<!-- Answer -->
				{#if question.answer}
					<div class="rounded-xl bg-primary-300 p-4 sm:p-6 dark:bg-primary-500">
						<div class="flex justify-between text-sm text-gray-700 dark:text-gray-300">
							<div class="flex items-center gap-3">
								<div
									class="flex h-11 w-11 shrink-0 items-center justify-center rounded-full font-semibold text-white"
									style="background-color: {partyToColor(question.answer.party)};"
								>
									{initials}
								</div>
								<div class="flex min-w-0 flex-col">
									<span class="truncate leading-tight font-bold"
										>{question.answer.delegateName}</span
									>
									<div class="mt-0.5 flex items-center gap-1.5">
										<div
											class="h-2 w-2 shrink-0 rounded-full"
											style="background-color: {partyToColor(question.answer.party)};"
										></div>
										<span class="truncate text-xs text-gray-700 dark:text-gray-300">
											{question.answer.party}
										</span>
									</div>
								</div>
							</div>
							<span class="shrink-0">{dashDateToDotDate(question.answer.date)}</span>
						</div>

						<p class="mt-4 whitespace-pre-line">{question.answer.text}</p>
					</div>
				{:else}
					<div
						class="rounded-xl bg-primary-300 p-5 text-center text-gray-700 italic dark:bg-primary-500 dark:text-gray-300"
					>
						{t('qa.unansweredHint')}
					</div>
				{/if}
			</div>

			<!-- Call to action -->
			<div
				class="shrink-0 rounded-xl bg-primary-300 p-5 lg:sticky lg:top-4 lg:w-72 dark:bg-primary-500"
			>
				{#if question.answer}
					<span class="block text-lg font-bold">{t('qa.askFromTitle')}</span>
					<div class="mt-3 flex items-center gap-2.5">
						<div
							class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full text-sm font-semibold text-white"
							style="background-color: {partyToColor(question.answer.party)};"
						>
							{initials}
						</div>
						<div class="flex min-w-0 flex-col">
							<span class="truncate leading-tight font-medium">{question.answer.delegateName}</span>
							<div class="mt-0.5 flex items-center gap-1.5">
								<div
									class="h-2 w-2 shrink-0 rounded-full"
									style="background-color: {partyToColor(question.answer.party)};"
								></div>
								<span class="truncate text-xs text-gray-700 dark:text-gray-300">
									{question.answer.party}
								</span>
							</div>
						</div>
					</div>
				{:else}
					<span class="block text-center text-lg font-bold">{t('qa.askTitle')}</span>
				{/if}
				<!-- TODO: hook up once submitting questions is possible via the API -->
				<button
					class="mt-4 w-full rounded-xl bg-secondary-500 px-3 py-2 text-white hover:cursor-pointer hover:bg-secondary-600"
				>
					{t('qa.askButton')}
				</button>
			</div>
		</div>
	{:else}
		<div class="rounded-xl bg-primary-300 p-5 text-center dark:bg-primary-500">
			{t('qa.notFound')}
		</div>
	{/if}
</Container>
