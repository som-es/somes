<script lang="ts">
	import Container from '$lib/components/Layout/Container.svelte';
	import Topics from '$lib/components/Topics/Topics.svelte';
	import type { PageProps } from './$types';
	import DelegateListItem from '$lib/components/Delegates/DelegateListItem.svelte';
	import { orientationQuizSession } from '$lib/stores/orientationQuizSession.svelte';
	import { getMandateLatestPeriod } from '../[parliament=parliament]/delegates/searchDelegates';
	import { toActualDateString } from '$lib/api/api';
	import { goto } from '$app/navigation';
	import { t } from '$lib/i18n/i18n.svelte';
	import { plink } from '$lib/api/parliament';

	type StrongReferenceAnswer = { answer: string };
	type OrientationQuestion = {
		id: number;
		question: string;
		is_left: boolean | null;
		is_liberal: boolean | null;
		is_part_of: string[];
		strong_reference_answers: StrongReferenceAnswer[];
		topics: string[];
	};

	let { data }: PageProps = $props();
	const allQuestions = (data.questions ?? []) as OrientationQuestion[];

	const session = orientationQuizSession;

	// Start: question counts for the short/long cards
	const shortCount = allQuestions.filter((q) => q.is_part_of.includes('short')).length;
	const longCount = allQuestions.filter((q) => q.is_part_of.includes('long')).length;

	// Quiz: questions of the chosen length, in order
	let filteredQuestions = $derived(() => {
		const type = session.quizType;
		if (!type) return [];
		return allQuestions.filter((q) => q.is_part_of.includes(type));
	});

	// Quiz: slider steps (0-100) with their labels; also used on the result page
	const scaleOptions = [
		{ value: 0, label: 'orientation.quiz.stronglyAgainst' },
		{ value: 25, label: 'orientation.quiz.against' },
		{ value: 50, label: 'orientation.quiz.neutral' },
		{ value: 75, label: 'orientation.quiz.for' },
		{ value: 100, label: 'orientation.quiz.stronglyFor' }
	] as const;

	// Start -> Quiz
	function startQuiz(type: 'short' | 'long') {
		session.quizType = type;
		session.currentIndex = 0;
		session.answers = {};
		session.step = 'quiz';
	}

	// Result -> Start
	function restart() {
		session.step = 'start';
		session.quizType = null;
	}

	// Quiz: navigation and answering
	function currentQuestion(): OrientationQuestion | null {
		const qs = filteredQuestions();
		return qs[session.currentIndex] ?? null;
	}

	function next() {
		const qs = filteredQuestions();
		if (session.currentIndex < qs.length - 1) {
			session.currentIndex++;
		} else {
			session.step = 'result';
		}
	}

	function prev() {
		if (session.currentIndex > 0) session.currentIndex--;
	}

	function setAnswer(val: number) {
		const q = currentQuestion();
		if (!q) return;
		session.answers[q.id] = val;
	}

	// Quiz + Result: human-readable label for a slider value
	function answerLabel(val: number | null | undefined): string {
		if (val == null) return t('orientation.result.noAnswer');
		const idx = Math.min(scaleOptions.length - 1, Math.round(val / 25));
		return t(scaleOptions[idx].label);
	}

	// Quiz + Result: how many questions have an answer
	const answeredCount = $derived(() => {
		const qs = filteredQuestions();
		return qs.filter((q) => session.answers[q.id] != null).length;
	});

	// Result: aggregate answers per topic into the four political axes
	function computeUserTopicScores() {
		const qs = filteredQuestions();
		interface Acc {
			liberal: number;
			authoritarian: number;
			socialist: number;
			capitalist: number;
			count: number;
		}
		const topicAcc: Record<string, Acc> = {};
		for (const q of qs) {
			const val = session.answers[q.id];
			if (val == null) continue;
			const pro = val / 100;
			const contra = 1 - pro;
			let liberal = 0;
			let authoritarian = 0;
			let socialist = 0;
			let capitalist = 0;
			if (q.is_left !== null) {
				if (q.is_left) {
					socialist += pro;
					capitalist += contra;
				} else {
					capitalist += pro;
					socialist += contra;
				}
			}
			if (q.is_liberal !== null) {
				if (q.is_liberal) {
					liberal += pro;
					authoritarian += contra;
				} else {
					authoritarian += pro;
					liberal += contra;
				}
			}

			const topics = q.topics ?? [];
			for (const t of topics) {
				if (!topicAcc[t]) {
					topicAcc[t] = { liberal: 0, authoritarian: 0, socialist: 0, capitalist: 0, count: 0 };
				}
				topicAcc[t].liberal += liberal;
				topicAcc[t].authoritarian += authoritarian;
				topicAcc[t].socialist += socialist;
				topicAcc[t].capitalist += capitalist;
				topicAcc[t].count += 1;
			}
		}
		return topicAcc;
	}

	// Result: rank delegates by distance to the user's topic scores
	function getTopSimilarDelegates(topN = 10) {
		const topicAcc = computeUserTopicScores();
		const delegateScores = data.delegateScores ?? [];
		const scored = delegateScores.map((d) => {
			const scores = d.scores ?? [];
			let totalDiff = 0;
			let matches = 0;
			for (const s of scores) {
				const user = topicAcc[s.topic];
				if (user == null) continue;
				totalDiff +=
					Math.abs(user.authoritarian - s.broken_down_score.authoritarian) +
					Math.abs(user.liberal - s.broken_down_score.liberal) +
					Math.abs(user.socialist - s.broken_down_score.socialist) +
					Math.abs(user.capitalist - s.broken_down_score.capitalist);
				matches++;
			}
			const avgDiff = matches > 6 ? totalDiff / matches : Infinity;
			return { delegate: d, avgDiff };
		});
		scored.sort((a, b) => a.avgDiff - b.avgDiff);
		return scored.slice(0, topN).filter((d) => Number.isFinite(d.avgDiff));
	}

	// Result: jump to the delegate detail page
	function openDelegate(d: (typeof data.delegateScores)[number]) {
		const { date, gp } = getMandateLatestPeriod(d.delegate, data.gps);
		goto(plink(`/delegates?gp=${gp}&date=${toActualDateString(date)}&delegate=${d.delegate.id}`));
	}
</script>

<svelte:head>
	<title>{t('orientation.page.title')}</title>
	<meta name="description" content={t('orientation.page.description')} />
</svelte:head>

{#snippet primaryButton(label: string, onclick: () => void, disabled = false)}
	<button
		type="button"
		{onclick}
		{disabled}
		class="rounded-full bg-primary-500 px-4 py-1.5 text-xs font-semibold text-white transition hover:bg-primary-600 disabled:cursor-not-allowed disabled:opacity-40 sm:text-sm dark:bg-primary-300 dark:text-gray-900 dark:hover:bg-primary-200"
	>
		{label}
	</button>
{/snippet}

{#snippet secondaryButton(label: string, onclick: () => void, disabled = false)}
	<button
		type="button"
		{onclick}
		{disabled}
		class="rounded-full px-4 py-1.5 text-xs font-semibold transition hover:bg-primary-400 disabled:cursor-not-allowed disabled:opacity-40 sm:text-sm dark:hover:bg-primary-600"
	>
		{label}
	</button>
{/snippet}

<Container class="pb-12">
	<!-- Start page -->
	{#if session.step === 'start'}
		<br />
		<div class="mb-6">
			<h1 class="text-3xl font-bold sm:text-4xl">{t('orientation.page.title')}</h1>
			<p class="mt-2 text-base text-gray-700 dark:text-gray-300">
				{t('orientation.start.chooseLength')}
			</p>
		</div>

		<div class="grid gap-4 sm:grid-cols-2">
			{#each [{ type: 'short', title: t('orientation.start.short'), desc: t('orientation.start.shortDesc'), count: shortCount }, { type: 'long', title: t('orientation.start.long'), desc: t('orientation.start.longDesc'), count: longCount }] as opt (opt.type)}
				<button
					type="button"
					class="flex flex-col gap-3 rounded-xl bg-primary-300 px-6 py-5 text-left shadow-sm transition hover:bg-primary-400 dark:bg-primary-500 dark:hover:bg-primary-600"
					onclick={() => startQuiz(opt.type as 'short' | 'long')}
				>
					<div class="flex items-center justify-between gap-3">
						<span class="text-xl font-bold">{opt.title}</span>
						<span class="badge bg-tertiary-400 text-black">
							{t('orientation.start.questions', { count: opt.count })}
						</span>
					</div>
					<p class="text-sm text-gray-700 dark:text-gray-300">{opt.desc}</p>
				</button>
			{/each}
		</div>

		<div
			class="mt-6 rounded-xl border border-gray-300 bg-surface-50 px-6 py-5 shadow-sm dark:border-surface-700 dark:bg-surface-700/60"
		>
			<p class="text-base font-bold md:text-lg">
				{t('orientation.start.hint')}
			</p>
			<div class="mt-3 space-y-3 text-sm text-gray-800 dark:text-gray-100">
				<p>
					<span class="font-bold">{t('orientation.start.hintBold')}</span>
					{t('orientation.start.hintText')}
				</p>
				<p>
					{t('orientation.start.hintText2')}
					<span class="font-bold">{t('orientation.start.hintBold2')}</span>
				</p>
			</div>
		</div>
		<!-- Quiz page -->
	{:else if session.step === 'quiz'}
		{@const q = currentQuestion()}
		{@const qs = filteredQuestions()}
		{#if q}
			{@const progress = ((session.currentIndex + 1) / qs.length) * 100}
			{@const answer = session.answers[q.id]}
			{@const isLast = session.currentIndex === qs.length - 1}
			<div class="mt-2 mb-8">
				<div class="flex flex-wrap items-baseline justify-between gap-x-4 gap-y-1">
					<h1 class="text-2xl font-bold sm:text-3xl">
						{t('orientation.quiz.progress', {
							current: session.currentIndex + 1,
							total: qs.length
						})}
					</h1>
					<span class="text-sm text-gray-600 dark:text-gray-300">
						{t('orientation.quiz.answered', { count: answeredCount(), total: qs.length })}
					</span>
				</div>
				<div
					class="mt-3 h-2 w-full overflow-hidden rounded-full bg-surface-200 dark:bg-surface-700"
					role="progressbar"
					aria-valuemin="0"
					aria-valuemax="100"
					aria-valuenow={Math.round(progress)}
				>
					<div
						class="h-full rounded-full bg-primary-500 transition-all dark:bg-primary-300"
						style="width: {progress}%"
					></div>
				</div>
			</div>

			<section class="overflow-hidden rounded-xl bg-primary-300 dark:bg-primary-500">
				<div class="p-4 pb-0 sm:p-6 sm:pb-0">
					<p class="text-lg leading-snug font-semibold sm:text-2xl">{q.question}</p>
					{#if q.topics.length}
						<div class="-mx-1 mt-2">
							<Topics topics={q.topics.map((t) => ({ topic: t }))} />
						</div>
					{/if}
				</div>

				<div class="p-4 pt-8 sm:p-6 sm:pt-10">
					{#if q.strong_reference_answers.length >= 2}
						{@const pro = q.strong_reference_answers[0]}
						{@const contra = q.strong_reference_answers[1]}
						{@const value = answer ?? 50}
						<div class="grid gap-3 sm:grid-cols-2">
							<div
								class="rounded-lg border p-3 text-sm transition {value < 50
									? 'border-primary-600 bg-white dark:border-primary-200 dark:bg-surface-800'
									: 'border-transparent bg-white/70 dark:bg-surface-800/70'}"
							>
								<p
									class="mb-1 text-xs font-semibold tracking-wide text-gray-600 uppercase dark:text-gray-300"
								>
									{t('orientation.quiz.against')}
								</p>
								{contra.answer}
							</div>
							<div
								class="rounded-lg border p-3 text-sm transition {value > 50
									? 'border-primary-600 bg-white dark:border-primary-200 dark:bg-surface-800'
									: 'border-transparent bg-white/70 dark:bg-surface-800/70'}"
							>
								<p
									class="mb-1 text-xs font-semibold tracking-wide text-gray-600 uppercase dark:text-gray-300"
								>
									{t('orientation.quiz.for')}
								</p>
								{pro.answer}
							</div>
						</div>
						<div class="mt-6 flex items-center gap-3 sm:mt-4">
							<span class="shrink-0 text-xs text-gray-600 sm:text-sm dark:text-gray-300">
								{t('orientation.quiz.against')}
							</span>
							<div class="relative min-w-0 flex-1">
								<div
									class="pointer-events-none absolute inset-x-3 top-1/2 sm:inset-x-2"
									aria-hidden="true"
								>
									{#each scaleOptions as opt}
										<span
											class="stance-tick {answer === opt.value ? 'stance-tick-active' : ''}"
											style="left: {opt.value}%"
										></span>
									{/each}
								</div>
								<input
									type="range"
									min="0"
									max="100"
									{value}
									oninput={(e) => setAnswer(parseInt((e.target as HTMLInputElement).value))}
									aria-label={t('orientation.quiz.stance')}
									class="stance-slider relative w-full touch-manipulation"
								/>
							</div>
							<span class="shrink-0 text-xs text-gray-600 sm:text-sm dark:text-gray-300">
								{t('orientation.quiz.for')}
							</span>
						</div>
						<p class="mb-2 text-center text-sm font-semibold sm:mb-0">
							{answer == null ? t('orientation.quiz.notAnswered') : answerLabel(answer)}
						</p>
					{:else}
						<p class="text-sm text-gray-600 dark:text-gray-300">
							{t('orientation.quiz.noReferences')}
						</p>
					{/if}
				</div>

				<div class="p-4 pt-4 sm:p-6 sm:pt-6">
					<div class="flex justify-between gap-2">
						{@render secondaryButton(
							`← ${t('orientation.quiz.back')}`,
							prev,
							session.currentIndex === 0
						)}
						{@render primaryButton(
							isLast ? t('orientation.quiz.finish') : `${t('orientation.quiz.next')} →`,
							next
						)}
					</div>
				</div>
			</section>
		{/if}
		<!-- Result page -->
	{:else if session.step === 'result'}
		{@const qs = filteredQuestions()}
		{@const topDelegates = getTopSimilarDelegates(10)}
		{@const maxDiff = Math.max(...topDelegates.map((d) => d.avgDiff), 0.0001)}
		<div class="mt-2 mb-6">
			<h1 class="text-3xl font-bold sm:text-4xl">{t('orientation.result.title')}</h1>
			<p class="mt-2 text-base text-gray-700 dark:text-gray-300">
				{t('orientation.result.answered', { count: answeredCount() })}
			</p>
		</div>

		<section
			class="overflow-hidden rounded-xl border border-gray-300 bg-surface-50 shadow-sm dark:border-surface-700 dark:bg-surface-700/60"
		>
			<div class="bg-primary-300 p-4 sm:p-6 dark:bg-primary-500">
				<h2 class="text-xl font-bold sm:text-2xl">{t('orientation.result.similar')}</h2>
				<p class="mt-1 text-sm text-gray-700 dark:text-gray-300">
					{t('orientation.result.similarDesc')}
				</p>
			</div>
			{#if topDelegates.length === 0}
				<p class="p-4 text-sm text-gray-600 sm:p-6 dark:text-gray-300">
					{t('orientation.result.tooFewAnswers')}
				</p>
			{:else}
				<ol class="divide-y divide-gray-300 dark:divide-surface-600">
					{#each topDelegates as d, i (d.delegate.delegate.id)}
						<li class="flex items-center gap-3 px-4 py-2.5 sm:px-6">
							<span class="w-7 shrink-0 font-mono text-xs text-gray-500 dark:text-gray-400"
								>#{i + 1}</span
							>
							<div class="min-w-0 flex-1">
								<DelegateListItem
									delegate={d.delegate.delegate}
									class="w-full"
									onclick={() => openDelegate(d.delegate)}
								>
									<span class="text-[0.65rem] text-gray-600 sm:hidden dark:text-gray-300">
										{t('orientation.result.avgDiff', { value: d.avgDiff.toFixed(2) })}
									</span>
								</DelegateListItem>
							</div>
							<div class="hidden w-40 shrink-0 sm:block">
								<div
									class="h-1.5 w-full overflow-hidden rounded-full bg-surface-200 dark:bg-surface-600"
								>
									<div
										class="h-full rounded-full bg-primary-500 dark:bg-primary-300"
										style="width: {Math.max(8, 100 - (d.avgDiff / maxDiff) * 60)}%"
									></div>
								</div>
								<p class="mt-1 text-right text-[0.65rem] text-gray-600 dark:text-gray-300">
									{t('orientation.result.avgDiff', { value: d.avgDiff.toFixed(3) })}
								</p>
							</div>
						</li>
					{/each}
				</ol>
			{/if}
		</section>

		<details
			class="mt-6 rounded-xl border border-gray-300 bg-surface-50 shadow-sm dark:border-surface-700 dark:bg-surface-700/60"
		>
			<summary class="cursor-pointer p-4 text-lg font-bold sm:p-6">
				{t('orientation.result.answersOverview')}
			</summary>
			<ol
				class="divide-y divide-gray-300 border-t border-gray-300 dark:divide-surface-600 dark:border-surface-600"
			>
				{#each qs as q, i (q.id)}
					{@const val = session.answers[q.id]}
					<li
						class="flex flex-col gap-2 px-4 py-3 sm:flex-row sm:items-center sm:justify-between sm:px-6"
					>
						<p class="text-sm">
							<span class="mr-2 font-mono text-xs text-gray-500 dark:text-gray-400">{i + 1}</span>
							{q.question}
						</p>
						<span
							class="shrink-0 self-start rounded-full px-3 py-0.5 text-xs font-semibold sm:self-auto {val ==
							null
								? 'bg-surface-200 text-gray-600 dark:bg-surface-600 dark:text-gray-300'
								: 'bg-primary-300 text-black dark:bg-primary-400'}"
						>
							{answerLabel(val)}
						</span>
					</li>
				{/each}
			</ol>
		</details>

		<div class="mt-6">
			{@render secondaryButton(`← ${t('orientation.result.restart')}`, restart)}
		</div>
	{/if}
</Container>

<style>
	.stance-slider {
		appearance: none;
		height: 1.75rem;
		background: transparent;
		cursor: pointer;
	}

	.stance-slider::-webkit-slider-runnable-track {
		height: 2px;
		border-radius: 9999px;
		background: light-dark(rgb(107 114 128 / 0.6), rgb(229 231 235 / 0.6));
	}

	.stance-slider::-webkit-slider-thumb {
		appearance: none;
		margin-top: -7px;
		height: 16px;
		width: 16px;
		border-radius: 9999px;
		background: light-dark(var(--color-primary-500), var(--color-primary-300));
	}

	.stance-slider::-moz-range-track {
		height: 2px;
		border-radius: 9999px;
		background: light-dark(rgb(107 114 128 / 0.6), rgb(229 231 235 / 0.6));
	}

	.stance-slider::-moz-range-thumb {
		border: none;
		height: 16px;
		width: 16px;
		border-radius: 9999px;
		background: light-dark(var(--color-primary-500), var(--color-primary-300));
	}

	.stance-tick {
		position: absolute;
		top: 0;
		width: 2px;
		height: 8px;
		border-radius: 9999px;
		transform: translate(-50%, -50%);
		background: light-dark(rgb(107 114 128 / 0.6), rgb(229 231 235 / 0.6));
	}

	.stance-tick-active {
		background: light-dark(var(--color-primary-500), var(--color-primary-300));
	}

	/* Mobile: taller hit area and bigger thumb for touch */
	@media (max-width: 639px) {
		.stance-slider {
			height: 3rem;
		}

		.stance-slider::-webkit-slider-thumb {
			margin-top: -11px;
			height: 20px;
			width: 20px;
		}

		.stance-slider::-moz-range-thumb {
			height: 20px;
			width: 20px;
		}
	}
</style>
