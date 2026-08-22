<script lang="ts">
	import Container from '$lib/components/Layout/Container.svelte';
	import Topics from '$lib/components/Topics/Topics.svelte';
	import QuadrantChart from '$lib/components/GeneralCharts/QuadrantChart.svelte';
	import type { PageProps } from './$types';
	import DelegateListItem from '$lib/components/Delegates/DelegateListItem.svelte';
	import { orientationQuizSession } from '$lib/stores/orientationQuizSession.svelte';
	import { getMandateLatestPeriod } from '../[parliament=parliament]/delegates/searchDelegates';
	import { all_gps, toActualDateString } from '$lib/api/api';
	import { goto } from '$app/navigation';
	import { plink } from '$lib/api/parliament';

	type TopicInfluence = { topic: string; influence: number };
	type StrongReferenceAnswer = {
		id: number;
		question_id: number;
		answer: string;
		stance_llm: string;
		is_strong_reference?: boolean | null;
		model_used?: string | null;
		created_at: string;
		full_stance?: any;
	};
	type OrientationQuestion = {
		id: number;
		question: string;
		is_left: boolean | null;
		is_liberal: boolean | null;
		is_part_of: string[];
		strong_reference_answers: StrongReferenceAnswer[];
		topics: string[];
		topics_influence: TopicInfluence[];
		detailed_topics: string[];
		detailed_topics_influence: TopicInfluence[];
	};

	let { data }: PageProps = $props();
	const allQuestions: OrientationQuestion[] = data.questions ?? [];

	const session = orientationQuizSession;


	let filteredQuestions = $derived(() => {
		if (!session.quizType) return [];
		return allQuestions.filter((q) => q.is_part_of.includes(session.quizType));
	});


	function startQuiz(type: 'short' | 'long') {
		session.quizType = type;
		session.currentIndex = 0;
		session.answers = {};
		session.step = 'quiz';
	}

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

	function toggleStrongRef() {
		session.strongRefMode = !session.strongRefMode;
	}

	function setAnswer(val: number) {
		const q = currentQuestion();
		if (!q) return;
		session.answers[q.id] = val;
	}

	const answeredCount = $derived(() => {
		const qs = filteredQuestions();
		return qs.filter((q) => session.answers[q.id] !== undefined && session.answers[q.id] !== null).length;
	});

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
		const result: Record<string, number> = {};
		for (const t in topicAcc) {
			const a = topicAcc[t];
			const pos = a.socialist + a.liberal;
			const neg = a.authoritarian + a.capitalist;
			result[t] = (1.8 * (pos - neg)) / a.count;
		}
		return { result, topicAcc };
	}

	function getTopSimilarDelegates(topN = 10) {
		const { result, topicAcc } = computeUserTopicScores();
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
			return { delegate: d, avgDiff, matches };
		});
		scored.sort((a, b) => a.avgDiff - b.avgDiff);
		return scored.slice(0, topN);
	}
</script>

<svelte:head>
	<title>Politische Orientierung</title>
	<meta name="description" content="Politische Orientierung Fragebogen" />
</svelte:head>

<Container>
	{#if session.step === 'start'}
		<h1 class="mt-2 px-1 pt-2 text-3xl font-bold sm:mt-0 sm:p-0 sm:text-4xl">Politische Orientierung</h1>
		<p class="mt-3">Wähle die Länge des Fragebogens.</p>

		<div class="mt-4 card p-4 text-sm text-gray-700 dark:text-gray-300">
			<p class="mb-2 font-semibold">Hinweis</p>
			<p>
				<span class="font-bold"
					>Dieses Quiz ist rein als Orientierungshilfe gedacht und nicht zur Entscheidung bei Wahlen
					geeignet.
				</span>
				Es zeigt auch, was heute technisch stark automatisiert möglich ist. Der Vergleich mit anderen
				Abgeordneten am Ende des Quiz funktioniert mit dem politischen Analyseprofil der Abgeordneten,
				welches ebenfalls automatisiert generiert wird, auffindbar unter der somes-Abgeordnetenseite.
				Deine Daten bzw. Eingaben werden lediglich lokal am Gerät verarbeitet und im Browser temporär gespeichert.
			</p>
			<p class="mt-2">
				Die Pro/Contra-Positionen von "Hubert" wurden KI-generiert. Die Fragen basieren teilweise
				auf Anträgen aus dem Nationalrat der aktuellen Legislaturperiode. Dazu zählen z.B.
				Regierungsvorlagen sowie Entschließungsanträge bzw. sonstige Anträge der Opposition. Die
				Fragen wurden zu den jeweiligen Anträgen generiert und danach händisch selektiert. Dadurch
				entsteht Bias, u. a. durch den Auswahlprozess und die dominierenden Themen der Opposition
				wie Migration bzw. Klima.
				<span class="font-bold">KI-generierte (sowie menschliche) Inhalte enthalten Fehler.</span>
			</p>
		</div>

		<div class="mt-6 grid gap-4 sm:grid-cols-2">
			<button class="btn card p-6 text-left" onclick={() => startQuiz('short')}>
				<h3 class="text-xl font-semibold">Kurz</h3>
				<p class="mt-2 text-sm">Weniger Fragen, schneller Überblick.</p>
			</button>
			<button class="btn card p-6 text-left" onclick={() => startQuiz('long')}>
				<h3 class="text-xl font-semibold">Lang</h3>
				<p class="mt-2 text-sm">Umfassendere Abdeckung der Themen.</p>
			</button>
		</div>
	{:else if session.step === 'quiz'}
		{@const q = currentQuestion()}
		{@const qs = filteredQuestions()}
		{#if q}
			<h3 class="text-xl font-semibold sm:text-2xl mb-4">Frage {session.currentIndex + 1} von {qs.length}</h3>

			<div class="mt-4 card p-4">
				<div class="flex flex-col gap-8">
					<div class="flex flex-col gap-6 lg:flex-row lg:gap-10">
						<div class="min-w-0 flex-1">
							<p class="mb-3 text-xl font-medium">{q.question}</p>

							{#if q.is_part_of.length}
								<div class="mt-1 flex flex-wrap gap-1">
									{#each q.is_part_of as tag}
										{@const label = tag === 'short' ? 'Kurz' : tag === 'long' ? 'Lang' : tag}
										<span class="badge bg-primary-600 text-white dark:bg-primary-800">{label}</span>
									{/each}
								</div>
							{/if}
						</div>

						{#if q.topics.length}
							<div class="hidden w-full shrink-0 lg:block lg:w-72">
								<h4 class="mb-3 text-sm font-semibold">Themen</h4>
								<Topics topics={q.topics.map((t) => ({ topic: t }))} />
							</div>
						{/if}
					</div>

					<div class="w-full">
						{#if session.strongRefMode}
							{#if q.strong_reference_answers.length >= 2}
								{@const pro = q.strong_reference_answers[0]}
								{@const contra = q.strong_reference_answers[1]}
								<label class="mb-2 block text-sm">Stellungnahme</label>
								<input
									type="range"
									min="0"
									max="100"
									value={session.answers[q.id] ?? 50}
									oninput={(e) => setAnswer(parseInt((e.target as HTMLInputElement).value))}
									class="h-10 w-full touch-manipulation"
									style="width:100%"
								/>
								<div class="mt-2 flex justify-between gap-4">
									<span class="max-w-[45%] text-xs">{pro.answer}</span>
									<span class="max-w-[45%] text-right text-xs">{contra.answer}</span>
								</div>
							{:else}
								<p class="text-sm">Keine starken Referenzen vorhanden.</p>
							{/if}
						{:else}
							<div class="mt-4 grid grid-cols-2 gap-2 sm:grid-cols-5">
								{#each ['Stark dagegen', 'Dagegen', 'Neutral', 'Dafür', 'Stark dafür'] as label, i}
									<button
										class="btn bg-primary-400 {session.answers[q.id] === i * 25
											? 'bg-secondary-500 text-white'
											: ''}"
										onclick={() => setAnswer(i * 25)}
									>
										{label}
									</button>
								{/each}
							</div>
						{/if}
					</div>

					{#if q.topics.length}
						<div class="block lg:hidden">
							<h4 class="mb-3 text-sm font-semibold">Themen</h4>
							<Topics topics={q.topics.map((t) => ({ topic: t }))} />
						</div>
					{/if}

					<div class="flex justify-end gap-3">
						<button
							class="btn"
							onclick={() => {
						session.step = 'start';
						session.quizType = null;
					}}>Neu starten</button
						>
						<button class="btn" onclick={prev} disabled={session.currentIndex === 0}>← Zurück</button>
						<button class="btn" onclick={next}>Weiter →</button>
					</div>

					<div class="flex justify-start">
						<button class="btn" onclick={toggleStrongRef}>
							{session.strongRefMode ? 'Zu 5-Punkte Skala' : 'Zu Slider'}
						</button>
					</div>
				</div>
			</div>
		{/if}
	{:else if session.step === 'result'}
		<h1 class="mt-2 px-1 pt-2 text-3xl font-bold sm:mt-0 sm:p-0 sm:text-4xl">Ergebnis</h1>
		<p class="mt-3">Du hast {answeredCount()} Fragen beantwortet.</p>
		{@const topDelegates = getTopSimilarDelegates(10)}
			<div class="card p-4">
				<h3 class="mb-3 text-xl font-semibold sm:text-2xl">Ähnliche Abgeordnete</h3>
				<p class="mb-4 text-sm">
					Vergleich basierend auf thematischer Übereinstimmung deiner Antworten mit dem politischen
					Analyseprofil.
				</p>
				{#each topDelegates as d (d.delegate.delegate.id)}
					<div class="mb-2 flex items-center justify-between border-b pb-2">
					    <DelegateListItem
							delegate={d.delegate.delegate}
							class="w-full md:w-auto md:max-w-full"
							onclick={async () => {
							    const { date, gp } = getMandateLatestPeriod(d.delegate.delegate, data.gps);
								console.log(gp, date, d.delegate.delegate);
								goto(plink(`/delegates?gp=${gp}&date=${toActualDateString(date)}&delegate=${d.delegate.delegate.id}`))
							}}
						/>
						<span class="text-sm">Ø Abweichung: {d.avgDiff?.toFixed(3)}</span>
					</div>
				{/each}
			</div>
		<div class="mt-6 space-y-4">
			{#each filteredQuestions() as q}
				<div class="card p-4">
					<p class="font-medium">{q.question}</p>
					<p class="mt-1 text-sm">Antwort: {session.answers[q.id] ?? 'keine'}</p>
				</div>
			{/each}
		</div>
		<button
			class="mt-6 btn"
			onclick={() => {
				session.step = 'start';
				session.quizType = null;
			}}>← Neu starten</button
		>
	{/if}
</Container>
