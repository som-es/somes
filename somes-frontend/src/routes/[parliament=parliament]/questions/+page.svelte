<script lang="ts">
	import { onMount, untrack } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { SvelteSet } from 'svelte/reactivity';
	import { errorToNull, get_eurovoc_topics } from '$lib/api/api';
	import { plink } from '$lib/api/parliament';
	import { t, localeStore } from '$lib/i18n/i18n.svelte';
	import type { DelegateQuestionFilter, UniqueTopic } from '$lib/types';
	import { currentDelegateQuestionFilterStore } from '$lib/stores/stores';
	import Container from '$lib/components/Layout/Container.svelte';
	import SearchBar from '$lib/components/Filtering/SearchBar.svelte';
	import SortPopover from '$lib/components/Filtering/SortPopover.svelte';
	import TopicFilter from '$lib/components/Filtering/TopicFilter.svelte';
	import GenericFilters from '$lib/components/Filtering/GenericFilters.svelte';
	import DateRangeSnippet from '$lib/components/Filtering/GenericFilterSnippets/DataRangeSnippet.svelte';
	import { createFilterGroup } from '$lib/components/Filtering/filterGroup.svelte';
	import type { GenericFilterGroup } from '$lib/components/Filtering/types';
	import Pagination from '$lib/components/Pagination.svelte';
	import QuestionCard from '$lib/components/Questions/QuestionCard.svelte';
	import { latestAnswer } from '$lib/components/Questions/types';
	import { convertDelegateQuestionFilterToUrl } from '$lib/components/Questions/urlConversion';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	const result = $derived(data.result);

	// Die Reiter arbeiten nur auf der aktuell geladenen Seite - filtern und sortieren
	// ueber alle Fragen kann der Suchindex noch nicht.
	type Tab = 'latestQuestions' | 'latestAnswers' | 'unanswered';
	let activeTab: Tab = $state('latestQuestions');

	const tabs: { key: Tab; label: () => string }[] = [
		{ key: 'latestQuestions', label: () => t('qa.tab.latestQuestions') },
		{ key: 'latestAnswers', label: () => t('qa.tab.latestAnswers') },
		{ key: 'unanswered', label: () => t('qa.tab.unanswered') }
	];

	const displayedEntries = $derived.by(() => {
		switch (activeTab) {
			case 'latestQuestions':
				return result.entries;
			case 'unanswered':
				return result.entries.filter(({ question }) => question.answers.length === 0);
			case 'latestAnswers':
				// Einziger Reiter, der umsortiert - nach Antwortdatum kann der Index nicht sortieren.
				return result.entries
					.filter(({ question }) => question.answers.length > 0)
					.sort((a, b) =>
						(latestAnswer(b.question)?.received_at ?? '').localeCompare(
							latestAnswer(a.question)?.received_at ?? ''
						)
					);
		}
	});

	let currentPage: number | undefined = $state(undefined);
	let searchValue = $state('');
	let sortOrder: 'relevance' | 'Desc' | 'Asc' = $state('relevance');
	let selectedTopics: SvelteSet<string> = $state(new SvelteSet());

	let eurovocTopics: UniqueTopic[] = $state([]);
	let topics = $derived(eurovocTopics.map((topic) => topic.topic));
	let topicIdByName = $derived(new Map(eurovocTopics.map((topic) => [topic.topic, topic.id])));

	let genericFilters: [GenericFilterGroup<string>] = $state([
		createFilterGroup<string>({
			title: () => t('filter.date'),
			hidden: () => false,
			id: 'dateRange',
			data: { dateFrom: '', dateTo: '' },
			options: () => []
		})
	]);

	let updatedAt = $derived.by(() => {
		const locale = localeStore.value === 'de' ? 'de-AT' : 'en-AT';
		return result.updatedAt
			? new Intl.DateTimeFormat(locale, {
					day: '2-digit',
					month: '2-digit',
					year: 'numeric'
				}).format(new Date(result.updatedAt))
			: t('date.unknown');
	});

	onMount(async () => {
		const storedFilter = currentDelegateQuestionFilterStore.value;
		if (storedFilter !== null) {
			if (storedFilter.topics !== null) selectedTopics = new SvelteSet(storedFilter.topics);
			if (storedFilter.date_from) genericFilters[0].data!.dateFrom = storedFilter.date_from;
			if (storedFilter.date_to) genericFilters[0].data!.dateTo = storedFilter.date_to;
			if (storedFilter.page) currentPage = storedFilter.page;
		}

		eurovocTopics = errorToNull(await get_eurovoc_topics()) ?? [];
	});

	const update = () => {
		const filter: DelegateQuestionFilter = {
			topics: selectedTopics.size > 0 ? [...selectedTopics] : null,
			date_from: genericFilters[0].data?.dateFrom || null,
			date_to: genericFilters[0].data?.dateTo || null,
			page: currentPage ?? null
		};
		currentDelegateQuestionFilterStore.value = filter;

		const nextUrl = convertDelegateQuestionFilterToUrl(
			filter,
			searchValue,
			new URL(page.url),
			sortOrder,
			topicIdByName
		);

		goto(nextUrl, { keepFocus: true, replaceState: true, noScroll: true });
	};

	$effect(() => {
		void searchValue;
		void sortOrder;
		void selectedTopics.size;
		void genericFilters[0].data?.dateFrom;
		void genericFilters[0].data?.dateTo;
		// Die Themen kommen erst nach dem ersten Rendern - ohne sie fehlen die IDs.
		void topicIdByName;
		untrack(update);
	});

	$effect(() => {
		genericFilters[0].activeValue =
			genericFilters[0].data?.dateFrom || genericFilters[0].data?.dateTo ? 'set' : undefined;
	});
</script>

<svelte:head>
	<title>{t('qa.title')}</title>
	<meta name="description" content={t('qa.meta.description')} />
</svelte:head>

<Container>
	<h1 class="px-1 pt-2 text-3xl font-bold sm:p-0 sm:text-4xl">{t('qa.title')}</h1>
	<span class="mb-2 ml-1 block text-base text-gray-800 sm:mt-1 sm:ml-0 dark:text-gray-300">
		{t('qa.updated', { date: updatedAt })}
	</span>

	<div class="mt-5 md:flex">
		<SearchBar bind:searchValue placeholder={t('qa.searchPlaceholder')}>
			{#snippet rightSlot()}
				{#if searchValue.length > 0}
					<SortPopover bind:sortOrder />
				{/if}
			{/snippet}
		</SearchBar>

		<div class="mt-2 flex h-10 w-full gap-2 text-xs sm:text-base md:mt-0 md:ml-2 md:w-auto">
			<TopicFilter bind:selectedTopics {topics} />
			{#snippet dateRangeSnippet()}
				<DateRangeSnippet
					bind:dateFrom={genericFilters[0].data!.dateFrom}
					bind:dateTo={genericFilters[0].data!.dateTo}
				/>
			{/snippet}
			<GenericFilters bind:genericFilters snippets={{ dateRange: dateRangeSnippet }} />
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
			{#each displayedEntries as entry (entry.question.id)}
				<QuestionCard {entry} />
			{:else}
				<div class="rounded-xl bg-primary-300 p-5 text-center dark:bg-primary-500">
					{t('qa.noResults')}
				</div>
			{/each}

			<div class="flex justify-end">
				<Pagination bind:currentPage maxPage={result.maxPage} />
			</div>
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
