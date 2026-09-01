<script lang="ts">
	import Container from '$lib/components/Layout/Container.svelte';
	import SearchBar from '$lib/components/Filtering/SearchBar.svelte';
	import QuestionCard from '$lib/components/Questions/QuestionCard.svelte';
	import { latestAnswer, questionSlug } from '$lib/components/Questions/types';
	import { plink } from '$lib/api/parliament';
	import { t } from '$lib/i18n/i18n.svelte';
	import { formatDate } from '$lib/date';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	const entries = $derived(data.entries);

	type Tab = 'latestAnswers' | 'latestQuestions' | 'unanswered';
	let activeTab = $state<Tab>('latestAnswers');
	let searchValue = $state('');

	const tabs: { key: Tab; label: () => string }[] = [
		{ key: 'latestAnswers', label: () => t('qa.tab.latestAnswers') },
		{ key: 'latestQuestions', label: () => t('qa.tab.latestQuestions') },
		{ key: 'unanswered', label: () => t('qa.tab.unanswered') }
	];

	// ISO timestamps (UTC) sort correctly as strings.
	const updatedDate = $derived(
		entries
			.flatMap(({ question }) => [
				question.created_at,
				...question.answers.map((answer) => answer.received_at)
			])
			.sort()
			.at(-1) ?? null
	);

	const displayedEntries = $derived.by(() => {
		const search = searchValue.toLowerCase().trim();
		let result = entries.filter(({ question, delegate }) => {
			if (!search) return true;
			return [
				question.subject,
				question.body,
				delegate?.name ?? '',
				delegate?.party ?? '',
				...question.answers.map((answer) => answer.body)
			]
				.join(' ')
				.toLowerCase()
				.includes(search);
		});

		switch (activeTab) {
			case 'latestAnswers':
				result = result
					.filter(({ question }) => question.answers.length > 0)
					.sort((a, b) =>
						(latestAnswer(b.question)?.received_at ?? '').localeCompare(
							latestAnswer(a.question)?.received_at ?? ''
						)
					);
				break;
			case 'latestQuestions':
				result = [...result].sort((a, b) =>
					b.question.created_at.localeCompare(a.question.created_at)
				);
				break;
			case 'unanswered':
				result = result
					.filter(({ question }) => question.answers.length === 0)
					.sort((a, b) => b.question.created_at.localeCompare(a.question.created_at));
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
	{#if updatedDate}
		<span class="mb-2 ml-1 block text-base text-gray-800 sm:mt-1 sm:ml-0 dark:text-gray-300">
			{t('qa.updated', { date: formatDate(updatedDate) })}
		</span>
	{/if}

	<!-- Search -->
	<div class="mt-5 flex gap-2">
		<SearchBar bind:searchValue />
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
			{#each displayedEntries as entry (questionSlug(entry.question))}
				<QuestionCard {entry} />
			{:else}
				<div class="rounded-xl bg-primary-300 p-5 text-center dark:bg-primary-500">
					{t('qa.noResults')}
				</div>
			{/each}
		</div>

		<!-- Call to action: questions are asked on a delegate's profile -->
		<div
			class="w-full shrink-0 rounded-xl bg-primary-300 p-5 shadow-sm lg:sticky lg:top-4 lg:w-72 dark:bg-primary-500"
		>
			<span class="block text-center text-lg font-bold">{t('qa.askTitle')}</span>
			<a
				href={plink('/delegates')}
				class="mt-4 block w-full rounded-xl bg-secondary-500 px-3 py-2 text-center text-white transition-colors hover:cursor-pointer hover:bg-secondary-600"
			>
				{t('qa.askButton')}
			</a>
		</div>
	</div>
</Container>
