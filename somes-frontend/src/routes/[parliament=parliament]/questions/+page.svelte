<script lang="ts">
	import Container from '$lib/components/Layout/Container.svelte';
	import SearchBar from '$lib/components/Filtering/SearchBar.svelte';
	import MultiSelectFilter from '$lib/components/Filtering/MultiSelectFilter.svelte';
	import QuestionCard from '$lib/components/Questions/QuestionCard.svelte';
	import { mockQuestions } from '$lib/components/Questions/mock';
	import { getParliament } from '$lib/api/parliament';
	import { t } from '$lib/i18n/i18n.svelte';
	import { dashDateToDotDate } from '$lib/date';

	const parliament = $derived(getParliament());
	// TODO: replace mock data with the API call once the questions endpoint exists
	const questions = $derived(mockQuestions(parliament));

	type Tab = 'latestAnswers' | 'latestQuestions' | 'unanswered';
	let activeTab = $state<Tab>('latestAnswers');
	let searchValue = $state('');
	let selectedTopics = $state<string[]>([]);

	const tabs: { key: Tab; label: () => string }[] = [
		{ key: 'latestAnswers', label: () => t('qa.tab.latestAnswers') },
		{ key: 'latestQuestions', label: () => t('qa.tab.latestQuestions') },
		{ key: 'unanswered', label: () => t('qa.tab.unanswered') }
	];

	const topicItems = $derived(
		[...new Set(questions.flatMap((q) => q.topics))]
			.sort((a, b) => a.localeCompare(b, 'de'))
			.map((topic) => ({ value: topic, label: topic }))
	);

	const updatedDate = $derived(
		questions
			.flatMap((q) => [q.date, q.answer?.date ?? ''])
			.sort()
			.at(-1) ?? ''
	);

	const displayedQuestions = $derived.by(() => {
		const search = searchValue.toLowerCase().trim();
		let result = questions.filter((q) => {
			if (selectedTopics.length > 0 && !q.topics.some((topic) => selectedTopics.includes(topic))) {
				return false;
			}
			if (!search) return true;
			return [
				q.question,
				q.askedBy,
				q.answer?.delegateName ?? '',
				q.answer?.text ?? '',
				...q.topics
			]
				.join(' ')
				.toLowerCase()
				.includes(search);
		});

		switch (activeTab) {
			case 'latestAnswers':
				result = result
					.filter((q) => q.answer !== null)
					.sort((a, b) => (b.answer?.date ?? '').localeCompare(a.answer?.date ?? ''));
				break;
			case 'latestQuestions':
				result = result.sort((a, b) => b.date.localeCompare(a.date));
				break;
			case 'unanswered':
				result = result
					.filter((q) => q.answer === null)
					.sort((a, b) => b.date.localeCompare(a.date));
				break;
		}
		return result;
	});
</script>

<svelte:head>
	<title>{t('qa.title')}</title>
	<meta name="description" content={t('qa.meta.description')} />
</svelte:head>

<Container>
	<h1 class="px-1 pt-2 text-3xl font-bold sm:p-0 sm:text-4xl">{t('qa.title')}</h1>
	<span class="mb-2 ml-1 block text-base text-gray-800 sm:mt-1 sm:ml-0 dark:text-gray-300">
		{t('qa.updated', { date: dashDateToDotDate(updatedDate) })}
	</span>

	<!-- Search & topic filter -->
	<div class="mt-5 flex gap-2">
		<SearchBar bind:searchValue />
		<div class="flex h-10 shrink-0 touch-manipulation items-center">
			<MultiSelectFilter items={topicItems} bind:value={selectedTopics} allLabel={t('qa.filter')} />
		</div>
	</div>

	<!-- Tabs -->
	<div class="mt-3 flex flex-wrap gap-1">
		{#each tabs as tab (tab.key)}
			<button
				class="rounded-full px-3 py-1.5 text-sm font-medium transition-colors hover:cursor-pointer {activeTab ===
				tab.key
					? 'bg-surface-500 text-white'
					: 'hover:bg-primary-300 dark:hover:bg-primary-500'}"
				onclick={() => (activeTab = tab.key)}
			>
				{tab.label()}
			</button>
		{/each}
	</div>

	<div class="mt-4 flex flex-col gap-4 lg:flex-row lg:items-start">
		<!-- Question list -->
		<div class="flex min-w-0 flex-1 flex-col gap-4">
			{#each displayedQuestions as question (question.id)}
				<QuestionCard {question} />
			{:else}
				<div class="rounded-xl bg-primary-300 p-5 text-center dark:bg-primary-500">
					{t('qa.noResults')}
				</div>
			{/each}
		</div>

		<!-- Call to action -->
		<div
			class="w-full shrink-0 rounded-xl bg-primary-300 p-5 shadow-sm lg:sticky lg:top-4 lg:w-72 dark:bg-primary-500"
		>
			<span class="block text-center text-lg font-bold">{t('qa.askTitle')}</span>
			<!-- TODO: hook up once submitting questions is possible via the API -->
			<button
				class="mt-4 w-full rounded-xl bg-secondary-500 px-3 py-2 text-white transition-colors hover:cursor-pointer hover:bg-secondary-600"
			>
				{t('qa.askButton')}
			</button>
		</div>
	</div>
</Container>
